use crate::bidi::drivers::DriverConfiguration;
use crate::drivers::bidi::drivers::{BidiDrive, BidiDriver};
use crate::error::{EvaluateResultError, FindNodesError, OpenUrlError};
use crate::nodes::chrome::ChromeNode;
use crate::nodes::{Node, NodePosition};
use rustenium_bidi_commands::browsing_context::commands::NavigateResult;
use rustenium_bidi_commands::browsing_context::types::{BrowsingContext, Locator, ReadinessState};
use rustenium_bidi_commands::script::types::{LocalValue, RemoteReference, PrimitiveProtocolValue, RemoteValue, SerializationOptions, SerializationOptionsincludeShadowTreeUnion, SharedReference};
use rustenium_core::session::SessionConnectionType;
use rustenium_core::transport::ConnectionTransportConfig;
use rustenium_core::{find_free_port, transport::WebsocketConnectionTransport};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use rustenium_bidi_commands::{CommandData, ResultData};
use rustenium_core::error::SessionSendError;

#[derive(Debug, Clone)]
pub struct ChromeConfig {
    pub driver_executable_path: String,
    pub host: Option<String>,
    pub port: Option<u16>,
}

impl Default for ChromeConfig {
    fn default() -> Self {
        ChromeConfig {
            driver_executable_path: "".to_string(),
            host: None,
            port: None,
        }
    }
}

impl DriverConfiguration for ChromeConfig {
    fn exe_path(&self) -> &str {
        &self.driver_executable_path
    }

    fn flags(&self) -> Vec<String> {
        vec![
            format!(
                "--host={}",
                self.host.clone().unwrap_or(String::from("localhost"))
            ),
            format!("--port={}", self.port.unwrap_or(find_free_port().unwrap())),
        ]
        .into_iter()
        .map(String::from)
        .collect()
    }
}
pub struct ChromeDriver {
    connection_transport_config: ConnectionTransportConfig,
    pub driver: BidiDriver<WebsocketConnectionTransport>,
}

impl BidiDrive<WebsocketConnectionTransport> for ChromeDriver {}
impl ChromeDriver {
    pub async fn new(mut config: ChromeConfig) -> Self {
        let port = find_free_port().unwrap();
        config.port = Some(config.port.unwrap_or(port));
        let mut ct_config = ConnectionTransportConfig::default();
        ct_config.host = config.host.clone().unwrap_or(String::from("localhost"));
        ct_config.port = port;
        let result = Self::start(&config, &ct_config, SessionConnectionType::WebSocket).await;
        let mut driver = ChromeDriver {
            connection_transport_config: ct_config,
            driver: BidiDriver {
                exe_path: String::from("chromedriver"),
                flags: vec![],
                session: result.0,
                active_bc_index: 0,
                browsing_contexts: Arc::new(Mutex::new(Vec::new())),
                driver_process: result.1,
            },
        };
        driver.driver.listen_to_context_creation().await.unwrap();
        driver
    }

    pub async fn open_url(
        &mut self,
        url: String,
        wait: Option<ReadinessState>,
        context_id: Option<BrowsingContext>,
    ) -> Result<NavigateResult, OpenUrlError> {
        self.driver.open_url(url, wait, context_id).await
    }

    pub async fn find_nodes(
        &mut self,
        locator: Locator,
        context_id: Option<BrowsingContext>,
        max_node_count: Option<u64>,
        serialization_options: Option<SerializationOptions>,
        start_nodes: Option<Vec<SharedReference>>,
    ) -> Result<Vec<ChromeNode>, FindNodesError> {
        let new_so = SerializationOptions {
            max_dom_depth: Some(Some(99)),
            max_object_depth: Some(Some(99)),
            include_shadow_tree: Some(SerializationOptionsincludeShadowTreeUnion::Open),
        };
        let node_result = self
            .driver
            .find_nodes(
                locator.clone(),
                context_id,
                max_node_count,
                Some(new_so),
                start_nodes,
            )
            .await?;
        let mut chrome_nodes = Vec::new();
        for (i, node) in node_result.nodes.iter().enumerate() {
            let chrome_node = ChromeNode::from_bidi(node.clone(), locator.clone());
            chrome_nodes.push(chrome_node);
        }
        Ok(chrome_nodes)
    }

    pub async fn update_node_position_bidi(&mut self, node: &mut impl Node) -> Result<bool, EvaluateResultError> {
        let shared_id = match node.get_shared_id() {
            Some(id) => id.clone(),
            None => return Ok(false),
        };
        let shared_reference = LocalValue::RemoteReference(
            RemoteReference::SharedReference(SharedReference {
                shared_id,
                handle: node.get_handle().clone(),
                extensible: Default::default(),
            }),
        );
        let position = self.driver.get_node_position(shared_reference, node.get_bidi_locator()).await?;
        match position {
            Some(position) => {
                node.set_position(position);
                Ok(true)
            }
            None => Ok(false)
        }
    }

    pub async fn get_node_inner_text_bidi(&mut self, node: &mut impl Node) -> Result<String, EvaluateResultError> {
        let shared_id = match node.get_shared_id() {
            Some(id) => id.clone(),
            None => return Err(EvaluateResultError::NoSharedId),
        };

        let shared_reference = LocalValue::RemoteReference(
            RemoteReference::SharedReference(SharedReference {
                shared_id,
                handle: node.get_handle().clone(),
                extensible: Default::default(),
            }),
        );

        self.driver.get_node_inner_text(shared_reference).await
    }

    pub async fn get_node_text_content_bidi(&mut self, node: &mut impl Node) -> Result<String, EvaluateResultError> {
        let shared_id = match node.get_shared_id() {
            Some(id) => id.clone(),
            None => return Err(EvaluateResultError::NoSharedId),
        };

        let shared_reference = LocalValue::RemoteReference(
            RemoteReference::SharedReference(SharedReference {
                shared_id,
                handle: node.get_handle().clone(),
                extensible: Default::default(),
            }),
        );

        self.driver.get_node_text_content(shared_reference).await
    }

    pub async fn get_node_inner_html_bidi(&mut self, node: &mut impl Node) -> Result<String, EvaluateResultError> {
        let shared_id = match node.get_shared_id() {
            Some(id) => id.clone(),
            None => return Err(EvaluateResultError::NoSharedId),
        };

        let shared_reference = LocalValue::RemoteReference(
            RemoteReference::SharedReference(SharedReference {
                shared_id,
                handle: node.get_handle().clone(),
                extensible: Default::default(),
            }),
        );

        self.driver.get_node_inner_html(shared_reference).await
    }

    pub async fn send_bidi_command(&mut self, command: CommandData) -> Result<ResultData, SessionSendError> {
        return self.driver.send_command(command).await;
    }
}
