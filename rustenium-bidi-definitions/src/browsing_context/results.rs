use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivateResult {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureScreenshotResult {
    #[serde(rename = "data")]
    pub data: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseResult {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateResult {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTreeResult {
    #[serde(rename = "contexts")]
    pub contexts: super::types::InfoList,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandleUserPromptResult {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocateNodesResult {
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<crate::script::types::NodeRemoteValue>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigateResult {
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<super::types::Navigation>,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintResult {
    #[serde(rename = "data")]
    pub data: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadResult {
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<super::types::Navigation>,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetViewportResult {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraverseHistoryResult {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
