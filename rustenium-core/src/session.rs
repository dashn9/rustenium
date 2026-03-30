use crate::error::{CdpSessionSendError, ResponseReceiveTimeoutError, SessionSendError};
use crate::events::{BidiEvent, BidiEventManagement, CdpEvent, CdpEventManagement};
use crate::listeners::{
    CdpCommandResponseState,
    CommandResponseState,
};
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
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::oneshot;
use tokio::time::timeout;
use tracing;

pub struct BidiSession<T: ConnectionTransport> {
    id: String,
    connection: BidiConnection<T>,
    events: Arc<Mutex<Vec<BidiEvent>>>,
    /// Tracks network requests that have been handled, keyed by request ID
    pub handled_network_requests: Arc<Mutex<HashMap<String, NetworkRequestHandledState>>>,
    /// If the session was created via HTTP-first flow, stores the driver address for DELETE /session
    http_driver_addr: Option<String>,
}

pub enum SessionConnectionType {
    /// Direct WebSocket BiDi (chromedriver) — connect WS, send session.new over BiDi.
    WebSocket,
    /// HTTP-first BiDi (geckodriver) — POST /session via HTTP to get webSocketUrl,
    /// then connect WS to that URL.
    HttpFirst { host: String, port: u16 },
}

impl BidiSession<WebsocketConnectionTransport> {
    pub async fn new(
        connection_config: &ConnectionTransportConfig,
        connection_type: SessionConnectionType,
        capabilities: CapabilitiesRequest,
    ) -> Self {
        match connection_type {
            SessionConnectionType::WebSocket => {
                let mut session = Self::connect_ws(connection_config).await;

                let (_, event_tx) = session.event_dispatch().await;
                session.connection.register_event_listener_channel(event_tx).await;

                let command = NewBuilder::default()
                    .capabilities(capabilities)
                    .build()
                    .unwrap();
                let command_result = session.send(command).await;
                match command_result {
                    Ok(command_result) => {
                        let result: NewResult = command_result.result.clone().try_into().expect(
                            format!("Invalid command result: {:?}", command_result).as_str(),
                        );
                        session.id = result.session_id;
                    }
                    Err(e) => panic!("Error creating new session: {}", e),
                }

                session
            }
            SessionConnectionType::HttpFirst { host, port } => {
                let (ws_url, session_id) =
                    Self::post_session(&host, port, &capabilities).await
                        .expect("Failed to create HTTP-first BiDi session");

                let ws_config = ConnectionTransportConfig::from_ws_url(&ws_url)
                    .expect("Failed to parse webSocketUrl from driver response");

                let mut session = Self::connect_ws(&ws_config).await;
                session.id = session_id;
                session.http_driver_addr = Some(format!("{host}:{port}"));

                let (_, event_tx) = session.event_dispatch().await;
                session.connection.register_event_listener_channel(event_tx).await;

                session
            }
        }
    }

    fn connect_ws(
        config: &ConnectionTransportConfig,
    ) -> impl std::future::Future<Output = Self> + '_ {
        async move {
            let transport = WebsocketConnectionTransport::new(config).await.unwrap();
            tracing::info!("Connected to WebSocket at {}", config.full_endpoint());
            let connection = BidiConnection::new(transport);
            connection.start_listeners();
            Self {
                id: String::new(),
                connection,
                events: Arc::new(Mutex::new(Vec::new())),
                handled_network_requests: Arc::new(Mutex::new(HashMap::new())),
                http_driver_addr: None,
            }
        }
    }

    /// POST /session to the driver's HTTP endpoint.
    /// Returns (ws_url, session_id) — the WebSocket URL is built from the session ID.
    async fn post_session(
        host: &str,
        port: u16,
        capabilities: &CapabilitiesRequest,
    ) -> Result<(String, String), String> {
        let addr = format!("{host}:{port}");
        let body = serde_json::json!({ "capabilities": capabilities });
        let body_str = serde_json::to_string(&body).unwrap();
        tracing::debug!("[BidiSession]: POST /session to {addr} with body: {body_str}");

        let mut retries = 5;
        let stream = loop {
            match TcpStream::connect(&addr).await {
                Ok(s) => break s,
                Err(e) if retries > 0 => {
                    retries -= 1;
                    tracing::debug!("Driver not ready at {addr}, retrying... ({retries} left)");
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
                Err(e) => return Err(format!("connect to driver at {addr}: {e}")),
            }
        };

        let (reader, mut writer) = stream.into_split();
        let request = format!(
            "POST /session HTTP/1.1\r\n\
             Host: {addr}\r\n\
             Content-Type: application/json\r\n\
             Content-Length: {}\r\n\
             Connection: close\r\n\
             \r\n\
             {body_str}",
            body_str.len()
        );
        writer.write_all(request.as_bytes()).await
            .map_err(|e| format!("write request: {e}"))?;

        let mut buf_reader = BufReader::new(reader);
        let mut content_length: Option<usize> = None;
        loop {
            let mut line = String::new();
            buf_reader.read_line(&mut line).await
                .map_err(|e| format!("read header: {e}"))?;
            if line == "\r\n" || line.is_empty() {
                break;
            }
            let lower = line.to_lowercase();
            if let Some(val) = lower.strip_prefix("content-length:") {
                content_length = val.trim().parse().ok();
            }
        }

        let response_body = if let Some(len) = content_length {
            let mut buf = vec![0u8; len];
            tokio::io::AsyncReadExt::read_exact(&mut buf_reader, &mut buf).await
                .map_err(|e| format!("read body: {e}"))?;
            String::from_utf8(buf).map_err(|e| format!("invalid utf8: {e}"))?
        } else {
            let mut buf = String::new();
            tokio::io::AsyncReadExt::read_to_string(&mut buf_reader, &mut buf).await
                .map_err(|e| format!("read body: {e}"))?;
            buf
        };

        tracing::debug!("Driver HTTP session response: {response_body}");

        let json: serde_json::Value = serde_json::from_str(&response_body)
            .map_err(|e| format!("parse driver response: {e}"))?;

        let session_id = json["value"]["sessionId"]
            .as_str()
            .map(|s| s.to_string())
            .unwrap_or_default();

        let ws_url = json["value"]["capabilities"]["webSocketUrl"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| format!(
                "webSocketUrl not found in driver response (ensure webSocketUrl: true is in capabilities): {response_body}"
            ))?;

        Ok((ws_url, session_id))
    }

}

/// DELETE /session/{id} to end an HTTP-first session via WebDriver classic.
async fn delete_session_http(addr: &str, session_id: &str) -> Result<(), String> {
    let stream = TcpStream::connect(addr).await
        .map_err(|e| format!("connect to driver at {addr}: {e}"))?;

    let (reader, mut writer) = stream.into_split();
    let request = format!(
        "DELETE /session/{session_id} HTTP/1.1\r\n\
         Host: {addr}\r\n\
         Connection: close\r\n\
         \r\n"
    );
    writer.write_all(request.as_bytes()).await
        .map_err(|e| format!("write DELETE request: {e}"))?;

    let mut buf_reader = BufReader::new(reader);
    let mut buf = String::new();
    let _ = tokio::io::AsyncReadExt::read_to_string(&mut buf_reader, &mut buf).await;
    tracing::debug!("[BidiSession]: DELETE /session response: {buf}");

    Ok(())
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
        if let Some(ref addr) = self.http_driver_addr {
            // Session was created via HTTP (WebDriver classic) — must use DELETE /session/{id}
            tracing::debug!("[BidiSession]: DELETE /session/{} to {}", self.id, addr);
            let _ = delete_session_http(addr, &self.id).await;
            self.connection.close().await;
            return Ok(CommandResponse {
                r#type: rustenium_bidi_definitions::base::SuccessEnum::Success,
                id: 0,
                result: serde_json::Value::Object(serde_json::Map::new()),
                extensible: HashMap::new(),
            });
        }

        let result = self.send(EndBuilder::default().build()).await;
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
        let command_id = loop {
            let id = rand::rng().random::<u16>();
            if !self.connection.commands_response_subscriptions.lock().await.contains_key(&id) {
                break id;
            }
        };

        let command: CdpCommand = command.into();
        let msg = cdp_base::CommandMessage {
            id: command_id,
            command_data: command.into(),
        };

        let (tx, rx) = oneshot::channel::<CdpCommandResponseState>();
        self.connection.commands_response_subscriptions.lock().await.insert(command_id, tx);

        let raw = serde_json::to_string(&msg).unwrap();
        tracing::debug!(command_id = %command_id, raw_message = %raw, "Sending CDP command");
        self.connection.send(raw).await;

        rx
    }

    pub async fn send(
        &mut self,
        command: impl Into<CdpCommand>,
    ) -> Result<cdp_base::CommandResponse, CdpSessionSendError> {
        let rx = self.send_and_get_receiver(command).await;
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
