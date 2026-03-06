use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseResult {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUserContextResult {
    #[serde(rename = "userContext")]
    pub user_context: super::types::UserContext,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetClientWindowsResult {
    #[serde(rename = "clientWindows")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub client_windows: Vec<super::types::ClientWindowInfo>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserContextsResult {
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub user_contexts: Vec<super::types::UserContextInfo>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveUserContextResult {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDownloadBehaviorResult {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
