use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHighlightObjectForTestReturns {
    #[doc = "Highlight data for the node."]
    #[serde(rename = "highlight")]
    pub highlight: serde_json::Value,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridHighlightObjectsForTestReturns {
    #[doc = "Grid Highlight data for the node ids provided."]
    #[serde(rename = "highlights")]
    pub highlights: serde_json::Value,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSourceOrderHighlightObjectForTestReturns {
    #[doc = "Source order highlight data for the node id provided."]
    #[serde(rename = "highlight")]
    pub highlight: serde_json::Value,
}
