use serde::{Deserialize, Serialize};
#[doc = "Collects class names for the node with given id and all of it's child nodes.\n[collectClassNamesFromSubtree](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-collectClassNamesFromSubtree)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectClassNamesFromSubtreeParams {
    #[doc = "Id of the node to collect class names."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl CollectClassNamesFromSubtreeParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollectClassNamesFromSubtreeMethod {
    #[serde(rename = "DOM.collectClassNamesFromSubtree")]
    CollectClassNamesFromSubtree,
}
#[doc = "Collects class names for the node with given id and all of it's child nodes.\n[collectClassNamesFromSubtree](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-collectClassNamesFromSubtree)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectClassNamesFromSubtree {
    pub method: CollectClassNamesFromSubtreeMethod,
    pub params: CollectClassNamesFromSubtreeParams,
}
impl CollectClassNamesFromSubtree {
    pub const IDENTIFIER: &'static str = "DOM.collectClassNamesFromSubtree";
}
impl crate::CommandResult for CollectClassNamesFromSubtree {
    type Result = super::results::CollectClassNamesFromSubtreeResult;
}
#[doc = "Creates a deep copy of the specified node and places it into the target container before the\ngiven anchor.\n[copyTo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-copyTo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyToParams {
    #[doc = "Id of the node to copy."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "Id of the element to drop the copy into."]
    #[serde(rename = "targetNodeId")]
    pub target_node_id: super::types::NodeId,
    #[doc = "Drop the copy before this node (if absent, the copy becomes the last child of\n`targetNodeId`)."]
    #[serde(rename = "insertBeforeNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub insert_before_node_id: Option<super::types::NodeId>,
}
impl CopyToParams {
    pub fn new(
        node_id: impl Into<super::types::NodeId>,
        target_node_id: impl Into<super::types::NodeId>,
    ) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            target_node_id: target_node_id.into(),
            insert_before_node_id: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CopyToMethod {
    #[serde(rename = "DOM.copyTo")]
    CopyTo,
}
#[doc = "Creates a deep copy of the specified node and places it into the target container before the\ngiven anchor.\n[copyTo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-copyTo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyTo {
    pub method: CopyToMethod,
    pub params: CopyToParams,
}
impl CopyTo {
    pub const IDENTIFIER: &'static str = "DOM.copyTo";
}
impl crate::CommandResult for CopyTo {
    type Result = super::results::CopyToResult;
}
#[doc = "Describes node given its id, does not require domain to be enabled. Does not start tracking any\nobjects, can be used for automation.\n[describeNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-describeNode)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DescribeNodeParams {
    #[doc = "Identifier of the node."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<Box<super::types::NodeId>>,
    #[doc = "Identifier of the backend node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<Box<super::types::BackendNodeId>>,
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
    #[doc = "The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the\nentire subtree or provide an integer larger than 0."]
    #[serde(rename = "depth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub depth: Option<i64>,
    #[doc = "Whether or not iframes and shadow roots should be traversed when returning the subtree\n(default is false)."]
    #[serde(rename = "pierce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pierce: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DescribeNodeMethod {
    #[serde(rename = "DOM.describeNode")]
    DescribeNode,
}
#[doc = "Describes node given its id, does not require domain to be enabled. Does not start tracking any\nobjects, can be used for automation.\n[describeNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-describeNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribeNode {
    pub method: DescribeNodeMethod,
    pub params: DescribeNodeParams,
}
impl DescribeNode {
    pub const IDENTIFIER: &'static str = "DOM.describeNode";
}
impl crate::CommandResult for DescribeNode {
    type Result = super::results::DescribeNodeResult;
}
#[doc = "Scrolls the specified rect of the given node into view if not already visible.\nNote: exactly one between nodeId, backendNodeId and objectId should be passed\nto identify the node.\n[scrollIntoViewIfNeeded](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-scrollIntoViewIfNeeded)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScrollIntoViewIfNeededParams {
    #[doc = "Identifier of the node."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<Box<super::types::NodeId>>,
    #[doc = "Identifier of the backend node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<Box<super::types::BackendNodeId>>,
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
    #[doc = "The rect to be scrolled into view, relative to the node's border box, in CSS pixels.\nWhen omitted, center of the node will be used, similar to Element.scrollIntoView."]
    #[serde(rename = "rect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub rect: Option<super::types::Rect>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScrollIntoViewIfNeededMethod {
    #[serde(rename = "DOM.scrollIntoViewIfNeeded")]
    ScrollIntoViewIfNeeded,
}
#[doc = "Scrolls the specified rect of the given node into view if not already visible.\nNote: exactly one between nodeId, backendNodeId and objectId should be passed\nto identify the node.\n[scrollIntoViewIfNeeded](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-scrollIntoViewIfNeeded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrollIntoViewIfNeeded {
    pub method: ScrollIntoViewIfNeededMethod,
    pub params: ScrollIntoViewIfNeededParams,
}
impl ScrollIntoViewIfNeeded {
    pub const IDENTIFIER: &'static str = "DOM.scrollIntoViewIfNeeded";
}
impl crate::CommandResult for ScrollIntoViewIfNeeded {
    type Result = super::results::ScrollIntoViewIfNeededResult;
}
#[doc = "Disables DOM agent for the given page.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "DOM.disable")]
    Disable,
}
#[doc = "Disables DOM agent for the given page.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "DOM.disable";
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Discards search results from the session with the given id. `getSearchResults` should no longer\nbe called for that search.\n[discardSearchResults](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-discardSearchResults)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscardSearchResultsParams {
    #[doc = "Unique search session identifier."]
    #[serde(rename = "searchId")]
    pub search_id: String,
}
impl DiscardSearchResultsParams {
    pub fn new(search_id: impl Into<String>) -> Self {
        Self {
            search_id: search_id.into(),
        }
    }
}
impl<T: Into<String>> From<T> for DiscardSearchResultsParams {
    fn from(url: T) -> Self {
        DiscardSearchResultsParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiscardSearchResultsMethod {
    #[serde(rename = "DOM.discardSearchResults")]
    DiscardSearchResults,
}
#[doc = "Discards search results from the session with the given id. `getSearchResults` should no longer\nbe called for that search.\n[discardSearchResults](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-discardSearchResults)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiscardSearchResults {
    pub method: DiscardSearchResultsMethod,
    pub params: DiscardSearchResultsParams,
}
impl DiscardSearchResults {
    pub const IDENTIFIER: &'static str = "DOM.discardSearchResults";
}
impl crate::CommandResult for DiscardSearchResults {
    type Result = super::results::DiscardSearchResultsResult;
}
#[doc = "Enables DOM agent for the given page.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {
    #[doc = "Whether to include whitespaces in the children array of returned Nodes."]
    #[serde(rename = "includeWhitespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_whitespace: Option<EnableIncludeWhitespace>,
}
#[doc = "Whether to include whitespaces in the children array of returned Nodes."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EnableIncludeWhitespace {
    #[doc = "Strip whitespaces from child arrays (default)."]
    #[serde(rename = "none")]
    None,
    #[doc = "Return all children including block-level whitespace nodes."]
    #[serde(rename = "all")]
    All,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "DOM.enable")]
    Enable,
}
#[doc = "Enables DOM agent for the given page.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "DOM.enable";
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Focuses the given element.\n[focus](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-focus)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FocusParams {
    #[doc = "Identifier of the node."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<Box<super::types::NodeId>>,
    #[doc = "Identifier of the backend node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<Box<super::types::BackendNodeId>>,
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FocusMethod {
    #[serde(rename = "DOM.focus")]
    Focus,
}
#[doc = "Focuses the given element.\n[focus](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-focus)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Focus {
    pub method: FocusMethod,
    pub params: FocusParams,
}
impl Focus {
    pub const IDENTIFIER: &'static str = "DOM.focus";
}
impl crate::CommandResult for Focus {
    type Result = super::results::FocusResult;
}
#[doc = "Returns attributes for the specified node.\n[getAttributes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getAttributes)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAttributesParams {
    #[doc = "Id of the node to retrieve attributes for."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl GetAttributesParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetAttributesMethod {
    #[serde(rename = "DOM.getAttributes")]
    GetAttributes,
}
#[doc = "Returns attributes for the specified node.\n[getAttributes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getAttributes)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAttributes {
    pub method: GetAttributesMethod,
    pub params: GetAttributesParams,
}
impl GetAttributes {
    pub const IDENTIFIER: &'static str = "DOM.getAttributes";
}
impl crate::CommandResult for GetAttributes {
    type Result = super::results::GetAttributesResult;
}
#[doc = "Returns boxes for the given node.\n[getBoxModel](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getBoxModel)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetBoxModelParams {
    #[doc = "Identifier of the node."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<Box<super::types::NodeId>>,
    #[doc = "Identifier of the backend node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<Box<super::types::BackendNodeId>>,
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetBoxModelMethod {
    #[serde(rename = "DOM.getBoxModel")]
    GetBoxModel,
}
#[doc = "Returns boxes for the given node.\n[getBoxModel](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getBoxModel)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBoxModel {
    pub method: GetBoxModelMethod,
    pub params: GetBoxModelParams,
}
impl GetBoxModel {
    pub const IDENTIFIER: &'static str = "DOM.getBoxModel";
}
impl crate::CommandResult for GetBoxModel {
    type Result = super::results::GetBoxModelResult;
}
#[doc = "Returns quads that describe node position on the page. This method\nmight return multiple quads for inline nodes.\n[getContentQuads](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getContentQuads)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetContentQuadsParams {
    #[doc = "Identifier of the node."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<Box<super::types::NodeId>>,
    #[doc = "Identifier of the backend node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<Box<super::types::BackendNodeId>>,
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetContentQuadsMethod {
    #[serde(rename = "DOM.getContentQuads")]
    GetContentQuads,
}
#[doc = "Returns quads that describe node position on the page. This method\nmight return multiple quads for inline nodes.\n[getContentQuads](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getContentQuads)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContentQuads {
    pub method: GetContentQuadsMethod,
    pub params: GetContentQuadsParams,
}
impl GetContentQuads {
    pub const IDENTIFIER: &'static str = "DOM.getContentQuads";
}
impl crate::CommandResult for GetContentQuads {
    type Result = super::results::GetContentQuadsResult;
}
#[doc = "Returns the root DOM node (and optionally the subtree) to the caller.\nImplicitly enables the DOM domain events for the current target.\n[getDocument](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getDocument)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetDocumentParams {
    #[doc = "The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the\nentire subtree or provide an integer larger than 0."]
    #[serde(rename = "depth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub depth: Option<i64>,
    #[doc = "Whether or not iframes and shadow roots should be traversed when returning the subtree\n(default is false)."]
    #[serde(rename = "pierce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pierce: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetDocumentMethod {
    #[serde(rename = "DOM.getDocument")]
    GetDocument,
}
#[doc = "Returns the root DOM node (and optionally the subtree) to the caller.\nImplicitly enables the DOM domain events for the current target.\n[getDocument](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getDocument)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDocument {
    pub method: GetDocumentMethod,
    pub params: GetDocumentParams,
}
impl GetDocument {
    pub const IDENTIFIER: &'static str = "DOM.getDocument";
}
impl crate::CommandResult for GetDocument {
    type Result = super::results::GetDocumentResult;
}
#[doc = "Finds nodes with a given computed style in a subtree.\n[getNodesForSubtreeByStyle](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getNodesForSubtreeByStyle)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodesForSubtreeByStyleParams {
    #[doc = "Node ID pointing to the root of a subtree."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "The style to filter nodes by (includes nodes if any of properties matches)."]
    #[serde(rename = "computedStyles")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub computed_styles: Vec<super::types::CssComputedStyleProperty>,
    #[doc = "Whether or not iframes and shadow roots in the same target should be traversed when returning the\nresults (default is false)."]
    #[serde(rename = "pierce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pierce: Option<bool>,
}
impl GetNodesForSubtreeByStyleParams {
    pub fn new(
        node_id: impl Into<super::types::NodeId>,
        computed_styles: Vec<super::types::CssComputedStyleProperty>,
    ) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            computed_styles,
            pierce: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetNodesForSubtreeByStyleMethod {
    #[serde(rename = "DOM.getNodesForSubtreeByStyle")]
    GetNodesForSubtreeByStyle,
}
#[doc = "Finds nodes with a given computed style in a subtree.\n[getNodesForSubtreeByStyle](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getNodesForSubtreeByStyle)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodesForSubtreeByStyle {
    pub method: GetNodesForSubtreeByStyleMethod,
    pub params: GetNodesForSubtreeByStyleParams,
}
impl GetNodesForSubtreeByStyle {
    pub const IDENTIFIER: &'static str = "DOM.getNodesForSubtreeByStyle";
}
impl crate::CommandResult for GetNodesForSubtreeByStyle {
    type Result = super::results::GetNodesForSubtreeByStyleResult;
}
#[doc = "Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is\neither returned or not.\n[getNodeForLocation](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getNodeForLocation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeForLocationParams {
    #[doc = "X coordinate."]
    #[serde(rename = "x")]
    pub x: i64,
    #[doc = "Y coordinate."]
    #[serde(rename = "y")]
    pub y: i64,
    #[doc = "False to skip to the nearest non-UA shadow root ancestor (default: false)."]
    #[serde(rename = "includeUserAgentShadowDOM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_user_agent_shadow_dom: Option<bool>,
    #[doc = "Whether to ignore pointer-events: none on elements and hit test them."]
    #[serde(rename = "ignorePointerEventsNone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ignore_pointer_events_none: Option<bool>,
}
impl GetNodeForLocationParams {
    pub fn new(x: impl Into<i64>, y: impl Into<i64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            include_user_agent_shadow_dom: None,
            ignore_pointer_events_none: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetNodeForLocationMethod {
    #[serde(rename = "DOM.getNodeForLocation")]
    GetNodeForLocation,
}
#[doc = "Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is\neither returned or not.\n[getNodeForLocation](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getNodeForLocation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeForLocation {
    pub method: GetNodeForLocationMethod,
    pub params: GetNodeForLocationParams,
}
impl GetNodeForLocation {
    pub const IDENTIFIER: &'static str = "DOM.getNodeForLocation";
}
impl crate::CommandResult for GetNodeForLocation {
    type Result = super::results::GetNodeForLocationResult;
}
#[doc = "Returns node's HTML markup.\n[getOuterHTML](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getOuterHTML)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetOuterHtmlParams {
    #[doc = "Identifier of the node."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<Box<super::types::NodeId>>,
    #[doc = "Identifier of the backend node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<Box<super::types::BackendNodeId>>,
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
    #[doc = "Include all shadow roots. Equals to false if not specified."]
    #[serde(rename = "includeShadowDOM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_shadow_dom: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetOuterHtmlMethod {
    #[serde(rename = "DOM.getOuterHTML")]
    GetOuterHtml,
}
#[doc = "Returns node's HTML markup.\n[getOuterHTML](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getOuterHTML)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOuterHtml {
    pub method: GetOuterHtmlMethod,
    pub params: GetOuterHtmlParams,
}
impl GetOuterHtml {
    pub const IDENTIFIER: &'static str = "DOM.getOuterHTML";
}
impl crate::CommandResult for GetOuterHtml {
    type Result = super::results::GetOuterHtmlResult;
}
#[doc = "Returns the id of the nearest ancestor that is a relayout boundary.\n[getRelayoutBoundary](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getRelayoutBoundary)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRelayoutBoundaryParams {
    #[doc = "Id of the node."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl GetRelayoutBoundaryParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetRelayoutBoundaryMethod {
    #[serde(rename = "DOM.getRelayoutBoundary")]
    GetRelayoutBoundary,
}
#[doc = "Returns the id of the nearest ancestor that is a relayout boundary.\n[getRelayoutBoundary](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getRelayoutBoundary)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRelayoutBoundary {
    pub method: GetRelayoutBoundaryMethod,
    pub params: GetRelayoutBoundaryParams,
}
impl GetRelayoutBoundary {
    pub const IDENTIFIER: &'static str = "DOM.getRelayoutBoundary";
}
impl crate::CommandResult for GetRelayoutBoundary {
    type Result = super::results::GetRelayoutBoundaryResult;
}
#[doc = "Returns search results from given `fromIndex` to given `toIndex` from the search with the given\nidentifier.\n[getSearchResults](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getSearchResults)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchResultsParams {
    #[doc = "Unique search session identifier."]
    #[serde(rename = "searchId")]
    pub search_id: String,
    #[doc = "Start index of the search result to be returned."]
    #[serde(rename = "fromIndex")]
    pub from_index: i64,
    #[doc = "End index of the search result to be returned."]
    #[serde(rename = "toIndex")]
    pub to_index: i64,
}
impl GetSearchResultsParams {
    pub fn new(
        search_id: impl Into<String>,
        from_index: impl Into<i64>,
        to_index: impl Into<i64>,
    ) -> Self {
        Self {
            search_id: search_id.into(),
            from_index: from_index.into(),
            to_index: to_index.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetSearchResultsMethod {
    #[serde(rename = "DOM.getSearchResults")]
    GetSearchResults,
}
#[doc = "Returns search results from given `fromIndex` to given `toIndex` from the search with the given\nidentifier.\n[getSearchResults](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getSearchResults)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchResults {
    pub method: GetSearchResultsMethod,
    pub params: GetSearchResultsParams,
}
impl GetSearchResults {
    pub const IDENTIFIER: &'static str = "DOM.getSearchResults";
}
impl crate::CommandResult for GetSearchResults {
    type Result = super::results::GetSearchResultsResult;
}
#[doc = "Hides any highlight.\n[hideHighlight](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-hideHighlight)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HideHighlightParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HideHighlightMethod {
    #[serde(rename = "DOM.hideHighlight")]
    HideHighlight,
}
#[doc = "Hides any highlight.\n[hideHighlight](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-hideHighlight)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HideHighlight {
    pub method: HideHighlightMethod,
    pub params: HideHighlightParams,
}
impl HideHighlight {
    pub const IDENTIFIER: &'static str = "DOM.hideHighlight";
}
impl crate::CommandResult for HideHighlight {
    type Result = super::results::HideHighlightResult;
}
#[doc = "Highlights DOM node.\n[highlightNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-highlightNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HighlightNodeParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HighlightNodeMethod {
    #[serde(rename = "DOM.highlightNode")]
    HighlightNode,
}
#[doc = "Highlights DOM node.\n[highlightNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-highlightNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HighlightNode {
    pub method: HighlightNodeMethod,
    pub params: HighlightNodeParams,
}
impl HighlightNode {
    pub const IDENTIFIER: &'static str = "DOM.highlightNode";
}
impl crate::CommandResult for HighlightNode {
    type Result = super::results::HighlightNodeResult;
}
#[doc = "Highlights given rectangle.\n[highlightRect](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-highlightRect)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HighlightRectParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HighlightRectMethod {
    #[serde(rename = "DOM.highlightRect")]
    HighlightRect,
}
#[doc = "Highlights given rectangle.\n[highlightRect](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-highlightRect)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HighlightRect {
    pub method: HighlightRectMethod,
    pub params: HighlightRectParams,
}
impl HighlightRect {
    pub const IDENTIFIER: &'static str = "DOM.highlightRect";
}
impl crate::CommandResult for HighlightRect {
    type Result = super::results::HighlightRectResult;
}
#[doc = "Marks last undoable state.\n[markUndoableState](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-markUndoableState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkUndoableStateParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarkUndoableStateMethod {
    #[serde(rename = "DOM.markUndoableState")]
    MarkUndoableState,
}
#[doc = "Marks last undoable state.\n[markUndoableState](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-markUndoableState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkUndoableState {
    pub method: MarkUndoableStateMethod,
    pub params: MarkUndoableStateParams,
}
impl MarkUndoableState {
    pub const IDENTIFIER: &'static str = "DOM.markUndoableState";
}
impl crate::CommandResult for MarkUndoableState {
    type Result = super::results::MarkUndoableStateResult;
}
#[doc = "Moves node into the new container, places it before the given anchor.\n[moveTo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-moveTo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveToParams {
    #[doc = "Id of the node to move."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "Id of the element to drop the moved node into."]
    #[serde(rename = "targetNodeId")]
    pub target_node_id: super::types::NodeId,
    #[doc = "Drop node before this one (if absent, the moved node becomes the last child of\n`targetNodeId`)."]
    #[serde(rename = "insertBeforeNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub insert_before_node_id: Option<super::types::NodeId>,
}
impl MoveToParams {
    pub fn new(
        node_id: impl Into<super::types::NodeId>,
        target_node_id: impl Into<super::types::NodeId>,
    ) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            target_node_id: target_node_id.into(),
            insert_before_node_id: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MoveToMethod {
    #[serde(rename = "DOM.moveTo")]
    MoveTo,
}
#[doc = "Moves node into the new container, places it before the given anchor.\n[moveTo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-moveTo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveTo {
    pub method: MoveToMethod,
    pub params: MoveToParams,
}
impl MoveTo {
    pub const IDENTIFIER: &'static str = "DOM.moveTo";
}
impl crate::CommandResult for MoveTo {
    type Result = super::results::MoveToResult;
}
#[doc = "Searches for a given string in the DOM tree. Use `getSearchResults` to access search results or\n`cancelSearch` to end this search session.\n[performSearch](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-performSearch)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformSearchParams {
    #[doc = "Plain text or query selector or XPath search query."]
    #[serde(rename = "query")]
    pub query: String,
    #[doc = "True to search in user agent shadow DOM."]
    #[serde(rename = "includeUserAgentShadowDOM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_user_agent_shadow_dom: Option<bool>,
}
impl PerformSearchParams {
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            include_user_agent_shadow_dom: None,
        }
    }
}
impl<T: Into<String>> From<T> for PerformSearchParams {
    fn from(url: T) -> Self {
        PerformSearchParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PerformSearchMethod {
    #[serde(rename = "DOM.performSearch")]
    PerformSearch,
}
#[doc = "Searches for a given string in the DOM tree. Use `getSearchResults` to access search results or\n`cancelSearch` to end this search session.\n[performSearch](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-performSearch)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformSearch {
    pub method: PerformSearchMethod,
    pub params: PerformSearchParams,
}
impl PerformSearch {
    pub const IDENTIFIER: &'static str = "DOM.performSearch";
}
impl crate::CommandResult for PerformSearch {
    type Result = super::results::PerformSearchResult;
}
#[doc = "Requests that the node is sent to the caller given its path. // FIXME, use XPath\n[pushNodeByPathToFrontend](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-pushNodeByPathToFrontend)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushNodeByPathToFrontendParams {
    #[doc = "Path to node in the proprietary format."]
    #[serde(rename = "path")]
    pub path: String,
}
impl PushNodeByPathToFrontendParams {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
    }
}
impl<T: Into<String>> From<T> for PushNodeByPathToFrontendParams {
    fn from(url: T) -> Self {
        PushNodeByPathToFrontendParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PushNodeByPathToFrontendMethod {
    #[serde(rename = "DOM.pushNodeByPathToFrontend")]
    PushNodeByPathToFrontend,
}
#[doc = "Requests that the node is sent to the caller given its path. // FIXME, use XPath\n[pushNodeByPathToFrontend](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-pushNodeByPathToFrontend)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushNodeByPathToFrontend {
    pub method: PushNodeByPathToFrontendMethod,
    pub params: PushNodeByPathToFrontendParams,
}
impl PushNodeByPathToFrontend {
    pub const IDENTIFIER: &'static str = "DOM.pushNodeByPathToFrontend";
}
impl crate::CommandResult for PushNodeByPathToFrontend {
    type Result = super::results::PushNodeByPathToFrontendResult;
}
#[doc = "Requests that a batch of nodes is sent to the caller given their backend node ids.\n[pushNodesByBackendIdsToFrontend](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-pushNodesByBackendIdsToFrontend)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushNodesByBackendIdsToFrontendParams {
    #[doc = "The array of backend node ids."]
    #[serde(rename = "backendNodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub backend_node_ids: Vec<super::types::BackendNodeId>,
}
impl PushNodesByBackendIdsToFrontendParams {
    pub fn new(backend_node_ids: Vec<super::types::BackendNodeId>) -> Self {
        Self { backend_node_ids }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PushNodesByBackendIdsToFrontendMethod {
    #[serde(rename = "DOM.pushNodesByBackendIdsToFrontend")]
    PushNodesByBackendIdsToFrontend,
}
#[doc = "Requests that a batch of nodes is sent to the caller given their backend node ids.\n[pushNodesByBackendIdsToFrontend](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-pushNodesByBackendIdsToFrontend)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushNodesByBackendIdsToFrontend {
    pub method: PushNodesByBackendIdsToFrontendMethod,
    pub params: PushNodesByBackendIdsToFrontendParams,
}
impl PushNodesByBackendIdsToFrontend {
    pub const IDENTIFIER: &'static str = "DOM.pushNodesByBackendIdsToFrontend";
}
impl crate::CommandResult for PushNodesByBackendIdsToFrontend {
    type Result = super::results::PushNodesByBackendIdsToFrontendResult;
}
#[doc = "Executes `querySelector` on a given node.\n[querySelector](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-querySelector)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuerySelectorParams {
    #[doc = "Id of the node to query upon."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "Selector string."]
    #[serde(rename = "selector")]
    pub selector: String,
}
impl QuerySelectorParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, selector: impl Into<String>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            selector: selector.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuerySelectorMethod {
    #[serde(rename = "DOM.querySelector")]
    QuerySelector,
}
#[doc = "Executes `querySelector` on a given node.\n[querySelector](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-querySelector)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuerySelector {
    pub method: QuerySelectorMethod,
    pub params: QuerySelectorParams,
}
impl QuerySelector {
    pub const IDENTIFIER: &'static str = "DOM.querySelector";
}
impl crate::CommandResult for QuerySelector {
    type Result = super::results::QuerySelectorResult;
}
#[doc = "Executes `querySelectorAll` on a given node.\n[querySelectorAll](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-querySelectorAll)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuerySelectorAllParams {
    #[doc = "Id of the node to query upon."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "Selector string."]
    #[serde(rename = "selector")]
    pub selector: String,
}
impl QuerySelectorAllParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, selector: impl Into<String>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            selector: selector.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuerySelectorAllMethod {
    #[serde(rename = "DOM.querySelectorAll")]
    QuerySelectorAll,
}
#[doc = "Executes `querySelectorAll` on a given node.\n[querySelectorAll](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-querySelectorAll)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuerySelectorAll {
    pub method: QuerySelectorAllMethod,
    pub params: QuerySelectorAllParams,
}
impl QuerySelectorAll {
    pub const IDENTIFIER: &'static str = "DOM.querySelectorAll";
}
impl crate::CommandResult for QuerySelectorAll {
    type Result = super::results::QuerySelectorAllResult;
}
#[doc = "Returns NodeIds of current top layer elements.\nTop layer is rendered closest to the user within a viewport, therefore its elements always\nappear on top of all other content.\n[getTopLayerElements](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getTopLayerElements)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTopLayerElementsParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetTopLayerElementsMethod {
    #[serde(rename = "DOM.getTopLayerElements")]
    GetTopLayerElements,
}
#[doc = "Returns NodeIds of current top layer elements.\nTop layer is rendered closest to the user within a viewport, therefore its elements always\nappear on top of all other content.\n[getTopLayerElements](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getTopLayerElements)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTopLayerElements {
    pub method: GetTopLayerElementsMethod,
    pub params: GetTopLayerElementsParams,
}
impl GetTopLayerElements {
    pub const IDENTIFIER: &'static str = "DOM.getTopLayerElements";
}
impl crate::CommandResult for GetTopLayerElements {
    type Result = super::results::GetTopLayerElementsResult;
}
#[doc = "Returns the NodeId of the matched element according to certain relations.\n[getElementByRelation](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getElementByRelation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetElementByRelationParams {
    #[doc = "Id of the node from which to query the relation."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "Type of relation to get."]
    #[serde(rename = "relation")]
    pub relation: GetElementByRelationRelation,
}
#[doc = "Type of relation to get."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GetElementByRelationRelation {
    #[doc = "Get the popover target for a given element. In this case, this given\nelement can only be an HTMLFormControlElement (<input>, <button>)."]
    #[serde(rename = "PopoverTarget")]
    PopoverTarget,
    #[doc = "Get the interestfor target (the attribute used to be named\n`interesttarget`) for for a given element."]
    #[serde(rename = "InterestTarget")]
    InterestTarget,
    #[doc = "Get the commandfor target for a given element. In this case, this given\nelement can only be an HTMLButtonElement."]
    #[serde(rename = "CommandFor")]
    CommandFor,
}
impl GetElementByRelationParams {
    pub fn new(
        node_id: impl Into<super::types::NodeId>,
        relation: impl Into<GetElementByRelationRelation>,
    ) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            relation: relation.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetElementByRelationMethod {
    #[serde(rename = "DOM.getElementByRelation")]
    GetElementByRelation,
}
#[doc = "Returns the NodeId of the matched element according to certain relations.\n[getElementByRelation](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getElementByRelation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetElementByRelation {
    pub method: GetElementByRelationMethod,
    pub params: GetElementByRelationParams,
}
impl GetElementByRelation {
    pub const IDENTIFIER: &'static str = "DOM.getElementByRelation";
}
impl crate::CommandResult for GetElementByRelation {
    type Result = super::results::GetElementByRelationResult;
}
#[doc = "Re-does the last undone action.\n[redo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-redo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RedoParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RedoMethod {
    #[serde(rename = "DOM.redo")]
    Redo,
}
#[doc = "Re-does the last undone action.\n[redo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-redo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Redo {
    pub method: RedoMethod,
    pub params: RedoParams,
}
impl Redo {
    pub const IDENTIFIER: &'static str = "DOM.redo";
}
impl crate::CommandResult for Redo {
    type Result = super::results::RedoResult;
}
#[doc = "Removes attribute with given name from an element with given id.\n[removeAttribute](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-removeAttribute)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveAttributeParams {
    #[doc = "Id of the element to remove attribute from."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "Name of the attribute to remove."]
    #[serde(rename = "name")]
    pub name: String,
}
impl RemoveAttributeParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, name: impl Into<String>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            name: name.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveAttributeMethod {
    #[serde(rename = "DOM.removeAttribute")]
    RemoveAttribute,
}
#[doc = "Removes attribute with given name from an element with given id.\n[removeAttribute](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-removeAttribute)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveAttribute {
    pub method: RemoveAttributeMethod,
    pub params: RemoveAttributeParams,
}
impl RemoveAttribute {
    pub const IDENTIFIER: &'static str = "DOM.removeAttribute";
}
impl crate::CommandResult for RemoveAttribute {
    type Result = super::results::RemoveAttributeResult;
}
#[doc = "Removes node with given id.\n[removeNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-removeNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveNodeParams {
    #[doc = "Id of the node to remove."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl RemoveNodeParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveNodeMethod {
    #[serde(rename = "DOM.removeNode")]
    RemoveNode,
}
#[doc = "Removes node with given id.\n[removeNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-removeNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveNode {
    pub method: RemoveNodeMethod,
    pub params: RemoveNodeParams,
}
impl RemoveNode {
    pub const IDENTIFIER: &'static str = "DOM.removeNode";
}
impl crate::CommandResult for RemoveNode {
    type Result = super::results::RemoveNodeResult;
}
#[doc = "Requests that children of the node with given id are returned to the caller in form of\n`setChildNodes` events where not only immediate children are retrieved, but all children down to\nthe specified depth.\n[requestChildNodes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-requestChildNodes)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestChildNodesParams {
    #[doc = "Id of the node to get children for."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "The maximum depth at which children should be retrieved, defaults to 1. Use -1 for the\nentire subtree or provide an integer larger than 0."]
    #[serde(rename = "depth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub depth: Option<i64>,
    #[doc = "Whether or not iframes and shadow roots should be traversed when returning the sub-tree\n(default is false)."]
    #[serde(rename = "pierce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pierce: Option<bool>,
}
impl RequestChildNodesParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            depth: None,
            pierce: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RequestChildNodesMethod {
    #[serde(rename = "DOM.requestChildNodes")]
    RequestChildNodes,
}
#[doc = "Requests that children of the node with given id are returned to the caller in form of\n`setChildNodes` events where not only immediate children are retrieved, but all children down to\nthe specified depth.\n[requestChildNodes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-requestChildNodes)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestChildNodes {
    pub method: RequestChildNodesMethod,
    pub params: RequestChildNodesParams,
}
impl RequestChildNodes {
    pub const IDENTIFIER: &'static str = "DOM.requestChildNodes";
}
impl crate::CommandResult for RequestChildNodes {
    type Result = super::results::RequestChildNodesResult;
}
#[doc = "Requests that the node is sent to the caller given the JavaScript node object reference. All\nnodes that form the path from the node to the root are also sent to the client as a series of\n`setChildNodes` notifications.\n[requestNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-requestNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestNodeParams {
    #[doc = "JavaScript object id to convert into node."]
    #[serde(rename = "objectId")]
    pub object_id: crate::js_protocol::runtime::types::RemoteObjectId,
}
impl RequestNodeParams {
    pub fn new(object_id: impl Into<crate::js_protocol::runtime::types::RemoteObjectId>) -> Self {
        Self {
            object_id: object_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RequestNodeMethod {
    #[serde(rename = "DOM.requestNode")]
    RequestNode,
}
#[doc = "Requests that the node is sent to the caller given the JavaScript node object reference. All\nnodes that form the path from the node to the root are also sent to the client as a series of\n`setChildNodes` notifications.\n[requestNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-requestNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestNode {
    pub method: RequestNodeMethod,
    pub params: RequestNodeParams,
}
impl RequestNode {
    pub const IDENTIFIER: &'static str = "DOM.requestNode";
}
impl crate::CommandResult for RequestNode {
    type Result = super::results::RequestNodeResult;
}
#[doc = "Resolves the JavaScript node object for a given NodeId or BackendNodeId.\n[resolveNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-resolveNode)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResolveNodeParams {
    #[doc = "Id of the node to resolve."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<Box<super::types::NodeId>>,
    #[doc = "Backend identifier of the node to resolve."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<Box<super::types::BackendNodeId>>,
    #[doc = "Symbolic group name that can be used to release multiple objects."]
    #[serde(rename = "objectGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_group: Option<String>,
    #[doc = "Execution context in which to resolve the node."]
    #[serde(rename = "executionContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub execution_context_id: Option<crate::js_protocol::runtime::types::ExecutionContextId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResolveNodeMethod {
    #[serde(rename = "DOM.resolveNode")]
    ResolveNode,
}
#[doc = "Resolves the JavaScript node object for a given NodeId or BackendNodeId.\n[resolveNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-resolveNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveNode {
    pub method: ResolveNodeMethod,
    pub params: ResolveNodeParams,
}
impl ResolveNode {
    pub const IDENTIFIER: &'static str = "DOM.resolveNode";
}
impl crate::CommandResult for ResolveNode {
    type Result = super::results::ResolveNodeResult;
}
#[doc = "Sets attribute for an element with given id.\n[setAttributeValue](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setAttributeValue)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAttributeValueParams {
    #[doc = "Id of the element to set attribute for."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "Attribute name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Attribute value."]
    #[serde(rename = "value")]
    pub value: String,
}
impl SetAttributeValueParams {
    pub fn new(
        node_id: impl Into<super::types::NodeId>,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            name: name.into(),
            value: value.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetAttributeValueMethod {
    #[serde(rename = "DOM.setAttributeValue")]
    SetAttributeValue,
}
#[doc = "Sets attribute for an element with given id.\n[setAttributeValue](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setAttributeValue)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAttributeValue {
    pub method: SetAttributeValueMethod,
    pub params: SetAttributeValueParams,
}
impl SetAttributeValue {
    pub const IDENTIFIER: &'static str = "DOM.setAttributeValue";
}
impl crate::CommandResult for SetAttributeValue {
    type Result = super::results::SetAttributeValueResult;
}
#[doc = "Sets attributes on element with given id. This method is useful when user edits some existing\nattribute value and types in several attribute name/value pairs.\n[setAttributesAsText](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setAttributesAsText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAttributesAsTextParams {
    #[doc = "Id of the element to set attributes for."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "Text with a number of attributes. Will parse this text using HTML parser."]
    #[serde(rename = "text")]
    pub text: String,
    #[doc = "Attribute name to replace with new attributes derived from text in case text parsed\nsuccessfully."]
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
}
impl SetAttributesAsTextParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, text: impl Into<String>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            text: text.into(),
            name: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetAttributesAsTextMethod {
    #[serde(rename = "DOM.setAttributesAsText")]
    SetAttributesAsText,
}
#[doc = "Sets attributes on element with given id. This method is useful when user edits some existing\nattribute value and types in several attribute name/value pairs.\n[setAttributesAsText](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setAttributesAsText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAttributesAsText {
    pub method: SetAttributesAsTextMethod,
    pub params: SetAttributesAsTextParams,
}
impl SetAttributesAsText {
    pub const IDENTIFIER: &'static str = "DOM.setAttributesAsText";
}
impl crate::CommandResult for SetAttributesAsText {
    type Result = super::results::SetAttributesAsTextResult;
}
#[doc = "Sets files for the given file input element.\n[setFileInputFiles](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setFileInputFiles)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetFileInputFilesParams {
    #[doc = "Array of file paths to set."]
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
    #[doc = "Identifier of the node."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<Box<super::types::NodeId>>,
    #[doc = "Identifier of the backend node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<Box<super::types::BackendNodeId>>,
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<crate::js_protocol::runtime::types::RemoteObjectId>,
}
impl SetFileInputFilesParams {
    pub fn new(files: Vec<String>) -> Self {
        Self {
            files,
            node_id: None,
            backend_node_id: None,
            object_id: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetFileInputFilesMethod {
    #[serde(rename = "DOM.setFileInputFiles")]
    SetFileInputFiles,
}
#[doc = "Sets files for the given file input element.\n[setFileInputFiles](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setFileInputFiles)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetFileInputFiles {
    pub method: SetFileInputFilesMethod,
    pub params: SetFileInputFilesParams,
}
impl SetFileInputFiles {
    pub const IDENTIFIER: &'static str = "DOM.setFileInputFiles";
}
impl crate::CommandResult for SetFileInputFiles {
    type Result = super::results::SetFileInputFilesResult;
}
#[doc = "Sets if stack traces should be captured for Nodes. See `Node.getNodeStackTraces`. Default is disabled.\n[setNodeStackTracesEnabled](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setNodeStackTracesEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetNodeStackTracesEnabledParams {
    #[doc = "Enable or disable."]
    #[serde(rename = "enable")]
    pub enable: bool,
}
impl SetNodeStackTracesEnabledParams {
    pub fn new(enable: impl Into<bool>) -> Self {
        Self {
            enable: enable.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetNodeStackTracesEnabledMethod {
    #[serde(rename = "DOM.setNodeStackTracesEnabled")]
    SetNodeStackTracesEnabled,
}
#[doc = "Sets if stack traces should be captured for Nodes. See `Node.getNodeStackTraces`. Default is disabled.\n[setNodeStackTracesEnabled](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setNodeStackTracesEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetNodeStackTracesEnabled {
    pub method: SetNodeStackTracesEnabledMethod,
    pub params: SetNodeStackTracesEnabledParams,
}
impl SetNodeStackTracesEnabled {
    pub const IDENTIFIER: &'static str = "DOM.setNodeStackTracesEnabled";
}
impl crate::CommandResult for SetNodeStackTracesEnabled {
    type Result = super::results::SetNodeStackTracesEnabledResult;
}
#[doc = "Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.\n[getNodeStackTraces](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getNodeStackTraces)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeStackTracesParams {
    #[doc = "Id of the node to get stack traces for."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl GetNodeStackTracesParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetNodeStackTracesMethod {
    #[serde(rename = "DOM.getNodeStackTraces")]
    GetNodeStackTraces,
}
#[doc = "Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.\n[getNodeStackTraces](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getNodeStackTraces)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeStackTraces {
    pub method: GetNodeStackTracesMethod,
    pub params: GetNodeStackTracesParams,
}
impl GetNodeStackTraces {
    pub const IDENTIFIER: &'static str = "DOM.getNodeStackTraces";
}
impl crate::CommandResult for GetNodeStackTraces {
    type Result = super::results::GetNodeStackTracesResult;
}
#[doc = "Returns file information for the given\nFile wrapper.\n[getFileInfo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getFileInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFileInfoParams {
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    pub object_id: crate::js_protocol::runtime::types::RemoteObjectId,
}
impl GetFileInfoParams {
    pub fn new(object_id: impl Into<crate::js_protocol::runtime::types::RemoteObjectId>) -> Self {
        Self {
            object_id: object_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetFileInfoMethod {
    #[serde(rename = "DOM.getFileInfo")]
    GetFileInfo,
}
#[doc = "Returns file information for the given\nFile wrapper.\n[getFileInfo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getFileInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFileInfo {
    pub method: GetFileInfoMethod,
    pub params: GetFileInfoParams,
}
impl GetFileInfo {
    pub const IDENTIFIER: &'static str = "DOM.getFileInfo";
}
impl crate::CommandResult for GetFileInfo {
    type Result = super::results::GetFileInfoResult;
}
#[doc = "Returns list of detached nodes\n[getDetachedDomNodes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getDetachedDomNodes)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDetachedDomNodesParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetDetachedDomNodesMethod {
    #[serde(rename = "DOM.getDetachedDomNodes")]
    GetDetachedDomNodes,
}
#[doc = "Returns list of detached nodes\n[getDetachedDomNodes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getDetachedDomNodes)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDetachedDomNodes {
    pub method: GetDetachedDomNodesMethod,
    pub params: GetDetachedDomNodesParams,
}
impl GetDetachedDomNodes {
    pub const IDENTIFIER: &'static str = "DOM.getDetachedDomNodes";
}
impl crate::CommandResult for GetDetachedDomNodes {
    type Result = super::results::GetDetachedDomNodesResult;
}
#[doc = "Enables console to refer to the node with given id via $x (see Command Line API for more details\n$x functions).\n[setInspectedNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setInspectedNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInspectedNodeParams {
    #[doc = "DOM node id to be accessible by means of $x command line API."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl SetInspectedNodeParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetInspectedNodeMethod {
    #[serde(rename = "DOM.setInspectedNode")]
    SetInspectedNode,
}
#[doc = "Enables console to refer to the node with given id via $x (see Command Line API for more details\n$x functions).\n[setInspectedNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setInspectedNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInspectedNode {
    pub method: SetInspectedNodeMethod,
    pub params: SetInspectedNodeParams,
}
impl SetInspectedNode {
    pub const IDENTIFIER: &'static str = "DOM.setInspectedNode";
}
impl crate::CommandResult for SetInspectedNode {
    type Result = super::results::SetInspectedNodeResult;
}
#[doc = "Sets node name for a node with given id.\n[setNodeName](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setNodeName)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetNodeNameParams {
    #[doc = "Id of the node to set name for."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "New node's name."]
    #[serde(rename = "name")]
    pub name: String,
}
impl SetNodeNameParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, name: impl Into<String>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            name: name.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetNodeNameMethod {
    #[serde(rename = "DOM.setNodeName")]
    SetNodeName,
}
#[doc = "Sets node name for a node with given id.\n[setNodeName](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setNodeName)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetNodeName {
    pub method: SetNodeNameMethod,
    pub params: SetNodeNameParams,
}
impl SetNodeName {
    pub const IDENTIFIER: &'static str = "DOM.setNodeName";
}
impl crate::CommandResult for SetNodeName {
    type Result = super::results::SetNodeNameResult;
}
#[doc = "Sets node value for a node with given id.\n[setNodeValue](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setNodeValue)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetNodeValueParams {
    #[doc = "Id of the node to set value for."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "New node's value."]
    #[serde(rename = "value")]
    pub value: String,
}
impl SetNodeValueParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, value: impl Into<String>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            value: value.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetNodeValueMethod {
    #[serde(rename = "DOM.setNodeValue")]
    SetNodeValue,
}
#[doc = "Sets node value for a node with given id.\n[setNodeValue](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setNodeValue)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetNodeValue {
    pub method: SetNodeValueMethod,
    pub params: SetNodeValueParams,
}
impl SetNodeValue {
    pub const IDENTIFIER: &'static str = "DOM.setNodeValue";
}
impl crate::CommandResult for SetNodeValue {
    type Result = super::results::SetNodeValueResult;
}
#[doc = "Sets node HTML markup, returns new node id.\n[setOuterHTML](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setOuterHTML)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetOuterHtmlParams {
    #[doc = "Id of the node to set markup for."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "Outer HTML markup to set."]
    #[serde(rename = "outerHTML")]
    pub outer_html: String,
}
impl SetOuterHtmlParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, outer_html: impl Into<String>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            outer_html: outer_html.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetOuterHtmlMethod {
    #[serde(rename = "DOM.setOuterHTML")]
    SetOuterHtml,
}
#[doc = "Sets node HTML markup, returns new node id.\n[setOuterHTML](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setOuterHTML)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetOuterHtml {
    pub method: SetOuterHtmlMethod,
    pub params: SetOuterHtmlParams,
}
impl SetOuterHtml {
    pub const IDENTIFIER: &'static str = "DOM.setOuterHTML";
}
impl crate::CommandResult for SetOuterHtml {
    type Result = super::results::SetOuterHtmlResult;
}
#[doc = "Undoes the last performed action.\n[undo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-undo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UndoParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UndoMethod {
    #[serde(rename = "DOM.undo")]
    Undo,
}
#[doc = "Undoes the last performed action.\n[undo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-undo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Undo {
    pub method: UndoMethod,
    pub params: UndoParams,
}
impl Undo {
    pub const IDENTIFIER: &'static str = "DOM.undo";
}
impl crate::CommandResult for Undo {
    type Result = super::results::UndoResult;
}
#[doc = "Returns iframe node that owns iframe with the given domain.\n[getFrameOwner](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getFrameOwner)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFrameOwnerParams {
    #[serde(rename = "frameId")]
    pub frame_id: crate::browser_protocol::page::types::FrameId,
}
impl GetFrameOwnerParams {
    pub fn new(frame_id: impl Into<crate::browser_protocol::page::types::FrameId>) -> Self {
        Self {
            frame_id: frame_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetFrameOwnerMethod {
    #[serde(rename = "DOM.getFrameOwner")]
    GetFrameOwner,
}
#[doc = "Returns iframe node that owns iframe with the given domain.\n[getFrameOwner](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getFrameOwner)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFrameOwner {
    pub method: GetFrameOwnerMethod,
    pub params: GetFrameOwnerParams,
}
impl GetFrameOwner {
    pub const IDENTIFIER: &'static str = "DOM.getFrameOwner";
}
impl crate::CommandResult for GetFrameOwner {
    type Result = super::results::GetFrameOwnerResult;
}
#[doc = "Returns the query container of the given node based on container query\nconditions: containerName, physical and logical axes, and whether it queries\nscroll-state or anchored elements. If no axes are provided and\nqueriesScrollState is false, the style container is returned, which is the\ndirect parent or the closest element with a matching container-name.\n[getContainerForNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getContainerForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContainerForNodeParams {
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[serde(rename = "containerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub container_name: Option<String>,
    #[serde(rename = "physicalAxes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub physical_axes: Option<super::types::PhysicalAxes>,
    #[serde(rename = "logicalAxes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub logical_axes: Option<super::types::LogicalAxes>,
    #[serde(rename = "queriesScrollState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub queries_scroll_state: Option<bool>,
    #[serde(rename = "queriesAnchored")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub queries_anchored: Option<bool>,
}
impl GetContainerForNodeParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            container_name: None,
            physical_axes: None,
            logical_axes: None,
            queries_scroll_state: None,
            queries_anchored: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetContainerForNodeMethod {
    #[serde(rename = "DOM.getContainerForNode")]
    GetContainerForNode,
}
#[doc = "Returns the query container of the given node based on container query\nconditions: containerName, physical and logical axes, and whether it queries\nscroll-state or anchored elements. If no axes are provided and\nqueriesScrollState is false, the style container is returned, which is the\ndirect parent or the closest element with a matching container-name.\n[getContainerForNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getContainerForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContainerForNode {
    pub method: GetContainerForNodeMethod,
    pub params: GetContainerForNodeParams,
}
impl GetContainerForNode {
    pub const IDENTIFIER: &'static str = "DOM.getContainerForNode";
}
impl crate::CommandResult for GetContainerForNode {
    type Result = super::results::GetContainerForNodeResult;
}
#[doc = "Returns the descendants of a container query container that have\ncontainer queries against this container.\n[getQueryingDescendantsForContainer](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getQueryingDescendantsForContainer)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetQueryingDescendantsForContainerParams {
    #[doc = "Id of the container node to find querying descendants from."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl GetQueryingDescendantsForContainerParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetQueryingDescendantsForContainerMethod {
    #[serde(rename = "DOM.getQueryingDescendantsForContainer")]
    GetQueryingDescendantsForContainer,
}
#[doc = "Returns the descendants of a container query container that have\ncontainer queries against this container.\n[getQueryingDescendantsForContainer](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getQueryingDescendantsForContainer)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetQueryingDescendantsForContainer {
    pub method: GetQueryingDescendantsForContainerMethod,
    pub params: GetQueryingDescendantsForContainerParams,
}
impl GetQueryingDescendantsForContainer {
    pub const IDENTIFIER: &'static str = "DOM.getQueryingDescendantsForContainer";
}
impl crate::CommandResult for GetQueryingDescendantsForContainer {
    type Result = super::results::GetQueryingDescendantsForContainerResult;
}
#[doc = "Returns the target anchor element of the given anchor query according to\nhttps://www.w3.org/TR/css-anchor-position-1/#target.\n[getAnchorElement](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getAnchorElement)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAnchorElementParams {
    #[doc = "Id of the positioned element from which to find the anchor."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "An optional anchor specifier, as defined in\nhttps://www.w3.org/TR/css-anchor-position-1/#anchor-specifier.\nIf not provided, it will return the implicit anchor element for\nthe given positioned element."]
    #[serde(rename = "anchorSpecifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub anchor_specifier: Option<String>,
}
impl GetAnchorElementParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            anchor_specifier: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetAnchorElementMethod {
    #[serde(rename = "DOM.getAnchorElement")]
    GetAnchorElement,
}
#[doc = "Returns the target anchor element of the given anchor query according to\nhttps://www.w3.org/TR/css-anchor-position-1/#target.\n[getAnchorElement](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getAnchorElement)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAnchorElement {
    pub method: GetAnchorElementMethod,
    pub params: GetAnchorElementParams,
}
impl GetAnchorElement {
    pub const IDENTIFIER: &'static str = "DOM.getAnchorElement";
}
impl crate::CommandResult for GetAnchorElement {
    type Result = super::results::GetAnchorElementResult;
}
#[doc = "When enabling, this API force-opens the popover identified by nodeId\nand keeps it open until disabled.\n[forceShowPopover](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-forceShowPopover)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForceShowPopoverParams {
    #[doc = "Id of the popover HTMLElement"]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "If true, opens the popover and keeps it open. If false, closes the\npopover if it was previously force-opened."]
    #[serde(rename = "enable")]
    pub enable: bool,
}
impl ForceShowPopoverParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, enable: impl Into<bool>) -> Self {
        Self {
            node_id: Box::new(node_id.into()),
            enable: enable.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ForceShowPopoverMethod {
    #[serde(rename = "DOM.forceShowPopover")]
    ForceShowPopover,
}
#[doc = "When enabling, this API force-opens the popover identified by nodeId\nand keeps it open until disabled.\n[forceShowPopover](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-forceShowPopover)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForceShowPopover {
    pub method: ForceShowPopoverMethod,
    pub params: ForceShowPopoverParams,
}
impl ForceShowPopover {
    pub const IDENTIFIER: &'static str = "DOM.forceShowPopover";
}
impl crate::CommandResult for ForceShowPopover {
    type Result = super::results::ForceShowPopoverResult;
}
group_enum ! (DomCommands { CollectClassNamesFromSubtree (CollectClassNamesFromSubtree) , CopyTo (CopyTo) , DescribeNode (DescribeNode) , ScrollIntoViewIfNeeded (ScrollIntoViewIfNeeded) , Disable (Disable) , DiscardSearchResults (DiscardSearchResults) , Enable (Enable) , Focus (Focus) , GetAttributes (GetAttributes) , GetBoxModel (GetBoxModel) , GetContentQuads (GetContentQuads) , GetDocument (GetDocument) , GetNodesForSubtreeByStyle (GetNodesForSubtreeByStyle) , GetNodeForLocation (GetNodeForLocation) , GetOuterHtml (GetOuterHtml) , GetRelayoutBoundary (GetRelayoutBoundary) , GetSearchResults (GetSearchResults) , HideHighlight (HideHighlight) , HighlightNode (HighlightNode) , HighlightRect (HighlightRect) , MarkUndoableState (MarkUndoableState) , MoveTo (MoveTo) , PerformSearch (PerformSearch) , PushNodeByPathToFrontend (PushNodeByPathToFrontend) , PushNodesByBackendIdsToFrontend (PushNodesByBackendIdsToFrontend) , QuerySelector (QuerySelector) , QuerySelectorAll (QuerySelectorAll) , GetTopLayerElements (GetTopLayerElements) , GetElementByRelation (GetElementByRelation) , Redo (Redo) , RemoveAttribute (RemoveAttribute) , RemoveNode (RemoveNode) , RequestChildNodes (RequestChildNodes) , RequestNode (RequestNode) , ResolveNode (ResolveNode) , SetAttributeValue (SetAttributeValue) , SetAttributesAsText (SetAttributesAsText) , SetFileInputFiles (SetFileInputFiles) , SetNodeStackTracesEnabled (SetNodeStackTracesEnabled) , GetNodeStackTraces (GetNodeStackTraces) , GetFileInfo (GetFileInfo) , GetDetachedDomNodes (GetDetachedDomNodes) , SetInspectedNode (SetInspectedNode) , SetNodeName (SetNodeName) , SetNodeValue (SetNodeValue) , SetOuterHtml (SetOuterHtml) , Undo (Undo) , GetFrameOwner (GetFrameOwner) , GetContainerForNode (GetContainerForNode) , GetQueryingDescendantsForContainer (GetQueryingDescendantsForContainer) , GetAnchorElement (GetAnchorElement) , ForceShowPopover (ForceShowPopover) });
