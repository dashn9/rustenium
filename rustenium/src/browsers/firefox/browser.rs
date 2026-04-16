use super::capabilities::FirefoxCapabilities;
use crate::browsers::BidiBrowser;
use crate::conduit::bidi::drivers::BidiDriver;
use crate::error::bidi::BrowserCloseError;
use crate::nodes::FirefoxNode;
use rustenium_bidi_definitions::browsing_context::types::{BrowsingContext, Locator};
use rustenium_bidi_definitions::script::types::NodeRemoteValue;
use rustenium_bidi_definitions::session::types::ProxyConfiguration;
use rustenium_core::find_free_port;
use rustenium_core::process::Process;
use rustenium_core::transport::{ConnectionTransportConfig, WebsocketConnectionTransport};
use std::sync::{Arc, Mutex};

/// How Firefox is launched and managed.
#[derive(Debug, Clone)]
pub enum FirefoxLaunchMode {
    /// Rustenium starts Firefox and connects to its BiDi WebSocket directly (default).
    SpawnAndAttach,
    /// Connect to an existing Firefox instance on the specified remote debugging port.
    Remote(u16),
}

impl Default for FirefoxLaunchMode {
    fn default() -> Self {
        FirefoxLaunchMode::SpawnAndAttach
    }
}

/// Configuration for Firefox browser.
#[derive(Debug, Clone)]
pub struct FirefoxConfig {
    /// Host (default: localhost).
    pub host: Option<String>,

    /// Firefox capabilities for configuring browser behavior.
    pub capabilities: FirefoxCapabilities,

    /// Optional proxy configuration.
    pub proxy: Option<ProxyConfiguration>,

    /// How Firefox is launched and managed.
    pub launch_mode: FirefoxLaunchMode,

    /// Remote debugging port for Firefox BiDi WebSocket. Auto-assigned if None.
    pub remote_debugging_port: Option<u16>,

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
            host: None,
            capabilities: FirefoxCapabilities::default(),
            proxy: None,
            launch_mode: FirefoxLaunchMode::default(),
            remote_debugging_port: None,
            firefox_executable_path: None,
            profile_dir: None,
            browser_flags: None,
        }
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
        let host = config.host.clone().unwrap_or(String::from("localhost"));
        let firefox_port = match &config.launch_mode {
            FirefoxLaunchMode::Remote(port) => *port,
            FirefoxLaunchMode::SpawnAndAttach => config
                .remote_debugging_port
                .unwrap_or_else(|| find_free_port().unwrap()),
        };
        config.remote_debugging_port = Some(firefox_port);

        let firefox_process = Self::init_firefox(&mut config, firefox_port).await;

        let mut ct_config = ConnectionTransportConfig::default();
        ct_config.host = host;
        ct_config.port = firefox_port;

        let driver = Self::init_bidi(&mut config, &ct_config).await;

        FirefoxBrowser {
            config,
            driver: Some(driver),
            firefox_process,
        }
    }

    async fn init_firefox(config: &mut FirefoxConfig, firefox_port: u16) -> Option<Process> {
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
                        .join(format!("rustenium-firefox-{}", firefox_port))
                        .display()
                        .to_string()
                });

                let _ = std::fs::create_dir_all(&profile_dir);

                let mut firefox_args = vec![
                    format!("--remote-debugging-port={}", firefox_port),
                    "--profile".to_string(),
                    profile_dir,
                    "--no-remote".to_string(),
                ];

                if let Some(ref flags) = config.browser_flags {
                    firefox_args.extend(flags.iter().cloned());
                }

                if let Some(proxy) = config.proxy.clone() {
                    config.capabilities.base_capabilities.proxy = Some(proxy);
                }

                let firefox_proc = Process::create_with_env(
                    firefox_exe,
                    firefox_args,
                    [("MOZ_LAUNCHER_PROCESS".to_string(), "0".to_string())],
                );

                // Wait for Firefox to start
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

                Some(firefox_proc)
            }
        }
    }

    async fn init_bidi(
        config: &mut FirefoxConfig,
        ct_config: &ConnectionTransportConfig,
    ) -> BidiDriver<WebsocketConnectionTransport> {
        let capabilities = config.capabilities.clone().build();

        // Firefox exposes BiDi WebSocket directly — connect the same way as Chrome
        let session = rustenium_core::BidiSession::<WebsocketConnectionTransport>::new(
            ct_config,
            capabilities,
        )
        .await;

        let session = Arc::new(tokio::sync::Mutex::new(session));

        let mut driver = BidiDriver::new(
            String::from("firefox"),
            vec![],
            session,
            0,
            Arc::new(Mutex::new(Vec::new())),
            // No driver process — Firefox handles BiDi natively
            #[cfg(unix)]
            Process::create("echo", vec!["hello".to_string()]),
            #[cfg(windows)]
            Process::create(
                "cmd",
                vec!["/C".to_string(), "echo".to_string(), "hello".to_string()],
            ),
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
        let port = self.config.remote_debugging_port.unwrap();
        let mut ct_config = ConnectionTransportConfig::default();
        ct_config.host = host;
        ct_config.port = port;
        self.driver = Some(Self::init_bidi(&mut self.config, &ct_config).await);
    }

    pub fn get_config(&self) -> &FirefoxConfig {
        &self.config
    }

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
            driver.keyboard.clone(),
        )
    }

    async fn close(mut self) -> Result<(), BrowserCloseError> {
        tracing::debug!("Closing FirefoxBrowser");
        if let Some(ref mut driver) = self.driver {
            driver.end_session().await?;
        }
        // Drop the stored process (best-effort; may be stale if Firefox respawned)
        drop(self.firefox_process.take());
        // Kill by port to catch the actual Firefox process regardless of PID
        if let Some(port) = self.config.remote_debugging_port {
            rustenium_core::process::kill_process_on_port(port);
        }
        tracing::debug!("FirefoxBrowser closed");
        Ok(())
    }
}

pub async fn firefox(config: Option<FirefoxConfig>) -> FirefoxBrowser {
    FirefoxBrowser::new(config.unwrap_or_default()).await
}
