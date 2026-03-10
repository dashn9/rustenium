use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivateResult {
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl TryFrom<serde_json::Value> for ActivateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureScreenshotResult {
    #[serde(rename = "data")]
    pub data: String,
}
impl TryFrom<serde_json::Value> for CaptureScreenshotResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
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
pub struct CreateResult {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
}
impl TryFrom<serde_json::Value> for CreateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTreeResult {
    #[serde(rename = "contexts")]
    pub contexts: super::types::InfoList,
}
impl TryFrom<serde_json::Value> for GetTreeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandleUserPromptResult {
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl TryFrom<serde_json::Value> for HandleUserPromptResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocateNodesResult {
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<crate::script::types::NodeRemoteValue>,
}
impl TryFrom<serde_json::Value> for LocateNodesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
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
impl TryFrom<serde_json::Value> for NavigateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintResult {
    #[serde(rename = "data")]
    pub data: String,
}
impl TryFrom<serde_json::Value> for PrintResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
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
impl TryFrom<serde_json::Value> for ReloadResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetViewportResult {
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl TryFrom<serde_json::Value> for SetViewportResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraverseHistoryResult {
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl TryFrom<serde_json::Value> for TraverseHistoryResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
