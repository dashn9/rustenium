use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusResult {
    #[serde(rename = "ready")]
    pub ready: bool,
    #[serde(rename = "message")]
    pub message: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewResult {
    #[serde(rename = "sessionId")]
    pub session_id: String,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub proxy: Option<super::types::ProxyConfiguration>,
    #[serde(rename = "unhandledPromptBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unhandled_prompt_behavior: Option<super::types::UserPromptHandler>,
    #[serde(rename = "webSocketUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub web_socket_url: Option<String>,
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndResult {
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribeResult {
    #[serde(rename = "subscription")]
    pub subscription: super::types::Subscription,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribeResult {
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
