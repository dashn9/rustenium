use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
impl TryFrom<serde_json::Value> for DisableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
impl TryFrom<serde_json::Value> for EnableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSnapshotResult {
    #[doc = "The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document."]
    #[serde(rename = "domNodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dom_nodes: Vec<super::types::DomNode>,
    #[doc = "The nodes in the layout tree."]
    #[serde(rename = "layoutTreeNodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub layout_tree_nodes: Vec<super::types::LayoutTreeNode>,
    #[doc = "Whitelisted ComputedStyle properties for each node in the layout tree."]
    #[serde(rename = "computedStyles")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub computed_styles: Vec<super::types::ComputedStyle>,
}
impl TryFrom<serde_json::Value> for GetSnapshotResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureSnapshotResult {
    #[doc = "The nodes in the DOM tree. The DOMNode at index 0 corresponds to the root document."]
    #[serde(rename = "documents")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub documents: Vec<super::types::DocumentSnapshot>,
    #[doc = "Shared string table that all string properties refer to with indexes."]
    #[serde(rename = "strings")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub strings: Vec<String>,
}
impl TryFrom<serde_json::Value> for CaptureSnapshotResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
