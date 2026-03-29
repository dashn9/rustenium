use std::{collections::HashMap, sync::Arc};

use rustenium_bidi_definitions::base::{CommandResponse, ErrorResponse, EventResponse, Message};
use rustenium_cdp_definitions::base as cdp_base;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc::{UnboundedReceiver, UnboundedSender}, oneshot, Mutex};
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct Listener {
    rx: UnboundedReceiver<String>,
    pub command_response_tx: UnboundedSender<CommandResponseState>,
    pub event_tx: UnboundedSender<EventResponse>,
}

impl Listener {
    pub fn new(
        rx: UnboundedReceiver<String>,
        command_response_tx: UnboundedSender<CommandResponseState>,
        event_tx: UnboundedSender<EventResponse>,
    ) -> Self {
        Self {
            rx,
            command_response_tx,
            event_tx,
        }
    }
    pub fn start(mut self) {
        tokio::spawn(async move {
            while let Some(message) = self.rx.recv().await {
                let parsed_message = match serde_json::from_str::<Message>(&message) {
                    Ok(result) => result,
                    Err(_) => continue
                };
                match parsed_message {
                    Message::CommandResponse(command_response) => {
                        self.command_response_tx
                            .send(CommandResponseState::Success(command_response))
                            .expect("Failed to send command response");
                    }
                    Message::Event(event) => {
                        self.event_tx.send(event).expect("Failed to send event to event channel");
                    }
                    Message::ErrorResponse(error_response) => {
                        self.command_response_tx
                            .send(CommandResponseState::Error(error_response))
                            .expect("Failed to send error response");
                    }
                }
            }
        });
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandResponseState {
    Success(CommandResponse),
    Error(ErrorResponse),
}
pub struct CommandResponseListener {
    subscriptions: Arc<Mutex<HashMap<u64, oneshot::Sender<CommandResponseState>>>>,
    rx: UnboundedReceiver<CommandResponseState>,
}

impl CommandResponseListener {
    pub fn new(rx: UnboundedReceiver<CommandResponseState>, subscriptions: Arc<Mutex<HashMap<u64, oneshot::Sender<CommandResponseState>>>>) -> Self {
        Self { rx, subscriptions }
    }
    pub fn start(mut self) {
        tokio::spawn(async move {
            while let Some(command_response) = self.rx.recv().await {
                match command_response {
                    CommandResponseState::Success(command_response) => {
                        let sender = self.subscriptions.lock().await.remove(&command_response.id);
                        if let Some(sender) = sender {
                            if !sender.is_closed() {
                                sender.send(CommandResponseState::Success(command_response)).unwrap();
                            }
                        }
                    }
                    CommandResponseState::Error(error_response) => {
                        let id = error_response.id;
                        if let Some(id) = id {
                            if let Some(sender) = self.subscriptions.lock().await.remove(&id) {
                                if !sender.is_closed() {
                                    sender.send(CommandResponseState::Error(error_response)).unwrap();
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

pub struct EventListener {
    pub listeners: Arc<Mutex<Vec<UnboundedSender<EventResponse>>>>,
}

impl EventListener {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn start(&self, mut rx: UnboundedReceiver<EventResponse>) -> JoinHandle<()>{
        let listeners = self.listeners.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                for listener in listeners.lock().await.iter_mut() {
                    listener.send(event.clone()).expect("Unable to send event to event channel");
                }
            }
        })
    }
}

// ── CDP Listeners ─────────────────────────────────────────────────────────────

#[derive(Debug)]
pub enum CdpCommandResponseState {
    Success(cdp_base::CommandResponse),
    Error(cdp_base::ErrorResponse),
}

pub struct CdpListener {
    rx: UnboundedReceiver<String>,
    pub command_response_tx: UnboundedSender<CdpCommandResponseState>,
    pub event_tx: UnboundedSender<cdp_base::EventResponse>,
}

impl CdpListener {
    pub fn new(
        rx: UnboundedReceiver<String>,
        command_response_tx: UnboundedSender<CdpCommandResponseState>,
        event_tx: UnboundedSender<cdp_base::EventResponse>,
    ) -> Self {
        Self { rx, command_response_tx, event_tx }
    }

    pub fn start(mut self) {
        tokio::spawn(async move {
            while let Some(message) = self.rx.recv().await {
                let parsed = match serde_json::from_str::<cdp_base::Message>(&message) {
                    Ok(r) => r,
                    Err(_) => continue,
                };
                match parsed {
                    cdp_base::Message::CommandResponse(r) => {
                        let _ = self.command_response_tx.send(CdpCommandResponseState::Success(r));
                    }
                    cdp_base::Message::ErrorResponse(e) => {
                        tracing::debug!("[CdpListener]: Error Response: {}", e);
                        let _ = self.command_response_tx.send(CdpCommandResponseState::Error(e));
                    }
                    cdp_base::Message::Event(e) => {
                        let _ = self.event_tx.send(e);
                    }
                }
            }
        });
    }
}

pub struct CdpCommandResponseListener {
    subscriptions: Arc<Mutex<HashMap<u16, oneshot::Sender<CdpCommandResponseState>>>>,
    rx: UnboundedReceiver<CdpCommandResponseState>,
}

impl CdpCommandResponseListener {
    pub fn new(
        rx: UnboundedReceiver<CdpCommandResponseState>,
        subscriptions: Arc<Mutex<HashMap<u16, oneshot::Sender<CdpCommandResponseState>>>>,
    ) -> Self {
        Self { rx, subscriptions }
    }

    pub fn start(mut self) {
        tokio::spawn(async move {
            while let Some(response) = self.rx.recv().await {
                let id = match &response {
                    CdpCommandResponseState::Success(r) => Some(r.id),
                    CdpCommandResponseState::Error(r) => r.id,
                };
                if let Some(id) = id {
                    if let Some(sender) = self.subscriptions.lock().await.remove(&id) {
                        if !sender.is_closed() {
                            let _ = sender.send(response);
                        }
                    }
                }
            }
        });
    }
}

pub struct CdpEventListener {
    pub listeners: Arc<Mutex<Vec<UnboundedSender<cdp_base::EventResponse>>>>,
}

impl CdpEventListener {
    pub fn new() -> Self {
        Self {
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn start(&self, mut rx: UnboundedReceiver<cdp_base::EventResponse>) -> JoinHandle<()> {
        let listeners = self.listeners.clone();
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                for listener in listeners.lock().await.iter() {
                    let _ = listener.send(event.clone());
                }
            }
        })
    }
}