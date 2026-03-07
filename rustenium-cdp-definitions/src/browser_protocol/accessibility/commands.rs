use serde::{Deserialize, Serialize};
#[doc = "Disables the accessibility domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Accessibility.disable")]
    Disable,
}
#[doc = "Disables the accessibility domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "Accessibility.disable";
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables the accessibility domain which causes `AXNodeId`s to remain consistent between method calls.\nThis turns on accessibility for the page, which can impact performance until accessibility is disabled.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Accessibility.enable")]
    Enable,
}
#[doc = "Enables the accessibility domain which causes `AXNodeId`s to remain consistent between method calls.\nThis turns on accessibility for the page, which can impact performance until accessibility is disabled.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "Accessibility.enable";
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists.\n[getPartialAXTree](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-getPartialAXTree)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetPartialAxTreeParams {
    #[doc = "Identifier of the node to get the partial accessibility tree for."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<crate::browser_protocol::dom::types::NodeId>,
    #[doc = "Identifier of the backend node to get the partial accessibility tree for."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
    #[doc = "JavaScript object id of the node wrapper to get the partial accessibility tree for."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
    #[doc = "Whether to fetch this node's ancestors, siblings and children. Defaults to true."]
    #[serde(rename = "fetchRelatives")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub fetch_relatives: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetPartialAxTreeMethod {
    #[serde(rename = "Accessibility.getPartialAXTree")]
    GetPartialAxTree,
}
#[doc = "Fetches the accessibility node and partial accessibility tree for this DOM node, if it exists.\n[getPartialAXTree](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-getPartialAXTree)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetPartialAxTree {
    pub method: GetPartialAxTreeMethod,
    pub params: GetPartialAxTreeParams,
}
impl GetPartialAxTree {
    pub const IDENTIFIER: &'static str = "Accessibility.getPartialAXTree";
}
impl crate::CommandResult for GetPartialAxTree {
    type Result = super::results::GetPartialAxTreeResult;
}
#[doc = "Fetches the entire accessibility tree for the root Document\n[getFullAXTree](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-getFullAXTree)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetFullAxTreeParams {
    #[doc = "The maximum depth at which descendants of the root node should be retrieved.\nIf omitted, the full tree is returned."]
    #[serde(rename = "depth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub depth: Option<i64>,
    #[doc = "The frame for whose document the AX tree should be retrieved.\nIf omitted, the root frame is used."]
    #[serde(rename = "frameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_id: Option<crate::browser_protocol::page::types::FrameId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetFullAxTreeMethod {
    #[serde(rename = "Accessibility.getFullAXTree")]
    GetFullAxTree,
}
#[doc = "Fetches the entire accessibility tree for the root Document\n[getFullAXTree](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-getFullAXTree)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetFullAxTree {
    pub method: GetFullAxTreeMethod,
    pub params: GetFullAxTreeParams,
}
impl GetFullAxTree {
    pub const IDENTIFIER: &'static str = "Accessibility.getFullAXTree";
}
impl crate::CommandResult for GetFullAxTree {
    type Result = super::results::GetFullAxTreeResult;
}
#[doc = "Fetches the root node.\nRequires `enable()` to have been called previously.\n[getRootAXNode](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-getRootAXNode)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetRootAxNodeParams {
    #[doc = "The frame in whose document the node resides.\nIf omitted, the root frame is used."]
    #[serde(rename = "frameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_id: Option<crate::browser_protocol::page::types::FrameId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetRootAxNodeMethod {
    #[serde(rename = "Accessibility.getRootAXNode")]
    GetRootAxNode,
}
#[doc = "Fetches the root node.\nRequires `enable()` to have been called previously.\n[getRootAXNode](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-getRootAXNode)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetRootAxNode {
    pub method: GetRootAxNodeMethod,
    pub params: GetRootAxNodeParams,
}
impl GetRootAxNode {
    pub const IDENTIFIER: &'static str = "Accessibility.getRootAXNode";
}
impl crate::CommandResult for GetRootAxNode {
    type Result = super::results::GetRootAxNodeResult;
}
#[doc = "Fetches a node and all ancestors up to and including the root.\nRequires `enable()` to have been called previously.\n[getAXNodeAndAncestors](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-getAXNodeAndAncestors)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetAxNodeAndAncestorsParams {
    #[doc = "Identifier of the node to get."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<crate::browser_protocol::dom::types::NodeId>,
    #[doc = "Identifier of the backend node to get."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
    #[doc = "JavaScript object id of the node wrapper to get."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetAxNodeAndAncestorsMethod {
    #[serde(rename = "Accessibility.getAXNodeAndAncestors")]
    GetAxNodeAndAncestors,
}
#[doc = "Fetches a node and all ancestors up to and including the root.\nRequires `enable()` to have been called previously.\n[getAXNodeAndAncestors](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-getAXNodeAndAncestors)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetAxNodeAndAncestors {
    pub method: GetAxNodeAndAncestorsMethod,
    pub params: GetAxNodeAndAncestorsParams,
}
impl GetAxNodeAndAncestors {
    pub const IDENTIFIER: &'static str = "Accessibility.getAXNodeAndAncestors";
}
impl crate::CommandResult for GetAxNodeAndAncestors {
    type Result = super::results::GetAxNodeAndAncestorsResult;
}
#[doc = "Fetches a particular accessibility node by AXNodeId.\nRequires `enable()` to have been called previously.\n[getChildAXNodes](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-getChildAXNodes)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetChildAxNodesParams {
    #[serde(rename = "id")]
    pub id: super::types::AxNodeId,
    #[doc = "The frame in whose document the node resides.\nIf omitted, the root frame is used."]
    #[serde(rename = "frameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_id: Option<crate::browser_protocol::page::types::FrameId>,
}
impl GetChildAxNodesParams {
    pub fn new(id: impl Into<super::types::AxNodeId>) -> Self {
        Self {
            id: id.into(),
            frame_id: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetChildAxNodesMethod {
    #[serde(rename = "Accessibility.getChildAXNodes")]
    GetChildAxNodes,
}
#[doc = "Fetches a particular accessibility node by AXNodeId.\nRequires `enable()` to have been called previously.\n[getChildAXNodes](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-getChildAXNodes)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetChildAxNodes {
    pub method: GetChildAxNodesMethod,
    pub params: GetChildAxNodesParams,
}
impl GetChildAxNodes {
    pub const IDENTIFIER: &'static str = "Accessibility.getChildAXNodes";
}
impl crate::CommandResult for GetChildAxNodes {
    type Result = super::results::GetChildAxNodesResult;
}
#[doc = "Query a DOM node's accessibility subtree for accessible name and role.\nThis command computes the name and role for all nodes in the subtree, including those that are\nignored for accessibility, and returns those that match the specified name and role. If no DOM\nnode is specified, or the DOM node does not exist, the command returns an error. If neither\n`accessibleName` or `role` is specified, it returns all the accessibility nodes in the subtree.\n[queryAXTree](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-queryAXTree)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct QueryAxTreeParams {
    #[doc = "Identifier of the node for the root to query."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<crate::browser_protocol::dom::types::NodeId>,
    #[doc = "Identifier of the backend node for the root to query."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
    #[doc = "JavaScript object id of the node wrapper for the root to query."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
    #[doc = "Find nodes with this computed name."]
    #[serde(rename = "accessibleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub accessible_name: Option<String>,
    #[doc = "Find nodes with this computed role."]
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub role: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QueryAxTreeMethod {
    #[serde(rename = "Accessibility.queryAXTree")]
    QueryAxTree,
}
#[doc = "Query a DOM node's accessibility subtree for accessible name and role.\nThis command computes the name and role for all nodes in the subtree, including those that are\nignored for accessibility, and returns those that match the specified name and role. If no DOM\nnode is specified, or the DOM node does not exist, the command returns an error. If neither\n`accessibleName` or `role` is specified, it returns all the accessibility nodes in the subtree.\n[queryAXTree](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#method-queryAXTree)"]
#[derive(Debug, Clone, PartialEq)]
pub struct QueryAxTree {
    pub method: QueryAxTreeMethod,
    pub params: QueryAxTreeParams,
}
impl QueryAxTree {
    pub const IDENTIFIER: &'static str = "Accessibility.queryAXTree";
}
impl crate::CommandResult for QueryAxTree {
    type Result = super::results::QueryAxTreeResult;
}
group_enum ! (AccessibilityCommands { Disable (Disable) , Enable (Enable) , GetPartialAxTree (GetPartialAxTree) , GetFullAxTree (GetFullAxTree) , GetRootAxNode (GetRootAxNode) , GetAxNodeAndAncestors (GetAxNodeAndAncestors) , GetChildAxNodes (GetChildAxNodes) , QueryAxTree (QueryAxTree) });
