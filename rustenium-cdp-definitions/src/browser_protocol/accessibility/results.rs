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
pub struct GetPartialAxTreeResult {
    #[doc = "The `Accessibility.AXNode` for this DOM node, if it exists, plus its ancestors, siblings and\nchildren, if requested."]
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
impl TryFrom<serde_json::Value> for GetPartialAxTreeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFullAxTreeResult {
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
impl TryFrom<serde_json::Value> for GetFullAxTreeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRootAxNodeResult {
    #[serde(rename = "node")]
    pub node: super::types::AxNode,
}
impl TryFrom<serde_json::Value> for GetRootAxNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAxNodeAndAncestorsResult {
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
impl TryFrom<serde_json::Value> for GetAxNodeAndAncestorsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetChildAxNodesResult {
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
impl TryFrom<serde_json::Value> for GetChildAxNodesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryAxTreeResult {
    #[doc = "A list of `Accessibility.AXNode` matching the specified attributes,\nincluding nodes that are ignored for accessibility."]
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
impl TryFrom<serde_json::Value> for QueryAxTreeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
