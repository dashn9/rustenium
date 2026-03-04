use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectClassNamesFromSubtreeReturns {
    #[doc = "Class name list."]
    #[serde(rename = "classNames")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub class_names: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyToReturns {
    #[doc = "Id of the node clone."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribeNodeReturns {
    #[doc = "Node description."]
    #[serde(rename = "node")]
    pub node: super::types::Node,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAttributesReturns {
    #[doc = "An interleaved array of node attribute names and values."]
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attributes: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBoxModelReturns {
    #[doc = "Box model for the node."]
    #[serde(rename = "model")]
    pub model: super::types::BoxModel,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetContentQuadsReturns {
    #[doc = "Quads that describe node layout relative to viewport."]
    #[serde(rename = "quads")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub quads: Vec<super::types::Quad>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDocumentReturns {
    #[doc = "Resulting node."]
    #[serde(rename = "root")]
    pub root: super::types::Node,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFlattenedDocumentReturns {
    #[doc = "Resulting node."]
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::Node>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodesForSubtreeByStyleReturns {
    #[doc = "Resulting nodes."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNodeForLocationReturns {
    #[doc = "Resulting node."]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: super::types::BackendNodeId,
    #[doc = "Frame this node belongs to."]
    #[serde(rename = "frameId")]
    pub frame_id: super::super::page::types::FrameId,
    #[doc = "Id of the node at given coordinates, only when enabled and requested document."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<super::types::NodeId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOuterHtmlReturns {
    #[doc = "Outer HTML markup."]
    #[serde(rename = "outerHTML")]
    pub outer_html: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRelayoutBoundaryReturns {
    #[doc = "Relayout boundary node id for the given node."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSearchResultsReturns {
    #[doc = "Ids of the search result nodes."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoveToReturns {
    #[doc = "New id of the moved node."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformSearchReturns {
    #[doc = "Unique search session identifier."]
    #[serde(rename = "searchId")]
    pub search_id: String,
    #[doc = "Number of search results."]
    #[serde(rename = "resultCount")]
    pub result_count: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushNodeByPathToFrontendReturns {
    #[doc = "Id of the node for given path."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushNodesByBackendIdsToFrontendReturns {
    #[doc = "The array of ids of pushed nodes that correspond to the backend ids specified in\nbackendNodeIds."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuerySelectorReturns {
    #[doc = "Query selector result."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuerySelectorAllReturns {
    #[doc = "Query selector result."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTopLayerElementsReturns {
    #[doc = "NodeIds of top layer elements"]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetElementByRelationReturns {
    #[doc = "NodeId of the element matching the queried relation."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestNodeReturns {
    #[doc = "Node id for given object."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveNodeReturns {
    #[doc = "JavaScript object wrapper for given node."]
    #[serde(rename = "object")]
    pub object: super::super::super::js_protocol::runtime::types::RemoteObject,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetNodeStackTracesReturns {
    #[doc = "Creation stack trace, if available."]
    #[serde(rename = "creation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub creation: Option<super::super::super::js_protocol::runtime::types::StackTrace>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFileInfoReturns {
    #[serde(rename = "path")]
    pub path: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDetachedDomNodesReturns {
    #[doc = "The list of detached nodes"]
    #[serde(rename = "detachedNodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub detached_nodes: Vec<super::types::DetachedElementInfo>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetNodeNameReturns {
    #[doc = "New node's id."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFrameOwnerReturns {
    #[doc = "Resulting node."]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: super::types::BackendNodeId,
    #[doc = "Id of the node at given coordinates, only when enabled and requested document."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<super::types::NodeId>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetContainerForNodeReturns {
    #[doc = "The container node for the given node, or null if not found."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<super::types::NodeId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetQueryingDescendantsForContainerReturns {
    #[doc = "Descendant nodes with container queries against the given container."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAnchorElementReturns {
    #[doc = "The anchor element of the given anchor query."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForceShowPopoverReturns {
    #[doc = "List of popovers that were closed in order to respect popover stacking order."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
