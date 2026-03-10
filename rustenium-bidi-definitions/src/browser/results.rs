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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUserContextResult {
    #[serde(rename = "userContext")]
    pub user_context: super::types::UserContext,
}
impl TryFrom<serde_json::Value> for CreateUserContextResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetClientWindowStateResultState {
    #[serde(rename = "fullscreen")]
    Fullscreen,
    #[serde(rename = "maximized")]
    Maximized,
    #[serde(rename = "minimized")]
    Minimized,
    #[serde(rename = "normal")]
    Normal,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetClientWindowStateResult {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "clientWindow")]
    pub client_window: super::types::ClientWindow,
    #[serde(rename = "height")]
    pub height: u64,
    #[serde(rename = "state")]
    pub state: SetClientWindowStateResultState,
    #[serde(rename = "width")]
    pub width: u64,
    #[serde(rename = "x")]
    pub x: i64,
    #[serde(rename = "y")]
    pub y: i64,
}
impl TryFrom<serde_json::Value> for SetClientWindowStateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
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
