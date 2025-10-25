use std::sync::{Arc, Mutex};
use tokio::time::{sleep, Duration};
use rustenium_core::{find_free_port, transport::WebsocketConnectionTransport};
use rustenium_core::contexts::BrowsingContext;
use rustenium_core::events::EventManagement;
use rustenium_core::session::SessionConnectionType;
use rustenium_core::transport::ConnectionTransportConfig;
use crate::bidi::drivers::DriverConfiguration;
use crate::drivers::bidi::drivers::{BidiDrive, BidiDriver};


pub struct ChromeConfig {
    pub driver_executable_path: String,
    pub host: Option<String>,
    pub port: Option<u16>,
}

impl DriverConfiguration for ChromeConfig {
    fn exe_path(&self) -> &str {
        &self.driver_executable_path
    }

    fn flags(&self) -> Vec<String> {
        return vec![
            format!("--host={}", self.host.unwrap_or(String::from("localhost"))),
            format!("--port={}", self.connection_transport_config.port),
        ].into_iter()
            .map(String::from)
            .collect();
    }
}
pub struct ChromeDriver {
    connection_transport_config: ConnectionTransportConfig,
    pub driver: BidiDriver<WebsocketConnectionTransport>,
}

impl ChromeDriver {
    pub async fn new(config: ChromeConfig) -> Self {
        let result = self.start(&config).await;
        self.driver.session = Some(result.0);
        self.driver.driver_process = Some(result.1);
        match self.driver.new_session(SessionConnectionType::WebSocket).await {
            Ok(session) => (),
            Err(e) => panic!("A problem occurred creating the session: {e:?}"),
        }
        self.driver.listen_to_context_creation().await.unwrap();
        ChromeDriver {
            connection_transport_config: Default::default(),
            driver: BidiDriver {
                exe_path: String::from("chromedriver"),
                flags: vec![],
                session: None,
                active_bc_index: 0,
                browsing_contexts: Arc::new(Mutex::new(Vec::new())),
                driver_process: None,
            },
        }
    }

    pub async fn open_url()
}
