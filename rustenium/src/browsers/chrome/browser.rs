use super::capabilities::ChromeCapabilities;
use crate::browsers::BidiBrowser;
use crate::browsers::cdp_browser::CdpBrowser;
use crate::conduit::bidi::drivers::{BidiDriver, DriverConfiguration, start_bidi_driver};
use crate::conduit::cdp::adapter::{CdpAdapter, fetch_ws_debugger_url, start_cdp_session};
use crate::error::bidi::BrowserCloseError;
use crate::nodes::ChromeNode;
use rustenium_bidi_definitions::browsing_context::types::{BrowsingContext, Locator};
use rustenium_bidi_definitions::script::types::NodeRemoteValue;
use rustenium_bidi_definitions::session::types::ProxyConfiguration;
use rustenium_core::find_free_port;
use rustenium_core::process::Process;
use rustenium_core::session::SessionConnectionType;
use rustenium_core::transport::{ConnectionTransportConfig, WebsocketConnectionTransport};
use std::sync::{Arc, Mutex};

/// How Chrome is launched and managed.
#[derive(Debug, Clone)]
pub enum ChromeLaunchMode {
    /// Rustenium starts Chrome and attaches chromedriver to it (default).
    SpawnAndAttach,
    /// Connect to an existing Chrome instance on the specified debugging port.
    Remote(u16),
    /// Let chromedriver spawn and manage Chrome.
    DriverManaged,
}

impl Default for ChromeLaunchMode {
    fn default() -> Self {
        ChromeLaunchMode::SpawnAndAttach
    }
}

/// Configuration for Chrome browser and chromedriver.
///
/// # Examples
///
/// ```
/// use rustenium::browsers::{ChromeConfig, ChromeLaunchMode};
///
/// // Basic configuration (Rustenium manages Chrome)
/// let config = ChromeConfig {
///     driver_executable_path: "chromedriver".to_string(),
///     ..Default::default()
/// };
///
/// // Let chromedriver manage Chrome
/// let config = ChromeConfig {
///     driver_executable_path: "chromedriver".to_string(),
///     launch_mode: ChromeLaunchMode::DriverManaged,
///     ..Default::default()
/// };
///
/// // Connect to an existing Chrome instance
/// let config = ChromeConfig {
///     driver_executable_path: "chromedriver".to_string(),
///     launch_mode: ChromeLaunchMode::Remote(9222),
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

    /// How Chrome is launched and managed.
    pub launch_mode: ChromeLaunchMode,

    /// Chrome remote debugging port. Auto-assigned in Managed mode,
    /// derived from launch_mode in Remote mode.
    pub remote_debugging_port: Option<u16>,

    /// Path to Chrome executable. Used in Managed mode.
    /// Defaults to auto-downloaded Chrome if not specified.
    pub chrome_executable_path: Option<String>,

    /// Chrome user data directory. Used in Managed mode.
    /// If not specified, uses a temporary directory.
    pub user_data_dir: Option<String>,

    /// Additional Chrome command-line arguments (browser flags).
    /// Used in Managed and DriverManaged modes.
    pub browser_flags: Option<Vec<String>>,

    /// Enable BiDi (WebDriver BiDi) connection via chromedriver (default: true).
    pub enable_bidi: bool,

    /// Enable CDP (Chrome DevTools Protocol) connection (default: false).
    pub enable_cdp: bool,
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
            launch_mode: ChromeLaunchMode::default(),
            remote_debugging_port: None,
            chrome_executable_path: None,
            user_data_dir: None,
            browser_flags: None,
            enable_bidi: true,
            enable_cdp: false,
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
    driver: Option<BidiDriver<WebsocketConnectionTransport>>,
    chrome_process: Option<Process>,
    cdp_adapter: Option<CdpAdapter<WebsocketConnectionTransport>>,
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
        if matches!(config.launch_mode, ChromeLaunchMode::DriverManaged) && !config.enable_bidi {
            panic!("Config Enable Bidi must be set to true to use DriverManaged Mode");
        }
        if config.driver_executable_path.is_empty() {
            config.driver_executable_path = crate::downloader::ensure_chromedriver()
                .to_string_lossy()
                .into_owned();
        }
        let port = find_free_port().unwrap();
        config.port = Some(config.port.unwrap_or(port));

        let host = config.host.clone().unwrap_or(String::from("localhost"));
        let chrome_port = match &config.launch_mode {
            ChromeLaunchMode::Remote(port) => *port,
            ChromeLaunchMode::SpawnAndAttach => find_free_port().unwrap(),
            ChromeLaunchMode::DriverManaged => config
                .remote_debugging_port
                .unwrap_or_else(|| find_free_port().unwrap()),
        };
        config.remote_debugging_port = Some(chrome_port);

        let chrome_process = Self::init_chrome(&mut config, chrome_port).await;

        let mut ct_config = ConnectionTransportConfig::default();
        ct_config.host = host.clone();
        ct_config.port = port;

        let (cdp_adapter, driver) = match (config.enable_cdp, config.enable_bidi) {
            (true, true) => {
                let (cdp, bidi) = tokio::join!(
                    Self::init_cdp(&host, chrome_port),
                    Self::init_bidi(&config, &ct_config),
                );
                (Some(cdp), Some(bidi))
            }
            (true, false) => {
                let cdp = Self::init_cdp(&host, chrome_port).await;
                (Some(cdp), None)
            }
            (false, true) => {
                let bidi = Self::init_bidi(&config, &ct_config).await;
                (None, Some(bidi))
            }
            (false, false) => (None, None),
        };

        ChromeBrowser {
            config,
            driver,
            chrome_process,
            cdp_adapter,
        }
    }

    async fn init_chrome(config: &mut ChromeConfig, chrome_port: u16) -> Option<Process> {
        let (debugger_address, chrome_process) = match &config.launch_mode {
            ChromeLaunchMode::Remote(port) => (Some(format!("localhost:{}", port)), None),
            ChromeLaunchMode::SpawnAndAttach => {
                let chrome_exe = config.chrome_executable_path.clone().unwrap_or_else(|| {
                    crate::downloader::ensure_chrome()
                        .to_string_lossy()
                        .into_owned()
                });

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
                    "--start-maximized".to_string(),
                    "--disable-infobars".to_string(),
                ];
                if !config.sandbox {
                    chrome_args.push("--no-sandbox".to_string());
                }

                if let Some(ref flags) = config.browser_flags {
                    chrome_args.extend(flags.iter().cloned());
                }

                let chrome_proc = Process::create(chrome_exe, chrome_args);

                // Wait briefly for Chrome to start
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

                (
                    Some(format!("localhost:{}", chrome_port)),
                    Some(chrome_proc),
                )
            }
            ChromeLaunchMode::DriverManaged => {
                // Chromedriver spawns Chrome itself
                config
                    .capabilities
                    .add_arg(format!("remote-debugging-port={}", chrome_port));
                (None, None)
            }
        };

        if let Some(addr) = debugger_address {
            config.capabilities.debugger_address(addr);
        }
        config.capabilities.add_arg("start-maximized".to_string());
        config.capabilities.add_arg("disable-infobars".to_string());

        if !config.sandbox {
            config.capabilities.add_arg("no-sandbox".to_string());
        }

        if let Some(ref flags) = config.browser_flags {
            for flag in flags {
                config.capabilities.add_arg(flag.clone());
            }
        }

        if let Some(proxy) = config.proxy.clone() {
            config.capabilities.base_capabilities.proxy = Some(proxy);
        }

        chrome_process
    }

    async fn init_bidi(
        config: &ChromeConfig,
        ct_config: &ConnectionTransportConfig,
    ) -> BidiDriver<WebsocketConnectionTransport> {
        let capabilities = config.capabilities.clone().build();
        let (session, process) = start_bidi_driver(
            config,
            ct_config,
            SessionConnectionType::WebSocket,
            capabilities,
        )
        .await;

        let mut driver = BidiDriver::new(
            String::from("chromedriver"),
            vec![],
            session,
            0,
            Arc::new(Mutex::new(Vec::new())),
            process,
        );
        driver.listen_to_context_creation().await.unwrap();
        driver
    }

    async fn init_cdp(host: &str, chrome_port: u16) -> CdpAdapter<WebsocketConnectionTransport> {
        let ws_debugger_url = fetch_ws_debugger_url(host, chrome_port).await.unwrap();
        let cdp_cc = ConnectionTransportConfig::from_ws_url(&ws_debugger_url).unwrap();
        let cdp_session = start_cdp_session(&cdp_cc).await;
        let mut cdp_adapter = CdpAdapter::new(cdp_session);
        cdp_adapter.listen_to_target_creation().await.unwrap();
        cdp_adapter
    }

    pub async fn connect_bidi(&mut self) {
        if self.driver.is_some() {
            return;
        }
        let host = self
            .config
            .host
            .clone()
            .unwrap_or(String::from("localhost"));
        let port = self.config.port.unwrap();
        let mut ct_config = ConnectionTransportConfig::default();
        ct_config.host = host;
        ct_config.port = port;
        self.driver = Some(Self::init_bidi(&self.config, &ct_config).await);
    }

    pub async fn connect_cdp(&mut self) {
        if self.cdp_adapter.is_some() {
            return;
        }
        let host = self
            .config
            .host
            .clone()
            .unwrap_or(String::from("localhost"));
        let chrome_port = self
            .config
            .remote_debugging_port
            .expect("Remote debugging port not set");
        self.cdp_adapter = Some(Self::init_cdp(&host, chrome_port).await);
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
        self.driver
            .as_ref()
            .expect("BiDi is not enabled. Set `enable_bidi: true` in ChromeConfig.")
    }

    fn driver_mut(&mut self) -> &mut BidiDriver<WebsocketConnectionTransport> {
        self.driver
            .as_mut()
            .expect("BiDi is not enabled. Set `enable_bidi: true` in ChromeConfig.")
    }

    fn build_node(
        &self,
        raw_node: NodeRemoteValue,
        locator: Locator,
        context: BrowsingContext,
    ) -> ChromeNode<WebsocketConnectionTransport> {
        let driver = self.driver();
        ChromeNode::from_bidi(
            raw_node,
            locator,
            driver.session.clone(),
            context,
            driver.mouse.clone(),
        )
    }

    async fn close(mut self) -> Result<(), BrowserCloseError> {
        tracing::debug!("Closing ChromeBrowser");
        if let Some(ref mut driver) = self.driver {
            driver.end_session().await?;
        }
        if let Some(ref mut adapter) = self.cdp_adapter {
            adapter.close().await;
        }
        if let Some(mut process) = self.chrome_process.take() {
            process.kill()?;
        }
        tracing::debug!("ChromeBrowser closed");
        Ok(())
    }
}

impl CdpBrowser for ChromeBrowser {
    fn adapter(&self) -> &CdpAdapter<WebsocketConnectionTransport> {
        self.cdp_adapter
            .as_ref()
            .expect("CDP is not enabled. Set `enable_cdp: true` in ChromeConfig.")
    }

    fn adapter_mut(&mut self) -> &mut CdpAdapter<WebsocketConnectionTransport> {
        self.cdp_adapter
            .as_mut()
            .expect("CDP is not enabled. Set `enable_cdp: true` in ChromeConfig.")
    }
}

pub async fn create_chrome_browser(config: Option<ChromeConfig>) -> ChromeBrowser {
    let chrome_browser = ChromeBrowser::new(config.unwrap_or_default()).await;
    chrome_browser
}
