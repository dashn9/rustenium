use rustenium_bidi_commands::session::types::CapabilityRequest;
use rustenium_bidi_commands::Extensible;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

/// Chrome-specific capabilities for configuring browser behavior.
///
/// This struct provides a fluent API for setting Chrome capabilities and options.
/// Use the builder methods to configure the browser, then call `build()` to generate
/// the capabilities request for session creation.
///
/// # Examples
///
/// ```
/// use rustenium::browsers::ChromeCapabilities;
///
/// let mut caps = ChromeCapabilities::default();
/// caps.add_arg("--headless")
///     .add_arg("--disable-gpu")
///     .accept_insecure_certs(true);
///
/// let capabilities_request = caps.build();
/// ```
#[derive(Debug, Clone)]
pub struct ChromeCapabilities {
    pub base_capabilities: CapabilityRequest,
    pub chrome_options: ChromeOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ChromeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_state: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefs: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detach: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debugger_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_switches: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minidump_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_emulation: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perf_logging_prefs: Option<PerfLoggingPrefs>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_extension_targets: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_automation_extension: Option<bool>,
}

/// Performance logging preferences for Chrome.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PerfLoggingPrefs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_network: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_page: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_categories: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_usage_reporting_interval: Option<u32>,
}

impl PerfLoggingPrefs {
    /// Creates new performance logging preferences.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enables or disables network performance logging.
    pub fn enable_network(&mut self, enable: bool) -> &mut Self {
        self.enable_network = Some(enable);
        self
    }

    /// Enables or disables page performance logging.
    pub fn enable_page(&mut self, enable: bool) -> &mut Self {
        self.enable_page = Some(enable);
        self
    }

    /// Sets the trace categories to capture.
    pub fn trace_categories(&mut self, categories: impl Into<String>) -> &mut Self {
        self.trace_categories = Some(categories.into());
        self
    }

    /// Sets the buffer usage reporting interval in milliseconds.
    pub fn buffer_usage_reporting_interval(&mut self, interval: u32) -> &mut Self {
        self.buffer_usage_reporting_interval = Some(interval);
        self
    }
}

impl Default for ChromeCapabilities {
    fn default() -> Self {
        ChromeCapabilities {
            base_capabilities: CapabilityRequest {
                accept_insecure_certs: None,
                browser_name: Some("chrome".to_string()),
                browser_version: None,
                platform_name: None,
                proxy: None,
                unhandled_prompt_behavior: None,
                extensible: Extensible::default(),
            },
            chrome_options: ChromeOptions::default(),
        }
    }
}

impl ChromeCapabilities {
    /// Creates a new ChromeCapabilities instance.
    pub fn new(base_capabilities: Option<CapabilityRequest>) -> Self {
        let mut caps = Self::default();
        if let Some(base) = base_capabilities {
            caps.base_capabilities = base;
        }
        caps
    }

    /// Sets whether to accept insecure (self-signed) certificates.
    pub fn accept_insecure_certs(&mut self, accept: bool) -> &mut Self {
        self.base_capabilities.accept_insecure_certs = Some(accept);
        self
    }

    /// Sets the browser name.
    pub fn browser_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.base_capabilities.browser_name = Some(name.into());
        self
    }

    /// Sets the desired browser version.
    pub fn browser_version(&mut self, version: impl Into<String>) -> &mut Self {
        self.base_capabilities.browser_version = Some(version.into());
        self
    }

    /// Sets the platform name.
    pub fn platform_name(&mut self, platform: impl Into<String>) -> &mut Self {
        self.base_capabilities.platform_name = Some(platform.into());
        self
    }

    /// Configures proxy settings.
    pub fn proxy(&mut self, proxy: rustenium_bidi_commands::session::types::ProxyConfiguration) -> &mut Self {
        self.base_capabilities.proxy = Some(proxy);
        self
    }

    /// Sets how to handle unhandled prompts.
    pub fn unhandled_prompt_behavior(&mut self, behavior: rustenium_bidi_commands::session::types::UserPromptHandler) -> &mut Self {
        self.base_capabilities.unhandled_prompt_behavior = Some(behavior);
        self
    }

    /// Sets Chrome command-line arguments.
    pub fn args(&mut self, args: Vec<String>) -> &mut Self {
        self.chrome_options.args = Some(args);
        self
    }

    /// Adds a single Chrome command-line argument (e.g., "--headless", "--disable-gpu").
    pub fn add_arg(&mut self, arg: impl Into<String>) -> &mut Self {
        if let Some(ref mut args) = self.chrome_options.args {
            args.push(arg.into());
        } else {
            self.chrome_options.args = Some(vec![arg.into()]);
        }
        self
    }

    /// Adds multiple Chrome command-line arguments.
    pub fn add_args<I, S>(&mut self, new_args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        if let Some(ref mut args) = self.chrome_options.args {
            args.extend(new_args.into_iter().map(|s| s.into()));
        } else {
            self.chrome_options.args = Some(new_args.into_iter().map(|s| s.into()).collect());
        }
        self
    }

    /// Sets the path to the Chrome binary.
    pub fn binary(&mut self, binary: impl Into<String>) -> &mut Self {
        self.chrome_options.binary = Some(binary.into());
        self
    }

    /// Sets Chrome extensions to load (as base64-encoded .crx files).
    pub fn extensions(&mut self, extensions: Vec<String>) -> &mut Self {
        self.chrome_options.extensions = Some(extensions);
        self
    }

    /// Adds a Chrome extension to load.
    pub fn add_extension(&mut self, extension: impl Into<String>) -> &mut Self {
        if let Some(ref mut extensions) = self.chrome_options.extensions {
            extensions.push(extension.into());
        } else {
            self.chrome_options.extensions = Some(vec![extension.into()]);
        }
        self
    }

    /// Sets Chrome's local state preferences.
    pub fn local_state(&mut self, local_state: HashMap<String, Value>) -> &mut Self {
        self.chrome_options.local_state = Some(local_state);
        self
    }

    /// Sets Chrome user preferences.
    pub fn prefs(&mut self, prefs: HashMap<String, Value>) -> &mut Self {
        self.chrome_options.prefs = Some(prefs);
        self
    }

    /// Adds a single Chrome user preference.
    pub fn add_pref(&mut self, key: impl Into<String>, value: Value) -> &mut Self {
        if let Some(ref mut prefs) = self.chrome_options.prefs {
            prefs.insert(key.into(), value);
        } else {
            let mut prefs = HashMap::new();
            prefs.insert(key.into(), value);
            self.chrome_options.prefs = Some(prefs);
        }
        self
    }

    /// Sets whether Chrome should continue running after the driver disconnects.
    pub fn detach(&mut self, detach: bool) -> &mut Self {
        self.chrome_options.detach = Some(detach);
        self
    }

    /// Sets the debugger address to connect to an existing Chrome instance.
    pub fn debugger_address(&mut self, address: impl Into<String>) -> &mut Self {
        self.chrome_options.debugger_address = Some(address.into());
        self
    }

    /// Sets command-line switches to exclude.
    pub fn exclude_switches(&mut self, switches: Vec<String>) -> &mut Self {
        self.chrome_options.exclude_switches = Some(switches);
        self
    }

    /// Adds a command-line switch to exclude.
    pub fn add_exclude_switch(&mut self, switch: impl Into<String>) -> &mut Self {
        if let Some(ref mut switches) = self.chrome_options.exclude_switches {
            switches.push(switch.into());
        } else {
            self.chrome_options.exclude_switches = Some(vec![switch.into()]);
        }
        self
    }

    /// Sets the directory path for Chrome minidump files.
    pub fn minidump_path(&mut self, path: impl Into<String>) -> &mut Self {
        self.chrome_options.minidump_path = Some(path.into());
        self
    }

    /// Configures mobile device emulation.
    pub fn mobile_emulation(&mut self, emulation: HashMap<String, Value>) -> &mut Self {
        self.chrome_options.mobile_emulation = Some(emulation);
        self
    }

    /// Sets performance logging preferences.
    pub fn perf_logging_prefs(&mut self, prefs: PerfLoggingPrefs) -> &mut Self {
        self.chrome_options.perf_logging_prefs = Some(prefs);
        self
    }

    /// Sets which window types to consider.
    pub fn window_types(&mut self, types: Vec<String>) -> &mut Self {
        self.chrome_options.window_types = Some(types);
        self
    }

    /// Adds a window type to consider.
    pub fn add_window_type(&mut self, window_type: impl Into<String>) -> &mut Self {
        if let Some(ref mut types) = self.chrome_options.window_types {
            types.push(window_type.into());
        } else {
            self.chrome_options.window_types = Some(vec![window_type.into()]);
        }
        self
    }

    /// Sets whether to enable extension targets.
    pub fn enable_extension_targets(&mut self, enable: bool) -> &mut Self {
        self.chrome_options.enable_extension_targets = Some(enable);
        self
    }

    /// Sets whether to use the automation extension.
    pub fn use_automation_extension(&mut self, use_it: bool) -> &mut Self {
        self.chrome_options.use_automation_extension = Some(use_it);
        self
    }

    /// Builds the final CapabilitiesRequest for session creation.
    pub fn build(mut self) -> rustenium_bidi_commands::session::types::CapabilitiesRequest {
        // Serialize chrome_options and add to extensible under goog:chromeOptions
        let chrome_opts_json = serde_json::to_value(&self.chrome_options).unwrap();

        self.base_capabilities.extensible.insert(
            "goog:chromeOptions".to_string(),
            chrome_opts_json,
        );

        // Create CapabilitiesRequest
        rustenium_bidi_commands::session::types::CapabilitiesRequest {
            always_match: Some(self.base_capabilities),
            first_match: None,
        }
    }
}
