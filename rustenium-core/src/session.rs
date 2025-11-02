use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration};
use rand::Rng;
use rustenium_bidi_commands::{Command, CommandData, CommandResponse, ErrorResponse, ResultData};
use rustenium_bidi_commands::session::commands::{New as SessionNew, SessionNewMethod, NewParameters as SessionNewParameters, SessionCommand, SessionResult};
use rustenium_bidi_commands::session::types::CapabilitiesRequest;
use tokio::sync::oneshot;
use tokio::time::timeout;
use crate::listeners::CommandResponseState;
use crate::{
    connection::Connection,
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
};
use crate::contexts::BrowsingContext;
use crate::error::{ResponseReceiveTimeoutError, SessionSendError};
use crate::events::{BidiEvent, EventManagement};

pub struct Session<T: ConnectionTransport> {
    id: Option<String>,
    connection: Connection<T>,
    bidi_events: Arc<Mutex<Vec<BidiEvent>>>,
}

pub enum SessionConnectionType {
    WebSocket
}

impl<T: ConnectionTransport> Session<T> {
    pub async fn ws_new(
        connection_config: &ConnectionTransportConfig,
    ) -> Session<WebsocketConnectionTransport> {
        let connection_transport = WebsocketConnectionTransport::new(connection_config)
            .await
            .unwrap();
        let connection = Connection::new(connection_transport);
        connection.start_listeners();
        Session { id: None, connection, bidi_events: Arc::new(Mutex::new(Vec::new())) }
    }

    pub async fn create_new_bidi_session(&mut self, connection_type: SessionConnectionType, capabilities: Option<CapabilitiesRequest>) -> () {
        match connection_type {
            SessionConnectionType::WebSocket => {
                let command = SessionNew {
                    method: SessionNewMethod::SessionNew,
                    params: SessionNewParameters {
                        capabilities: capabilities.unwrap_or(CapabilitiesRequest {
                            always_match: None,
                            first_match: None,
                        }),
                    }
                };
                let (_, event_tx) = self.event_dispatch().await;
                self.connection.register_event_listener_channel(event_tx).await;
                let command_result = self.send(CommandData::SessionCommand(SessionCommand::New(command.clone()))).await;
                match command_result {
                    Ok(command_result) => {
                            match command_result {
                            ResultData::SessionResult(session_result) => {
                                match session_result {
                                    SessionResult::NewResult(new_session_result) => {
                                        self.id = Some(new_session_result.session_id);
                                    }
                                    _ => panic!("Invalid session result: {:?}", session_result)
                                }
                            }
                            _ => panic!("Invalid command result: {:?}", command_result)
                        }
                    }
                    Err(e) => panic!("Error creating new session: {}", e)
                }
            }
        }
    }

    pub async fn send(&mut self, command_data: CommandData) -> Result<ResultData, SessionSendError>  {
        let command_id = loop {
            let id = rand::rng().random::<u32>() as u64;
            if !self.connection.commands_response_subscriptions.lock().await.contains_key(&id) {
                break id;
            }
        };

        let command = Command {
            id : command_id,
            command_data,
            extensible: HashMap::new(),
        };
        let (tx, rx) = oneshot::channel::<CommandResponseState>();
        self.connection.commands_response_subscriptions.lock().await.insert(command_id, tx);
        let raw_message = serde_json::to_string(&command).unwrap();
        self.connection.send(raw_message).await;

        match timeout(Duration::from_secs(100), rx).await {
            Ok(Ok(command_result)) => match command_result {
                CommandResponseState::Success(response) => Ok(response.result),
                CommandResponseState::Error(err) => Err(SessionSendError::ErrorResponse(err))
            }
            Ok(Err(err)) => panic!("A recv error occurred: {}", err),
            // I might need to remove command from commands response subscriptions
            Err(_) => Err(SessionSendError::ResponseReceiveTimeoutError(ResponseReceiveTimeoutError))
        }
    }
}

impl <T: ConnectionTransport>EventManagement for Session<T> {
    async fn send_event(&mut self, command_data: CommandData) -> Result<ResultData, SessionSendError> {
        self.send(command_data).await
    }

    fn get_bidi_events(&mut self) -> &mut Arc<Mutex<Vec<BidiEvent>>> {
        &mut self.bidi_events
    }

    fn push_event(&mut self, event: BidiEvent) {
        self.bidi_events.lock().unwrap().push(event);
    }
}