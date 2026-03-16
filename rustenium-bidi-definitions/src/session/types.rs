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
    pub unhandled_prompt_behavior: Option<UnhandledPromptBehavior>,
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
#[serde(untagged)]
pub enum ProxyConfiguration {
    AutodetectProxyConfiguration(AutodetectProxyConfiguration),
    DirectProxyConfiguration(DirectProxyConfiguration),
    ManualProxyConfiguration(ManualProxyConfiguration),
    PacProxyConfiguration(PacProxyConfiguration),
    SystemProxyConfiguration(SystemProxyConfiguration),
    SocksProxyConfiguration(SocksProxyConfiguration),
    EmptyProxyConfiguration {},
}
impl From<AutodetectProxyConfiguration> for ProxyConfiguration {
    fn from(v: AutodetectProxyConfiguration) -> Self {
        ProxyConfiguration::AutodetectProxyConfiguration(v)
    }
}
impl TryFrom<ProxyConfiguration> for AutodetectProxyConfiguration {
    type Error = ProxyConfiguration;
    fn try_from(e: ProxyConfiguration) -> Result<Self, Self::Error> {
        match e {
            ProxyConfiguration::AutodetectProxyConfiguration(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<DirectProxyConfiguration> for ProxyConfiguration {
    fn from(v: DirectProxyConfiguration) -> Self {
        ProxyConfiguration::DirectProxyConfiguration(v)
    }
}
impl TryFrom<ProxyConfiguration> for DirectProxyConfiguration {
    type Error = ProxyConfiguration;
    fn try_from(e: ProxyConfiguration) -> Result<Self, Self::Error> {
        match e {
            ProxyConfiguration::DirectProxyConfiguration(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<ManualProxyConfiguration> for ProxyConfiguration {
    fn from(v: ManualProxyConfiguration) -> Self {
        ProxyConfiguration::ManualProxyConfiguration(v)
    }
}
impl TryFrom<ProxyConfiguration> for ManualProxyConfiguration {
    type Error = ProxyConfiguration;
    fn try_from(e: ProxyConfiguration) -> Result<Self, Self::Error> {
        match e {
            ProxyConfiguration::ManualProxyConfiguration(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<PacProxyConfiguration> for ProxyConfiguration {
    fn from(v: PacProxyConfiguration) -> Self {
        ProxyConfiguration::PacProxyConfiguration(v)
    }
}
impl TryFrom<ProxyConfiguration> for PacProxyConfiguration {
    type Error = ProxyConfiguration;
    fn try_from(e: ProxyConfiguration) -> Result<Self, Self::Error> {
        match e {
            ProxyConfiguration::PacProxyConfiguration(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<SystemProxyConfiguration> for ProxyConfiguration {
    fn from(v: SystemProxyConfiguration) -> Self {
        ProxyConfiguration::SystemProxyConfiguration(v)
    }
}
impl TryFrom<ProxyConfiguration> for SystemProxyConfiguration {
    type Error = ProxyConfiguration;
    fn try_from(e: ProxyConfiguration) -> Result<Self, Self::Error> {
        match e {
            ProxyConfiguration::SystemProxyConfiguration(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<SocksProxyConfiguration> for ProxyConfiguration {
    fn from(v: SocksProxyConfiguration) -> Self {
        ProxyConfiguration::SocksProxyConfiguration(v)
    }
}
impl TryFrom<ProxyConfiguration> for SocksProxyConfiguration {
    type Error = ProxyConfiguration;
    fn try_from(e: ProxyConfiguration) -> Result<Self, Self::Error> {
        match e {
            ProxyConfiguration::SocksProxyConfiguration(inner) => Ok(inner),
            other => Err(other),
        }
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
    #[serde(rename = "dismiss and notify")]
    DismissAndNotify,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewResultCapabilities {
    #[serde(rename = "acceptInsecureCerts")]
    pub accept_insecure_certs: bool,
    #[serde(rename = "browserName")]
    pub browser_name: String,
    #[serde(rename = "browserVersion")]
    pub browser_version: String,
    #[serde(rename = "platformName")]
    pub platform_name: String,
    #[serde(rename = "setWindowRect")]
    pub set_window_rect: bool,
    #[serde(rename = "userAgent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_agent: Option<String>,
    #[serde(rename = "proxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub proxy: Option<ProxyConfiguration>,
    #[serde(rename = "unhandledPromptBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unhandled_prompt_behavior: Option<UnhandledPromptBehavior>,
    #[serde(rename = "webSocketUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub web_socket_url: Option<String>,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl NewResultCapabilities {
    pub const IDENTIFIER: &'static str = "";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnhandledPromptBehavior {
    UserPromptHandler(UserPromptHandler),
    UserPromptHandlerType(UserPromptHandlerType),
}
impl From<UserPromptHandler> for UnhandledPromptBehavior {
    fn from(v: UserPromptHandler) -> Self {
        UnhandledPromptBehavior::UserPromptHandler(v)
    }
}
impl TryFrom<UnhandledPromptBehavior> for UserPromptHandler {
    type Error = UnhandledPromptBehavior;
    fn try_from(e: UnhandledPromptBehavior) -> Result<Self, Self::Error> {
        match e {
            UnhandledPromptBehavior::UserPromptHandler(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<UserPromptHandlerType> for UnhandledPromptBehavior {
    fn from(v: UserPromptHandlerType) -> Self {
        UnhandledPromptBehavior::UserPromptHandlerType(v)
    }
}
impl TryFrom<UnhandledPromptBehavior> for UserPromptHandlerType {
    type Error = UnhandledPromptBehavior;
    fn try_from(e: UnhandledPromptBehavior) -> Result<Self, Self::Error> {
        match e {
            UnhandledPromptBehavior::UserPromptHandlerType(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
