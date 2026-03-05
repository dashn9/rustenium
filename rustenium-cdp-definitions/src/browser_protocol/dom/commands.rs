use serde::{Deserialize, Serialize};
#[doc = "Collects class names for the node with given id and all of it's child nodes.\n[collectClassNamesFromSubtree](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-collectClassNamesFromSubtree)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectClassNamesFromSubtreeParams {
    #[doc = "Id of the node to collect class names."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
impl CollectClassNamesFromSubtreeParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CollectClassNamesFromSubtreeMethod {
    #[serde(rename = "DOM.collectClassNamesFromSubtree")]
    CollectClassNamesFromSubtree,
}
impl CollectClassNamesFromSubtreeMethod {
    pub const IDENTIFIER: &'static str = "DOM.collectClassNamesFromSubtree";
}
#[doc = "Collects class names for the node with given id and all of it's child nodes.\n[collectClassNamesFromSubtree](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-collectClassNamesFromSubtree)"]
#[derive(Debug, Clone, PartialEq)]
pub struct CollectClassNamesFromSubtree {
    pub method: CollectClassNamesFromSubtreeMethod,
    pub params: CollectClassNamesFromSubtreeParams,
}
#[doc = "Creates a deep copy of the specified node and places it into the target container before the\ngiven anchor.\n[copyTo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-copyTo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyToParams {
    #[doc = "Id of the node to copy."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
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
            node_id: node_id.into(),
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
impl CopyToMethod {
    pub const IDENTIFIER: &'static str = "DOM.copyTo";
}
#[doc = "Creates a deep copy of the specified node and places it into the target container before the\ngiven anchor.\n[copyTo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-copyTo)"]
#[derive(Debug, Clone, PartialEq)]
pub struct CopyTo {
    pub method: CopyToMethod,
    pub params: CopyToParams,
}
#[doc = "Describes node given its id, does not require domain to be enabled. Does not start tracking any\nobjects, can be used for automation.\n[describeNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-describeNode)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DescribeNodeParams {
    #[doc = "Identifier of the node."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<super::types::NodeId>,
    #[doc = "Identifier of the backend node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<super::types::BackendNodeId>,
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
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
impl DescribeNodeMethod {
    pub const IDENTIFIER: &'static str = "DOM.describeNode";
}
#[doc = "Describes node given its id, does not require domain to be enabled. Does not start tracking any\nobjects, can be used for automation.\n[describeNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-describeNode)"]
#[derive(Debug, Clone, PartialEq)]
pub struct DescribeNode {
    pub method: DescribeNodeMethod,
    pub params: DescribeNodeParams,
}
#[doc = "Scrolls the specified rect of the given node into view if not already visible.\nNote: exactly one between nodeId, backendNodeId and objectId should be passed\nto identify the node.\n[scrollIntoViewIfNeeded](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-scrollIntoViewIfNeeded)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScrollIntoViewIfNeededParams {
    #[doc = "Identifier of the node."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<super::types::NodeId>,
    #[doc = "Identifier of the backend node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<super::types::BackendNodeId>,
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
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
impl ScrollIntoViewIfNeededMethod {
    pub const IDENTIFIER: &'static str = "DOM.scrollIntoViewIfNeeded";
}
#[doc = "Scrolls the specified rect of the given node into view if not already visible.\nNote: exactly one between nodeId, backendNodeId and objectId should be passed\nto identify the node.\n[scrollIntoViewIfNeeded](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-scrollIntoViewIfNeeded)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ScrollIntoViewIfNeeded {
    pub method: ScrollIntoViewIfNeededMethod,
    pub params: ScrollIntoViewIfNeededParams,
}
#[doc = "Disables DOM agent for the given page.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "DOM.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "DOM.disable";
}
#[doc = "Disables DOM agent for the given page.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
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
impl DiscardSearchResultsMethod {
    pub const IDENTIFIER: &'static str = "DOM.discardSearchResults";
}
#[doc = "Discards search results from the session with the given id. `getSearchResults` should no longer\nbe called for that search.\n[discardSearchResults](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-discardSearchResults)"]
#[derive(Debug, Clone, PartialEq)]
pub struct DiscardSearchResults {
    pub method: DiscardSearchResultsMethod,
    pub params: DiscardSearchResultsParams,
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
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "DOM.enable";
}
#[doc = "Enables DOM agent for the given page.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
#[doc = "Focuses the given element.\n[focus](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-focus)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FocusParams {
    #[doc = "Identifier of the node."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<super::types::NodeId>,
    #[doc = "Identifier of the backend node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<super::types::BackendNodeId>,
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FocusMethod {
    #[serde(rename = "DOM.focus")]
    Focus,
}
impl FocusMethod {
    pub const IDENTIFIER: &'static str = "DOM.focus";
}
#[doc = "Focuses the given element.\n[focus](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-focus)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Focus {
    pub method: FocusMethod,
    pub params: FocusParams,
}
#[doc = "Returns attributes for the specified node.\n[getAttributes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getAttributes)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAttributesParams {
    #[doc = "Id of the node to retrieve attributes for."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
impl GetAttributesParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetAttributesMethod {
    #[serde(rename = "DOM.getAttributes")]
    GetAttributes,
}
impl GetAttributesMethod {
    pub const IDENTIFIER: &'static str = "DOM.getAttributes";
}
#[doc = "Returns attributes for the specified node.\n[getAttributes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getAttributes)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetAttributes {
    pub method: GetAttributesMethod,
    pub params: GetAttributesParams,
}
#[doc = "Returns boxes for the given node.\n[getBoxModel](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getBoxModel)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetBoxModelParams {
    #[doc = "Identifier of the node."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<super::types::NodeId>,
    #[doc = "Identifier of the backend node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<super::types::BackendNodeId>,
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetBoxModelMethod {
    #[serde(rename = "DOM.getBoxModel")]
    GetBoxModel,
}
impl GetBoxModelMethod {
    pub const IDENTIFIER: &'static str = "DOM.getBoxModel";
}
#[doc = "Returns boxes for the given node.\n[getBoxModel](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getBoxModel)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetBoxModel {
    pub method: GetBoxModelMethod,
    pub params: GetBoxModelParams,
}
#[doc = "Returns quads that describe node position on the page. This method\nmight return multiple quads for inline nodes.\n[getContentQuads](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getContentQuads)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetContentQuadsParams {
    #[doc = "Identifier of the node."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<super::types::NodeId>,
    #[doc = "Identifier of the backend node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<super::types::BackendNodeId>,
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetContentQuadsMethod {
    #[serde(rename = "DOM.getContentQuads")]
    GetContentQuads,
}
impl GetContentQuadsMethod {
    pub const IDENTIFIER: &'static str = "DOM.getContentQuads";
}
#[doc = "Returns quads that describe node position on the page. This method\nmight return multiple quads for inline nodes.\n[getContentQuads](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getContentQuads)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetContentQuads {
    pub method: GetContentQuadsMethod,
    pub params: GetContentQuadsParams,
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
impl GetDocumentMethod {
    pub const IDENTIFIER: &'static str = "DOM.getDocument";
}
#[doc = "Returns the root DOM node (and optionally the subtree) to the caller.\nImplicitly enables the DOM domain events for the current target.\n[getDocument](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getDocument)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetDocument {
    pub method: GetDocumentMethod,
    pub params: GetDocumentParams,
}
#[doc = "Finds nodes with a given computed style in a subtree.\n[getNodesForSubtreeByStyle](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getNodesForSubtreeByStyle)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodesForSubtreeByStyleParams {
    #[doc = "Node ID pointing to the root of a subtree."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
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
            node_id: node_id.into(),
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
impl GetNodesForSubtreeByStyleMethod {
    pub const IDENTIFIER: &'static str = "DOM.getNodesForSubtreeByStyle";
}
#[doc = "Finds nodes with a given computed style in a subtree.\n[getNodesForSubtreeByStyle](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getNodesForSubtreeByStyle)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetNodesForSubtreeByStyle {
    pub method: GetNodesForSubtreeByStyleMethod,
    pub params: GetNodesForSubtreeByStyleParams,
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
impl GetNodeForLocationMethod {
    pub const IDENTIFIER: &'static str = "DOM.getNodeForLocation";
}
#[doc = "Returns node id at given location. Depending on whether DOM domain is enabled, nodeId is\neither returned or not.\n[getNodeForLocation](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getNodeForLocation)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetNodeForLocation {
    pub method: GetNodeForLocationMethod,
    pub params: GetNodeForLocationParams,
}
#[doc = "Returns node's HTML markup.\n[getOuterHTML](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getOuterHTML)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetOuterHtmlParams {
    #[doc = "Identifier of the node."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<super::types::NodeId>,
    #[doc = "Identifier of the backend node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<super::types::BackendNodeId>,
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
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
impl GetOuterHtmlMethod {
    pub const IDENTIFIER: &'static str = "DOM.getOuterHTML";
}
#[doc = "Returns node's HTML markup.\n[getOuterHTML](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getOuterHTML)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetOuterHtml {
    pub method: GetOuterHtmlMethod,
    pub params: GetOuterHtmlParams,
}
#[doc = "Returns the id of the nearest ancestor that is a relayout boundary.\n[getRelayoutBoundary](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getRelayoutBoundary)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRelayoutBoundaryParams {
    #[doc = "Id of the node."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
impl GetRelayoutBoundaryParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetRelayoutBoundaryMethod {
    #[serde(rename = "DOM.getRelayoutBoundary")]
    GetRelayoutBoundary,
}
impl GetRelayoutBoundaryMethod {
    pub const IDENTIFIER: &'static str = "DOM.getRelayoutBoundary";
}
#[doc = "Returns the id of the nearest ancestor that is a relayout boundary.\n[getRelayoutBoundary](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getRelayoutBoundary)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetRelayoutBoundary {
    pub method: GetRelayoutBoundaryMethod,
    pub params: GetRelayoutBoundaryParams,
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
impl GetSearchResultsMethod {
    pub const IDENTIFIER: &'static str = "DOM.getSearchResults";
}
#[doc = "Returns search results from given `fromIndex` to given `toIndex` from the search with the given\nidentifier.\n[getSearchResults](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getSearchResults)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetSearchResults {
    pub method: GetSearchResultsMethod,
    pub params: GetSearchResultsParams,
}
#[doc = "Hides any highlight.\n[hideHighlight](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-hideHighlight)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HideHighlightParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HideHighlightMethod {
    #[serde(rename = "DOM.hideHighlight")]
    HideHighlight,
}
impl HideHighlightMethod {
    pub const IDENTIFIER: &'static str = "DOM.hideHighlight";
}
#[doc = "Hides any highlight.\n[hideHighlight](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-hideHighlight)"]
#[derive(Debug, Clone, PartialEq)]
pub struct HideHighlight {
    pub method: HideHighlightMethod,
    pub params: HideHighlightParams,
}
#[doc = "Highlights DOM node.\n[highlightNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-highlightNode)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HighlightNodeParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HighlightNodeMethod {
    #[serde(rename = "DOM.highlightNode")]
    HighlightNode,
}
impl HighlightNodeMethod {
    pub const IDENTIFIER: &'static str = "DOM.highlightNode";
}
#[doc = "Highlights DOM node.\n[highlightNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-highlightNode)"]
#[derive(Debug, Clone, PartialEq)]
pub struct HighlightNode {
    pub method: HighlightNodeMethod,
    pub params: HighlightNodeParams,
}
#[doc = "Highlights given rectangle.\n[highlightRect](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-highlightRect)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HighlightRectParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HighlightRectMethod {
    #[serde(rename = "DOM.highlightRect")]
    HighlightRect,
}
impl HighlightRectMethod {
    pub const IDENTIFIER: &'static str = "DOM.highlightRect";
}
#[doc = "Highlights given rectangle.\n[highlightRect](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-highlightRect)"]
#[derive(Debug, Clone, PartialEq)]
pub struct HighlightRect {
    pub method: HighlightRectMethod,
    pub params: HighlightRectParams,
}
#[doc = "Marks last undoable state.\n[markUndoableState](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-markUndoableState)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MarkUndoableStateParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarkUndoableStateMethod {
    #[serde(rename = "DOM.markUndoableState")]
    MarkUndoableState,
}
impl MarkUndoableStateMethod {
    pub const IDENTIFIER: &'static str = "DOM.markUndoableState";
}
#[doc = "Marks last undoable state.\n[markUndoableState](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-markUndoableState)"]
#[derive(Debug, Clone, PartialEq)]
pub struct MarkUndoableState {
    pub method: MarkUndoableStateMethod,
    pub params: MarkUndoableStateParams,
}
#[doc = "Moves node into the new container, places it before the given anchor.\n[moveTo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-moveTo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveToParams {
    #[doc = "Id of the node to move."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
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
            node_id: node_id.into(),
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
impl MoveToMethod {
    pub const IDENTIFIER: &'static str = "DOM.moveTo";
}
#[doc = "Moves node into the new container, places it before the given anchor.\n[moveTo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-moveTo)"]
#[derive(Debug, Clone, PartialEq)]
pub struct MoveTo {
    pub method: MoveToMethod,
    pub params: MoveToParams,
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
impl PerformSearchMethod {
    pub const IDENTIFIER: &'static str = "DOM.performSearch";
}
#[doc = "Searches for a given string in the DOM tree. Use `getSearchResults` to access search results or\n`cancelSearch` to end this search session.\n[performSearch](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-performSearch)"]
#[derive(Debug, Clone, PartialEq)]
pub struct PerformSearch {
    pub method: PerformSearchMethod,
    pub params: PerformSearchParams,
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
impl PushNodeByPathToFrontendMethod {
    pub const IDENTIFIER: &'static str = "DOM.pushNodeByPathToFrontend";
}
#[doc = "Requests that the node is sent to the caller given its path. // FIXME, use XPath\n[pushNodeByPathToFrontend](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-pushNodeByPathToFrontend)"]
#[derive(Debug, Clone, PartialEq)]
pub struct PushNodeByPathToFrontend {
    pub method: PushNodeByPathToFrontendMethod,
    pub params: PushNodeByPathToFrontendParams,
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
impl PushNodesByBackendIdsToFrontendMethod {
    pub const IDENTIFIER: &'static str = "DOM.pushNodesByBackendIdsToFrontend";
}
#[doc = "Requests that a batch of nodes is sent to the caller given their backend node ids.\n[pushNodesByBackendIdsToFrontend](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-pushNodesByBackendIdsToFrontend)"]
#[derive(Debug, Clone, PartialEq)]
pub struct PushNodesByBackendIdsToFrontend {
    pub method: PushNodesByBackendIdsToFrontendMethod,
    pub params: PushNodesByBackendIdsToFrontendParams,
}
#[doc = "Executes `querySelector` on a given node.\n[querySelector](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-querySelector)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuerySelectorParams {
    #[doc = "Id of the node to query upon."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "Selector string."]
    #[serde(rename = "selector")]
    pub selector: String,
}
impl QuerySelectorParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, selector: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            selector: selector.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuerySelectorMethod {
    #[serde(rename = "DOM.querySelector")]
    QuerySelector,
}
impl QuerySelectorMethod {
    pub const IDENTIFIER: &'static str = "DOM.querySelector";
}
#[doc = "Executes `querySelector` on a given node.\n[querySelector](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-querySelector)"]
#[derive(Debug, Clone, PartialEq)]
pub struct QuerySelector {
    pub method: QuerySelectorMethod,
    pub params: QuerySelectorParams,
}
#[doc = "Executes `querySelectorAll` on a given node.\n[querySelectorAll](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-querySelectorAll)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuerySelectorAllParams {
    #[doc = "Id of the node to query upon."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "Selector string."]
    #[serde(rename = "selector")]
    pub selector: String,
}
impl QuerySelectorAllParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, selector: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            selector: selector.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuerySelectorAllMethod {
    #[serde(rename = "DOM.querySelectorAll")]
    QuerySelectorAll,
}
impl QuerySelectorAllMethod {
    pub const IDENTIFIER: &'static str = "DOM.querySelectorAll";
}
#[doc = "Executes `querySelectorAll` on a given node.\n[querySelectorAll](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-querySelectorAll)"]
#[derive(Debug, Clone, PartialEq)]
pub struct QuerySelectorAll {
    pub method: QuerySelectorAllMethod,
    pub params: QuerySelectorAllParams,
}
#[doc = "Returns NodeIds of current top layer elements.\nTop layer is rendered closest to the user within a viewport, therefore its elements always\nappear on top of all other content.\n[getTopLayerElements](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getTopLayerElements)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetTopLayerElementsParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetTopLayerElementsMethod {
    #[serde(rename = "DOM.getTopLayerElements")]
    GetTopLayerElements,
}
impl GetTopLayerElementsMethod {
    pub const IDENTIFIER: &'static str = "DOM.getTopLayerElements";
}
#[doc = "Returns NodeIds of current top layer elements.\nTop layer is rendered closest to the user within a viewport, therefore its elements always\nappear on top of all other content.\n[getTopLayerElements](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getTopLayerElements)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetTopLayerElements {
    pub method: GetTopLayerElementsMethod,
    pub params: GetTopLayerElementsParams,
}
#[doc = "Returns the NodeId of the matched element according to certain relations.\n[getElementByRelation](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getElementByRelation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetElementByRelationParams {
    #[doc = "Id of the node from which to query the relation."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
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
            node_id: node_id.into(),
            relation: relation.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetElementByRelationMethod {
    #[serde(rename = "DOM.getElementByRelation")]
    GetElementByRelation,
}
impl GetElementByRelationMethod {
    pub const IDENTIFIER: &'static str = "DOM.getElementByRelation";
}
#[doc = "Returns the NodeId of the matched element according to certain relations.\n[getElementByRelation](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getElementByRelation)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetElementByRelation {
    pub method: GetElementByRelationMethod,
    pub params: GetElementByRelationParams,
}
#[doc = "Re-does the last undone action.\n[redo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-redo)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RedoParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RedoMethod {
    #[serde(rename = "DOM.redo")]
    Redo,
}
impl RedoMethod {
    pub const IDENTIFIER: &'static str = "DOM.redo";
}
#[doc = "Re-does the last undone action.\n[redo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-redo)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Redo {
    pub method: RedoMethod,
    pub params: RedoParams,
}
#[doc = "Removes attribute with given name from an element with given id.\n[removeAttribute](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-removeAttribute)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveAttributeParams {
    #[doc = "Id of the element to remove attribute from."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "Name of the attribute to remove."]
    #[serde(rename = "name")]
    pub name: String,
}
impl RemoveAttributeParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, name: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            name: name.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveAttributeMethod {
    #[serde(rename = "DOM.removeAttribute")]
    RemoveAttribute,
}
impl RemoveAttributeMethod {
    pub const IDENTIFIER: &'static str = "DOM.removeAttribute";
}
#[doc = "Removes attribute with given name from an element with given id.\n[removeAttribute](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-removeAttribute)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RemoveAttribute {
    pub method: RemoveAttributeMethod,
    pub params: RemoveAttributeParams,
}
#[doc = "Removes node with given id.\n[removeNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-removeNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveNodeParams {
    #[doc = "Id of the node to remove."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
impl RemoveNodeParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveNodeMethod {
    #[serde(rename = "DOM.removeNode")]
    RemoveNode,
}
impl RemoveNodeMethod {
    pub const IDENTIFIER: &'static str = "DOM.removeNode";
}
#[doc = "Removes node with given id.\n[removeNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-removeNode)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RemoveNode {
    pub method: RemoveNodeMethod,
    pub params: RemoveNodeParams,
}
#[doc = "Requests that children of the node with given id are returned to the caller in form of\n`setChildNodes` events where not only immediate children are retrieved, but all children down to\nthe specified depth.\n[requestChildNodes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-requestChildNodes)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestChildNodesParams {
    #[doc = "Id of the node to get children for."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
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
            node_id: node_id.into(),
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
impl RequestChildNodesMethod {
    pub const IDENTIFIER: &'static str = "DOM.requestChildNodes";
}
#[doc = "Requests that children of the node with given id are returned to the caller in form of\n`setChildNodes` events where not only immediate children are retrieved, but all children down to\nthe specified depth.\n[requestChildNodes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-requestChildNodes)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestChildNodes {
    pub method: RequestChildNodesMethod,
    pub params: RequestChildNodesParams,
}
#[doc = "Requests that the node is sent to the caller given the JavaScript node object reference. All\nnodes that form the path from the node to the root are also sent to the client as a series of\n`setChildNodes` notifications.\n[requestNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-requestNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestNodeParams {
    #[doc = "JavaScript object id to convert into node."]
    #[serde(rename = "objectId")]
    pub object_id: super::super::super::js_protocol::runtime::types::RemoteObjectId,
}
impl RequestNodeParams {
    pub fn new(
        object_id: impl Into<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
    ) -> Self {
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
impl RequestNodeMethod {
    pub const IDENTIFIER: &'static str = "DOM.requestNode";
}
#[doc = "Requests that the node is sent to the caller given the JavaScript node object reference. All\nnodes that form the path from the node to the root are also sent to the client as a series of\n`setChildNodes` notifications.\n[requestNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-requestNode)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestNode {
    pub method: RequestNodeMethod,
    pub params: RequestNodeParams,
}
#[doc = "Resolves the JavaScript node object for a given NodeId or BackendNodeId.\n[resolveNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-resolveNode)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResolveNodeParams {
    #[doc = "Id of the node to resolve."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<super::types::NodeId>,
    #[doc = "Backend identifier of the node to resolve."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<super::types::BackendNodeId>,
    #[doc = "Symbolic group name that can be used to release multiple objects."]
    #[serde(rename = "objectGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_group: Option<String>,
    #[doc = "Execution context in which to resolve the node."]
    #[serde(rename = "executionContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub execution_context_id:
        Option<super::super::super::js_protocol::runtime::types::ExecutionContextId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResolveNodeMethod {
    #[serde(rename = "DOM.resolveNode")]
    ResolveNode,
}
impl ResolveNodeMethod {
    pub const IDENTIFIER: &'static str = "DOM.resolveNode";
}
#[doc = "Resolves the JavaScript node object for a given NodeId or BackendNodeId.\n[resolveNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-resolveNode)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ResolveNode {
    pub method: ResolveNodeMethod,
    pub params: ResolveNodeParams,
}
#[doc = "Sets attribute for an element with given id.\n[setAttributeValue](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setAttributeValue)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAttributeValueParams {
    #[doc = "Id of the element to set attribute for."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
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
            node_id: node_id.into(),
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
impl SetAttributeValueMethod {
    pub const IDENTIFIER: &'static str = "DOM.setAttributeValue";
}
#[doc = "Sets attribute for an element with given id.\n[setAttributeValue](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setAttributeValue)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetAttributeValue {
    pub method: SetAttributeValueMethod,
    pub params: SetAttributeValueParams,
}
#[doc = "Sets attributes on element with given id. This method is useful when user edits some existing\nattribute value and types in several attribute name/value pairs.\n[setAttributesAsText](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setAttributesAsText)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAttributesAsTextParams {
    #[doc = "Id of the element to set attributes for."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
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
            node_id: node_id.into(),
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
impl SetAttributesAsTextMethod {
    pub const IDENTIFIER: &'static str = "DOM.setAttributesAsText";
}
#[doc = "Sets attributes on element with given id. This method is useful when user edits some existing\nattribute value and types in several attribute name/value pairs.\n[setAttributesAsText](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setAttributesAsText)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetAttributesAsText {
    pub method: SetAttributesAsTextMethod,
    pub params: SetAttributesAsTextParams,
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
    pub node_id: Option<super::types::NodeId>,
    #[doc = "Identifier of the backend node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<super::types::BackendNodeId>,
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
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
impl SetFileInputFilesMethod {
    pub const IDENTIFIER: &'static str = "DOM.setFileInputFiles";
}
#[doc = "Sets files for the given file input element.\n[setFileInputFiles](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setFileInputFiles)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetFileInputFiles {
    pub method: SetFileInputFilesMethod,
    pub params: SetFileInputFilesParams,
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
impl SetNodeStackTracesEnabledMethod {
    pub const IDENTIFIER: &'static str = "DOM.setNodeStackTracesEnabled";
}
#[doc = "Sets if stack traces should be captured for Nodes. See `Node.getNodeStackTraces`. Default is disabled.\n[setNodeStackTracesEnabled](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setNodeStackTracesEnabled)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetNodeStackTracesEnabled {
    pub method: SetNodeStackTracesEnabledMethod,
    pub params: SetNodeStackTracesEnabledParams,
}
#[doc = "Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.\n[getNodeStackTraces](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getNodeStackTraces)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeStackTracesParams {
    #[doc = "Id of the node to get stack traces for."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
impl GetNodeStackTracesParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetNodeStackTracesMethod {
    #[serde(rename = "DOM.getNodeStackTraces")]
    GetNodeStackTraces,
}
impl GetNodeStackTracesMethod {
    pub const IDENTIFIER: &'static str = "DOM.getNodeStackTraces";
}
#[doc = "Gets stack traces associated with a Node. As of now, only provides stack trace for Node creation.\n[getNodeStackTraces](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getNodeStackTraces)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetNodeStackTraces {
    pub method: GetNodeStackTracesMethod,
    pub params: GetNodeStackTracesParams,
}
#[doc = "Returns file information for the given\nFile wrapper.\n[getFileInfo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getFileInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFileInfoParams {
    #[doc = "JavaScript object id of the node wrapper."]
    #[serde(rename = "objectId")]
    pub object_id: super::super::super::js_protocol::runtime::types::RemoteObjectId,
}
impl GetFileInfoParams {
    pub fn new(
        object_id: impl Into<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
    ) -> Self {
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
impl GetFileInfoMethod {
    pub const IDENTIFIER: &'static str = "DOM.getFileInfo";
}
#[doc = "Returns file information for the given\nFile wrapper.\n[getFileInfo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getFileInfo)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetFileInfo {
    pub method: GetFileInfoMethod,
    pub params: GetFileInfoParams,
}
#[doc = "Returns list of detached nodes\n[getDetachedDomNodes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getDetachedDomNodes)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetDetachedDomNodesParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetDetachedDomNodesMethod {
    #[serde(rename = "DOM.getDetachedDomNodes")]
    GetDetachedDomNodes,
}
impl GetDetachedDomNodesMethod {
    pub const IDENTIFIER: &'static str = "DOM.getDetachedDomNodes";
}
#[doc = "Returns list of detached nodes\n[getDetachedDomNodes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getDetachedDomNodes)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetDetachedDomNodes {
    pub method: GetDetachedDomNodesMethod,
    pub params: GetDetachedDomNodesParams,
}
#[doc = "Enables console to refer to the node with given id via $x (see Command Line API for more details\n$x functions).\n[setInspectedNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setInspectedNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInspectedNodeParams {
    #[doc = "DOM node id to be accessible by means of $x command line API."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
impl SetInspectedNodeParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetInspectedNodeMethod {
    #[serde(rename = "DOM.setInspectedNode")]
    SetInspectedNode,
}
impl SetInspectedNodeMethod {
    pub const IDENTIFIER: &'static str = "DOM.setInspectedNode";
}
#[doc = "Enables console to refer to the node with given id via $x (see Command Line API for more details\n$x functions).\n[setInspectedNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setInspectedNode)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetInspectedNode {
    pub method: SetInspectedNodeMethod,
    pub params: SetInspectedNodeParams,
}
#[doc = "Sets node name for a node with given id.\n[setNodeName](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setNodeName)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetNodeNameParams {
    #[doc = "Id of the node to set name for."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "New node's name."]
    #[serde(rename = "name")]
    pub name: String,
}
impl SetNodeNameParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, name: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            name: name.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetNodeNameMethod {
    #[serde(rename = "DOM.setNodeName")]
    SetNodeName,
}
impl SetNodeNameMethod {
    pub const IDENTIFIER: &'static str = "DOM.setNodeName";
}
#[doc = "Sets node name for a node with given id.\n[setNodeName](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setNodeName)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetNodeName {
    pub method: SetNodeNameMethod,
    pub params: SetNodeNameParams,
}
#[doc = "Sets node value for a node with given id.\n[setNodeValue](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setNodeValue)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetNodeValueParams {
    #[doc = "Id of the node to set value for."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "New node's value."]
    #[serde(rename = "value")]
    pub value: String,
}
impl SetNodeValueParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, value: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            value: value.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetNodeValueMethod {
    #[serde(rename = "DOM.setNodeValue")]
    SetNodeValue,
}
impl SetNodeValueMethod {
    pub const IDENTIFIER: &'static str = "DOM.setNodeValue";
}
#[doc = "Sets node value for a node with given id.\n[setNodeValue](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setNodeValue)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetNodeValue {
    pub method: SetNodeValueMethod,
    pub params: SetNodeValueParams,
}
#[doc = "Sets node HTML markup, returns new node id.\n[setOuterHTML](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setOuterHTML)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetOuterHtmlParams {
    #[doc = "Id of the node to set markup for."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "Outer HTML markup to set."]
    #[serde(rename = "outerHTML")]
    pub outer_html: String,
}
impl SetOuterHtmlParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, outer_html: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            outer_html: outer_html.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetOuterHtmlMethod {
    #[serde(rename = "DOM.setOuterHTML")]
    SetOuterHtml,
}
impl SetOuterHtmlMethod {
    pub const IDENTIFIER: &'static str = "DOM.setOuterHTML";
}
#[doc = "Sets node HTML markup, returns new node id.\n[setOuterHTML](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-setOuterHTML)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetOuterHtml {
    pub method: SetOuterHtmlMethod,
    pub params: SetOuterHtmlParams,
}
#[doc = "Undoes the last performed action.\n[undo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-undo)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UndoParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UndoMethod {
    #[serde(rename = "DOM.undo")]
    Undo,
}
impl UndoMethod {
    pub const IDENTIFIER: &'static str = "DOM.undo";
}
#[doc = "Undoes the last performed action.\n[undo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-undo)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Undo {
    pub method: UndoMethod,
    pub params: UndoParams,
}
#[doc = "Returns iframe node that owns iframe with the given domain.\n[getFrameOwner](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getFrameOwner)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFrameOwnerParams {
    #[serde(rename = "frameId")]
    pub frame_id: super::super::page::types::FrameId,
}
impl GetFrameOwnerParams {
    pub fn new(frame_id: impl Into<super::super::page::types::FrameId>) -> Self {
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
impl GetFrameOwnerMethod {
    pub const IDENTIFIER: &'static str = "DOM.getFrameOwner";
}
#[doc = "Returns iframe node that owns iframe with the given domain.\n[getFrameOwner](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getFrameOwner)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetFrameOwner {
    pub method: GetFrameOwnerMethod,
    pub params: GetFrameOwnerParams,
}
#[doc = "Returns the query container of the given node based on container query\nconditions: containerName, physical and logical axes, and whether it queries\nscroll-state or anchored elements. If no axes are provided and\nqueriesScrollState is false, the style container is returned, which is the\ndirect parent or the closest element with a matching container-name.\n[getContainerForNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getContainerForNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContainerForNodeParams {
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
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
            node_id: node_id.into(),
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
impl GetContainerForNodeMethod {
    pub const IDENTIFIER: &'static str = "DOM.getContainerForNode";
}
#[doc = "Returns the query container of the given node based on container query\nconditions: containerName, physical and logical axes, and whether it queries\nscroll-state or anchored elements. If no axes are provided and\nqueriesScrollState is false, the style container is returned, which is the\ndirect parent or the closest element with a matching container-name.\n[getContainerForNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getContainerForNode)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetContainerForNode {
    pub method: GetContainerForNodeMethod,
    pub params: GetContainerForNodeParams,
}
#[doc = "Returns the descendants of a container query container that have\ncontainer queries against this container.\n[getQueryingDescendantsForContainer](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getQueryingDescendantsForContainer)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetQueryingDescendantsForContainerParams {
    #[doc = "Id of the container node to find querying descendants from."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
impl GetQueryingDescendantsForContainerParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetQueryingDescendantsForContainerMethod {
    #[serde(rename = "DOM.getQueryingDescendantsForContainer")]
    GetQueryingDescendantsForContainer,
}
impl GetQueryingDescendantsForContainerMethod {
    pub const IDENTIFIER: &'static str = "DOM.getQueryingDescendantsForContainer";
}
#[doc = "Returns the descendants of a container query container that have\ncontainer queries against this container.\n[getQueryingDescendantsForContainer](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getQueryingDescendantsForContainer)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetQueryingDescendantsForContainer {
    pub method: GetQueryingDescendantsForContainerMethod,
    pub params: GetQueryingDescendantsForContainerParams,
}
#[doc = "Returns the target anchor element of the given anchor query according to\nhttps://www.w3.org/TR/css-anchor-position-1/#target.\n[getAnchorElement](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getAnchorElement)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAnchorElementParams {
    #[doc = "Id of the positioned element from which to find the anchor."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "An optional anchor specifier, as defined in\nhttps://www.w3.org/TR/css-anchor-position-1/#anchor-specifier.\nIf not provided, it will return the implicit anchor element for\nthe given positioned element."]
    #[serde(rename = "anchorSpecifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub anchor_specifier: Option<String>,
}
impl GetAnchorElementParams {
    pub fn new(node_id: impl Into<super::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
            anchor_specifier: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetAnchorElementMethod {
    #[serde(rename = "DOM.getAnchorElement")]
    GetAnchorElement,
}
impl GetAnchorElementMethod {
    pub const IDENTIFIER: &'static str = "DOM.getAnchorElement";
}
#[doc = "Returns the target anchor element of the given anchor query according to\nhttps://www.w3.org/TR/css-anchor-position-1/#target.\n[getAnchorElement](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-getAnchorElement)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetAnchorElement {
    pub method: GetAnchorElementMethod,
    pub params: GetAnchorElementParams,
}
#[doc = "When enabling, this API force-opens the popover identified by nodeId\nand keeps it open until disabled.\n[forceShowPopover](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-forceShowPopover)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForceShowPopoverParams {
    #[doc = "Id of the popover HTMLElement"]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "If true, opens the popover and keeps it open. If false, closes the\npopover if it was previously force-opened."]
    #[serde(rename = "enable")]
    pub enable: bool,
}
impl ForceShowPopoverParams {
    pub fn new(node_id: impl Into<super::types::NodeId>, enable: impl Into<bool>) -> Self {
        Self {
            node_id: node_id.into(),
            enable: enable.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ForceShowPopoverMethod {
    #[serde(rename = "DOM.forceShowPopover")]
    ForceShowPopover,
}
impl ForceShowPopoverMethod {
    pub const IDENTIFIER: &'static str = "DOM.forceShowPopover";
}
#[doc = "When enabling, this API force-opens the popover identified by nodeId\nand keeps it open until disabled.\n[forceShowPopover](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#method-forceShowPopover)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ForceShowPopover {
    pub method: ForceShowPopoverMethod,
    pub params: ForceShowPopoverParams,
}
group_enum ! (DomCommands { CollectClassNamesFromSubtree (CollectClassNamesFromSubtree) , CopyTo (CopyTo) , DescribeNode (DescribeNode) , ScrollIntoViewIfNeeded (ScrollIntoViewIfNeeded) , Disable (Disable) , DiscardSearchResults (DiscardSearchResults) , Enable (Enable) , Focus (Focus) , GetAttributes (GetAttributes) , GetBoxModel (GetBoxModel) , GetContentQuads (GetContentQuads) , GetDocument (GetDocument) , GetNodesForSubtreeByStyle (GetNodesForSubtreeByStyle) , GetNodeForLocation (GetNodeForLocation) , GetOuterHtml (GetOuterHtml) , GetRelayoutBoundary (GetRelayoutBoundary) , GetSearchResults (GetSearchResults) , HideHighlight (HideHighlight) , HighlightNode (HighlightNode) , HighlightRect (HighlightRect) , MarkUndoableState (MarkUndoableState) , MoveTo (MoveTo) , PerformSearch (PerformSearch) , PushNodeByPathToFrontend (PushNodeByPathToFrontend) , PushNodesByBackendIdsToFrontend (PushNodesByBackendIdsToFrontend) , QuerySelector (QuerySelector) , QuerySelectorAll (QuerySelectorAll) , GetTopLayerElements (GetTopLayerElements) , GetElementByRelation (GetElementByRelation) , Redo (Redo) , RemoveAttribute (RemoveAttribute) , RemoveNode (RemoveNode) , RequestChildNodes (RequestChildNodes) , RequestNode (RequestNode) , ResolveNode (ResolveNode) , SetAttributeValue (SetAttributeValue) , SetAttributesAsText (SetAttributesAsText) , SetFileInputFiles (SetFileInputFiles) , SetNodeStackTracesEnabled (SetNodeStackTracesEnabled) , GetNodeStackTraces (GetNodeStackTraces) , GetFileInfo (GetFileInfo) , GetDetachedDomNodes (GetDetachedDomNodes) , SetInspectedNode (SetInspectedNode) , SetNodeName (SetNodeName) , SetNodeValue (SetNodeValue) , SetOuterHtml (SetOuterHtml) , Undo (Undo) , GetFrameOwner (GetFrameOwner) , GetContainerForNode (GetContainerForNode) , GetQueryingDescendantsForContainer (GetQueryingDescendantsForContainer) , GetAnchorElement (GetAnchorElement) , ForceShowPopover (ForceShowPopover) });
