use serde::{Deserialize, Serialize};
#[doc = "A Node in the DOM tree.\n[DOMNode](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#type-DOMNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomNode {
    #[doc = "`Node`'s nodeType."]
    #[serde(rename = "nodeType")]
    pub node_type: i64,
    #[doc = "`Node`'s nodeName."]
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[doc = "`Node`'s nodeValue."]
    #[serde(rename = "nodeValue")]
    pub node_value: String,
    #[doc = "Only set for textarea elements, contains the text value."]
    #[serde(rename = "textValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub text_value: Option<String>,
    #[doc = "Only set for input elements, contains the input's associated text value."]
    #[serde(rename = "inputValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub input_value: Option<String>,
    #[doc = "Only set for radio and checkbox input elements, indicates if the element has been checked"]
    #[serde(rename = "inputChecked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub input_checked: Option<bool>,
    #[doc = "Only set for option elements, indicates if the element has been selected"]
    #[serde(rename = "optionSelected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub option_selected: Option<bool>,
    #[doc = "`Node`'s id, corresponds to DOM.Node.backendNodeId."]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: crate::browser_protocol::dom::types::BackendNodeId,
    #[doc = "The indexes of the node's child nodes in the `domNodes` array returned by `getSnapshot`, if\nany."]
    #[serde(rename = "childNodeIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub child_node_indexes: Option<Vec<i64>>,
    #[doc = "Attributes of an `Element` node."]
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub attributes: Option<Vec<NameValue>>,
    #[doc = "Indexes of pseudo elements associated with this node in the `domNodes` array returned by\n`getSnapshot`, if any."]
    #[serde(rename = "pseudoElementIndexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pseudo_element_indexes: Option<Vec<i64>>,
    #[doc = "The index of the node's related layout tree node in the `layoutTreeNodes` array returned by\n`getSnapshot`, if any."]
    #[serde(rename = "layoutNodeIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub layout_node_index: Option<i64>,
    #[doc = "Document URL that `Document` or `FrameOwner` node points to."]
    #[serde(rename = "documentURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub document_url: Option<String>,
    #[doc = "Base URL that `Document` or `FrameOwner` node uses for URL completion."]
    #[serde(rename = "baseURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub base_url: Option<String>,
    #[doc = "Only set for documents, contains the document's content language."]
    #[serde(rename = "contentLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content_language: Option<String>,
    #[doc = "Only set for documents, contains the document's character set encoding."]
    #[serde(rename = "documentEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub document_encoding: Option<String>,
    #[doc = "`DocumentType` node's publicId."]
    #[serde(rename = "publicId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub public_id: Option<String>,
    #[doc = "`DocumentType` node's systemId."]
    #[serde(rename = "systemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub system_id: Option<String>,
    #[doc = "Frame ID for frame owner elements and also for the document node."]
    #[serde(rename = "frameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_id: Option<crate::browser_protocol::page::types::FrameId>,
    #[doc = "The index of a frame owner element's content document in the `domNodes` array returned by\n`getSnapshot`, if any."]
    #[serde(rename = "contentDocumentIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content_document_index: Option<i64>,
    #[doc = "Type of a pseudo element node."]
    #[serde(rename = "pseudoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pseudo_type: Option<crate::browser_protocol::dom::types::PseudoType>,
    #[doc = "Shadow root type."]
    #[serde(rename = "shadowRootType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shadow_root_type: Option<crate::browser_protocol::dom::types::ShadowRootType>,
    #[doc = "Whether this DOM node responds to mouse clicks. This includes nodes that have had click\nevent listeners attached via JavaScript as well as anchor tags that naturally navigate when\nclicked."]
    #[serde(rename = "isClickable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_clickable: Option<bool>,
    #[doc = "Details of the node's event listeners, if any."]
    #[serde(rename = "eventListeners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub event_listeners: Option<Vec<crate::browser_protocol::dom_debugger::types::EventListener>>,
    #[doc = "The selected url for nodes with a srcset attribute."]
    #[serde(rename = "currentSourceURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub current_source_url: Option<String>,
    #[doc = "The url of the script (if any) that generates this node."]
    #[serde(rename = "originURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub origin_url: Option<String>,
    #[doc = "Scroll offsets, set when this node is a Document."]
    #[serde(rename = "scrollOffsetX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scroll_offset_x: Option<f64>,
    #[serde(rename = "scrollOffsetY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scroll_offset_y: Option<f64>,
}
impl DomNode {
    pub fn new(
        node_type: impl Into<i64>,
        node_name: impl Into<String>,
        node_value: impl Into<String>,
        backend_node_id: impl Into<crate::browser_protocol::dom::types::BackendNodeId>,
    ) -> Self {
        Self {
            node_type: node_type.into(),
            node_name: node_name.into(),
            node_value: node_value.into(),
            backend_node_id: backend_node_id.into(),
            text_value: None,
            input_value: None,
            input_checked: None,
            option_selected: None,
            child_node_indexes: None,
            attributes: None,
            pseudo_element_indexes: None,
            layout_node_index: None,
            document_url: None,
            base_url: None,
            content_language: None,
            document_encoding: None,
            public_id: None,
            system_id: None,
            frame_id: None,
            content_document_index: None,
            pseudo_type: None,
            shadow_root_type: None,
            is_clickable: None,
            event_listeners: None,
            current_source_url: None,
            origin_url: None,
            scroll_offset_x: None,
            scroll_offset_y: None,
        }
    }
}
impl DomNode {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.DOMNode";
}
#[doc = "Details of post layout rendered text positions. The exact layout should not be regarded as\nstable and may change between versions.\n[InlineTextBox](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#type-InlineTextBox)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InlineTextBox {
    #[doc = "The bounding box in document coordinates. Note that scroll offset of the document is ignored."]
    #[serde(rename = "boundingBox")]
    pub bounding_box: crate::browser_protocol::dom::types::Rect,
    #[doc = "The starting index in characters, for this post layout textbox substring. Characters that\nwould be represented as a surrogate pair in UTF-16 have length 2."]
    #[serde(rename = "startCharacterIndex")]
    pub start_character_index: i64,
    #[doc = "The number of characters in this post layout textbox substring. Characters that would be\nrepresented as a surrogate pair in UTF-16 have length 2."]
    #[serde(rename = "numCharacters")]
    pub num_characters: i64,
}
impl InlineTextBox {
    pub fn new(
        bounding_box: impl Into<crate::browser_protocol::dom::types::Rect>,
        start_character_index: impl Into<i64>,
        num_characters: impl Into<i64>,
    ) -> Self {
        Self {
            bounding_box: bounding_box.into(),
            start_character_index: start_character_index.into(),
            num_characters: num_characters.into(),
        }
    }
}
impl InlineTextBox {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.InlineTextBox";
}
#[doc = "Details of an element in the DOM tree with a LayoutObject.\n[LayoutTreeNode](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#type-LayoutTreeNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutTreeNode {
    #[doc = "The index of the related DOM node in the `domNodes` array returned by `getSnapshot`."]
    #[serde(rename = "domNodeIndex")]
    pub dom_node_index: i64,
    #[doc = "The bounding box in document coordinates. Note that scroll offset of the document is ignored."]
    #[serde(rename = "boundingBox")]
    pub bounding_box: crate::browser_protocol::dom::types::Rect,
    #[doc = "Contents of the LayoutText, if any."]
    #[serde(rename = "layoutText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub layout_text: Option<String>,
    #[doc = "The post-layout inline text nodes, if any."]
    #[serde(rename = "inlineTextNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub inline_text_nodes: Option<Vec<InlineTextBox>>,
    #[doc = "Index into the `computedStyles` array returned by `getSnapshot`."]
    #[serde(rename = "styleIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub style_index: Option<i64>,
    #[doc = "Global paint order index, which is determined by the stacking order of the nodes. Nodes\nthat are painted together will have the same index. Only provided if includePaintOrder in\ngetSnapshot was true."]
    #[serde(rename = "paintOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub paint_order: Option<i64>,
    #[doc = "Set to true to indicate the element begins a new stacking context."]
    #[serde(rename = "isStackingContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_stacking_context: Option<bool>,
}
impl LayoutTreeNode {
    pub fn new(
        dom_node_index: impl Into<i64>,
        bounding_box: impl Into<crate::browser_protocol::dom::types::Rect>,
    ) -> Self {
        Self {
            dom_node_index: dom_node_index.into(),
            bounding_box: bounding_box.into(),
            layout_text: None,
            inline_text_nodes: None,
            style_index: None,
            paint_order: None,
            is_stacking_context: None,
        }
    }
}
impl LayoutTreeNode {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.LayoutTreeNode";
}
#[doc = "A subset of the full ComputedStyle as defined by the request whitelist.\n[ComputedStyle](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#type-ComputedStyle)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputedStyle {
    #[doc = "Name/value pairs of computed style properties."]
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<NameValue>,
}
impl ComputedStyle {
    pub fn new(properties: Vec<NameValue>) -> Self {
        Self { properties }
    }
}
impl ComputedStyle {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.ComputedStyle";
}
#[doc = "A name/value pair.\n[NameValue](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#type-NameValue)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NameValue {
    #[doc = "Attribute/property name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Attribute/property value."]
    #[serde(rename = "value")]
    pub value: String,
}
impl NameValue {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
impl NameValue {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.NameValue";
}
#[doc = "Index of the string in the strings table.\n[StringIndex](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#type-StringIndex)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Copy, Hash)]
pub struct StringIndex(i64);
impl StringIndex {
    pub fn new(val: impl Into<i64>) -> Self {
        StringIndex(val.into())
    }
    pub fn inner(&self) -> &i64 {
        &self.0
    }
}
impl StringIndex {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.StringIndex";
}
#[doc = "Index of the string in the strings table.\n[ArrayOfStrings](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#type-ArrayOfStrings)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayOfStrings(Vec<StringIndex>);
impl ArrayOfStrings {
    pub fn new(val: impl Into<Vec<StringIndex>>) -> Self {
        ArrayOfStrings(val.into())
    }
    pub fn inner(&self) -> &Vec<StringIndex> {
        &self.0
    }
}
impl ArrayOfStrings {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.ArrayOfStrings";
}
#[doc = "Data that is only present on rare nodes.\n[RareStringData](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#type-RareStringData)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RareStringData {
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub index: Vec<i64>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<StringIndex>,
}
impl RareStringData {
    pub fn new(index: Vec<i64>, value: Vec<StringIndex>) -> Self {
        Self { index, value }
    }
}
impl RareStringData {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.RareStringData";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RareBooleanData {
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub index: Vec<i64>,
}
impl RareBooleanData {
    pub fn new(index: Vec<i64>) -> Self {
        Self { index }
    }
}
impl RareBooleanData {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.RareBooleanData";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RareIntegerData {
    #[serde(rename = "index")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub index: Vec<i64>,
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<i64>,
}
impl RareIntegerData {
    pub fn new(index: Vec<i64>, value: Vec<i64>) -> Self {
        Self { index, value }
    }
}
impl RareIntegerData {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.RareIntegerData";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rectangle(Vec<f64>);
impl Rectangle {
    pub fn new(val: impl Into<Vec<f64>>) -> Self {
        Rectangle(val.into())
    }
    pub fn inner(&self) -> &Vec<f64> {
        &self.0
    }
}
impl Rectangle {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.Rectangle";
}
#[doc = "Document snapshot.\n[DocumentSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#type-DocumentSnapshot)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentSnapshot {
    #[doc = "Document URL that `Document` or `FrameOwner` node points to."]
    #[serde(rename = "documentURL")]
    pub document_url: StringIndex,
    #[doc = "Document title."]
    #[serde(rename = "title")]
    pub title: StringIndex,
    #[doc = "Base URL that `Document` or `FrameOwner` node uses for URL completion."]
    #[serde(rename = "baseURL")]
    pub base_url: StringIndex,
    #[doc = "Contains the document's content language."]
    #[serde(rename = "contentLanguage")]
    pub content_language: StringIndex,
    #[doc = "Contains the document's character set encoding."]
    #[serde(rename = "encodingName")]
    pub encoding_name: StringIndex,
    #[doc = "`DocumentType` node's publicId."]
    #[serde(rename = "publicId")]
    pub public_id: StringIndex,
    #[doc = "`DocumentType` node's systemId."]
    #[serde(rename = "systemId")]
    pub system_id: StringIndex,
    #[doc = "Frame ID for frame owner elements and also for the document node."]
    #[serde(rename = "frameId")]
    pub frame_id: StringIndex,
    #[doc = "A table with dom nodes."]
    #[serde(rename = "nodes")]
    pub nodes: NodeTreeSnapshot,
    #[doc = "The nodes in the layout tree."]
    #[serde(rename = "layout")]
    pub layout: LayoutTreeSnapshot,
    #[doc = "The post-layout inline text nodes."]
    #[serde(rename = "textBoxes")]
    pub text_boxes: TextBoxSnapshot,
    #[doc = "Horizontal scroll offset."]
    #[serde(rename = "scrollOffsetX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scroll_offset_x: Option<f64>,
    #[doc = "Vertical scroll offset."]
    #[serde(rename = "scrollOffsetY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scroll_offset_y: Option<f64>,
    #[doc = "Document content width."]
    #[serde(rename = "contentWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content_width: Option<f64>,
    #[doc = "Document content height."]
    #[serde(rename = "contentHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content_height: Option<f64>,
}
impl DocumentSnapshot {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.DocumentSnapshot";
}
#[doc = "Table containing nodes.\n[NodeTreeSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#type-NodeTreeSnapshot)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NodeTreeSnapshot {
    #[doc = "Parent node index."]
    #[serde(rename = "parentIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_index: Option<Vec<i64>>,
    #[doc = "`Node`'s nodeType."]
    #[serde(rename = "nodeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_type: Option<Vec<i64>>,
    #[doc = "Type of the shadow root the `Node` is in. String values are equal to the `ShadowRootType` enum."]
    #[serde(rename = "shadowRootType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shadow_root_type: Option<RareStringData>,
    #[doc = "`Node`'s nodeName."]
    #[serde(rename = "nodeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_name: Option<Vec<StringIndex>>,
    #[doc = "`Node`'s nodeValue."]
    #[serde(rename = "nodeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_value: Option<Vec<StringIndex>>,
    #[doc = "`Node`'s id, corresponds to DOM.Node.backendNodeId."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<Vec<crate::browser_protocol::dom::types::BackendNodeId>>,
    #[doc = "Attributes of an `Element` node. Flatten name, value pairs."]
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub attributes: Option<Vec<ArrayOfStrings>>,
    #[doc = "Only set for textarea elements, contains the text value."]
    #[serde(rename = "textValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub text_value: Option<RareStringData>,
    #[doc = "Only set for input elements, contains the input's associated text value."]
    #[serde(rename = "inputValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub input_value: Option<RareStringData>,
    #[doc = "Only set for radio and checkbox input elements, indicates if the element has been checked"]
    #[serde(rename = "inputChecked")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub input_checked: Option<RareBooleanData>,
    #[doc = "Only set for option elements, indicates if the element has been selected"]
    #[serde(rename = "optionSelected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub option_selected: Option<RareBooleanData>,
    #[doc = "The index of the document in the list of the snapshot documents."]
    #[serde(rename = "contentDocumentIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content_document_index: Option<RareIntegerData>,
    #[doc = "Type of a pseudo element node."]
    #[serde(rename = "pseudoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pseudo_type: Option<RareStringData>,
    #[doc = "Pseudo element identifier for this node. Only present if there is a\nvalid pseudoType."]
    #[serde(rename = "pseudoIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pseudo_identifier: Option<RareStringData>,
    #[doc = "Whether this DOM node responds to mouse clicks. This includes nodes that have had click\nevent listeners attached via JavaScript as well as anchor tags that naturally navigate when\nclicked."]
    #[serde(rename = "isClickable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_clickable: Option<RareBooleanData>,
    #[doc = "The selected url for nodes with a srcset attribute."]
    #[serde(rename = "currentSourceURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub current_source_url: Option<RareStringData>,
    #[doc = "The url of the script (if any) that generates this node."]
    #[serde(rename = "originURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub origin_url: Option<RareStringData>,
}
impl NodeTreeSnapshot {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.NodeTreeSnapshot";
}
#[doc = "Table of details of an element in the DOM tree with a LayoutObject.\n[LayoutTreeSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#type-LayoutTreeSnapshot)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutTreeSnapshot {
    #[doc = "Index of the corresponding node in the `NodeTreeSnapshot` array returned by `captureSnapshot`."]
    #[serde(rename = "nodeIndex")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_index: Vec<i64>,
    #[doc = "Array of indexes specifying computed style strings, filtered according to the `computedStyles` parameter passed to `captureSnapshot`."]
    #[serde(rename = "styles")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub styles: Vec<ArrayOfStrings>,
    #[doc = "The absolute position bounding box."]
    #[serde(rename = "bounds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bounds: Vec<Rectangle>,
    #[doc = "Contents of the LayoutText, if any."]
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub text: Vec<StringIndex>,
    #[doc = "Stacking context information."]
    #[serde(rename = "stackingContexts")]
    pub stacking_contexts: RareBooleanData,
    #[doc = "Global paint order index, which is determined by the stacking order of the nodes. Nodes\nthat are painted together will have the same index. Only provided if includePaintOrder in\ncaptureSnapshot was true."]
    #[serde(rename = "paintOrders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub paint_orders: Option<Vec<i64>>,
    #[doc = "The offset rect of nodes. Only available when includeDOMRects is set to true"]
    #[serde(rename = "offsetRects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub offset_rects: Option<Vec<Rectangle>>,
    #[doc = "The scroll rect of nodes. Only available when includeDOMRects is set to true"]
    #[serde(rename = "scrollRects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scroll_rects: Option<Vec<Rectangle>>,
    #[doc = "The client rect of nodes. Only available when includeDOMRects is set to true"]
    #[serde(rename = "clientRects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub client_rects: Option<Vec<Rectangle>>,
    #[doc = "The list of background colors that are blended with colors of overlapping elements."]
    #[serde(rename = "blendedBackgroundColors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub blended_background_colors: Option<Vec<StringIndex>>,
    #[doc = "The list of computed text opacities."]
    #[serde(rename = "textColorOpacities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub text_color_opacities: Option<Vec<f64>>,
}
impl LayoutTreeSnapshot {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.LayoutTreeSnapshot";
}
#[doc = "Table of details of the post layout rendered text positions. The exact layout should not be regarded as\nstable and may change between versions.\n[TextBoxSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#type-TextBoxSnapshot)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextBoxSnapshot {
    #[doc = "Index of the layout tree node that owns this box collection."]
    #[serde(rename = "layoutIndex")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub layout_index: Vec<i64>,
    #[doc = "The absolute position bounding box."]
    #[serde(rename = "bounds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bounds: Vec<Rectangle>,
    #[doc = "The starting index in characters, for this post layout textbox substring. Characters that\nwould be represented as a surrogate pair in UTF-16 have length 2."]
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub start: Vec<i64>,
    #[doc = "The number of characters in this post layout textbox substring. Characters that would be\nrepresented as a surrogate pair in UTF-16 have length 2."]
    #[serde(rename = "length")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub length: Vec<i64>,
}
impl TextBoxSnapshot {
    pub fn new(
        layout_index: Vec<i64>,
        bounds: Vec<Rectangle>,
        start: Vec<i64>,
        length: Vec<i64>,
    ) -> Self {
        Self {
            layout_index,
            bounds,
            start,
            length,
        }
    }
}
impl TextBoxSnapshot {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.TextBoxSnapshot";
}
group_enum ! (DomSnapshotTypes { DomNode (DomNode) , InlineTextBox (InlineTextBox) , LayoutTreeNode (LayoutTreeNode) , ComputedStyle (ComputedStyle) , NameValue (NameValue) , StringIndex (StringIndex) , ArrayOfStrings (ArrayOfStrings) , RareStringData (RareStringData) , RareBooleanData (RareBooleanData) , RareIntegerData (RareIntegerData) , Rectangle (Rectangle) , DocumentSnapshot (DocumentSnapshot) , NodeTreeSnapshot (NodeTreeSnapshot) , LayoutTreeSnapshot (LayoutTreeSnapshot) , TextBoxSnapshot (TextBoxSnapshot) });
