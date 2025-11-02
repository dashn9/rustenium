use rustenium_bidi_commands::session::types::CapabilityRequest;
use rustenium_bidi_commands::Extensible;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChromeCapabilities {
    #[serde(flatten)]
    pub base_capabilities: CapabilityRequest,
    pub args: Option<Vec<String>>,
    pub binary: Option<String>,
    pub extensions: Option<Vec<String>>,
    pub local_state: Option<HashMap<String, Value>>,
    pub prefs: Option<HashMap<String, Value>>,
    pub detach: Option<bool>,
    pub debugger_address: Option<String>,
    pub exclude_switches: Option<Vec<String>>,
    pub minidump_path: Option<String>,
    pub mobile_emulation: Option<HashMap<String, Value>>,
    pub perf_logging_prefs: Option<PerfLoggingPrefs>,
    pub window_types: Option<Vec<String>>,
    pub enable_extension_targets: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfLoggingPrefs {
    pub enable_network: Option<bool>,
    pub enable_page: Option<bool>,
    pub trace_categories: Option<String>,
    pub buffer_usage_reporting_interval: Option<u32>,
}

impl Default for PerfLoggingPrefs {
    fn default() -> Self {
        PerfLoggingPrefs {
            enable_network: None,
            enable_page: None,
            trace_categories: None,
            buffer_usage_reporting_interval: None,
        }
    }
}

impl PerfLoggingPrefs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn enable_network(mut self, enable: bool) -> Self {
        self.enable_network = Some(enable);
        self
    }

    pub fn enable_page(mut self, enable: bool) -> Self {
        self.enable_page = Some(enable);
        self
    }

    pub fn trace_categories(mut self, categories: String) -> Self {
        self.trace_categories = Some(categories);
        self
    }

    pub fn buffer_usage_reporting_interval(mut self, interval: u32) -> Self {
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
            args: None,
            binary: None,
            extensions: None,
            local_state: None,
            prefs: None,
            detach: None,
            debugger_address: None,
            exclude_switches: None,
            minidump_path: None,
            mobile_emulation: None,
            perf_logging_prefs: None,
            window_types: None,
            enable_extension_targets: None,
        }
    }
}

impl ChromeCapabilities {
    pub fn new(base_capabilities: Option<CapabilityRequest>) -> Self {
        let mut caps = Self::default();
        if let Some(base) = base_capabilities {
            caps.base_capabilities = base;
        }
        caps
    }

    // CapabilityRequest setters
    pub fn accept_insecure_certs(mut self, accept: bool) -> Self {
        self.base_capabilities.accept_insecure_certs = Some(accept);
        self
    }

    pub fn browser_name(mut self, name: String) -> Self {
        self.base_capabilities.browser_name = Some(name);
        self
    }

    pub fn browser_version(mut self, version: String) -> Self {
        self.base_capabilities.browser_version = Some(version);
        self
    }

    pub fn platform_name(mut self, platform: String) -> Self {
        self.base_capabilities.platform_name = Some(platform);
        self
    }

    pub fn proxy(mut self, proxy: rustenium_bidi_commands::session::types::ProxyConfiguration) -> Self {
        self.base_capabilities.proxy = Some(proxy);
        self
    }

    pub fn unhandled_prompt_behavior(mut self, behavior: rustenium_bidi_commands::session::types::UserPromptHandler) -> Self {
        self.base_capabilities.unhandled_prompt_behavior = Some(behavior);
        self
    }

    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args = Some(args);
        self
    }

    pub fn add_arg(mut self, arg: String) -> Self {
        if let Some(ref mut args) = self.args {
            args.push(arg);
        } else {
            self.args = Some(vec![arg]);
        }
        self
    }

    pub fn binary(mut self, binary: String) -> Self {
        self.binary = Some(binary);
        self
    }

    pub fn extensions(mut self, extensions: Vec<String>) -> Self {
        self.extensions = Some(extensions);
        self
    }

    pub fn add_extension(mut self, extension: String) -> Self {
        if let Some(ref mut extensions) = self.extensions {
            extensions.push(extension);
        } else {
            self.extensions = Some(vec![extension]);
        }
        self
    }

    pub fn local_state(mut self, local_state: HashMap<String, Value>) -> Self {
        self.local_state = Some(local_state);
        self
    }

    pub fn prefs(mut self, prefs: HashMap<String, Value>) -> Self {
        self.prefs = Some(prefs);
        self
    }

    pub fn add_pref(mut self, key: String, value: Value) -> Self {
        if let Some(ref mut prefs) = self.prefs {
            prefs.insert(key, value);
        } else {
            let mut prefs = HashMap::new();
            prefs.insert(key, value);
            self.prefs = Some(prefs);
        }
        self
    }

    pub fn detach(mut self, detach: bool) -> Self {
        self.detach = Some(detach);
        self
    }

    pub fn debugger_address(mut self, address: String) -> Self {
        self.debugger_address = Some(address);
        self
    }

    pub fn exclude_switches(mut self, switches: Vec<String>) -> Self {
        self.exclude_switches = Some(switches);
        self
    }

    pub fn add_exclude_switch(mut self, switch: String) -> Self {
        if let Some(ref mut switches) = self.exclude_switches {
            switches.push(switch);
        } else {
            self.exclude_switches = Some(vec![switch]);
        }
        self
    }

    pub fn minidump_path(mut self, path: String) -> Self {
        self.minidump_path = Some(path);
        self
    }

    pub fn mobile_emulation(mut self, emulation: HashMap<String, Value>) -> Self {
        self.mobile_emulation = Some(emulation);
        self
    }

    pub fn perf_logging_prefs(mut self, prefs: PerfLoggingPrefs) -> Self {
        self.perf_logging_prefs = Some(prefs);
        self
    }

    pub fn window_types(mut self, types: Vec<String>) -> Self {
        self.window_types = Some(types);
        self
    }

    pub fn add_window_type(mut self, window_type: String) -> Self {
        if let Some(ref mut types) = self.window_types {
            types.push(window_type);
        } else {
            self.window_types = Some(vec![window_type]);
        }
        self
    }

    pub fn enable_extension_targets(mut self, enable: bool) -> Self {
        self.enable_extension_targets = Some(enable);
        self
    }

    /// Build CapabilitiesRequest for session creation
    pub fn build(self) -> rustenium_bidi_commands::session::types::CapabilitiesRequest {
        use serde_json::json;

        // Serialize ChromeCapabilities to JSON
        let chrome_caps_json = serde_json::to_value(&self).unwrap();

        // Create CapabilitiesRequest with ChromeCapabilities as alwaysMatch
        rustenium_bidi_commands::session::types::CapabilitiesRequest {
            always_match: Some(serde_json::from_value(chrome_caps_json).unwrap()),
            first_match: None,
        }
    }
}