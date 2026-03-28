use std::{collections::HashMap, net::TcpListener};
use std::sync::Arc;
use rustenium_bidi_definitions::base::EventResponse;
use rustenium_cdp_definitions::base;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::{oneshot, Mutex};

use crate::listeners::{
    CommandResponseListener, CommandResponseState, EventListener, Listener,
    CdpCommandResponseListener, CdpCommandResponseState, CdpEventListener, CdpListener,
};
use crate::transport::ConnectionTransport;

pub fn find_free_port() -> std::io::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    Ok(port)
}

// ── BiDi Connection ──────────────────────────────────────────────────────────

pub struct BidiConnection<T: ConnectionTransport> {
    transport: T,
    pub commands_response_subscriptions: Arc<Mutex<HashMap<u64, oneshot::Sender<CommandResponseState>>>>,
    event_listener: EventListener,
}

impl<T: ConnectionTransport> BidiConnection<T> {
    pub fn new(connection_transport: T) -> Self {
        Self {
            transport: connection_transport,
            commands_response_subscriptions: Arc::new(Mutex::new(HashMap::new())),
            event_listener: EventListener::new(),
        }
    }

    pub async fn register_event_listener_channel(&mut self, channel: UnboundedSender<EventResponse>) {
        self.event_listener.listeners.lock().await.push(channel);
    }

    pub fn start_listeners(&self) {
        let (listener_tx, listener_rx) = unbounded_channel::<String>();
        let (command_response_tx, command_response_rx) = unbounded_channel::<CommandResponseState>();
        let (event_tx, event_rx) = unbounded_channel::<EventResponse>();

        self.transport.listen(listener_tx);

        let listener = Listener::new(listener_rx, command_response_tx, event_tx);
        listener.start();

        let commands_response_listener = CommandResponseListener::new(command_response_rx, self.commands_response_subscriptions.clone());
        commands_response_listener.start();
        self.event_listener.start(event_rx);
    }

    pub async fn send(&mut self, data: String) {
        self.transport.send(data).await;
    }

    pub async fn close(&self) {
        self.transport.close().await;
    }
}

// ── CDP Connection ───────────────────────────────────────────────────────────

pub struct CdpConnection<T: ConnectionTransport> {
    transport: T,
    pub commands_response_subscriptions: Arc<Mutex<HashMap<u64, oneshot::Sender<CdpCommandResponseState>>>>,
    event_listener: CdpEventListener,
}

impl<T: ConnectionTransport> CdpConnection<T> {
    pub fn new(connection_transport: T) -> Self {
        Self {
            transport: connection_transport,
            commands_response_subscriptions: Arc::new(Mutex::new(HashMap::new())),
            event_listener: CdpEventListener::new(),
        }
    }

    pub async fn register_event_listener_channel(&mut self, channel: UnboundedSender<base::EventResponse>) {
        self.event_listener.listeners.lock().await.push(channel);
    }

    pub fn start_listeners(&self) {
        let (listener_tx, listener_rx) = unbounded_channel::<String>();
        let (command_response_tx, command_response_rx) = unbounded_channel::<CdpCommandResponseState>();
        let (event_tx, event_rx) = unbounded_channel::<base::EventResponse>();

        self.transport.listen(listener_tx);

        let listener = CdpListener::new(listener_rx, command_response_tx, event_tx);
        listener.start();

        let commands_response_listener = CdpCommandResponseListener::new(command_response_rx, self.commands_response_subscriptions.clone());
        commands_response_listener.start();
        self.event_listener.start(event_rx);
    }

    pub async fn send(&mut self, data: String) {
        self.transport.send(data).await;
    }

    pub async fn close(&self) {
        self.transport.close().await;
    }
}
