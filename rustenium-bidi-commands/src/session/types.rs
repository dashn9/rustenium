// Generated types for module

use serde::{Serialize, Deserialize};
use crate::browsing_context::types::BrowsingContext;
use crate::Extensible;
use serde_valid::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AutodetectEnum {
    #[serde(rename = "autodetect")]
    Autodetect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutodetectProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: AutodetectEnum,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DirectEnum {
    #[serde(rename = "direct")]
    Direct,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: DirectEnum,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ManualEnum {
    #[serde(rename = "manual")]
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SocksProxyConfiguration {
    #[serde(rename = "socksProxy")]
    pub socks_proxy: String,
    #[serde(rename = "socksVersion")]
    #[validate(minimum = 0)]
    #[validate(maximum = 255)]
    pub socks_version: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: ManualEnum,
    #[serde(rename = "httpProxy")]
    pub http_proxy: Option<String>,
    #[serde(rename = "sslProxy")]
    pub ssl_proxy: Option<String>,
    #[serde(flatten)]
    pub socks_proxy_configuration: Option<SocksProxyConfiguration>,
    #[serde(rename = "noProxy")]
    pub no_proxy: Option<Vec<String>>,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PacEnum {
    #[serde(rename = "pac")]
    Pac,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PacProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: PacEnum,
    #[serde(rename = "proxyAutoconfigUrl")]
    pub proxy_autoconfig_url: String,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SystemEnum {
    #[serde(rename = "system")]
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemProxyConfiguration {
    #[serde(rename = "proxyType")]
    pub proxy_type: SystemEnum,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProxyConfiguration {
    AutodetectProxyConfiguration(AutodetectProxyConfiguration),
    DirectProxyConfiguration(DirectProxyConfiguration),
    ManualProxyConfiguration(ManualProxyConfiguration),
    PacProxyConfiguration(PacProxyConfiguration),
    SystemProxyConfiguration(SystemProxyConfiguration),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UserPromptHandlerType {
    #[serde(rename = "accept")]
    Accept,
    #[serde(rename = "dismiss")]
    Dismiss,
    #[serde(rename = "ignore")]
    Ignore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPromptHandler {
    #[serde(rename = "alert")]
    pub alert: Option<UserPromptHandlerType>,
    #[serde(rename = "beforeUnload")]
    pub before_unload: Option<UserPromptHandlerType>,
    #[serde(rename = "confirm")]
    pub confirm: Option<UserPromptHandlerType>,
    #[serde(rename = "default")]
    pub default: Option<UserPromptHandlerType>,
    #[serde(rename = "file")]
    pub file: Option<UserPromptHandlerType>,
    #[serde(rename = "prompt")]
    pub prompt: Option<UserPromptHandlerType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequest {
    #[serde(rename = "acceptInsecureCerts")]
    pub accept_insecure_certs: Option<bool>,
    #[serde(rename = "browserName")]
    pub browser_name: Option<String>,
    #[serde(rename = "browserVersion")]
    pub browser_version: Option<String>,
    #[serde(rename = "platformName")]
    pub platform_name: Option<String>,
    #[serde(rename = "proxy")]
    pub proxy: Option<ProxyConfiguration>,
    #[serde(rename = "unhandledPromptBehavior")]
    pub unhandled_prompt_behavior: Option<UserPromptHandler>,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilitiesRequest {
    #[serde(rename = "alwaysMatch")]
    pub always_match: Option<CapabilityRequest>,
    #[serde(rename = "firstMatch")]
    pub first_match: Option<Vec<CapabilityRequest>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeByAttributesRequest {
    #[serde(rename = "events")]
    pub events: Vec<String>,
    #[serde(rename = "contexts")]
    pub contexts: Option<Vec<BrowsingContext>>,
}

pub type Subscription = String;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscribeByIDRequest {
    #[serde(rename = "subscriptions")]
    pub subscriptions: Vec<Subscription>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub user_agent: String,
    #[serde(rename = "proxy")]
    pub proxy: Option<ProxyConfiguration>,
    #[serde(rename = "unhandledPromptBehavior")]
    pub unhandled_prompt_behavior: Option<UserPromptHandler>,
    #[serde(rename = "webSocketUrl")]
    pub web_socket_url: Option<String>,
    #[serde(flatten)]
    pub extensible: Extensible,
}

