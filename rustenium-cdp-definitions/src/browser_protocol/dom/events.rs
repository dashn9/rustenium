use serde::{Deserialize, Serialize};
#[doc = "Fired when `Element`'s attribute is modified.\n[attributeModified](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-attributeModified)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeModified {
    #[doc = "Id of the node that has changed."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "Attribute name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Attribute value."]
    #[serde(rename = "value")]
    pub value: String,
}
impl AttributeModified {
    pub const IDENTIFIER: &'static str = "DOM.attributeModified";
}
#[doc = "Fired when `Element`'s adoptedStyleSheets are modified.\n[adoptedStyleSheetsModified](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-adoptedStyleSheetsModified)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdoptedStyleSheetsModified {
    #[doc = "Id of the node that has changed."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "New adoptedStyleSheets array."]
    #[serde(rename = "adoptedStyleSheets")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub adopted_style_sheets: Vec<super::types::StyleSheetId>,
}
impl AdoptedStyleSheetsModified {
    pub const IDENTIFIER: &'static str = "DOM.adoptedStyleSheetsModified";
}
#[doc = "Fired when `Element`'s attribute is removed.\n[attributeRemoved](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-attributeRemoved)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeRemoved {
    #[doc = "Id of the node that has changed."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "A ttribute name."]
    #[serde(rename = "name")]
    pub name: String,
}
impl AttributeRemoved {
    pub const IDENTIFIER: &'static str = "DOM.attributeRemoved";
}
#[doc = "Mirrors `DOMCharacterDataModified` event.\n[characterDataModified](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-characterDataModified)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterDataModified {
    #[doc = "Id of the node that has changed."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "New text value."]
    #[serde(rename = "characterData")]
    pub character_data: String,
}
impl CharacterDataModified {
    pub const IDENTIFIER: &'static str = "DOM.characterDataModified";
}
#[doc = "Fired when `Container`'s child node count has changed.\n[childNodeCountUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-childNodeCountUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildNodeCountUpdated {
    #[doc = "Id of the node that has changed."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "New node count."]
    #[serde(rename = "childNodeCount")]
    pub child_node_count: i64,
}
impl ChildNodeCountUpdated {
    pub const IDENTIFIER: &'static str = "DOM.childNodeCountUpdated";
}
#[doc = "Mirrors `DOMNodeInserted` event.\n[childNodeInserted](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-childNodeInserted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildNodeInserted {
    #[doc = "Id of the node that has changed."]
    #[serde(rename = "parentNodeId")]
    pub parent_node_id: super::types::NodeId,
    #[doc = "Id of the previous sibling."]
    #[serde(rename = "previousNodeId")]
    pub previous_node_id: super::types::NodeId,
    #[doc = "Inserted node data."]
    #[serde(rename = "node")]
    pub node: super::types::Node,
}
impl ChildNodeInserted {
    pub const IDENTIFIER: &'static str = "DOM.childNodeInserted";
}
#[doc = "Mirrors `DOMNodeRemoved` event.\n[childNodeRemoved](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-childNodeRemoved)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildNodeRemoved {
    #[doc = "Parent id."]
    #[serde(rename = "parentNodeId")]
    pub parent_node_id: super::types::NodeId,
    #[doc = "Id of the node that has been removed."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
}
impl ChildNodeRemoved {
    pub const IDENTIFIER: &'static str = "DOM.childNodeRemoved";
}
#[doc = "Called when distribution is changed.\n[distributedNodesUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-distributedNodesUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributedNodesUpdated {
    #[doc = "Insertion point where distributed nodes were updated."]
    #[serde(rename = "insertionPointId")]
    pub insertion_point_id: super::types::NodeId,
    #[doc = "Distributed nodes for given insertion point."]
    #[serde(rename = "distributedNodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub distributed_nodes: Vec<super::types::BackendNode>,
}
impl DistributedNodesUpdated {
    pub const IDENTIFIER: &'static str = "DOM.distributedNodesUpdated";
}
#[doc = "Fired when `Document` has been totally updated. Node ids are no longer valid.\n[documentUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-documentUpdated)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DocumentUpdated {}
impl DocumentUpdated {
    pub const IDENTIFIER: &'static str = "DOM.documentUpdated";
}
#[doc = "Fired when `Element`'s inline style is modified via a CSS property modification.\n[inlineStyleInvalidated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-inlineStyleInvalidated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineStyleInvalidated {
    #[doc = "Ids of the nodes for which the inline styles have been invalidated."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
impl InlineStyleInvalidated {
    pub const IDENTIFIER: &'static str = "DOM.inlineStyleInvalidated";
}
#[doc = "Called when a pseudo element is added to an element.\n[pseudoElementAdded](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-pseudoElementAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PseudoElementAdded {
    #[doc = "Pseudo element's parent element id."]
    #[serde(rename = "parentId")]
    pub parent_id: super::types::NodeId,
    #[doc = "The added pseudo element."]
    #[serde(rename = "pseudoElement")]
    pub pseudo_element: super::types::Node,
}
impl PseudoElementAdded {
    pub const IDENTIFIER: &'static str = "DOM.pseudoElementAdded";
}
#[doc = "Called when top layer elements are changed.\n[topLayerElementsUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-topLayerElementsUpdated)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TopLayerElementsUpdated {}
impl TopLayerElementsUpdated {
    pub const IDENTIFIER: &'static str = "DOM.topLayerElementsUpdated";
}
#[doc = "Fired when a node's scrollability state changes.\n[scrollableFlagUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-scrollableFlagUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrollableFlagUpdated {
    #[doc = "The id of the node."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "If the node is scrollable."]
    #[serde(rename = "isScrollable")]
    pub is_scrollable: bool,
}
impl ScrollableFlagUpdated {
    pub const IDENTIFIER: &'static str = "DOM.scrollableFlagUpdated";
}
#[doc = "Fired when a node's ad related state changes.\n[adRelatedStateUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-adRelatedStateUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdRelatedStateUpdated {
    #[doc = "The id of the node."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "If the node is ad related."]
    #[serde(rename = "isAdRelated")]
    pub is_ad_related: bool,
}
impl AdRelatedStateUpdated {
    pub const IDENTIFIER: &'static str = "DOM.adRelatedStateUpdated";
}
#[doc = "Fired when a node's starting styles changes.\n[affectedByStartingStylesFlagUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-affectedByStartingStylesFlagUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectedByStartingStylesFlagUpdated {
    #[doc = "The id of the node."]
    #[serde(rename = "nodeId")]
    pub node_id: super::types::NodeId,
    #[doc = "If the node has starting styles."]
    #[serde(rename = "affectedByStartingStyles")]
    pub affected_by_starting_styles: bool,
}
impl AffectedByStartingStylesFlagUpdated {
    pub const IDENTIFIER: &'static str = "DOM.affectedByStartingStylesFlagUpdated";
}
#[doc = "Called when a pseudo element is removed from an element.\n[pseudoElementRemoved](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-pseudoElementRemoved)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PseudoElementRemoved {
    #[doc = "Pseudo element's parent element id."]
    #[serde(rename = "parentId")]
    pub parent_id: super::types::NodeId,
    #[doc = "The removed pseudo element id."]
    #[serde(rename = "pseudoElementId")]
    pub pseudo_element_id: super::types::NodeId,
}
impl PseudoElementRemoved {
    pub const IDENTIFIER: &'static str = "DOM.pseudoElementRemoved";
}
#[doc = "Fired when backend wants to provide client with the missing DOM structure. This happens upon\nmost of the calls requesting node ids.\n[setChildNodes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-setChildNodes)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetChildNodes {
    #[doc = "Parent node id to populate with children."]
    #[serde(rename = "parentId")]
    pub parent_id: super::types::NodeId,
    #[doc = "Child nodes array."]
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::Node>,
}
impl SetChildNodes {
    pub const IDENTIFIER: &'static str = "DOM.setChildNodes";
}
#[doc = "Called when shadow root is popped from the element.\n[shadowRootPopped](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-shadowRootPopped)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowRootPopped {
    #[doc = "Host element id."]
    #[serde(rename = "hostId")]
    pub host_id: super::types::NodeId,
    #[doc = "Shadow root id."]
    #[serde(rename = "rootId")]
    pub root_id: super::types::NodeId,
}
impl ShadowRootPopped {
    pub const IDENTIFIER: &'static str = "DOM.shadowRootPopped";
}
#[doc = "Called when shadow root is pushed into the element.\n[shadowRootPushed](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-shadowRootPushed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowRootPushed {
    #[doc = "Host element id."]
    #[serde(rename = "hostId")]
    pub host_id: super::types::NodeId,
    #[doc = "Shadow root."]
    #[serde(rename = "root")]
    pub root: super::types::Node,
}
impl ShadowRootPushed {
    pub const IDENTIFIER: &'static str = "DOM.shadowRootPushed";
}
group_enum ! (Event { AttributeModified (AttributeModified) , AdoptedStyleSheetsModified (AdoptedStyleSheetsModified) , AttributeRemoved (AttributeRemoved) , CharacterDataModified (CharacterDataModified) , ChildNodeCountUpdated (ChildNodeCountUpdated) , ChildNodeInserted (ChildNodeInserted) , ChildNodeRemoved (ChildNodeRemoved) , DistributedNodesUpdated (DistributedNodesUpdated) , DocumentUpdated (DocumentUpdated) , InlineStyleInvalidated (InlineStyleInvalidated) , PseudoElementAdded (PseudoElementAdded) , TopLayerElementsUpdated (TopLayerElementsUpdated) , ScrollableFlagUpdated (ScrollableFlagUpdated) , AdRelatedStateUpdated (AdRelatedStateUpdated) , AffectedByStartingStylesFlagUpdated (AffectedByStartingStylesFlagUpdated) , PseudoElementRemoved (PseudoElementRemoved) , SetChildNodes (SetChildNodes) , ShadowRootPopped (ShadowRootPopped) , ShadowRootPushed (ShadowRootPushed) });
