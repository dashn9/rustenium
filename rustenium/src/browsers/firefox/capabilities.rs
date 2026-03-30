use rustenium_bidi_definitions::session::types::{
    CapabilitiesRequest, CapabilityRequest, ProxyConfiguration, UnhandledPromptBehavior,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FirefoxCapabilities {
    pub base_capabilities: CapabilityRequest,
    pub firefox_options: FirefoxOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirefoxOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefs: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<FirefoxLog>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_package: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirefoxLog {
    pub level: String,
}

impl Default for FirefoxCapabilities {
    fn default() -> Self {
        FirefoxCapabilities {
            base_capabilities: CapabilityRequest {
                accept_insecure_certs: None,
                browser_name: Some("firefox".to_string()),
                browser_version: None,
                platform_name: None,
                proxy: None,
                unhandled_prompt_behavior: None,
                extensible: HashMap::new(),
            },
            firefox_options: FirefoxOptions::default(),
        }
    }
}

impl FirefoxCapabilities {
    pub fn new(base_capabilities: Option<CapabilityRequest>) -> Self {
        let mut caps = Self::default();
        if let Some(base) = base_capabilities {
            caps.base_capabilities = base;
        }
        caps
    }

    pub fn accept_insecure_certs(&mut self, accept: bool) -> &mut Self {
        self.base_capabilities.accept_insecure_certs = Some(accept);
        self
    }

    pub fn browser_name(&mut self, name: impl Into<String>) -> &mut Self {
        self.base_capabilities.browser_name = Some(name.into());
        self
    }

    pub fn browser_version(&mut self, version: impl Into<String>) -> &mut Self {
        self.base_capabilities.browser_version = Some(version.into());
        self
    }

    pub fn platform_name(&mut self, platform: impl Into<String>) -> &mut Self {
        self.base_capabilities.platform_name = Some(platform.into());
        self
    }

    pub fn proxy(&mut self, proxy: ProxyConfiguration) -> &mut Self {
        self.base_capabilities.proxy = Some(proxy);
        self
    }

    pub fn unhandled_prompt_behavior(&mut self, behavior: UnhandledPromptBehavior) -> &mut Self {
        self.base_capabilities.unhandled_prompt_behavior = Some(behavior);
        self
    }

    pub fn args(&mut self, args: Vec<String>) -> &mut Self {
        self.firefox_options.args = Some(args);
        self
    }

    pub fn add_arg(&mut self, arg: impl Into<String>) -> &mut Self {
        if let Some(ref mut args) = self.firefox_options.args {
            args.push(arg.into());
        } else {
            self.firefox_options.args = Some(vec![arg.into()]);
        }
        self
    }

    pub fn add_args<I, S>(&mut self, new_args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        if let Some(ref mut args) = self.firefox_options.args {
            args.extend(new_args.into_iter().map(|s| s.into()));
        } else {
            self.firefox_options.args = Some(new_args.into_iter().map(|s| s.into()).collect());
        }
        self
    }

    pub fn binary(&mut self, binary: impl Into<String>) -> &mut Self {
        self.firefox_options.binary = Some(binary.into());
        self
    }

    pub fn profile(&mut self, profile: impl Into<String>) -> &mut Self {
        self.firefox_options.profile = Some(profile.into());
        self
    }

    pub fn prefs(&mut self, prefs: HashMap<String, Value>) -> &mut Self {
        self.firefox_options.prefs = Some(prefs);
        self
    }

    pub fn add_pref(&mut self, key: impl Into<String>, value: Value) -> &mut Self {
        if let Some(ref mut prefs) = self.firefox_options.prefs {
            prefs.insert(key.into(), value);
        } else {
            let mut prefs = HashMap::new();
            prefs.insert(key.into(), value);
            self.firefox_options.prefs = Some(prefs);
        }
        self
    }

    pub fn log_level(&mut self, level: impl Into<String>) -> &mut Self {
        self.firefox_options.log = Some(FirefoxLog {
            level: level.into(),
        });
        self
    }

    pub fn env(&mut self, env: HashMap<String, String>) -> &mut Self {
        self.firefox_options.env = Some(env);
        self
    }

    pub fn add_env(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        if let Some(ref mut env) = self.firefox_options.env {
            env.insert(key.into(), value.into());
        } else {
            let mut env = HashMap::new();
            env.insert(key.into(), value.into());
            self.firefox_options.env = Some(env);
        }
        self
    }

    pub fn build(mut self) -> CapabilitiesRequest {
        let firefox_opts_json = serde_json::to_value(&self.firefox_options).unwrap();

        self.base_capabilities
            .extensible
            .insert("moz:firefoxOptions".to_string(), firefox_opts_json);

        // Request BiDi WebSocket URL from geckodriver
        self.base_capabilities
            .extensible
            .insert("webSocketUrl".to_string(), serde_json::Value::Bool(true));

        CapabilitiesRequest {
            always_match: Some(self.base_capabilities),
            first_match: None,
        }
    }
}
