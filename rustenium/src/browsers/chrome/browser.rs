use super::capabilities::ChromeCapabilities;
use crate::browsers::bidi_browser::BidiBrowser;
use crate::browsers::cdp_browser::CdpBrowser;
use crate::cdp::adapter::{AdaptCdp, CdpAdapter};
use crate::conduit::bidi::drivers::{BidiDrive, BidiDriver, DriverConfiguration};
use crate::error::bidi::BrowserCloseError;
use crate::nodes::ChromeNode;
use rustenium_bidi_definitions::browsing_context::types::{BrowsingContext, Locator};
use rustenium_bidi_definitions::script::types::NodeRemoteValue;
use rustenium_bidi_definitions::session::types::ProxyConfiguration;
use rustenium_core::process::Process;
use rustenium_core::session::SessionConnectionType;
use rustenium_core::transport::{ConnectionTransportConfig, WebsocketConnectionTransport};
use rustenium_core::find_free_port;
use std::sync::{Arc, Mutex};

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
    cdp_adapter: CdpAdapter<WebsocketConnectionTransport>
}

impl std::fmt::Debug for ChromeBrowser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChromeBrowser")
            .field("config", &self.config)
            .field("chrome_process", &self.chrome_process)
            .finish_non_exhaustive()
    }
}


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
        
        let chrome_port = find_free_port().unwrap();

        // Handle remote debugging modes FIRST, before modifying capabilities
        let (debugger_address, chrome_process) = match config.remote_debugging_port {
            Some(0) => {
                // Auto mode (port 0): Start Chrome ourselves, then attach
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
        let result = <ChromeBrowser as BidiDrive<WebsocketConnectionTransport>>::start(
            &config,
            &ct_config,
            SessionConnectionType::WebSocket,
            capabilities,
        )
        .await;

        let ws_debugger_url = <ChromeBrowser as AdaptCdp<WebsocketConnectionTransport>>::fetch_ws_debugger_url(&ct_config.host, chrome_port).await.unwrap();

        let cdp_cc = ConnectionTransportConfig::from_ws_url(&ws_debugger_url).unwrap();

        let cdp_session = <ChromeBrowser as AdaptCdp<WebsocketConnectionTransport>>::start(&cdp_cc).await;

        let cdp_adapter = CdpAdapter::new(cdp_session);

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
            cdp_adapter: cdp_adapter,
        };
        browser.driver.listen_to_context_creation().await.unwrap();
        browser.cdp_adapter.listen_to_target_creation().await.unwrap();
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

impl BidiDrive<WebsocketConnectionTransport> for ChromeBrowser {}

impl AdaptCdp<WebsocketConnectionTransport> for ChromeBrowser {}

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

    async fn close(mut self) -> Result<(), BrowserCloseError> {
        tracing::debug!("Closing ChromeBrowser");
        self.driver.end_session().await?;
        if let Some(mut process) = self.chrome_process.take() {
            process.kill()?;
        }
        tracing::debug!("ChromeBrowser closed");
        Ok(())
    }
}

impl CdpBrowser for ChromeBrowser {
    fn adapter(&self) -> &CdpAdapter<WebsocketConnectionTransport> {
        &self.cdp_adapter
    }

    fn adapter_mut(&mut self) -> &mut CdpAdapter<WebsocketConnectionTransport> {
        &mut self.cdp_adapter
    }
}

pub async fn create_chrome_browser(config: Option<ChromeConfig>) -> ChromeBrowser {
    let chrome_browser = ChromeBrowser::new(config.unwrap_or_default()).await;
    chrome_browser
}
