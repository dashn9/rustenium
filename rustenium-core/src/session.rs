use crate::error::{CdpSessionSendError, ResponseReceiveTimeoutError, SessionSendError};
use crate::events::{BidiEvent, BidiEventManagement, CdpEvent, CdpEventManagement};
use crate::listeners::{CdpCommandResponseState, CommandResponseState};
use crate::network::NetworkRequestHandledState;
use crate::{
    connection::{BidiConnection, CdpConnection},
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
};
use rand::Rng;
use rustenium_bidi_definitions::Command;
use rustenium_bidi_definitions::base::{CommandMessage, CommandResponse};
use rustenium_bidi_definitions::session::command_builders::{EndBuilder, NewBuilder};
use rustenium_bidi_definitions::session::results::NewResult;
use rustenium_bidi_definitions::session::types::CapabilitiesRequest;
use rustenium_cdp_definitions::Command as CdpCommand;
use rustenium_cdp_definitions::base as cdp_base;
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time::timeout;
use tracing;

pub struct BidiSession<T: ConnectionTransport> {
    id: String,
    connection: BidiConnection<T>,
    events: Arc<Mutex<Vec<BidiEvent>>>,
    /// Tracks network requests that have been handled, keyed by request ID
    pub handled_network_requests: Arc<Mutex<HashMap<String, NetworkRequestHandledState>>>,
}

impl BidiSession<WebsocketConnectionTransport> {
    pub async fn new(
        connection_config: &ConnectionTransportConfig,
        capabilities: CapabilitiesRequest,
    ) -> Self {
        let transport = WebsocketConnectionTransport::new(connection_config)
            .await
            .unwrap();
        tracing::info!(
            "Connected to WebSocket at {}",
            connection_config.full_endpoint()
        );
        let connection = BidiConnection::new(transport);
        connection.start_listeners();

        let mut session = Self {
            id: String::new(),
            connection,
            events: Arc::new(Mutex::new(Vec::new())),
            handled_network_requests: Arc::new(Mutex::new(HashMap::new())),
        };

        let (_, event_tx) = session.event_dispatch().await;
        session
            .connection
            .register_event_listener_channel(event_tx)
            .await;

        let command = NewBuilder::default()
            .capabilities(capabilities)
            .build()
            .unwrap();
        let command_result = session.send(command).await;
        match command_result {
            Ok(command_result) => {
                let result: NewResult = command_result
                    .result
                    .clone()
                    .try_into()
                    .unwrap_or_else(|_| panic!("Invalid command result: {:?}", command_result));
                session.id = result.session_id;
            }
            Err(e) => panic!("Error creating new session: {}", e),
        }

        session
    }
}

impl<T: ConnectionTransport> BidiSession<T> {
    /// Send a command and return the receiver to wait for response.
    /// This allows the caller to release locks before waiting for the response.
    pub async fn send_and_get_receiver(
        &mut self,
        command: impl Into<Command>,
    ) -> oneshot::Receiver<CommandResponseState> {
        let command_id = loop {
            let id = rand::rng().random::<u32>() as u64;
            if !self
                .connection
                .commands_response_subscriptions
                .lock()
                .await
                .contains_key(&id)
            {
                break id;
            }
        };

        let command = CommandMessage {
            id: command_id,
            command_data: command.into(),
            extensible: HashMap::new(),
        };
        let (tx, rx) = oneshot::channel::<CommandResponseState>();
        self.connection
            .commands_response_subscriptions
            .lock()
            .await
            .insert(command_id, tx);
        let raw_message = serde_json::to_string(&command).unwrap();
        tracing::debug!(command_id = %command_id, raw_message = %raw_message, "Sending command");

        self.connection.send(raw_message).await;

        rx
    }

    pub async fn send(
        &mut self,
        command: impl Into<Command>,
    ) -> Result<CommandResponse, SessionSendError> {
        let rx = self.send_and_get_receiver(command).await;
        let response = timeout(Duration::from_secs(5), rx).await;
        match response {
            Ok(Ok(command_result)) => match command_result {
                CommandResponseState::Success(response) => {
                    tracing::debug!(id = response.id, raw_message = %response.result, "Command response success");
                    Ok(response)
                }
                CommandResponseState::Error(err) => {
                    tracing::debug!(id = err.id, stacktrace = err.stacktrace, code = %err.error, "Command response failed");
                    Err(SessionSendError::ErrorResponse(err))
                }
            },
            Ok(Err(err)) => panic!("A recv error occurred: {}", err),
            Err(_) => Err(SessionSendError::ResponseReceiveTimeoutError(
                ResponseReceiveTimeoutError,
            )),
        }
    }

    pub async fn end_session(&mut self) -> Result<CommandResponse, SessionSendError> {
        let result = self.send(EndBuilder.build()).await;
        self.connection.close().await;
        result
    }
}

impl<T: ConnectionTransport> BidiEventManagement for BidiSession<T> {
    async fn send_event(
        &mut self,
        command: impl Into<Command>,
    ) -> Result<CommandResponse, SessionSendError> {
        self.send(command).await
    }

    fn get_events(&mut self) -> &mut Arc<Mutex<Vec<BidiEvent>>> {
        &mut self.events
    }

    fn push_event(&mut self, event: BidiEvent) {
        self.events.lock().unwrap().push(event);
    }
}

// ── CDP Session ──────────────────────────────────────────────────────────────

pub struct CdpSession<T: ConnectionTransport> {
    connection: CdpConnection<T>,
    events: Arc<Mutex<Vec<CdpEvent>>>,
    pub session_id: Option<String>,
}

impl<T: ConnectionTransport> CdpSession<T> {
    pub async fn ws_new(
        config: &ConnectionTransportConfig,
    ) -> CdpSession<WebsocketConnectionTransport> {
        let transport = WebsocketConnectionTransport::new(config).await.unwrap();
        tracing::info!("Successfully connected to Browser CDP");
        let connection = CdpConnection::new(transport);
        connection.start_listeners();
        let events = Arc::new(Mutex::new(Vec::new()));

        let mut session = CdpSession {
            connection,
            events,
            session_id: None,
        };

        let (_, dispatch_tx) = session.event_dispatch().await;
        session
            .connection
            .register_event_listener_channel(dispatch_tx)
            .await;

        session
    }

    pub async fn register_event_listener(
        &mut self,
        tx: tokio::sync::mpsc::UnboundedSender<cdp_base::EventResponse>,
    ) {
        self.connection.register_event_listener_channel(tx).await;
    }

    pub async fn send_and_get_receiver(
        &mut self,
        command: impl Into<CdpCommand>,
    ) -> oneshot::Receiver<CdpCommandResponseState> {
        let session_id = self.session_id.clone();
        self.send_and_get_receiver_for(command, session_id).await
    }

    /// Recursively merge `extension` into `target`. Objects merge key by key;
    /// anything else overwrites. Depth matters: extending a nested field must not
    /// discard its siblings.
    fn merge_json(target: &mut serde_json::Value, extension: &serde_json::Value) {
        match (target, extension) {
            (serde_json::Value::Object(t), serde_json::Value::Object(e)) => {
                for (key, value) in e {
                    match t.get_mut(key) {
                        Some(existing) => Self::merge_json(existing, value),
                        None => {
                            t.insert(key.clone(), value.clone());
                        }
                    }
                }
            }
            (t, e) => *t = e.clone(),
        }
    }

    /// Send addressed at an explicit session rather than this one's.
    ///
    /// A connection to the browser endpoint multiplexes every attached target,
    /// so one `CdpSession` can drive all of them: `None` targets the browser
    /// session (where `Target.*` lives), and a child session id — handed to us by
    /// `Target.attachedToTarget` — targets that page or worker. Responses are
    /// matched by command id, which is unique per connection, so no extra
    /// bookkeeping is needed to demultiplex them.
    pub async fn send_and_get_receiver_for(
        &mut self,
        command: impl Into<CdpCommand>,
        session_id: Option<String>,
    ) -> oneshot::Receiver<CdpCommandResponseState> {
        self.send_and_get_receiver_extended(command, None, session_id)
            .await
    }

    /// As above, with extra fields merged into the command's `params`.
    ///
    /// The generated bindings can only express what the vendored protocol
    /// definition describes, and that lags the browser in both directions: fields
    /// added to newer builds are absent, and fields marked deprecated are dropped
    /// even where the browser still honours them and offers no replacement. This
    /// is the escape hatch for both, without waiting on a regeneration.
    ///
    /// Merging is recursive, so extending one key of a nested object leaves its
    /// siblings intact. The command is still a typed one, so `identifier()` and
    /// anything keyed on it continue to work.
    pub async fn send_and_get_receiver_extended(
        &mut self,
        command: impl Into<CdpCommand>,
        extensions: Option<serde_json::Value>,
        session_id: Option<String>,
    ) -> oneshot::Receiver<CdpCommandResponseState> {
        let command_id = loop {
            let id = rand::rng().random::<u16>();
            if !self
                .connection
                .commands_response_subscriptions
                .lock()
                .await
                .contains_key(&id)
            {
                break id;
            }
        };

        let command: CdpCommand = command.into();
        let msg = cdp_base::CommandMessage {
            id: command_id,
            session_id,
            command_data: command,
        };

        let (tx, rx) = oneshot::channel::<CdpCommandResponseState>();
        self.connection
            .commands_response_subscriptions
            .lock()
            .await
            .insert(command_id, tx);

        let raw = match extensions {
            None => serde_json::to_string(&msg).unwrap(),
            Some(ext) => {
                let mut value = serde_json::to_value(&msg).unwrap();
                if let Some(object) = value.as_object_mut() {
                    let params = object
                        .entry("params")
                        .or_insert_with(|| serde_json::Value::Object(Default::default()));
                    Self::merge_json(params, &ext);
                }
                value.to_string()
            }
        };
        tracing::debug!(command_id = %command_id, raw_message = %raw, "Sending CDP command");
        self.connection.send(raw).await;

        rx
    }

    pub async fn send(
        &mut self,
        command: impl Into<CdpCommand>,
    ) -> Result<cdp_base::CommandResponse, CdpSessionSendError> {
        let session_id = self.session_id.clone();
        self.send_to(command, session_id).await
    }

    /// `send`, addressed at an explicit session. See `send_and_get_receiver_for`.
    pub async fn send_to(
        &mut self,
        command: impl Into<CdpCommand>,
        session_id: Option<String>,
    ) -> Result<cdp_base::CommandResponse, CdpSessionSendError> {
        self.send_extended(command, None, session_id).await
    }

    /// `send_to`, with extra fields merged into `params`.
    /// See `send_and_get_receiver_extended`.
    pub async fn send_extended(
        &mut self,
        command: impl Into<CdpCommand>,
        extensions: Option<serde_json::Value>,
        session_id: Option<String>,
    ) -> Result<cdp_base::CommandResponse, CdpSessionSendError> {
        let rx = self
            .send_and_get_receiver_extended(command, extensions, session_id)
            .await;
        match timeout(Duration::from_secs(20), rx).await {
            Ok(Ok(state)) => match state {
                CdpCommandResponseState::Success(response) => {
                    tracing::debug!(id = response.id, raw_message = %response.result, "CDP command response success");
                    Ok(response)
                }
                CdpCommandResponseState::Error(err) => {
                    tracing::debug!(id = ?err.id, error = %err.error, "CDP command response failed");
                    Err(CdpSessionSendError::ErrorResponse(err))
                }
            },
            Ok(Err(e)) => panic!("CDP recv error: {}", e),
            Err(_) => Err(CdpSessionSendError::ResponseReceiveTimeoutError(
                ResponseReceiveTimeoutError,
            )),
        }
    }

    pub async fn close(&self) {
        self.connection.close().await;
    }
}

impl<T: ConnectionTransport> CdpEventManagement for CdpSession<T> {
    fn get_events(&mut self) -> &mut Arc<Mutex<Vec<CdpEvent>>> {
        &mut self.events
    }

    fn push_event(&mut self, event: CdpEvent) {
        self.events.lock().unwrap().push(event);
    }
}
