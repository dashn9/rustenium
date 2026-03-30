use super::capabilities::FirefoxCapabilities;
use crate::browsers::BidiBrowser;
use crate::conduit::bidi::drivers::{BidiDriver, DriverConfiguration, start_bidi_driver};
use crate::error::bidi::BrowserCloseError;
use crate::nodes::FirefoxNode;
use rustenium_bidi_definitions::browsing_context::types::{BrowsingContext, Locator};
use rustenium_bidi_definitions::script::types::NodeRemoteValue;
use rustenium_bidi_definitions::session::types::ProxyConfiguration;
use rustenium_core::find_free_port;
use rustenium_core::process::Process;
use rustenium_core::session::SessionConnectionType;
use rustenium_core::transport::{ConnectionTransportConfig, WebsocketConnectionTransport};
use std::sync::{Arc, Mutex};

/// How Firefox is launched and managed.
#[derive(Debug, Clone)]
pub enum FirefoxLaunchMode {
    /// Rustenium starts Firefox and attaches geckodriver to it (default).
    SpawnAndAttach,
    /// Connect to an existing Firefox instance on the specified Marionette port.
    Remote(u16),
    /// Let geckodriver spawn and manage Firefox.
    DriverManaged,
}

impl Default for FirefoxLaunchMode {
    fn default() -> Self {
        FirefoxLaunchMode::SpawnAndAttach
    }
}

/// Configuration for Firefox browser and geckodriver.
#[derive(Debug, Clone)]
pub struct FirefoxConfig {
    /// Path to the geckodriver executable.
    pub driver_executable_path: String,

    /// Host for the WebDriver server (default: localhost).
    pub host: Option<String>,

    /// Port for the WebDriver server (auto-assigned if None).
    pub port: Option<u16>,

    /// Additional flags to pass to geckodriver.
    pub driver_flags: Vec<&'static str>,

    /// Firefox capabilities for configuring browser behavior.
    pub capabilities: FirefoxCapabilities,

    /// Optional proxy configuration.
    pub proxy: Option<ProxyConfiguration>,

    /// How Firefox is launched and managed.
    pub launch_mode: FirefoxLaunchMode,

    /// Marionette port for Firefox. Auto-assigned if None.
    pub marionette_port: Option<u16>,

    /// Path to Firefox executable.
    /// Defaults to auto-downloaded Firefox if not specified.
    pub firefox_executable_path: Option<String>,

    /// Firefox profile directory. If not specified, uses a temporary directory.
    pub profile_dir: Option<String>,

    /// Additional Firefox command-line arguments.
    pub browser_flags: Option<Vec<String>>,
}

impl Default for FirefoxConfig {
    fn default() -> Self {
        FirefoxConfig {
            driver_executable_path: "".to_string(),
            host: None,
            port: None,
            driver_flags: Vec::new(),
            capabilities: FirefoxCapabilities::default(),
            proxy: None,
            launch_mode: FirefoxLaunchMode::default(),
            marionette_port: None,
            firefox_executable_path: None,
            profile_dir: None,
            browser_flags: None,
        }
    }
}

impl DriverConfiguration for FirefoxConfig {
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

        if let Some(marionette_port) = self.marionette_port {
            flags.push(format!("--marionette-port={}", marionette_port));
        }

        flags.extend(self.driver_flags.iter().map(|s| s.to_string()));

        flags
    }
}

pub struct FirefoxBrowser {
    config: FirefoxConfig,
    driver: Option<BidiDriver<WebsocketConnectionTransport>>,
    firefox_process: Option<Process>,
}

impl std::fmt::Debug for FirefoxBrowser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FirefoxBrowser")
            .field("config", &self.config)
            .field("firefox_process", &self.firefox_process)
            .finish_non_exhaustive()
    }
}

impl FirefoxBrowser {
    pub async fn new(mut config: FirefoxConfig) -> FirefoxBrowser {
        let port = find_free_port().unwrap();
        config.port = Some(config.port.unwrap_or(port));

        let host = config.host.clone().unwrap_or(String::from("localhost"));
        let marionette_port = match &config.launch_mode {
            FirefoxLaunchMode::Remote(port) => *port,
            FirefoxLaunchMode::SpawnAndAttach => config
                .marionette_port
                .unwrap_or_else(|| find_free_port().unwrap()),
            FirefoxLaunchMode::DriverManaged => config
                .marionette_port
                .unwrap_or_else(|| find_free_port().unwrap()),
        };
        config.marionette_port = Some(marionette_port);

        let firefox_process = Self::init_firefox(&mut config, marionette_port).await;

        let mut ct_config = ConnectionTransportConfig::default();
        ct_config.host = host;
        ct_config.port = port;

        let driver = Self::init_bidi(&mut config, &ct_config).await;

        FirefoxBrowser {
            config,
            driver: Some(driver),
            firefox_process,
        }
    }

    async fn init_firefox(config: &mut FirefoxConfig, marionette_port: u16) -> Option<Process> {
        match &config.launch_mode {
            FirefoxLaunchMode::Remote(_) => None,
            FirefoxLaunchMode::SpawnAndAttach => {
                let firefox_exe = config.firefox_executable_path.clone().unwrap_or_else(|| {
                    crate::downloader::ensure_firefox()
                        .to_string_lossy()
                        .into_owned()
                });

                let profile_dir = config.profile_dir.clone().unwrap_or_else(|| {
                    std::env::temp_dir()
                        .join(format!("rustenium-firefox-{}", marionette_port))
                        .display()
                        .to_string()
                });

                // Ensure profile directory exists
                let _ = std::fs::create_dir_all(&profile_dir);

                let mut firefox_args = vec![
                    "--marionette".to_string(),
                    format!("--marionette-port={}", marionette_port),
                    "--profile".to_string(),
                    profile_dir,
                    "--no-remote".to_string(),
                ];

                if let Some(ref flags) = config.browser_flags {
                    firefox_args.extend(flags.iter().cloned());
                }

                // Set the binary in capabilities so geckodriver knows where Firefox is
                config.capabilities.binary(firefox_exe.clone());

                let firefox_proc = Process::create(firefox_exe, firefox_args);

                // Wait briefly for Firefox to start
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

                Some(firefox_proc)
            }
            FirefoxLaunchMode::DriverManaged => {
                // geckodriver spawns Firefox itself
                if let Some(ref exe) = config.firefox_executable_path {
                    config.capabilities.binary(exe.clone());
                }
                None
            }
        }
    }

    async fn init_bidi(
        config: &mut FirefoxConfig,
        ct_config: &ConnectionTransportConfig,
    ) -> BidiDriver<WebsocketConnectionTransport> {
        if config.driver_executable_path.is_empty() {
            config.driver_executable_path = crate::downloader::ensure_geckodriver()
                .to_string_lossy()
                .into_owned();
        }

        if let Some(proxy) = config.proxy.clone() {
            config.capabilities.base_capabilities.proxy = Some(proxy);
        }

        let capabilities = config.capabilities.clone().build();

        let (session, process) = start_bidi_driver(
            config,
            ct_config,
            SessionConnectionType::HttpFirst {
                host: ct_config.host.clone(),
                port: ct_config.port,
            },
            capabilities,
        )
        .await;

        let mut driver = BidiDriver::new(
            String::from("geckodriver"),
            vec![],
            session,
            0,
            Arc::new(Mutex::new(Vec::new())),
            process,
        );
        driver.listen_to_context_creation().await.unwrap();
        driver
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
        self.driver = Some(Self::init_bidi(&mut self.config, &ct_config).await);
    }

    /// Returns a reference to the Firefox configuration.
    pub fn get_config(&self) -> &FirefoxConfig {
        &self.config
    }

    /// Returns a reference to the Firefox browser process.
    pub fn get_browser_process(&self) -> &Option<Process> {
        &self.firefox_process
    }
}

impl BidiBrowser for FirefoxBrowser {
    type Transport = WebsocketConnectionTransport;
    type BrowserNode = FirefoxNode<WebsocketConnectionTransport>;

    fn driver(&self) -> &BidiDriver<WebsocketConnectionTransport> {
        self.driver
            .as_ref()
            .expect("BiDi driver is not initialized.")
    }

    fn driver_mut(&mut self) -> &mut BidiDriver<WebsocketConnectionTransport> {
        self.driver
            .as_mut()
            .expect("BiDi driver is not initialized.")
    }

    fn build_node(
        &self,
        raw_node: NodeRemoteValue,
        locator: Locator,
        context: BrowsingContext,
    ) -> FirefoxNode<WebsocketConnectionTransport> {
        let driver = self.driver();
        FirefoxNode::from_bidi(
            raw_node,
            locator,
            driver.session.clone(),
            context,
            driver.mouse.clone(),
        )
    }

    async fn close(mut self) -> Result<(), BrowserCloseError> {
        tracing::debug!("Closing FirefoxBrowser");
        if let Some(ref mut driver) = self.driver {
            driver.end_session().await?;
        }
        if let Some(mut process) = self.firefox_process.take() {
            process.kill()?;
        }
        tracing::debug!("FirefoxBrowser closed");
        Ok(())
    }
}

pub async fn firefox(config: Option<FirefoxConfig>) -> FirefoxBrowser {
    FirefoxBrowser::new(config.unwrap_or_default()).await
}
