use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseResult {
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl TryFrom<serde_json::Value> for CloseResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
pub type CreateUserContextResult = super::types::UserContextInfo;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetClientWindowsResult {
    #[serde(rename = "clientWindows")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub client_windows: Vec<super::types::ClientWindowInfo>,
}
impl TryFrom<serde_json::Value> for GetClientWindowsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserContextsResult {
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub user_contexts: Vec<super::types::UserContextInfo>,
}
impl TryFrom<serde_json::Value> for GetUserContextsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveUserContextResult {
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl TryFrom<serde_json::Value> for RemoveUserContextResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
pub type SetClientWindowStateResult = super::types::ClientWindowInfo;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDownloadBehaviorResult {
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl TryFrom<serde_json::Value> for SetDownloadBehaviorResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
