use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPartialAxTreeResult {
    #[doc = "The `Accessibility.AXNode` for this DOM node, if it exists, plus its ancestors, siblings and\nchildren, if requested."]
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFullAxTreeResult {
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRootAxNodeResult {
    #[serde(rename = "node")]
    pub node: super::types::AxNode,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAxNodeAndAncestorsResult {
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetChildAxNodesResult {
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryAxTreeResult {
    #[doc = "A list of `Accessibility.AXNode` matching the specified attributes,\nincluding nodes that are ignored for accessibility."]
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
