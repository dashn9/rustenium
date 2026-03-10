use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectClassNamesFromSubtreeResult {
    #[doc = "Class name list."]
    #[serde(rename = "classNames")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub class_names: Vec<String>,
}
impl TryFrom<serde_json::Value> for CollectClassNamesFromSubtreeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyToResult {
    #[doc = "Id of the node clone."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for CopyToResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribeNodeResult {
    #[doc = "Node description."]
    #[serde(rename = "node")]
    pub node: Box<super::types::Node>,
}
impl TryFrom<serde_json::Value> for DescribeNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScrollIntoViewIfNeededResult {}
impl TryFrom<serde_json::Value> for ScrollIntoViewIfNeededResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
impl TryFrom<serde_json::Value> for DisableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DiscardSearchResultsResult {}
impl TryFrom<serde_json::Value> for DiscardSearchResultsResult {
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
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FocusResult {}
impl TryFrom<serde_json::Value> for FocusResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAttributesResult {
    #[doc = "An interleaved array of node attribute names and values."]
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<String>,
}
impl TryFrom<serde_json::Value> for GetAttributesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBoxModelResult {
    #[doc = "Box model for the node."]
    #[serde(rename = "model")]
    pub model: super::types::BoxModel,
}
impl TryFrom<serde_json::Value> for GetBoxModelResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContentQuadsResult {
    #[doc = "Quads that describe node layout relative to viewport."]
    #[serde(rename = "quads")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub quads: Vec<super::types::Quad>,
}
impl TryFrom<serde_json::Value> for GetContentQuadsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDocumentResult {
    #[doc = "Resulting node."]
    #[serde(rename = "root")]
    pub root: super::types::Node,
}
impl TryFrom<serde_json::Value> for GetDocumentResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFlattenedDocumentResult {
    #[doc = "Resulting node."]
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::Node>,
}
impl TryFrom<serde_json::Value> for GetFlattenedDocumentResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodesForSubtreeByStyleResult {
    #[doc = "Resulting nodes."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for GetNodesForSubtreeByStyleResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeForLocationResult {
    #[doc = "Resulting node."]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Box<super::types::BackendNodeId>,
    #[doc = "Frame this node belongs to."]
    #[serde(rename = "frameId")]
    pub frame_id: crate::browser_protocol::page::types::FrameId,
    #[doc = "Id of the node at given coordinates, only when enabled and requested document."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<Box<super::types::NodeId>>,
}
impl TryFrom<serde_json::Value> for GetNodeForLocationResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOuterHtmlResult {
    #[doc = "Outer HTML markup."]
    #[serde(rename = "outerHTML")]
    pub outer_html: String,
}
impl TryFrom<serde_json::Value> for GetOuterHtmlResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRelayoutBoundaryResult {
    #[doc = "Relayout boundary node id for the given node."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for GetRelayoutBoundaryResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchResultsResult {
    #[doc = "Ids of the search result nodes."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for GetSearchResultsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HideHighlightResult {}
impl TryFrom<serde_json::Value> for HideHighlightResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HighlightNodeResult {}
impl TryFrom<serde_json::Value> for HighlightNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HighlightRectResult {}
impl TryFrom<serde_json::Value> for HighlightRectResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MarkUndoableStateResult {}
impl TryFrom<serde_json::Value> for MarkUndoableStateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveToResult {
    #[doc = "New id of the moved node."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for MoveToResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformSearchResult {
    #[doc = "Unique search session identifier."]
    #[serde(rename = "searchId")]
    pub search_id: String,
    #[doc = "Number of search results."]
    #[serde(rename = "resultCount")]
    pub result_count: i64,
}
impl TryFrom<serde_json::Value> for PerformSearchResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushNodeByPathToFrontendResult {
    #[doc = "Id of the node for given path."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for PushNodeByPathToFrontendResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushNodesByBackendIdsToFrontendResult {
    #[doc = "The array of ids of pushed nodes that correspond to the backend ids specified in\nbackendNodeIds."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for PushNodesByBackendIdsToFrontendResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuerySelectorResult {
    #[doc = "Query selector result."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for QuerySelectorResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuerySelectorAllResult {
    #[doc = "Query selector result."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for QuerySelectorAllResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTopLayerElementsResult {
    #[doc = "NodeIds of top layer elements"]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for GetTopLayerElementsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetElementByRelationResult {
    #[doc = "NodeId of the element matching the queried relation."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for GetElementByRelationResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RedoResult {}
impl TryFrom<serde_json::Value> for RedoResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveAttributeResult {}
impl TryFrom<serde_json::Value> for RemoveAttributeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveNodeResult {}
impl TryFrom<serde_json::Value> for RemoveNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestChildNodesResult {}
impl TryFrom<serde_json::Value> for RequestChildNodesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestNodeResult {
    #[doc = "Node id for given object."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for RequestNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveNodeResult {
    #[doc = "JavaScript object wrapper for given node."]
    #[serde(rename = "object")]
    pub object: crate::js_protocol::runtime::types::RemoteObject,
}
impl TryFrom<serde_json::Value> for ResolveNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAttributeValueResult {}
impl TryFrom<serde_json::Value> for SetAttributeValueResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAttributesAsTextResult {}
impl TryFrom<serde_json::Value> for SetAttributesAsTextResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetFileInputFilesResult {}
impl TryFrom<serde_json::Value> for SetFileInputFilesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetNodeStackTracesEnabledResult {}
impl TryFrom<serde_json::Value> for SetNodeStackTracesEnabledResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetNodeStackTracesResult {
    #[doc = "Creation stack trace, if available."]
    #[serde(rename = "creation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub creation: Option<crate::js_protocol::runtime::types::StackTrace>,
}
impl TryFrom<serde_json::Value> for GetNodeStackTracesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFileInfoResult {
    #[serde(rename = "path")]
    pub path: String,
}
impl TryFrom<serde_json::Value> for GetFileInfoResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDetachedDomNodesResult {
    #[doc = "The list of detached nodes"]
    #[serde(rename = "detachedNodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub detached_nodes: Vec<super::types::DetachedElementInfo>,
}
impl TryFrom<serde_json::Value> for GetDetachedDomNodesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetInspectedNodeResult {}
impl TryFrom<serde_json::Value> for SetInspectedNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetNodeNameResult {
    #[doc = "New node's id."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for SetNodeNameResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetNodeValueResult {}
impl TryFrom<serde_json::Value> for SetNodeValueResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetOuterHtmlResult {}
impl TryFrom<serde_json::Value> for SetOuterHtmlResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UndoResult {}
impl TryFrom<serde_json::Value> for UndoResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFrameOwnerResult {
    #[doc = "Resulting node."]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: Box<super::types::BackendNodeId>,
    #[doc = "Id of the node at given coordinates, only when enabled and requested document."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<Box<super::types::NodeId>>,
}
impl TryFrom<serde_json::Value> for GetFrameOwnerResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetContainerForNodeResult {
    #[doc = "The container node for the given node, or null if not found."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<Box<super::types::NodeId>>,
}
impl TryFrom<serde_json::Value> for GetContainerForNodeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetQueryingDescendantsForContainerResult {
    #[doc = "Descendant nodes with container queries against the given container."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for GetQueryingDescendantsForContainerResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAnchorElementResult {
    #[doc = "The anchor element of the given anchor query."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for GetAnchorElementResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForceShowPopoverResult {
    #[doc = "List of popovers that were closed in order to respect popover stacking order."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
impl TryFrom<serde_json::Value> for ForceShowPopoverResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
