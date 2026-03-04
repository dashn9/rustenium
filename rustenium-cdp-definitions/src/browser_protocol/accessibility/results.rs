use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPartialAxTreeReturns {
    #[doc = "The `Accessibility.AXNode` for this DOM node, if it exists, plus its ancestors, siblings and\nchildren, if requested."]
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFullAxTreeReturns {
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRootAxNodeReturns {
    #[serde(rename = "node")]
    pub node: super::types::AxNode,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAxNodeAndAncestorsReturns {
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetChildAxNodesReturns {
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryAxTreeReturns {
    #[doc = "A list of `Accessibility.AXNode` matching the specified attributes,\nincluding nodes that are ignored for accessibility."]
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
