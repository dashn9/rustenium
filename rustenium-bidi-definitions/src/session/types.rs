use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CapabilitiesRequest {
    #[serde(rename = "alwaysMatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub always_match: Option<CapabilityRequest>,
    #[serde(rename = "firstMatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub first_match: Option<Vec<CapabilityRequest>>,
}
impl CapabilitiesRequest {
    pub const IDENTIFIER: &'static str = "session.CapabilitiesRequest";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityRequest {
    #[serde(rename = "acceptInsecureCerts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub accept_insecure_certs: Option<bool>,
    #[serde(rename = "browserName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub browser_name: Option<String>,
    #[serde(rename = "browserVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub browser_version: Option<String>,
    #[serde(rename = "platformName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub platform_name: Option<String>,
    #[serde(rename = "proxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub proxy: Option<ProxyConfiguration>,
    #[serde(rename = "unhandledPromptBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unhandled_prompt_behavior: Option<UserPromptHandler>,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl CapabilityRequest {
    pub fn new(
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            extensible: extensible.into(),
            accept_insecure_certs: None,
            browser_name: None,
            browser_version: None,
            platform_name: None,
            proxy: None,
            unhandled_prompt_behavior: None,
        }
    }
}
impl CapabilityRequest {
    pub const IDENTIFIER: &'static str = "session.CapabilityRequest";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: String,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl ProxyConfiguration {
    pub fn new(
        proxy_type: impl Into<String>,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            proxy_type: proxy_type.into(),
            extensible: extensible.into(),
        }
    }
}
impl ProxyConfiguration {
    pub const IDENTIFIER: &'static str = "session.ProxyConfiguration";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutodetectProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: String,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl AutodetectProxyConfiguration {
    pub fn new(
        proxy_type: impl Into<String>,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            proxy_type: proxy_type.into(),
            extensible: extensible.into(),
        }
    }
}
impl AutodetectProxyConfiguration {
    pub const IDENTIFIER: &'static str = "session.AutodetectProxyConfiguration";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: String,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl DirectProxyConfiguration {
    pub fn new(
        proxy_type: impl Into<String>,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            proxy_type: proxy_type.into(),
            extensible: extensible.into(),
        }
    }
}
impl DirectProxyConfiguration {
    pub const IDENTIFIER: &'static str = "session.DirectProxyConfiguration";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManualProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: String,
    #[serde(rename = "httpProxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub http_proxy: Option<String>,
    #[serde(rename = "sslProxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ssl_proxy: Option<String>,
    #[serde(rename = "noProxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub no_proxy: Option<Vec<String>>,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl ManualProxyConfiguration {
    pub fn new(
        proxy_type: impl Into<String>,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            proxy_type: proxy_type.into(),
            extensible: extensible.into(),
            http_proxy: None,
            ssl_proxy: None,
            no_proxy: None,
        }
    }
}
impl ManualProxyConfiguration {
    pub const IDENTIFIER: &'static str = "session.ManualProxyConfiguration";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SocksProxyConfiguration {
    #[serde(rename = "socksProxy")]
    pub socks_proxy: String,
    #[serde(rename = "socksVersion")]
    pub socks_version: u64,
}
impl SocksProxyConfiguration {
    pub fn new(socks_proxy: impl Into<String>, socks_version: impl Into<u64>) -> Self {
        Self {
            socks_proxy: socks_proxy.into(),
            socks_version: socks_version.into(),
        }
    }
}
impl SocksProxyConfiguration {
    pub const IDENTIFIER: &'static str = "session.SocksProxyConfiguration";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: String,
    #[serde(rename = "proxyAutoconfigUrl")]
    pub proxy_autoconfig_url: String,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl PacProxyConfiguration {
    pub fn new(
        proxy_type: impl Into<String>,
        proxy_autoconfig_url: impl Into<String>,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            proxy_type: proxy_type.into(),
            proxy_autoconfig_url: proxy_autoconfig_url.into(),
            extensible: extensible.into(),
        }
    }
}
impl PacProxyConfiguration {
    pub const IDENTIFIER: &'static str = "session.PacProxyConfiguration";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: String,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl SystemProxyConfiguration {
    pub fn new(
        proxy_type: impl Into<String>,
        extensible: impl Into<std::collections::HashMap<String, serde_json::Value>>,
    ) -> Self {
        Self {
            proxy_type: proxy_type.into(),
            extensible: extensible.into(),
        }
    }
}
impl SystemProxyConfiguration {
    pub const IDENTIFIER: &'static str = "session.SystemProxyConfiguration";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UserPromptHandler {
    #[serde(rename = "alert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub alert: Option<UserPromptHandlerType>,
    #[serde(rename = "beforeUnload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub before_unload: Option<UserPromptHandlerType>,
    #[serde(rename = "confirm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub confirm: Option<UserPromptHandlerType>,
    #[serde(rename = "default")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub default: Option<UserPromptHandlerType>,
    #[serde(rename = "file")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub file: Option<UserPromptHandlerType>,
    #[serde(rename = "prompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub prompt: Option<UserPromptHandlerType>,
}
impl UserPromptHandler {
    pub const IDENTIFIER: &'static str = "session.UserPromptHandler";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserPromptHandlerType {
    #[serde(rename = "accept")]
    Accept,
    #[serde(rename = "dismiss")]
    Dismiss,
    #[serde(rename = "ignore")]
    Ignore,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct Subscription(String);
impl Subscription {
    pub fn new(val: impl Into<String>) -> Self {
        Subscription(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for Subscription {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<Subscription> for String {
    fn from(el: Subscription) -> String {
        el.0
    }
}
impl From<String> for Subscription {
    fn from(expr: String) -> Self {
        Subscription(expr)
    }
}
impl Subscription {
    pub const IDENTIFIER: &'static str = "session.Subscription";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribeByIdRequest {
    #[serde(rename = "subscriptions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subscriptions: Vec<Subscription>,
}
impl UnsubscribeByIdRequest {
    pub fn new(subscriptions: Vec<Subscription>) -> Self {
        Self { subscriptions }
    }
}
impl UnsubscribeByIdRequest {
    pub const IDENTIFIER: &'static str = "session.UnsubscribeByIDRequest";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribeByAttributesRequest {
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
}
impl UnsubscribeByAttributesRequest {
    pub fn new(events: Vec<String>) -> Self {
        Self { events }
    }
}
impl UnsubscribeByAttributesRequest {
    pub const IDENTIFIER: &'static str = "session.UnsubscribeByAttributesRequest";
}
group_enum ! (SessionTypes { CapabilitiesRequest (CapabilitiesRequest) , CapabilityRequest (CapabilityRequest) , ProxyConfiguration (ProxyConfiguration) , AutodetectProxyConfiguration (AutodetectProxyConfiguration) , DirectProxyConfiguration (DirectProxyConfiguration) , ManualProxyConfiguration (ManualProxyConfiguration) , SocksProxyConfiguration (SocksProxyConfiguration) , PacProxyConfiguration (PacProxyConfiguration) , SystemProxyConfiguration (SystemProxyConfiguration) , UserPromptHandler (UserPromptHandler) , UserPromptHandlerType (UserPromptHandlerType) , Subscription (Subscription) , UnsubscribeByIdRequest (UnsubscribeByIdRequest) , UnsubscribeByAttributesRequest (UnsubscribeByAttributesRequest) });
