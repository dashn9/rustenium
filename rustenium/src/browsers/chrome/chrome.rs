use rustenium_bidi_commands::browsing_context::commands::{LocateNodes, LocateNodesResult, NavigateResult};
use rustenium_bidi_commands::browsing_context::types::{BrowsingContext, Locator, ReadinessState};
use rustenium_bidi_commands::{CommandData, ResultData, Event, EventData, NetworkEvent};
use rustenium_bidi_commands::script::types::{
    ChannelValue, LocalValue, PrimitiveProtocolValue, RemoteReference, RemoteValue,
    SerializationOptions, SerializationOptionsincludeShadowTreeUnion, SharedReference
};
use rustenium_bidi_commands::session::types::CapabilitiesRequest;
use rustenium_bidi_commands::network::commands::{AddIntercept, NetworkAddInterceptMethod, AddInterceptParameters};
use rustenium_bidi_commands::network::types::{InterceptPhase, UrlPattern};
use rustenium_core::error::{CommandResultError, SessionSendError};
use rustenium_core::transport::{ConnectionTransportConfig, WebsocketConnectionTransport};
use rustenium_core::{find_free_port, Context, NetworkRequest};
use rustenium_core::events::EventManagement;
use rustenium_core::session::SessionConnectionType;
use crate::drivers::bidi::drivers::{BidiDriver, BidiDrive, DriverConfiguration};
use crate::error::{EvaluateResultError, FindNodesError, OpenUrlError, InterceptNetworkError};
use crate::nodes::chrome::ChromeNode;
use crate::nodes::{Node, NodePosition};
use super::capabilities::ChromeCapabilities;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::future::Future;

#[derive(Debug, Clone)]
pub struct ChromeConfig {
    pub driver_executable_path: String,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub driver_flags: Vec<&'static str>,
    pub capabilities: ChromeCapabilities,
    pub sandbox: bool,
}

impl Default for ChromeConfig {
    fn default() -> Self {
        ChromeConfig {
            driver_executable_path: "".to_string(),
            host: None,
            port: None,
            driver_flags: Vec::new(),
            capabilities: ChromeCapabilities::default(),
            sandbox: false,
        }
    }
}

impl DriverConfiguration for ChromeConfig {
    fn exe_path(&self) -> &str {
        &self.driver_executable_path
    }

    fn flags(&self) -> Vec<String> {
        let mut flags = vec![
            format!(
                "--host={}",
                self.host.clone().unwrap_or(String::from("localhost"))
            ),
            format!("--port={}", self.port.unwrap_or(find_free_port().unwrap())),
        ];

        // Convert &'static str driver_flags to String and append
        flags.extend(self.driver_flags.iter().map(|s| s.to_string()));

        flags
    }
}

pub struct ChromeBrowser {
    config: ChromeConfig,
    driver: BidiDriver<WebsocketConnectionTransport>,
}

impl BidiDrive<WebsocketConnectionTransport> for ChromeBrowser {}

impl ChromeBrowser {
    pub async fn new(mut config: ChromeConfig) -> ChromeBrowser {
        let port = find_free_port().unwrap();
        config.port = Some(config.port.unwrap_or(port));
        let mut ct_config = ConnectionTransportConfig::default();
        ct_config.host = config.host.clone().unwrap_or(String::from("localhost"));
        ct_config.port = port;

        config.capabilities.add_arg("start-maximized".to_string());
        config.capabilities.add_arg("disable-infobars".to_string());

        // Add --no-sandbox arg to Chrome options if sandbox is disabled
        if !config.sandbox {
            config.capabilities.add_arg("no-sandbox".to_string());
        }

        // Convert ChromeCapabilities to CapabilitiesRequest
        let capabilities = Some(config.capabilities.clone().build());

        let result = Self::start(&config, &ct_config, SessionConnectionType::WebSocket, capabilities).await;

        let session = result.0;

        let mut browser = ChromeBrowser {
            config,
            driver: BidiDriver::new(
                String::from("chromedriver"),
                vec![],
                session,
                0,
                Arc::new(Mutex::new(Vec::new())),
                result.1,
            ),
        };
        browser.driver.listen_to_context_creation().await.unwrap();
        browser
    }

    pub async fn open_url(
        &mut self,
        url: &str,
        wait: Option<ReadinessState>,
        context_id: Option<BrowsingContext>,
    ) -> Result<NavigateResult, OpenUrlError> {
        self.driver.open_url(url.to_string(), wait, context_id).await
    }

    /// Create a new browsing context (tab or window)
    pub async fn create_context_bidi(
        &mut self,
        context_type: Option<rustenium_bidi_commands::browsing_context::types::CreateType>,
        reference_context: Option<&rustenium_core::Context>,
        background: bool,
    ) -> Result<rustenium_core::Context, rustenium_core::error::CommandResultError> {
        self.driver.create_context(context_type, reference_context, background).await
    }

    pub async fn find_nodes(
        &mut self,
        locator: Locator,
        context_id: Option<BrowsingContext>,
        max_node_count: Option<u64>,
        serialization_options: Option<SerializationOptions>,
        start_nodes: Option<Vec<SharedReference>>,
    ) -> Result<Vec<ChromeNode<WebsocketConnectionTransport>>, FindNodesError> {
        let serialization_options = serialization_options.unwrap_or(SerializationOptions {
            max_dom_depth: Some(Some(99)),
            max_object_depth: Some(Some(99)),
            include_shadow_tree: Some(SerializationOptionsincludeShadowTreeUnion::Open),
        });
        let context = context_id.clone().unwrap_or_else(|| self.driver.get_active_context_id().unwrap());
        let node_result = self
            .driver
            .find_nodes(
                locator.clone(),
                context_id,
                max_node_count,
                Some(serialization_options),
                start_nodes,
            )
            .await?;
        let mut chrome_nodes = Vec::new();
        for (i, node) in node_result.nodes.iter().enumerate() {
            let chrome_node = ChromeNode::from_bidi(
                node.clone(),
                locator.clone(),
                self.driver.session.clone(),
                context.clone(),
            );
            chrome_nodes.push(chrome_node);
        }
        Ok(chrome_nodes)
    }

    pub async fn find_node(
        &mut self,
        locator: Locator,
        context_id: Option<BrowsingContext>,
        max_node_count: Option<u64>,
        serialization_options: Option<SerializationOptions>,
        start_nodes: Option<Vec<SharedReference>>,
    ) -> Result<Option<ChromeNode<WebsocketConnectionTransport>>, FindNodesError> {
        let nodes = self.find_nodes(locator, context_id, max_node_count, serialization_options, start_nodes).await?;
        Ok(nodes.into_iter().next())
    }

    /// Wait for nodes to appear with a timeout (in milliseconds)
    pub async fn wait_for_nodes(
        &mut self,
        locator: Locator,
        context_id: Option<BrowsingContext>,
        timeout_ms: Option<u64>,
        poll_interval_ms: Option<u64>,
    ) -> Result<Vec<ChromeNode<WebsocketConnectionTransport>>, FindNodesError> {
        let timeout = timeout_ms.unwrap_or(4000);
        let poll_interval = poll_interval_ms.unwrap_or(timeout / 6);
        let start = std::time::Instant::now();

        loop {
            let nodes = self.find_nodes(
                locator.clone(),
                context_id.clone(),
                None,
                None,
                None,
            ).await?;

            if !nodes.is_empty() {
                return Ok(nodes);
            }

            if start.elapsed().as_millis() as u64 >= timeout {
                return Ok(Vec::new());
            }

            tokio::time::sleep(tokio::time::Duration::from_millis(poll_interval)).await;
        }
    }

    /// Wait for a single node to appear with a timeout (in milliseconds)
    pub async fn wait_for_node(
        &mut self,
        locator: Locator,
        context_id: Option<BrowsingContext>,
        timeout_ms: Option<u64>,
        poll_interval_ms: Option<u64>,
    ) -> Result<Option<ChromeNode<WebsocketConnectionTransport>>, FindNodesError> {
        let nodes = self.wait_for_nodes(locator, context_id, timeout_ms, poll_interval_ms).await?;
        Ok(nodes.into_iter().next())
    }

    pub async fn send_bidi_command(&mut self, command: CommandData) -> Result<ResultData, SessionSendError> {
        return self.driver.send_command(command).await;
    }

    /// Register a handler to be called for each network request
    ///
    /// # Example
    /// ```ignore
    /// browser.on_request_bidi(|request| async move {
    ///     if request.params.base_parameters.request.url.contains("ads") {
    ///         let _ = request.abort().await;
    ///     } else {
    ///         let _ = request.continue_().await;
    ///     }
    /// }, None, None).await?;
    /// ```
    pub async fn on_request_bidi<F, Fut>(
        &mut self,
        handler: F,
        url_patterns: Option<Vec<UrlPattern>>,
        contexts: Option<Vec<String>>,
    ) -> Result<(), InterceptNetworkError>
    where
        F: Fn(NetworkRequest<WebsocketConnectionTransport>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        // Use active context if no contexts provided
        let contexts = match contexts {
            Some(ctxs) => Some(ctxs),
            None => Some(vec![self.driver.get_active_context_id()?]),
        };

        self.driver.on_request(handler, url_patterns, contexts).await
    }

    pub async fn subscribe_events<F, R>(
        &mut self,
        events: HashSet<&str>,
        handler: F,
        browsing_contexts: Option<Vec<String>>,
        user_contexts: Option<Vec<&str>>,
    ) -> Result<Option<rustenium_bidi_commands::session::commands::SubscribeResult>, CommandResultError>
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        self.driver.session.lock().await.subscribe_events(
            events,
            handler,
            browsing_contexts,
            user_contexts,
        ).await
    }

    /// Add an event handler without sending a subscription command
    /// Returns the handler ID (either provided or generated)
    pub async fn add_event_handler<F, R>(
        &mut self,
        events: HashSet<&str>,
        handler: F,
        handler_id: Option<String>,
    ) -> String
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        self.driver.add_event_handler(events, handler, handler_id).await
    }

    /// Evaluate a JavaScript expression in the browser context using BiDi
    ///
    /// # Example
    /// ```ignore
    /// let result = browser.evaluate_script_bidi(
    ///     "document.title".to_string(),
    ///     false,
    ///     None,
    ///     None,
    ///     None,
    ///     None,
    /// ).await?;
    /// ```
    pub async fn evaluate_script_bidi(
        &mut self,
        expression: String,
        await_promise: bool,
        target: Option<rustenium_bidi_commands::script::types::Target>,
        result_ownership: Option<rustenium_bidi_commands::script::types::ResultOwnership>,
        serialization_options: Option<SerializationOptions>,
        user_activation: Option<bool>,
    ) -> Result<rustenium_bidi_commands::script::types::EvaluateResultSuccess, EvaluateResultError> {
        self.driver.evaluate_script(
            expression,
            await_promise,
            target,
            result_ownership,
            serialization_options,
            user_activation,
        ).await
    }

    /// Get a reference to the BiDi mouse
    pub fn mouse(&self) -> &crate::input::BidiMouse<WebsocketConnectionTransport> {
        &self.driver.mouse
    }

    /// Get a reference to the Human mouse
    pub fn human_mouse(&self) -> &crate::input::HumanMouse<crate::input::BidiMouse<WebsocketConnectionTransport>> {
        &self.driver.human_mouse
    }

    /// Get a reference to the BiDi keyboard
    pub fn keyboard(&self) -> &crate::input::Keyboard<WebsocketConnectionTransport> {
        &self.driver.keyboard
    }

    /// Move mouse to the center of a node
    pub async fn move_mouse_to_node_bidi(
        &mut self,
        node: &mut ChromeNode,
        context: Option<&BrowsingContext>,
        scroll_into_view: bool,
    ) -> Result<(), crate::error::MouseInputError> {
        self.driver.move_mouse_to_node(node, context, scroll_into_view).await
    }

    /// Click on a node (scrolls into view and moves mouse first)
    pub async fn click_on_node_bidi(
        &mut self,
        node: &mut ChromeNode,
        context: Option<&BrowsingContext>,
        options: Option<crate::input::MouseClickOptions>,
    ) -> Result<(), crate::error::MouseInputError> {
        self.driver.click_on_node(node, context, options).await
    }

    /// Add a preload script that will be executed in new contexts
    pub async fn add_preload_script_bidi(
        &mut self,
        function_declaration: String,
        arguments: Option<Vec<ChannelValue>>,
        contexts: Option<Vec<String>>,
        user_contexts: Option<Vec<String>>,
        sandbox: Option<String>,
    ) -> Result<String, EvaluateResultError> {
        self.driver
            .add_preload_script(function_declaration, arguments, contexts, user_contexts, sandbox)
            .await
    }

    /// Remove a preload script by its ID
    pub async fn remove_preload_script_bidi(
        &mut self,
        script: String,
    ) -> Result<(), EvaluateResultError> {
        self.driver.remove_preload_script(script).await
    }

}


pub async fn create_chrome_browser(config: Option<ChromeConfig>) -> ChromeBrowser {
    let chrome_browser = ChromeBrowser::new(config.unwrap_or_default()).await;
    chrome_browser
}
