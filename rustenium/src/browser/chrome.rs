use std::sync::{Arc, Mutex};
use rustenium_core::{find_free_port, transport::WebsocketConnectionTransport};
use rustenium_core::session::SessionConnectionType;
use crate::Driver;
use rustenium_core::transport::ConnectionTransportConfig;
use crate::browser::bidi::drivers::Driver as DriverTrait;

pub struct ChromeDriver<'a> {
    connection_transport_config: ConnectionTransportConfig<'a>,
    pub driver: Driver<'a, WebsocketConnectionTransport<'a>>,
}

impl <'a>DriverTrait<'a, WebsocketConnectionTransport<'a>> for ChromeDriver<'a> {
    fn exe_path(&self) -> &str {
        return &self.driver.exe_path;
    }

    fn flags(&self) -> Vec<String> {
        return vec![
            format!("--host={}", self.connection_transport_config.host),
            format!("--port={}", self.connection_transport_config.port),
        ] .into_iter()
            .map(String::from)
            .collect();
    }
}

impl <'a>Default for ChromeDriver<'a> {
    fn default() -> Self {
        return ChromeDriver {
            connection_transport_config: Default::default(),
            driver: Driver {
                exe_path: "google-chrome",
                flags: vec![],
                session: None,
                browsing_contexts: Arc::new(Mutex::new(Vec::new())),
                driver_process: None,
            },
        };
    }
}

impl <'a>ChromeDriver<'a> {
    pub async fn launch(&'a mut self, host: Option<&'a str>, port: Option<u16>) -> () {
        let host = host.unwrap_or("localhost");
        let port = port.unwrap_or(find_free_port().unwrap());
        self.connection_transport_config.host = host;
        self.connection_transport_config.port = port;
        let connection_transport_config = &self.connection_transport_config;
        let result = self.start(connection_transport_config).await;
        self.driver.session = Some(result.0);
        self.driver.driver_process = Some(result.1);
        match self.driver.new_session(SessionConnectionType::WebSocket).await {
            Ok(session) => (),
            Err(e) => panic!("A problem occurred creating the session: {e:?}"),
        }
        self.driver.listen_to_context_creation().await.unwrap();
    }
}
