use crate::error::{ResponseReceiveTimeoutError, SessionSendError};
use crate::events::{BidiEvent, BidiEventManagement};
use crate::listeners::CommandResponseState;
use crate::network::NetworkRequestHandledState;
use crate::{
    connection::Connection,
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
};
use rand::Rng;
use rustenium_bidi_definitions::Command;
use rustenium_bidi_definitions::base::{CommandMessage, CommandResponse};
use rustenium_bidi_definitions::session::command_builders::{EndBuilder, NewBuilder};
use rustenium_bidi_definitions::session::results::NewResult;
use rustenium_bidi_definitions::session::types::CapabilitiesRequest;
use serde_json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::oneshot;
use tokio::time::timeout;
use tracing;

pub struct BidiSession<T: ConnectionTransport> {
    id: Option<String>,
    connection: Connection<T>,
    events: Arc<Mutex<Vec<BidiEvent>>>,
    /// Tracks network requests that have been handled, keyed by request ID
    pub handled_network_requests: Arc<Mutex<HashMap<String, NetworkRequestHandledState>>>,
}

pub enum SessionConnectionType {
    WebSocket,
}

impl<T: ConnectionTransport> BidiSession<T> {
    pub async fn ws_new(
        connection_config: &ConnectionTransportConfig,
    ) -> BidiSession<WebsocketConnectionTransport> {
        let connection_transport = WebsocketConnectionTransport::new(connection_config)
            .await
            .unwrap();
        let connection = Connection::new(connection_transport);
        connection.start_listeners();
        BidiSession {
            id: None,
            connection,
            events: Arc::new(Mutex::new(Vec::new())),
            handled_network_requests: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_new_bidi_session(
        &mut self,
        connection_type: SessionConnectionType,
        capabilities: CapabilitiesRequest,
    ) -> () {
        match connection_type {
            SessionConnectionType::WebSocket => {
                let command = NewBuilder::default()
                    .capabilities(capabilities)
                    .build()
                    .unwrap();
                let (_, event_tx) = self.event_dispatch().await;
                self.connection
                    .register_event_listener_channel(event_tx)
                    .await;
                let command_result = self.send(command).await;
                match command_result {
                    Ok(command_result) => {
                        let command_result: NewResult = command_result.result.clone().try_into().expect(
                            format!("Invalid command result: {:?}", command_result).as_str(),
                        );
                        self.id = Option::from(command_result.session_id);
                    }
                    Err(e) => panic!("Error creating new session: {}", e),
                }
            }
        }
    }

    /// Send a command and return the receiver to wait for response
    /// This allows the caller to release locks before waiting for the response
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
        match timeout(Duration::from_secs(100), rx).await {
            Ok(Ok(command_result)) => match command_result {
                CommandResponseState::Success(response) => Ok(response),
                CommandResponseState::Error(err) => Err(SessionSendError::ErrorResponse(err)),
            },
            Ok(Err(err)) => panic!("A recv error occurred: {}", err),
            // I might need to remove command from commands response subscriptions
            Err(_) => Err(SessionSendError::ResponseReceiveTimeoutError(
                ResponseReceiveTimeoutError,
            )),
        }
    }

    pub async fn end_session(&mut self) -> Result<CommandResponse, SessionSendError> {
        let result = self.send(EndBuilder::default().build()).await;

        // Close the connection after ending the session
        self.connection.close();

        result
    }
}

impl<T: ConnectionTransport> BidiEventManagement for BidiSession<T> {
    async fn send_event(&mut self, command: impl Into<Command>) -> Result<CommandResponse, SessionSendError> {
        self.send(command).await
    }

    fn get_events(&mut self) -> &mut Arc<Mutex<Vec<BidiEvent>>> {
        &mut self.events
    }

    fn push_event(&mut self, event: BidiEvent) {
        self.events.lock().unwrap().push(event);
    }
}
