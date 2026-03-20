use rustenium_bidi_definitions::browsing_context::command_builders::{
    CaptureScreenshotBuilder, CreateBuilder, LocateNodesBuilder, NavigateBuilder,
};
use rustenium_bidi_definitions::browsing_context::results::NavigateResult;
use rustenium_bidi_definitions::browsing_context::types::{
    BrowsingContext, CreateType, Locator,
};
use rustenium_bidi_definitions::browser::types::UserContext;
use rustenium_bidi_definitions::emulation::command_builders::SetTimezoneOverrideBuilder;
use rustenium_bidi_definitions::Event;
use rustenium_bidi_definitions::Command;
use rustenium_bidi_definitions::script::command_builders::{
    AddPreloadScriptBuilder, EvaluateBuilder, RemovePreloadScriptBuilder,
};
use rustenium_bidi_definitions::script::types::{
    ContextTarget, SerializationOptions,
    SerializationOptionsIncludeShadowTree, Target,
};
use rustenium_bidi_definitions::session::results::SubscribeResult;
use rustenium_bidi_definitions::session::types::ProxyConfiguration;
use rustenium_bidi_definitions::base::CommandResponse;
use rustenium_core::error::{CommandResultError, SessionSendError};
use rustenium_core::events::BidiEventManagement;
use rustenium_core::session::SessionConnectionType;
use rustenium_core::transport::{ConnectionTransportConfig, WebsocketConnectionTransport};
use rustenium_core::{find_free_port, BidiSession, NetworkRequest};
use crate::drivers::bidi::drivers::{BidiDriver, BidiDrive, DriverConfiguration};
use crate::error::{
    ContextCreationError, ContextIndexError, EmulationError,
    EvaluateResultError, FindNodesError, InterceptNetworkError, OpenUrlError, ScreenshotError,
};
use crate::nodes::ChromeNode;
use super::capabilities::ChromeCapabilities;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use std::future::Future;
use rustenium_core::process::Process;

pub mod options {
    use rustenium_bidi_definitions::browsing_context::commands::CaptureScreenshotOrigin;
    use rustenium_bidi_definitions::browsing_context::types::{
        BrowsingContext, ClipRectangle, CreateType, ImageFormat, ReadinessState,
    };
    use rustenium_bidi_definitions::script::types::{
        ChannelValue, ResultOwnership, SerializationOptions, SharedReference, Target,
    };
    use rustenium_bidi_definitions::network::types::UrlPattern;

    #[derive(Debug, Clone, Default)]
    pub struct BrowserScreenshotOptions {
        pub context_id: Option<BrowsingContext>,
        pub origin: Option<CaptureScreenshotOrigin>,
        pub format: Option<ImageFormat>,
        pub clip: Option<ClipRectangle>,
        pub save_path: Option<String>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct NavigateOptions {
        pub wait: Option<ReadinessState>,
        pub context_id: Option<BrowsingContext>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct CreateContextOptions {
        pub context_type: Option<CreateType>,
        pub reference_context: Option<BrowsingContext>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct FindNodesOptions {
        pub context_id: Option<BrowsingContext>,
        pub max_node_count: Option<u64>,
        pub serialization_options: Option<SerializationOptions>,
        pub start_nodes: Option<Vec<SharedReference>>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct WaitForNodesOptions {
        pub context_id: Option<BrowsingContext>,
        pub timeout_ms: Option<u64>,
        pub poll_interval_ms: Option<u64>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct OnRequestOptions {
        pub url_patterns: Option<Vec<UrlPattern>>,
        pub contexts: Option<Vec<String>>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct SubscribeEventsOptions {
        pub browsing_contexts: Option<Vec<String>>,
        pub user_contexts: Option<Vec<String>>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct EvaluateScriptOptions {
        pub target: Option<Target>,
        pub result_ownership: Option<ResultOwnership>,
        pub serialization_options: Option<SerializationOptions>,
        pub user_activation: Option<bool>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct AddPreloadScriptOptions {
        pub arguments: Option<Vec<ChannelValue>>,
        pub contexts: Option<Vec<BrowsingContext>>,
        pub user_contexts: Option<Vec<String>>,
        pub sandbox: Option<String>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct EmulateTimezoneOptions {
        pub contexts: Option<Vec<BrowsingContext>>,
        pub user_contexts: Option<Vec<String>>,
    }

    #[derive(Debug, Clone, Default)]
    pub struct AuthenticateOptions {
        pub url_patterns: Option<Vec<UrlPattern>>,
        pub contexts: Option<Vec<String>>,
    }
}

pub use options::{
    BrowserScreenshotOptions, NavigateOptions, CreateContextOptions,
    FindNodesOptions, WaitForNodesOptions, OnRequestOptions, SubscribeEventsOptions,
    EvaluateScriptOptions, AddPreloadScriptOptions, EmulateTimezoneOptions, AuthenticateOptions,
};

/// Configuration for Chrome browser and chromedriver.
///
/// This struct configures both the chromedriver process and the Chrome browser instance.
/// It supports three remote debugging modes via `remote_debugging_port`:
///
/// - **Normal mode (`None`)**: Chromedriver launches and manages Chrome automatically
/// - **Auto mode (`Some(0)`)**: Rustenium starts Chrome first, then chromedriver attaches to it
/// - **Manual mode (`Some(port)`)**: Connect to an existing Chrome instance on the specified port
///
/// # Examples
///
/// ```
/// use rustenium::browsers::ChromeConfig;
///
/// // Basic configuration
/// let config = ChromeConfig {
///     driver_executable_path: "chromedriver".to_string(),
///     ..Default::default()
/// };
///
/// // Headless mode with custom Chrome
/// let config = ChromeConfig {
///     driver_executable_path: "chromedriver".to_string(),
///     remote_debugging_port: Some(0), // Auto mode
///     chrome_executable_path: Some("/usr/bin/google-chrome".to_string()),
///     browser_flags: Some(vec!["--headless".to_string()]),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone)]
pub struct ChromeConfig {
    /// Path to the chromedriver executable.
    pub driver_executable_path: String,

    /// Host for the WebDriver server (default: localhost).
    pub host: Option<String>,

    /// Port for the WebDriver server (auto-assigned if None).
    pub port: Option<u16>,

    /// Additional flags to pass to chromedriver.
    pub driver_flags: Vec<&'static str>,

    /// Chrome capabilities for configuring browser behavior.
    pub capabilities: ChromeCapabilities,

    /// Whether to enable Chrome sandbox (default: false).
    pub sandbox: bool,

    /// Optional proxy configuration.
    pub proxy: Option<ProxyConfiguration>,

    /// Chrome remote debugging port. Controls how Chrome is launched and managed:
    /// - `None` (default): Normal mode - chromedriver launches Chrome
    /// - `Some(0)`: Auto mode - Rustenium starts Chrome and manages it internally
    /// - `Some(port)`: Manual mode - Connect to existing Chrome running on specified port
    pub remote_debugging_port: Option<u16>,

    /// Path to Chrome executable. Used when `remote_debugging_port` is `Some(0)`.
    /// Defaults to "google-chrome" if not specified.
    pub chrome_executable_path: Option<String>,

    /// Chrome user data directory. Used when `remote_debugging_port` is `Some(0)`.
    /// If not specified, uses a temporary directory.
    pub user_data_dir: Option<String>,

    /// Additional Chrome command-line arguments (browser flags).
    /// Used when `remote_debugging_port` is `Some(0)`.
    pub browser_flags: Option<Vec<String>>,
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
            proxy: None,
            remote_debugging_port: None,
            chrome_executable_path: None,
            user_data_dir: None,
            browser_flags: None,
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
    chrome_process: Option<Process>
}

impl BidiDrive<WebsocketConnectionTransport> for ChromeBrowser {}

impl ChromeBrowser {
    /// Creates a new Chrome browser instance with the specified configuration.
    ///
    /// This method initializes chromedriver, establishes a WebDriver BiDi session,
    /// and optionally launches or connects to a Chrome browser instance.
    ///
    /// # Arguments
    ///
    /// * `config` - Chrome configuration including driver path, browser options, and capabilities
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rustenium::browsers::{ChromeBrowser, ChromeConfig};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = ChromeConfig {
    ///     driver_executable_path: "chromedriver".to_string(),
    ///     ..Default::default()
    /// };
    ///
    /// let mut browser = ChromeBrowser::new(config).await;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(mut config: ChromeConfig) -> ChromeBrowser {
        if config.driver_executable_path.is_empty() {
            config.driver_executable_path = crate::downloader::ensure_chromedriver()
                .to_string_lossy()
                .into_owned();
        }
        let port = find_free_port().unwrap();
        config.port = Some(config.port.unwrap_or(port));
        let mut ct_config = ConnectionTransportConfig::default();
        ct_config.host = config.host.clone().unwrap_or(String::from("localhost"));
        ct_config.port = port;

        // Handle remote debugging modes FIRST, before modifying capabilities
        let (debugger_address, chrome_process) = match config.remote_debugging_port {
            Some(0) => {
                // Auto mode (port 0): Start Chrome ourselves, then attach
                let chrome_port = find_free_port().unwrap();
                let chrome_exe = config.chrome_executable_path
                    .clone()
                    .unwrap_or_else(|| {
                        crate::downloader::ensure_chrome()
                            .to_string_lossy()
                            .into_owned()
                    });

                // Use user-specified or default tmp directory for user data
                let user_data_dir = config.user_data_dir.clone().unwrap_or_else(|| {
                    std::env::temp_dir()
                        .join(format!("rustenium-chrome-{}", chrome_port))
                        .display()
                        .to_string()
                });

                let mut chrome_args = vec![
                    format!("--remote-debugging-port={}", chrome_port),
                    format!("--user-data-dir={}", user_data_dir),
                    "--no-first-run".to_string(),
                    "--no-default-browser-check".to_string(),
                ];

                // Add user-specified browser flags
                if let Some(ref flags) = config.browser_flags {
                    chrome_args.extend(flags.iter().cloned());
                }

                use rustenium_core::process::Process;
                let chrome_proc = Process::create(chrome_exe, chrome_args);

                // Wait briefly for Chrome to start
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

                (Some(format!("localhost:{}", chrome_port)), Some(chrome_proc))
            }
            Some(port) => {
                // Manual mode (specific port): Connect to existing Chrome on that port
                (Some(format!("localhost:{}", port)), None)
            }
            None => {
                // Normal mode: chromedriver will launch Chrome
                (None, None)
            }
        };

        // Set debugger_address capability if attaching
        if let Some(addr) = debugger_address {
            config.capabilities.debugger_address(addr);
        }

        // Continue with normal capability setup
        config.capabilities.add_arg("start-maximized".to_string());
        config.capabilities.add_arg("disable-infobars".to_string());

        // Add --no-sandbox arg to Chrome options if sandbox is disabled
        if !config.sandbox {
            config.capabilities.add_arg("no-sandbox".to_string());
        }

        // Set proxy if provided
        if let Some(proxy) = config.proxy.clone() {
            config.capabilities.base_capabilities.proxy = Some(proxy);
        }

        // Convert ChromeCapabilities to CapabilitiesRequest
        let capabilities = config.capabilities.clone().build();

        // Start chromedriver (it will attach to Chrome if debugger_address is set)
        let result = Self::start(&config, &ct_config, SessionConnectionType::WebSocket, capabilities).await;

        let session = result.0;
        let driver_process = result.1;

        let mut browser = ChromeBrowser {
            config,
            driver: BidiDriver::new(
                String::from("chromedriver"),
                vec![],
                session,
                0,
                Arc::new(Mutex::new(Vec::new())),
                driver_process,
            ),
            chrome_process
        };
        browser.driver.listen_to_context_creation().await.unwrap();
        browser
    }

    // ── Navigation ───────────────────────────────────────────────────────────

    /// Navigates to the specified URL in the active context.
    pub async fn navigate(&mut self, url: &str) -> Result<NavigateResult, OpenUrlError> {
        self.navigate_with_options(url, NavigateOptions::default()).await
    }

    /// Navigates to the specified URL with custom options (wait state, context).
    pub async fn navigate_with_options(
        &mut self,
        url: &str,
        options: NavigateOptions,
    ) -> Result<NavigateResult, OpenUrlError> {
        let context = options.context_id
            .unwrap_or_else(|| self.driver.get_active_context_id().unwrap());
        let mut builder = NavigateBuilder::default().url(url).context(context);
        if let Some(wait) = options.wait {
            builder = builder.wait(wait);
        }
        self.driver.navigate(builder.build().unwrap()).await
    }

    // ── Context ──────────────────────────────────────────────────────────────

    /// Creates a new browsing context (tab) with default options.
    pub async fn create_context_bidi(
        &mut self,
        background: bool
    ) -> Result<rustenium_core::BrowsingContext, ContextCreationError> {
        self.create_context_bidi_with_options(background, CreateContextOptions::default()).await
    }

    /// Creates a new browsing context with custom options (type, reference context, background).
    pub async fn create_context_bidi_with_options(
        &mut self,
        background: bool,
        options: CreateContextOptions,
    ) -> Result<rustenium_core::BrowsingContext, ContextCreationError> {
        let context_type = options.context_type.unwrap_or(CreateType::Tab);
        let mut builder = CreateBuilder::default().r#type(context_type);
        if let Some(ref_ctx) = options.reference_context {
            builder = builder.reference_context(ref_ctx);
        };
        builder = builder.background(background);
        self.driver.create_context(builder.build().unwrap()).await
    }

    // ── Node finding ─────────────────────────────────────────────────────────

    /// Finds all elements matching the given locator in the active context.
    pub async fn find_nodes(
        &mut self,
        locator: Locator,
    ) -> Result<Vec<ChromeNode<WebsocketConnectionTransport>>, FindNodesError> {
        self.find_nodes_with_options(locator, FindNodesOptions::default()).await
    }

    /// Finds all elements matching the given locator with custom options (context, count, serialization, start nodes).
    pub async fn find_nodes_with_options(
        &mut self,
        locator: Locator,
        options: FindNodesOptions,
    ) -> Result<Vec<ChromeNode<WebsocketConnectionTransport>>, FindNodesError> {
        let context = options.context_id.clone()
            .unwrap_or_else(|| self.driver.get_active_context_id().unwrap());
        let serialization_options = options.serialization_options.unwrap_or(SerializationOptions {
            max_dom_depth: Some(40),
            max_object_depth: Some(0),
            include_shadow_tree: Some(SerializationOptionsIncludeShadowTree::None),
        });
        let mut builder = LocateNodesBuilder::default()
            .context(context.clone())
            .locator(locator.clone())
            .serialization_options(serialization_options);
        if let Some(max_count) = options.max_node_count {
            builder = builder.max_node_count(max_count);
        }
        if let Some(start_nodes) = options.start_nodes {
            builder = builder.start_nodes(start_nodes);
        }
        let node_result = self.driver.find_nodes(builder.build().unwrap()).await?;
        let mut chrome_nodes = Vec::new();
        for node in node_result.nodes.iter() {
            let chrome_node = ChromeNode::from_bidi(
                node.clone(),
                locator.clone(),
                self.driver.session.clone(),
                context.clone(),
                self.driver.mouse.clone(),
            );
            chrome_nodes.push(chrome_node);
        }
        Ok(chrome_nodes)
    }

    /// Finds the first element matching the given locator in the active context.
    ///
    /// Returns `None` if no matching element is found.
    pub async fn find_node(
        &mut self,
        locator: Locator,
    ) -> Result<Option<ChromeNode<WebsocketConnectionTransport>>, FindNodesError> {
        self.find_node_with_options(locator, FindNodesOptions::default()).await
    }

    /// Finds the first element matching the given locator with custom options.
    ///
    /// Returns `None` if no matching element is found.
    pub async fn find_node_with_options(
        &mut self,
        locator: Locator,
        options: FindNodesOptions,
    ) -> Result<Option<ChromeNode<WebsocketConnectionTransport>>, FindNodesError> {
        let nodes = self.find_nodes_with_options(locator, options).await?;
        Ok(nodes.into_iter().next())
    }

    // ── Waiting ───────────────────────────────────────────────────────────────

    /// Waits for elements matching the locator to appear (default 4s timeout).
    pub async fn wait_for_nodes(
        &mut self,
        locator: Locator,
    ) -> Result<Vec<ChromeNode<WebsocketConnectionTransport>>, FindNodesError> {
        self.wait_for_nodes_with_options(locator, WaitForNodesOptions::default()).await
    }

    /// Waits for elements matching the locator with custom options (context, timeout, poll interval).
    pub async fn wait_for_nodes_with_options(
        &mut self,
        locator: Locator,
        options: WaitForNodesOptions,
    ) -> Result<Vec<ChromeNode<WebsocketConnectionTransport>>, FindNodesError> {
        let timeout = options.timeout_ms.unwrap_or(4000);
        let poll_interval = options.poll_interval_ms.unwrap_or(timeout / 6);
        let start = std::time::Instant::now();

        loop {
            let nodes = self.find_nodes_with_options(
                locator.clone(),
                FindNodesOptions { context_id: options.context_id.clone(), ..Default::default() },
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

    /// Waits for a single element matching the locator to appear (default 4s timeout).
    ///
    /// Returns `None` if no matching element appears within the timeout.
    pub async fn wait_for_node(
        &mut self,
        locator: Locator,
    ) -> Result<Option<ChromeNode<WebsocketConnectionTransport>>, FindNodesError> {
        self.wait_for_node_with_options(locator, WaitForNodesOptions::default()).await
    }

    /// Waits for a single element matching the locator with custom options.
    ///
    /// Returns `None` if no matching element appears within the timeout.
    pub async fn wait_for_node_with_options(
        &mut self,
        locator: Locator,
        options: WaitForNodesOptions,
    ) -> Result<Option<ChromeNode<WebsocketConnectionTransport>>, FindNodesError> {
        let nodes = self.wait_for_nodes_with_options(locator, options).await?;
        Ok(nodes.into_iter().next())
    }

    // ── Network interception ──────────────────────────────────────────────────

    /// Registers a handler called for each network request (all URLs, all contexts).
    pub async fn on_request_bidi<F, Fut>(
        &mut self,
        handler: F,
    ) -> Result<(), InterceptNetworkError>
    where
        F: Fn(NetworkRequest<WebsocketConnectionTransport>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        self.on_request_bidi_with_options(handler, OnRequestOptions::default()).await
    }

    /// Registers a handler called for each network request with custom URL pattern and context filters.
    pub async fn on_request_bidi_with_options<F, Fut>(
        &mut self,
        handler: F,
        options: OnRequestOptions,
    ) -> Result<(), InterceptNetworkError>
    where
        F: Fn(NetworkRequest<WebsocketConnectionTransport>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let mut builder = self.driver.on_request(handler);
        if let Some(patterns) = options.url_patterns {
            builder = builder.url_patterns(patterns);
        }
        if let Some(contexts) = options.contexts {
            builder = builder.contexts(contexts);
        }
        builder.execute().await
    }

    // ── Events ────────────────────────────────────────────────────────────────

    /// Subscribes to browser events and registers a handler.
    pub async fn subscribe_events<F, R>(
        &mut self,
        events: HashSet<&str>,
        handler: F,
    ) -> Result<Option<SubscribeResult>, CommandResultError>
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        self.subscribe_events_with_options(events, handler, SubscribeEventsOptions::default()).await
    }

    /// Subscribes to browser events with context and user context filters.
    pub async fn subscribe_events_with_options<F, R>(
        &mut self,
        events: HashSet<&str>,
        handler: F,
        options: SubscribeEventsOptions,
    ) -> Result<Option<SubscribeResult>, CommandResultError>
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        let mut bidi_event = {
            let mut session = self.driver.session.lock().await;
            session.create_event::<_, _, BidiSession<WebsocketConnectionTransport>>(events, handler)
        };
        if let Some(contexts) = options.browsing_contexts {
            for ctx in contexts {
                bidi_event.add_browsing_context(ctx);
            }
        }
        if let Some(user_ctxs) = options.user_contexts {
            for uctx in user_ctxs {
                bidi_event.add_user_context(uctx);
            }
        }
        self.driver.session.lock().await.subscribe_events(bidi_event).await
    }

    /// Adds an event handler without sending a subscription command.
    ///
    /// Returns the auto-generated handler ID.
    pub async fn add_event_handler<F, R>(
        &mut self,
        events: HashSet<&str>,
        handler: F,
    ) -> String
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        self.driver.add_event_handler(events, handler).await
    }

    // ── Script evaluation ─────────────────────────────────────────────────────

    /// Evaluates a JavaScript expression in the active browsing context.
    pub async fn evaluate_script_bidi(
        &mut self,
        expression: String,
        await_promise: bool,
    ) -> Result<rustenium_bidi_definitions::script::types::EvaluateResultSuccess, EvaluateResultError> {
        self.evaluate_script_bidi_with_options(expression, await_promise, EvaluateScriptOptions::default()).await
    }

    /// Evaluates a JavaScript expression with custom options (target, result ownership, serialization).
    pub async fn evaluate_script_bidi_with_options(
        &mut self,
        expression: String,
        await_promise: bool,
        options: EvaluateScriptOptions,
    ) -> Result<rustenium_bidi_definitions::script::types::EvaluateResultSuccess, EvaluateResultError> {
        let target = options.target.unwrap_or_else(|| {
            let context = self.driver.get_active_context_id().unwrap();
            Target::ContextTarget(ContextTarget::new(context))
        });
        let mut builder = EvaluateBuilder::default()
            .expression(expression)
            .await_promise(await_promise)
            .target(target);
        if let Some(ro) = options.result_ownership {
            builder = builder.result_ownership(ro);
        }
        if let Some(so) = options.serialization_options {
            builder = builder.serialization_options(so);
        }
        if let Some(ua) = options.user_activation {
            builder = builder.user_activation(ua);
        }
        self.driver.evaluate_script(builder.build().unwrap()).await
    }

    // ── Preload scripts ───────────────────────────────────────────────────────

    /// Adds a preload script that runs in every new browsing context.
    pub async fn add_preload_script_bidi(
        &mut self,
        function_declaration: String,
    ) -> Result<String, EvaluateResultError> {
        self.add_preload_script_bidi_with_options(function_declaration, AddPreloadScriptOptions::default()).await
    }

    /// Adds a preload script with custom options (arguments, contexts, sandbox).
    pub async fn add_preload_script_bidi_with_options(
        &mut self,
        function_declaration: String,
        options: AddPreloadScriptOptions,
    ) -> Result<String, EvaluateResultError> {
        let mut builder = AddPreloadScriptBuilder::default()
            .function_declaration(function_declaration);
        if let Some(args) = options.arguments {
            builder = builder.arguments(args);
        }
        if let Some(contexts) = options.contexts {
            builder = builder.contexts(contexts);
        }
        if let Some(user_contexts) = options.user_contexts {
            builder = builder.user_contexts(user_contexts.into_iter().map(UserContext::new));
        }
        if let Some(sandbox) = options.sandbox {
            builder = builder.sandbox(sandbox);
        }
        self.driver.add_preload_script(builder.build().unwrap()).await.map(|ps| ps.into())
    }

    /// Removes a preload script by its ID.
    pub async fn remove_preload_script_bidi(
        &mut self,
        script: String,
    ) -> Result<(), EvaluateResultError> {
        let remove_cmd = RemovePreloadScriptBuilder::default().script(script).build().unwrap();
        self.driver.remove_preload_script(remove_cmd).await
    }

    // ── Screenshot ────────────────────────────────────────────────────────────

    /// Captures a screenshot of the active browsing context and returns base64-encoded image data.
    pub async fn screenshot(&mut self) -> Result<String, ScreenshotError> {
        self.screenshot_with_options(BrowserScreenshotOptions::default()).await
    }

    /// Captures a screenshot with custom options (context, origin, format, clip, save path).
    ///
    /// If `save_path` is a directory, saves with auto-generated filename (screenshot_TIMESTAMP.png).
    /// If `save_path` is a file path, saves to that exact location and returns the path.
    /// If `save_path` is `None`, returns base64-encoded image data.
    pub async fn screenshot_with_options(&mut self, options: BrowserScreenshotOptions) -> Result<String, ScreenshotError> {
        let context = options.context_id
            .unwrap_or_else(|| self.driver.get_active_context_id().unwrap());
        let mut builder = CaptureScreenshotBuilder::default().context(context);
        if let Some(origin) = options.origin {
            builder = builder.origin(origin);
        }
        if let Some(format) = options.format {
            builder = builder.format(format);
        }
        if let Some(clip) = options.clip {
            builder = builder.clip(clip);
        }
        let command = builder.build().unwrap();
        self.driver.screenshot(command, options.save_path.as_deref()).await
    }

    // ── Emulation ─────────────────────────────────────────────────────────────

    /// Emulates a timezone for the active browsing context.
    ///
    /// Pass `None` to clear the override.
    pub async fn emulate_timezone(
        &mut self,
        timezone: Option<String>,
    ) -> Result<(), EmulationError> {
        self.emulate_timezone_with_options(timezone, EmulateTimezoneOptions::default()).await
    }

    /// Emulates a timezone with custom context and user context filters.
    ///
    /// Pass `None` for `timezone` to clear the override.
    pub async fn emulate_timezone_with_options(
        &mut self,
        timezone: Option<String>,
        options: EmulateTimezoneOptions,
    ) -> Result<(), EmulationError> {
        let mut builder = SetTimezoneOverrideBuilder::default();
        if let Some(tz) = timezone {
            builder = builder.timezone(tz);
        }
        if let Some(contexts) = options.contexts {
            builder = builder.contexts(contexts);
        }
        if let Some(user_contexts) = options.user_contexts {
            builder = builder.user_contexts(user_contexts.into_iter().map(UserContext::new));
        }
        self.driver.set_timezone_override(builder.build()).await
    }

    // ── Authentication ────────────────────────────────────────────────────────

    /// Sets HTTP authentication credentials for all requests.
    pub async fn authenticate(
        &mut self,
        username: impl Into<String> + Send + 'static,
        password: impl Into<String> + Send + 'static,
    ) -> Result<(), InterceptNetworkError> {
        self.authenticate_with_options(username, password, AuthenticateOptions::default()).await
    }

    /// Sets HTTP authentication credentials with URL pattern and context filters.
    pub async fn authenticate_with_options(
        &mut self,
        username: impl Into<String> + Send + 'static,
        password: impl Into<String> + Send + 'static,
        options: AuthenticateOptions,
    ) -> Result<(), InterceptNetworkError> {
        let mut builder = self.driver.authenticate(username, password);
        if let Some(patterns) = options.url_patterns {
            builder = builder.url_patterns(patterns);
        }
        if let Some(contexts) = options.contexts {
            builder = builder.contexts(contexts);
        }
        builder.execute().await
    }

    // ── Accessors ─────────────────────────────────────────────────────────────

    /// Returns a reference to the Chrome configuration.
    pub fn get_config(&self) -> &ChromeConfig {
        &self.config
    }

    /// Returns a reference to the Chrome browser process.
    pub fn get_browser_process(&self) -> &Option<Process> {
        &self.chrome_process
    }

    /// Returns a reference to the direct BiDi mouse for precise, instant movements.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustenium::browsers::ChromeBrowser;
    /// # use rustenium::input::Point;
    /// # use rustenium_bidi_commands::browsing_context::types::BrowsingContext;
    /// # async fn example(browser: ChromeBrowser, context: BrowsingContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let mouse = browser.mouse();
    /// mouse.move_to(Point { x: 100.0, y: 200.0 }, &context, None).await?;
    /// mouse.click(None, &context, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn mouse(&self) -> &crate::input::BidiMouse<WebsocketConnectionTransport> {
        self.driver.mouse.as_ref()
    }

    /// Returns a reference to the human mouse for realistic, human-like movements with Bezier curves and jitter.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustenium::browsers::ChromeBrowser;
    /// # use rustenium::input::Point;
    /// # use rustenium_bidi_commands::browsing_context::types::BrowsingContext;
    /// # async fn example(browser: ChromeBrowser, context: BrowsingContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let human_mouse = browser.human_mouse();
    /// // Moves with natural curve and realistic delays
    /// human_mouse.move_to(Point { x: 300.0, y: 400.0 }, &context, None).await?;
    /// human_mouse.click(None, &context, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn human_mouse(&self) -> &crate::input::HumanMouse<crate::input::BidiMouse<WebsocketConnectionTransport>> {
        self.driver.human_mouse.as_ref()
    }

    /// Returns a reference to the keyboard for text input and key presses.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustenium::browsers::ChromeBrowser;
    /// # use rustenium_bidi_commands::browsing_context::types::BrowsingContext;
    /// # async fn example(browser: ChromeBrowser, context: BrowsingContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let keyboard = browser.keyboard();
    /// keyboard.type_text("Hello, World!", &context).await?;
    /// keyboard.press("Enter", &context).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn keyboard(&self) -> &crate::input::Keyboard<WebsocketConnectionTransport> {
        self.driver.keyboard.as_ref()
    }

    pub fn get_active_context_id(&self) -> Result<BrowsingContext, ContextIndexError> {
        self.driver.get_active_context_id()
    }

    // ── Session ───────────────────────────────────────────────────────────────

    /// Sends a raw WebDriver BiDi command.
    pub async fn send_bidi_command(&mut self, command: Command) -> Result<CommandResponse, SessionSendError> {
        self.driver.send_command(command).await
    }

    /// Ends the BiDi session and cleans up resources.
    pub async fn end_bidi_session(&mut self) -> Result<(), SessionSendError> {
        self.driver.end_session().await
    }
}

pub async fn create_chrome_browser(config: Option<ChromeConfig>) -> ChromeBrowser {
    let chrome_browser = ChromeBrowser::new(config.unwrap_or_default()).await;
    chrome_browser
}