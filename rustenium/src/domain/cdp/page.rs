use std::sync::Arc;
use tokio::sync::Mutex;
use rustenium_core::transport::WebsocketConnectionTransport;
use crate::conduit::cdp::adapter::CdpAdapter;

pub trait Page {
    fn adapter(&self) -> Arc<Mutex<CdpAdapter<WebsocketConnectionTransport>>>;
}
