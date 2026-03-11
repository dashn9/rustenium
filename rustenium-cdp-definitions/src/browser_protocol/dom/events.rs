use serde::{Deserialize, Serialize};
#[doc = "Fired when `Element`'s attribute is modified.\n[attributeModified](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-attributeModified)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeModifiedParams {
    #[doc = "Id of the node that has changed."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "Attribute name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Attribute value."]
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttributeModifiedMethod {
    #[serde(rename = "DOM.attributeModified")]
    AttributeModified,
}
#[doc = "Fired when `Element`'s attribute is modified.\n[attributeModified](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-attributeModified)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeModified {
    pub method: AttributeModifiedMethod,
    pub params: AttributeModifiedParams,
}
impl AttributeModified {
    pub const IDENTIFIER: &'static str = "DOM.attributeModified";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Fired when `Element`'s adoptedStyleSheets are modified.\n[adoptedStyleSheetsModified](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-adoptedStyleSheetsModified)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdoptedStyleSheetsModifiedParams {
    #[doc = "Id of the node that has changed."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "New adoptedStyleSheets array."]
    #[serde(rename = "adoptedStyleSheets")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub adopted_style_sheets: Vec<super::types::StyleSheetId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdoptedStyleSheetsModifiedMethod {
    #[serde(rename = "DOM.adoptedStyleSheetsModified")]
    AdoptedStyleSheetsModified,
}
#[doc = "Fired when `Element`'s adoptedStyleSheets are modified.\n[adoptedStyleSheetsModified](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-adoptedStyleSheetsModified)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdoptedStyleSheetsModified {
    pub method: AdoptedStyleSheetsModifiedMethod,
    pub params: AdoptedStyleSheetsModifiedParams,
}
impl AdoptedStyleSheetsModified {
    pub const IDENTIFIER: &'static str = "DOM.adoptedStyleSheetsModified";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Fired when `Element`'s attribute is removed.\n[attributeRemoved](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-attributeRemoved)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeRemovedParams {
    #[doc = "Id of the node that has changed."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "A ttribute name."]
    #[serde(rename = "name")]
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttributeRemovedMethod {
    #[serde(rename = "DOM.attributeRemoved")]
    AttributeRemoved,
}
#[doc = "Fired when `Element`'s attribute is removed.\n[attributeRemoved](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-attributeRemoved)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeRemoved {
    pub method: AttributeRemovedMethod,
    pub params: AttributeRemovedParams,
}
impl AttributeRemoved {
    pub const IDENTIFIER: &'static str = "DOM.attributeRemoved";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Mirrors `DOMCharacterDataModified` event.\n[characterDataModified](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-characterDataModified)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterDataModifiedParams {
    #[doc = "Id of the node that has changed."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "New text value."]
    #[serde(rename = "characterData")]
    pub character_data: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CharacterDataModifiedMethod {
    #[serde(rename = "DOM.characterDataModified")]
    CharacterDataModified,
}
#[doc = "Mirrors `DOMCharacterDataModified` event.\n[characterDataModified](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-characterDataModified)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterDataModified {
    pub method: CharacterDataModifiedMethod,
    pub params: CharacterDataModifiedParams,
}
impl CharacterDataModified {
    pub const IDENTIFIER: &'static str = "DOM.characterDataModified";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Fired when `Container`'s child node count has changed.\n[childNodeCountUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-childNodeCountUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildNodeCountUpdatedParams {
    #[doc = "Id of the node that has changed."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "New node count."]
    #[serde(rename = "childNodeCount")]
    pub child_node_count: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChildNodeCountUpdatedMethod {
    #[serde(rename = "DOM.childNodeCountUpdated")]
    ChildNodeCountUpdated,
}
#[doc = "Fired when `Container`'s child node count has changed.\n[childNodeCountUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-childNodeCountUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildNodeCountUpdated {
    pub method: ChildNodeCountUpdatedMethod,
    pub params: ChildNodeCountUpdatedParams,
}
impl ChildNodeCountUpdated {
    pub const IDENTIFIER: &'static str = "DOM.childNodeCountUpdated";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Mirrors `DOMNodeInserted` event.\n[childNodeInserted](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-childNodeInserted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildNodeInsertedParams {
    #[doc = "Id of the node that has changed."]
    #[serde(rename = "parentNodeId")]
    pub parent_node_id: super::types::NodeId,
    #[doc = "Id of the previous sibling."]
    #[serde(rename = "previousNodeId")]
    pub previous_node_id: super::types::NodeId,
    #[doc = "Inserted node data."]
    #[serde(rename = "node")]
    pub node: Box<super::types::Node>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChildNodeInsertedMethod {
    #[serde(rename = "DOM.childNodeInserted")]
    ChildNodeInserted,
}
#[doc = "Mirrors `DOMNodeInserted` event.\n[childNodeInserted](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-childNodeInserted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildNodeInserted {
    pub method: ChildNodeInsertedMethod,
    pub params: ChildNodeInsertedParams,
}
impl ChildNodeInserted {
    pub const IDENTIFIER: &'static str = "DOM.childNodeInserted";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Mirrors `DOMNodeRemoved` event.\n[childNodeRemoved](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-childNodeRemoved)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildNodeRemovedParams {
    #[doc = "Parent id."]
    #[serde(rename = "parentNodeId")]
    pub parent_node_id: super::types::NodeId,
    #[doc = "Id of the node that has been removed."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChildNodeRemovedMethod {
    #[serde(rename = "DOM.childNodeRemoved")]
    ChildNodeRemoved,
}
#[doc = "Mirrors `DOMNodeRemoved` event.\n[childNodeRemoved](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-childNodeRemoved)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChildNodeRemoved {
    pub method: ChildNodeRemovedMethod,
    pub params: ChildNodeRemovedParams,
}
impl ChildNodeRemoved {
    pub const IDENTIFIER: &'static str = "DOM.childNodeRemoved";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Called when distribution is changed.\n[distributedNodesUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-distributedNodesUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributedNodesUpdatedParams {
    #[doc = "Insertion point where distributed nodes were updated."]
    #[serde(rename = "insertionPointId")]
    pub insertion_point_id: super::types::NodeId,
    #[doc = "Distributed nodes for given insertion point."]
    #[serde(rename = "distributedNodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub distributed_nodes: Vec<super::types::BackendNode>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DistributedNodesUpdatedMethod {
    #[serde(rename = "DOM.distributedNodesUpdated")]
    DistributedNodesUpdated,
}
#[doc = "Called when distribution is changed.\n[distributedNodesUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-distributedNodesUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributedNodesUpdated {
    pub method: DistributedNodesUpdatedMethod,
    pub params: DistributedNodesUpdatedParams,
}
impl DistributedNodesUpdated {
    pub const IDENTIFIER: &'static str = "DOM.distributedNodesUpdated";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Fired when `Document` has been totally updated. Node ids are no longer valid.\n[documentUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-documentUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentUpdatedParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DocumentUpdatedMethod {
    #[serde(rename = "DOM.documentUpdated")]
    DocumentUpdated,
}
#[doc = "Fired when `Document` has been totally updated. Node ids are no longer valid.\n[documentUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-documentUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentUpdated {
    pub method: DocumentUpdatedMethod,
    pub params: DocumentUpdatedParams,
}
impl DocumentUpdated {
    pub const IDENTIFIER: &'static str = "DOM.documentUpdated";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Fired when `Element`'s inline style is modified via a CSS property modification.\n[inlineStyleInvalidated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-inlineStyleInvalidated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineStyleInvalidatedParams {
    #[doc = "Ids of the nodes for which the inline styles have been invalidated."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::types::NodeId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InlineStyleInvalidatedMethod {
    #[serde(rename = "DOM.inlineStyleInvalidated")]
    InlineStyleInvalidated,
}
#[doc = "Fired when `Element`'s inline style is modified via a CSS property modification.\n[inlineStyleInvalidated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-inlineStyleInvalidated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineStyleInvalidated {
    pub method: InlineStyleInvalidatedMethod,
    pub params: InlineStyleInvalidatedParams,
}
impl InlineStyleInvalidated {
    pub const IDENTIFIER: &'static str = "DOM.inlineStyleInvalidated";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Called when a pseudo element is added to an element.\n[pseudoElementAdded](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-pseudoElementAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PseudoElementAddedParams {
    #[doc = "Pseudo element's parent element id."]
    #[serde(rename = "parentId")]
    pub parent_id: super::types::NodeId,
    #[doc = "The added pseudo element."]
    #[serde(rename = "pseudoElement")]
    pub pseudo_element: super::types::Node,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PseudoElementAddedMethod {
    #[serde(rename = "DOM.pseudoElementAdded")]
    PseudoElementAdded,
}
#[doc = "Called when a pseudo element is added to an element.\n[pseudoElementAdded](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-pseudoElementAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PseudoElementAdded {
    pub method: PseudoElementAddedMethod,
    pub params: PseudoElementAddedParams,
}
impl PseudoElementAdded {
    pub const IDENTIFIER: &'static str = "DOM.pseudoElementAdded";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Called when top layer elements are changed.\n[topLayerElementsUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-topLayerElementsUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopLayerElementsUpdatedParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TopLayerElementsUpdatedMethod {
    #[serde(rename = "DOM.topLayerElementsUpdated")]
    TopLayerElementsUpdated,
}
#[doc = "Called when top layer elements are changed.\n[topLayerElementsUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-topLayerElementsUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopLayerElementsUpdated {
    pub method: TopLayerElementsUpdatedMethod,
    pub params: TopLayerElementsUpdatedParams,
}
impl TopLayerElementsUpdated {
    pub const IDENTIFIER: &'static str = "DOM.topLayerElementsUpdated";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Fired when a node's scrollability state changes.\n[scrollableFlagUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-scrollableFlagUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrollableFlagUpdatedParams {
    #[doc = "The id of the node."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "If the node is scrollable."]
    #[serde(rename = "isScrollable")]
    pub is_scrollable: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScrollableFlagUpdatedMethod {
    #[serde(rename = "DOM.scrollableFlagUpdated")]
    ScrollableFlagUpdated,
}
#[doc = "Fired when a node's scrollability state changes.\n[scrollableFlagUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-scrollableFlagUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrollableFlagUpdated {
    pub method: ScrollableFlagUpdatedMethod,
    pub params: ScrollableFlagUpdatedParams,
}
impl ScrollableFlagUpdated {
    pub const IDENTIFIER: &'static str = "DOM.scrollableFlagUpdated";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Fired when a node's ad related state changes.\n[adRelatedStateUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-adRelatedStateUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdRelatedStateUpdatedParams {
    #[doc = "The id of the node."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "If the node is ad related."]
    #[serde(rename = "isAdRelated")]
    pub is_ad_related: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AdRelatedStateUpdatedMethod {
    #[serde(rename = "DOM.adRelatedStateUpdated")]
    AdRelatedStateUpdated,
}
#[doc = "Fired when a node's ad related state changes.\n[adRelatedStateUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-adRelatedStateUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdRelatedStateUpdated {
    pub method: AdRelatedStateUpdatedMethod,
    pub params: AdRelatedStateUpdatedParams,
}
impl AdRelatedStateUpdated {
    pub const IDENTIFIER: &'static str = "DOM.adRelatedStateUpdated";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Fired when a node's starting styles changes.\n[affectedByStartingStylesFlagUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-affectedByStartingStylesFlagUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectedByStartingStylesFlagUpdatedParams {
    #[doc = "The id of the node."]
    #[serde(rename = "nodeId")]
    pub node_id: Box<super::types::NodeId>,
    #[doc = "If the node has starting styles."]
    #[serde(rename = "affectedByStartingStyles")]
    pub affected_by_starting_styles: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AffectedByStartingStylesFlagUpdatedMethod {
    #[serde(rename = "DOM.affectedByStartingStylesFlagUpdated")]
    AffectedByStartingStylesFlagUpdated,
}
#[doc = "Fired when a node's starting styles changes.\n[affectedByStartingStylesFlagUpdated](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-affectedByStartingStylesFlagUpdated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffectedByStartingStylesFlagUpdated {
    pub method: AffectedByStartingStylesFlagUpdatedMethod,
    pub params: AffectedByStartingStylesFlagUpdatedParams,
}
impl AffectedByStartingStylesFlagUpdated {
    pub const IDENTIFIER: &'static str = "DOM.affectedByStartingStylesFlagUpdated";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Called when a pseudo element is removed from an element.\n[pseudoElementRemoved](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-pseudoElementRemoved)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PseudoElementRemovedParams {
    #[doc = "Pseudo element's parent element id."]
    #[serde(rename = "parentId")]
    pub parent_id: super::types::NodeId,
    #[doc = "The removed pseudo element id."]
    #[serde(rename = "pseudoElementId")]
    pub pseudo_element_id: super::types::NodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PseudoElementRemovedMethod {
    #[serde(rename = "DOM.pseudoElementRemoved")]
    PseudoElementRemoved,
}
#[doc = "Called when a pseudo element is removed from an element.\n[pseudoElementRemoved](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-pseudoElementRemoved)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PseudoElementRemoved {
    pub method: PseudoElementRemovedMethod,
    pub params: PseudoElementRemovedParams,
}
impl PseudoElementRemoved {
    pub const IDENTIFIER: &'static str = "DOM.pseudoElementRemoved";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Fired when backend wants to provide client with the missing DOM structure. This happens upon\nmost of the calls requesting node ids.\n[setChildNodes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-setChildNodes)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetChildNodesParams {
    #[doc = "Parent node id to populate with children."]
    #[serde(rename = "parentId")]
    pub parent_id: super::types::NodeId,
    #[doc = "Child nodes array."]
    #[serde(rename = "nodes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nodes: Vec<super::types::Node>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetChildNodesMethod {
    #[serde(rename = "DOM.setChildNodes")]
    SetChildNodes,
}
#[doc = "Fired when backend wants to provide client with the missing DOM structure. This happens upon\nmost of the calls requesting node ids.\n[setChildNodes](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-setChildNodes)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetChildNodes {
    pub method: SetChildNodesMethod,
    pub params: SetChildNodesParams,
}
impl SetChildNodes {
    pub const IDENTIFIER: &'static str = "DOM.setChildNodes";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Called when shadow root is popped from the element.\n[shadowRootPopped](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-shadowRootPopped)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowRootPoppedParams {
    #[doc = "Host element id."]
    #[serde(rename = "hostId")]
    pub host_id: super::types::NodeId,
    #[doc = "Shadow root id."]
    #[serde(rename = "rootId")]
    pub root_id: super::types::NodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShadowRootPoppedMethod {
    #[serde(rename = "DOM.shadowRootPopped")]
    ShadowRootPopped,
}
#[doc = "Called when shadow root is popped from the element.\n[shadowRootPopped](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-shadowRootPopped)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowRootPopped {
    pub method: ShadowRootPoppedMethod,
    pub params: ShadowRootPoppedParams,
}
impl ShadowRootPopped {
    pub const IDENTIFIER: &'static str = "DOM.shadowRootPopped";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Called when shadow root is pushed into the element.\n[shadowRootPushed](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-shadowRootPushed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowRootPushedParams {
    #[doc = "Host element id."]
    #[serde(rename = "hostId")]
    pub host_id: super::types::NodeId,
    #[doc = "Shadow root."]
    #[serde(rename = "root")]
    pub root: super::types::Node,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShadowRootPushedMethod {
    #[serde(rename = "DOM.shadowRootPushed")]
    ShadowRootPushed,
}
#[doc = "Called when shadow root is pushed into the element.\n[shadowRootPushed](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#event-shadowRootPushed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowRootPushed {
    pub method: ShadowRootPushedMethod,
    pub params: ShadowRootPushedParams,
}
impl ShadowRootPushed {
    pub const IDENTIFIER: &'static str = "DOM.shadowRootPushed";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (DomEvents { AttributeModified (AttributeModified) , AdoptedStyleSheetsModified (AdoptedStyleSheetsModified) , AttributeRemoved (AttributeRemoved) , CharacterDataModified (CharacterDataModified) , ChildNodeCountUpdated (ChildNodeCountUpdated) , ChildNodeInserted (ChildNodeInserted) , ChildNodeRemoved (ChildNodeRemoved) , DistributedNodesUpdated (DistributedNodesUpdated) , DocumentUpdated (DocumentUpdated) , InlineStyleInvalidated (InlineStyleInvalidated) , PseudoElementAdded (PseudoElementAdded) , TopLayerElementsUpdated (TopLayerElementsUpdated) , ScrollableFlagUpdated (ScrollableFlagUpdated) , AdRelatedStateUpdated (AdRelatedStateUpdated) , AffectedByStartingStylesFlagUpdated (AffectedByStartingStylesFlagUpdated) , PseudoElementRemoved (PseudoElementRemoved) , SetChildNodes (SetChildNodes) , ShadowRootPopped (ShadowRootPopped) , ShadowRootPushed (ShadowRootPushed) });
