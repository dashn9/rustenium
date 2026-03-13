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
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProxyConfiguration {
    #[serde(flatten)]
    #[serde(default)]
    pub autodetect_proxy_configuration: serde_json::Value,
    #[serde(flatten)]
    #[serde(default)]
    pub direct_proxy_configuration: serde_json::Value,
    #[serde(flatten)]
    #[serde(default)]
    pub manual_proxy_configuration: serde_json::Value,
    #[serde(flatten)]
    #[serde(default)]
    pub pac_proxy_configuration: serde_json::Value,
    #[serde(flatten)]
    #[serde(default)]
    pub system_proxy_configuration: SystemProxyConfiguration,
}
impl ProxyConfiguration {
    pub const IDENTIFIER: &'static str = "session.ProxyConfiguration";
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutodetectProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: AutodetectProxyConfigurationProxyType,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AutodetectProxyConfigurationProxyType {
    #[serde(rename = "autodetect")]
    Autodetect,
}
impl AutodetectProxyConfiguration {
    pub fn new(
        proxy_type: impl Into<AutodetectProxyConfigurationProxyType>,
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
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: DirectProxyConfigurationProxyType,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DirectProxyConfigurationProxyType {
    #[serde(rename = "direct")]
    Direct,
}
impl DirectProxyConfiguration {
    pub fn new(
        proxy_type: impl Into<DirectProxyConfigurationProxyType>,
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
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ManualProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: ManualProxyConfigurationProxyType,
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ManualProxyConfigurationProxyType {
    #[serde(rename = "manual")]
    Manual,
}
impl ManualProxyConfiguration {
    pub fn new(
        proxy_type: impl Into<ManualProxyConfigurationProxyType>,
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
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PacProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: PacProxyConfigurationProxyType,
    #[serde(rename = "proxyAutoconfigUrl")]
    pub proxy_autoconfig_url: String,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PacProxyConfigurationProxyType {
    #[serde(rename = "pac")]
    Pac,
}
impl PacProxyConfiguration {
    pub fn new(
        proxy_type: impl Into<PacProxyConfigurationProxyType>,
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
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SystemProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: SystemProxyConfigurationProxyType,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SystemProxyConfigurationProxyType {
    #[serde(rename = "system")]
    System,
}
impl SystemProxyConfiguration {
    pub fn new(
        proxy_type: impl Into<SystemProxyConfigurationProxyType>,
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
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub r#default: Option<UserPromptHandlerType>,
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
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnsubscribeParameters {
    UnsubscribeByAttributesRequest(UnsubscribeByAttributesRequest),
    UnsubscribeByIdRequest(UnsubscribeByIdRequest),
}
impl From<UnsubscribeByAttributesRequest> for UnsubscribeParameters {
    fn from(v: UnsubscribeByAttributesRequest) -> Self {
        UnsubscribeParameters::UnsubscribeByAttributesRequest(v)
    }
}
impl TryFrom<UnsubscribeParameters> for UnsubscribeByAttributesRequest {
    type Error = UnsubscribeParameters;
    fn try_from(e: UnsubscribeParameters) -> Result<Self, Self::Error> {
        match e {
            UnsubscribeParameters::UnsubscribeByAttributesRequest(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<UnsubscribeByIdRequest> for UnsubscribeParameters {
    fn from(v: UnsubscribeByIdRequest) -> Self {
        UnsubscribeParameters::UnsubscribeByIdRequest(v)
    }
}
impl TryFrom<UnsubscribeParameters> for UnsubscribeByIdRequest {
    type Error = UnsubscribeParameters;
    fn try_from(e: UnsubscribeParameters) -> Result<Self, Self::Error> {
        match e {
            UnsubscribeParameters::UnsubscribeByIdRequest(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
group_enum ! (SessionType { CapabilitiesRequest (CapabilitiesRequest) , CapabilityRequest (CapabilityRequest) , ProxyConfiguration (ProxyConfiguration) , AutodetectProxyConfiguration (AutodetectProxyConfiguration) , DirectProxyConfiguration (DirectProxyConfiguration) , ManualProxyConfiguration (ManualProxyConfiguration) , SocksProxyConfiguration (SocksProxyConfiguration) , PacProxyConfiguration (PacProxyConfiguration) , SystemProxyConfiguration (SystemProxyConfiguration) , UserPromptHandler (UserPromptHandler) , UserPromptHandlerType (UserPromptHandlerType) , Subscription (Subscription) , UnsubscribeByIdRequest (UnsubscribeByIdRequest) , UnsubscribeParameters (UnsubscribeParameters) });
