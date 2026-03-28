use std::sync::Arc;
use tokio::sync::Mutex;
use rustenium_core::transport::WebsocketConnectionTransport;
use crate::conduit::cdp::adapter::CdpAdapter;
use crate::domain::cdp::page::Page;

pub struct ChromePage {
    adapter: Arc<Mutex<CdpAdapter<WebsocketConnectionTransport>>>,
}

impl Page for ChromePage {
    fn adapter(&self) -> Arc<Mutex<CdpAdapter<WebsocketConnectionTransport>>> {
        self.adapter.clone()
    }
}
