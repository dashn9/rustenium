use std::{collections::HashMap, net::TcpListener};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use rustenium_bidi_commands::{CommandResponse, Event};
use tokio::sync::{oneshot, Mutex};

use crate::{listeners::CommandResponseState, transport::ConnectionTransport};
use crate::listeners::{CommandResponseListener, EventListener, Listener};
use crate::transport::WebsocketConnectionTransport;

pub struct Connection<T: ConnectionTransport> {
    transport: T,
    pub commands_response_subscriptions: Arc<Mutex<HashMap<u64, oneshot::Sender<CommandResponseState>>>>,
    event_listener: EventListener,
}

pub fn find_free_port() -> std::io::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    Ok(port)
}

impl<T> Connection<T>
where
    T: ConnectionTransport,
{
    pub fn new(connection_transport: T) -> Self {
        Self {
            transport: connection_transport,
            commands_response_subscriptions: Arc::new(Mutex::new(HashMap::new())),
            event_listener: EventListener::new(),
        }
    }

    pub async fn register_event_listener_channel(&mut self, channel: UnboundedSender<Event>) {
        self.event_listener.listeners.lock().await.push(channel);
    }

    pub fn start_listeners(&self) -> () {
        let (listener_tx, listener_rx) = unbounded_channel::<String>();
        let (command_response_tx, command_response_rx) = unbounded_channel::<CommandResponseState>();
        let (event_tx, event_rx) = unbounded_channel::<Event>();

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

    pub fn close(&self) {
        self.transport.close();
    }
}
