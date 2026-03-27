use super::capabilities::ChromeCapabilities;
use crate::browsers::bidi_browser::BidiBrowser;
use crate::drivers::bidi::drivers::{BidiDrive, BidiDriver, DriverConfiguration};
use crate::nodes::ChromeNode;
use rustenium_bidi_definitions::browsing_context::types::{BrowsingContext, Locator};
use rustenium_bidi_definitions::script::types::NodeRemoteValue;
use rustenium_bidi_definitions::session::types::ProxyConfiguration;
use rustenium_core::process::Process;
use rustenium_core::session::SessionConnectionType;
use rustenium_core::transport::{ConnectionTransportConfig, WebsocketConnectionTransport};
use rustenium_core::find_free_port;
use std::sync::{Arc, Mutex};

pub mod options {
    use rustenium_bidi_definitions::browsing_context::commands::CaptureScreenshotOrigin;
    use rustenium_bidi_definitions::browsing_context::types::{
        BrowsingContext, ClipRectangle, CreateType, ImageFormat, ReadinessState,
    };
    use rustenium_bidi_definitions::network::types::UrlPattern;
    use rustenium_bidi_definitions::script::types::{
        ChannelValue, ResultOwnership, SerializationOptions, SharedReference, Target,
    };

    #[derive(Debug, Clone, Default)]
    pub struct BrowserScreenshotOptions {
        pub context_id: Option<BrowsingContext>,
        pub origin: Option<CaptureScreenshotOrigin>,
        pub format: Option<ImageFormat>,
        pub clip: Option<ClipRectangle>,
        pub save_path: Option<String>,
    }


    #[derive(Default, Clone)]
    pub struct BrowserScreenshotOptionsBuilder {
        context_id: Option<BrowsingContext>,
        origin: Option<CaptureScreenshotOrigin>,
        format: Option<ImageFormat>,
        clip: Option<ClipRectangle>,
        save_path: Option<String>,
    }

    impl BrowserScreenshotOptionsBuilder {
        pub fn context_id(mut self, v: impl Into<BrowsingContext>) -> Self { self.context_id = Some(v.into()); self }
        pub fn origin(mut self, v: impl Into<CaptureScreenshotOrigin>) -> Self { self.origin = Some(v.into()); self }
        pub fn format(mut self, v: impl Into<ImageFormat>) -> Self { self.format = Some(v.into()); self }
        pub fn clip(mut self, v: impl Into<ClipRectangle>) -> Self { self.clip = Some(v.into()); self }
        pub fn save_path(mut self, v: impl Into<String>) -> Self { self.save_path = Some(v.into()); self }
        pub fn build(self) -> BrowserScreenshotOptions {
            BrowserScreenshotOptions {
                context_id: self.context_id, origin: self.origin, format: self.format,
                clip: self.clip, save_path: self.save_path,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct NavigateOptions {
        pub wait: Option<ReadinessState>,
        pub context_id: Option<BrowsingContext>,
    }

    #[derive(Default, Clone)]
    pub struct NavigateOptionsBuilder {
        wait: Option<ReadinessState>,
        context_id: Option<BrowsingContext>,
    }

    impl NavigateOptionsBuilder {
        pub fn wait(mut self, v: impl Into<ReadinessState>) -> Self { self.wait = Some(v.into()); self }
        pub fn context_id(mut self, v: impl Into<BrowsingContext>) -> Self { self.context_id = Some(v.into()); self }
        pub fn build(self) -> NavigateOptions {
            NavigateOptions { wait: self.wait, context_id: self.context_id }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct CreateContextOptions {
        pub context_type: Option<CreateType>,
        pub reference_context: Option<BrowsingContext>,
    }


    #[derive(Default, Clone)]
    pub struct CreateContextOptionsBuilder {
        context_type: Option<CreateType>,
        reference_context: Option<BrowsingContext>,
    }

    impl CreateContextOptionsBuilder {
        pub fn context_type(mut self, v: impl Into<CreateType>) -> Self { self.context_type = Some(v.into()); self }
        pub fn reference_context(mut self, v: impl Into<BrowsingContext>) -> Self { self.reference_context = Some(v.into()); self }
        pub fn build(self) -> CreateContextOptions {
            CreateContextOptions { context_type: self.context_type, reference_context: self.reference_context }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct FindNodesOptions {
        pub context_id: Option<BrowsingContext>,
        pub max_node_count: Option<u64>,
        pub serialization_options: Option<SerializationOptions>,
        pub start_nodes: Option<Vec<SharedReference>>,
    }


    #[derive(Default, Clone)]
    pub struct FindNodesOptionsBuilder {
        context_id: Option<BrowsingContext>,
        max_node_count: Option<u64>,
        serialization_options: Option<SerializationOptions>,
        start_nodes: Option<Vec<SharedReference>>,
    }

    impl FindNodesOptionsBuilder {
        pub fn context_id(mut self, v: impl Into<BrowsingContext>) -> Self { self.context_id = Some(v.into()); self }
        pub fn max_node_count(mut self, v: impl Into<u64>) -> Self { self.max_node_count = Some(v.into()); self }
        pub fn serialization_options(mut self, v: impl Into<SerializationOptions>) -> Self { self.serialization_options = Some(v.into()); self }
        pub fn start_nodes(mut self, v: Vec<SharedReference>) -> Self { self.start_nodes = Some(v); self }
        pub fn build(self) -> FindNodesOptions {
            FindNodesOptions {
                context_id: self.context_id, max_node_count: self.max_node_count,
                serialization_options: self.serialization_options, start_nodes: self.start_nodes,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct WaitForNodesOptions {
        pub context_id: Option<BrowsingContext>,
        pub timeout_ms: Option<u64>,
        pub poll_interval_ms: Option<u64>,
    }


    #[derive(Default, Clone)]
    pub struct WaitForNodesOptionsBuilder {
        context_id: Option<BrowsingContext>,
        timeout_ms: Option<u64>,
        poll_interval_ms: Option<u64>,
    }

    impl WaitForNodesOptionsBuilder {
        pub fn context_id(mut self, v: impl Into<BrowsingContext>) -> Self { self.context_id = Some(v.into()); self }
        pub fn timeout_ms(mut self, v: impl Into<u64>) -> Self { self.timeout_ms = Some(v.into()); self }
        pub fn poll_interval_ms(mut self, v: impl Into<u64>) -> Self { self.poll_interval_ms = Some(v.into()); self }
        pub fn build(self) -> WaitForNodesOptions {
            WaitForNodesOptions {
                context_id: self.context_id, timeout_ms: self.timeout_ms,
                poll_interval_ms: self.poll_interval_ms,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct OnRequestOptions {
        pub url_patterns: Option<Vec<UrlPattern>>,
        pub contexts: Option<Vec<String>>,
    }


    #[derive(Default, Clone)]
    pub struct OnRequestOptionsBuilder {
        url_patterns: Option<Vec<UrlPattern>>,
        contexts: Option<Vec<String>>,
    }

    impl OnRequestOptionsBuilder {
        pub fn url_patterns(mut self, v: Vec<UrlPattern>) -> Self { self.url_patterns = Some(v); self }
        pub fn contexts(mut self, v: Vec<String>) -> Self { self.contexts = Some(v); self }
        pub fn build(self) -> OnRequestOptions {
            OnRequestOptions { url_patterns: self.url_patterns, contexts: self.contexts }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct SubscribeEventsOptions {
        pub browsing_contexts: Option<Vec<String>>,
        pub user_contexts: Option<Vec<String>>,
    }


    #[derive(Default, Clone)]
    pub struct SubscribeEventsOptionsBuilder {
        browsing_contexts: Option<Vec<String>>,
        user_contexts: Option<Vec<String>>,
    }

    impl SubscribeEventsOptionsBuilder {
        pub fn browsing_contexts(mut self, v: Vec<String>) -> Self { self.browsing_contexts = Some(v); self }
        pub fn user_contexts(mut self, v: Vec<String>) -> Self { self.user_contexts = Some(v); self }
        pub fn build(self) -> SubscribeEventsOptions {
            SubscribeEventsOptions { browsing_contexts: self.browsing_contexts, user_contexts: self.user_contexts }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct EvaluateScriptOptions {
        pub target: Option<Target>,
        pub result_ownership: Option<ResultOwnership>,
        pub serialization_options: Option<SerializationOptions>,
        pub user_activation: Option<bool>,
    }


    #[derive(Default, Clone)]
    pub struct EvaluateScriptOptionsBuilder {
        target: Option<Target>,
        result_ownership: Option<ResultOwnership>,
        serialization_options: Option<SerializationOptions>,
        user_activation: Option<bool>,
    }

    impl EvaluateScriptOptionsBuilder {
        pub fn target(mut self, v: impl Into<Target>) -> Self { self.target = Some(v.into()); self }
        pub fn result_ownership(mut self, v: impl Into<ResultOwnership>) -> Self { self.result_ownership = Some(v.into()); self }
        pub fn serialization_options(mut self, v: impl Into<SerializationOptions>) -> Self { self.serialization_options = Some(v.into()); self }
        pub fn user_activation(mut self, v: impl Into<bool>) -> Self { self.user_activation = Some(v.into()); self }
        pub fn build(self) -> EvaluateScriptOptions {
            EvaluateScriptOptions {
                target: self.target, result_ownership: self.result_ownership,
                serialization_options: self.serialization_options, user_activation: self.user_activation,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct AddPreloadScriptOptions {
        pub arguments: Option<Vec<ChannelValue>>,
        pub contexts: Option<Vec<BrowsingContext>>,
        pub user_contexts: Option<Vec<String>>,
        pub sandbox: Option<String>,
    }


    #[derive(Default, Clone)]
    pub struct AddPreloadScriptOptionsBuilder {
        arguments: Option<Vec<ChannelValue>>,
        contexts: Option<Vec<BrowsingContext>>,
        user_contexts: Option<Vec<String>>,
        sandbox: Option<String>,
    }

    impl AddPreloadScriptOptionsBuilder {
        pub fn arguments(mut self, v: Vec<ChannelValue>) -> Self { self.arguments = Some(v); self }
        pub fn contexts(mut self, v: Vec<BrowsingContext>) -> Self { self.contexts = Some(v); self }
        pub fn user_contexts(mut self, v: Vec<String>) -> Self { self.user_contexts = Some(v); self }
        pub fn sandbox(mut self, v: impl Into<String>) -> Self { self.sandbox = Some(v.into()); self }
        pub fn build(self) -> AddPreloadScriptOptions {
            AddPreloadScriptOptions {
                arguments: self.arguments, contexts: self.contexts,
                user_contexts: self.user_contexts, sandbox: self.sandbox,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct EmulateTimezoneOptions {
        pub contexts: Option<Vec<BrowsingContext>>,
        pub user_contexts: Option<Vec<String>>,
    }


    #[derive(Default, Clone)]
    pub struct EmulateTimezoneOptionsBuilder {
        contexts: Option<Vec<BrowsingContext>>,
        user_contexts: Option<Vec<String>>,
    }

    impl EmulateTimezoneOptionsBuilder {
        pub fn contexts(mut self, v: Vec<BrowsingContext>) -> Self { self.contexts = Some(v); self }
        pub fn user_contexts(mut self, v: Vec<String>) -> Self { self.user_contexts = Some(v); self }
        pub fn build(self) -> EmulateTimezoneOptions {
            EmulateTimezoneOptions { contexts: self.contexts, user_contexts: self.user_contexts }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct AuthenticateOptions {
        pub url_patterns: Option<Vec<UrlPattern>>,
        pub contexts: Option<Vec<String>>,
    }


    #[derive(Default, Clone)]
    pub struct AuthenticateOptionsBuilder {
        url_patterns: Option<Vec<UrlPattern>>,
        contexts: Option<Vec<String>>,
    }

    impl AuthenticateOptionsBuilder {
        pub fn url_patterns(mut self, v: Vec<UrlPattern>) -> Self { self.url_patterns = Some(v); self }
        pub fn contexts(mut self, v: Vec<String>) -> Self { self.contexts = Some(v); self }
        pub fn build(self) -> AuthenticateOptions {
            AuthenticateOptions { url_patterns: self.url_patterns, contexts: self.contexts }
        }
    }
}

pub use options::{
    AddPreloadScriptOptions, AuthenticateOptions, BrowserScreenshotOptions, CreateContextOptions,
    EmulateTimezoneOptions, EvaluateScriptOptions, FindNodesOptions, NavigateOptions,
    OnRequestOptions, SubscribeEventsOptions, WaitForNodesOptions,
    AddPreloadScriptOptionsBuilder, AuthenticateOptionsBuilder, BrowserScreenshotOptionsBuilder,
    CreateContextOptionsBuilder, EmulateTimezoneOptionsBuilder, EvaluateScriptOptionsBuilder,
    FindNodesOptionsBuilder, NavigateOptionsBuilder, OnRequestOptionsBuilder,
    SubscribeEventsOptionsBuilder, WaitForNodesOptionsBuilder,
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
    chrome_process: Option<Process>,
}

impl std::fmt::Debug for ChromeBrowser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChromeBrowser")
            .field("config", &self.config)
            .field("chrome_process", &self.chrome_process)
            .finish_non_exhaustive()
    }
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
                let chrome_exe = config.chrome_executable_path.clone().unwrap_or_else(|| {
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

                (
                    Some(format!("localhost:{}", chrome_port)),
                    Some(chrome_proc),
                )
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
        let result = Self::start(
            &config,
            &ct_config,
            SessionConnectionType::WebSocket,
            capabilities,
        )
        .await;

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
            chrome_process,
        };
        browser.driver.listen_to_context_creation().await.unwrap();
        browser
    }

    /// Returns a reference to the Chrome configuration.
    pub fn get_config(&self) -> &ChromeConfig {
        &self.config
    }

    /// Returns a reference to the Chrome browser process.
    pub fn get_browser_process(&self) -> &Option<Process> {
        &self.chrome_process
    }
}

impl BidiBrowser for ChromeBrowser {
    type Transport = WebsocketConnectionTransport;
    type BrowserNode = ChromeNode<WebsocketConnectionTransport>;

    fn driver(&self) -> &BidiDriver<WebsocketConnectionTransport> {
        &self.driver
    }

    fn driver_mut(&mut self) -> &mut BidiDriver<WebsocketConnectionTransport> {
        &mut self.driver
    }

    fn build_node(
        &self,
        raw_node: NodeRemoteValue,
        locator: Locator,
        context: BrowsingContext,
    ) -> ChromeNode<WebsocketConnectionTransport> {
        ChromeNode::from_bidi(
            raw_node,
            locator,
            self.driver.session.clone(),
            context,
            self.driver.mouse.clone(),
        )
    }

    async fn close(mut self) -> Result<(), crate::error::BrowserCloseError> {
        tracing::debug!("Closing ChromeBrowser");
        self.driver.end_session().await?;
        if let Some(mut process) = self.chrome_process.take() {
            process.kill()?;
        }
        tracing::debug!("ChromeBrowser closed");
        Ok(())
    }
}

pub async fn create_chrome_browser(config: Option<ChromeConfig>) -> ChromeBrowser {
    let chrome_browser = ChromeBrowser::new(config.unwrap_or_default()).await;
    chrome_browser
}
