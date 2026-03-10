use serde::{Deserialize, Serialize};
#[doc = "The loadComplete event mirrors the load complete event sent by the browser to assistive\ntechnology when the web page has finished loading.\n[loadComplete](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#event-loadComplete)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadCompleteParams {
    #[doc = "New document root node."]
    #[serde(rename = "root")]
    pub root: super::types::AxNode,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoadCompleteMethod {
    #[serde(rename = "Accessibility.loadComplete")]
    LoadComplete,
}
#[doc = "The loadComplete event mirrors the load complete event sent by the browser to assistive\ntechnology when the web page has finished loading.\n[loadComplete](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#event-loadComplete)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadComplete {
    pub method: LoadCompleteMethod,
    pub params: LoadCompleteParams,
}
impl LoadComplete {
    pub const IDENTIFIER: &'static str = "Accessibility.loadComplete";
}
#[doc = "The nodesUpdated event is sent every time a previously requested node has changed the in tree.\n[nodesUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#event-nodesUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodesUpdatedParams {
    #[doc = "Updated node data."]
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::AxNode>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodesUpdatedMethod {
    #[serde(rename = "Accessibility.nodesUpdated")]
    NodesUpdated,
}
#[doc = "The nodesUpdated event is sent every time a previously requested node has changed the in tree.\n[nodesUpdated](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#event-nodesUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodesUpdated {
    pub method: NodesUpdatedMethod,
    pub params: NodesUpdatedParams,
}
impl NodesUpdated {
    pub const IDENTIFIER: &'static str = "Accessibility.nodesUpdated";
}
group_enum ! (AccessibilityEvents { LoadComplete (LoadComplete) , NodesUpdated (NodesUpdated) });
