use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusResult {
    #[serde(rename = "ready")]
    pub ready: bool,
    #[serde(rename = "message")]
    pub message: String,
}
impl TryFrom<serde_json::Value> for StatusResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewResult {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "capabilities")]
    pub capabilities: super::types::NewResultCapabilities,
}
impl TryFrom<serde_json::Value> for NewResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndResult {
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl TryFrom<serde_json::Value> for EndResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribeResult {
    #[serde(rename = "subscription")]
    pub subscription: super::types::Subscription,
}
impl TryFrom<serde_json::Value> for SubscribeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribeResult {
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl TryFrom<serde_json::Value> for UnsubscribeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
