use serde::{Deserialize, Serialize};
#[doc = "Unique DOM node identifier.\n[NodeId](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-NodeId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Copy, Hash)]
pub struct NodeId(i64);
impl NodeId {
    pub fn new(val: impl Into<i64>) -> Self {
        NodeId(val.into())
    }
    pub fn inner(&self) -> &i64 {
        &self.0
    }
}
impl NodeId {
    pub const IDENTIFIER: &'static str = "DOM.NodeId";
}
#[doc = "Unique DOM node identifier used to reference a node that may not have been pushed to the\nfront-end.\n[BackendNodeId](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-BackendNodeId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Copy, Hash)]
pub struct BackendNodeId(i64);
impl BackendNodeId {
    pub fn new(val: impl Into<i64>) -> Self {
        BackendNodeId(val.into())
    }
    pub fn inner(&self) -> &i64 {
        &self.0
    }
}
impl BackendNodeId {
    pub const IDENTIFIER: &'static str = "DOM.BackendNodeId";
}
#[doc = "Unique identifier for a CSS stylesheet.\n[StyleSheetId](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-StyleSheetId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct StyleSheetId(String);
impl StyleSheetId {
    pub fn new(val: impl Into<String>) -> Self {
        StyleSheetId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for StyleSheetId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<StyleSheetId> for String {
    fn from(el: StyleSheetId) -> String {
        el.0
    }
}
impl From<String> for StyleSheetId {
    fn from(expr: String) -> Self {
        StyleSheetId(expr)
    }
}
impl std::borrow::Borrow<str> for StyleSheetId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl StyleSheetId {
    pub const IDENTIFIER: &'static str = "DOM.StyleSheetId";
}
#[doc = "Backend node with a friendly name.\n[BackendNode](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-BackendNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackendNode {
    #[doc = "`Node`'s nodeType."]
    #[serde(rename = "nodeType")]
    pub node_type: i64,
    #[doc = "`Node`'s nodeName."]
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: BackendNodeId,
}
impl BackendNode {
    pub fn new(
        node_type: impl Into<i64>,
        node_name: impl Into<String>,
        backend_node_id: impl Into<BackendNodeId>,
    ) -> Self {
        Self {
            node_type: node_type.into(),
            node_name: node_name.into(),
            backend_node_id: backend_node_id.into(),
        }
    }
}
impl BackendNode {
    pub const IDENTIFIER: &'static str = "DOM.BackendNode";
}
#[doc = "Pseudo element type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PseudoType {
    #[serde(rename = "first-line")]
    FirstLine,
    #[serde(rename = "first-letter")]
    FirstLetter,
    #[serde(rename = "checkmark")]
    Checkmark,
    #[serde(rename = "before")]
    Before,
    #[serde(rename = "after")]
    After,
    #[serde(rename = "picker-icon")]
    PickerIcon,
    #[serde(rename = "interest-hint")]
    InterestHint,
    #[serde(rename = "marker")]
    Marker,
    #[serde(rename = "backdrop")]
    Backdrop,
    #[serde(rename = "column")]
    Column,
    #[serde(rename = "selection")]
    Selection,
    #[serde(rename = "search-text")]
    SearchText,
    #[serde(rename = "target-text")]
    TargetText,
    #[serde(rename = "spelling-error")]
    SpellingError,
    #[serde(rename = "grammar-error")]
    GrammarError,
    #[serde(rename = "highlight")]
    Highlight,
    #[serde(rename = "first-line-inherited")]
    FirstLineInherited,
    #[serde(rename = "scroll-marker")]
    ScrollMarker,
    #[serde(rename = "scroll-marker-group")]
    ScrollMarkerGroup,
    #[serde(rename = "scroll-button")]
    ScrollButton,
    #[serde(rename = "scrollbar")]
    Scrollbar,
    #[serde(rename = "scrollbar-thumb")]
    ScrollbarThumb,
    #[serde(rename = "scrollbar-button")]
    ScrollbarButton,
    #[serde(rename = "scrollbar-track")]
    ScrollbarTrack,
    #[serde(rename = "scrollbar-track-piece")]
    ScrollbarTrackPiece,
    #[serde(rename = "scrollbar-corner")]
    ScrollbarCorner,
    #[serde(rename = "resizer")]
    Resizer,
    #[serde(rename = "input-list-button")]
    InputListButton,
    #[serde(rename = "view-transition")]
    ViewTransition,
    #[serde(rename = "view-transition-group")]
    ViewTransitionGroup,
    #[serde(rename = "view-transition-image-pair")]
    ViewTransitionImagePair,
    #[serde(rename = "view-transition-group-children")]
    ViewTransitionGroupChildren,
    #[serde(rename = "view-transition-old")]
    ViewTransitionOld,
    #[serde(rename = "view-transition-new")]
    ViewTransitionNew,
    #[serde(rename = "placeholder")]
    Placeholder,
    #[serde(rename = "file-selector-button")]
    FileSelectorButton,
    #[serde(rename = "details-content")]
    DetailsContent,
    #[serde(rename = "picker")]
    Picker,
    #[serde(rename = "permission-icon")]
    PermissionIcon,
    #[serde(rename = "overscroll-area-parent")]
    OverscrollAreaParent,
}
#[doc = "Shadow root type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShadowRootType {
    #[serde(rename = "user-agent")]
    UserAgent,
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
}
#[doc = "Document compatibility mode."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CompatibilityMode {
    #[serde(rename = "QuirksMode")]
    QuirksMode,
    #[serde(rename = "LimitedQuirksMode")]
    LimitedQuirksMode,
    #[serde(rename = "NoQuirksMode")]
    NoQuirksMode,
}
#[doc = "ContainerSelector physical axes"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PhysicalAxes {
    #[serde(rename = "Horizontal")]
    Horizontal,
    #[serde(rename = "Vertical")]
    Vertical,
    #[serde(rename = "Both")]
    Both,
}
#[doc = "ContainerSelector logical axes"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LogicalAxes {
    #[serde(rename = "Inline")]
    Inline,
    #[serde(rename = "Block")]
    Block,
    #[serde(rename = "Both")]
    Both,
}
#[doc = "Physical scroll orientation"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScrollOrientation {
    #[serde(rename = "horizontal")]
    Horizontal,
    #[serde(rename = "vertical")]
    Vertical,
}
#[doc = "DOM interaction is implemented in terms of mirror objects that represent the actual DOM nodes.\nDOMNode is a base node mirror type.\n[Node](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-Node)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    #[doc = "Node identifier that is passed into the rest of the DOM messages as the `nodeId`. Backend\nwill only push node with given `id` once. It is aware of all requested nodes and will only\nfire DOM events for nodes known to the client."]
    #[serde(rename = "nodeId")]
    pub node_id: NodeId,
    #[doc = "The id of the parent node if any."]
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_id: Option<NodeId>,
    #[doc = "The BackendNodeId for this node."]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: BackendNodeId,
    #[doc = "`Node`'s nodeType."]
    #[serde(rename = "nodeType")]
    pub node_type: i64,
    #[doc = "`Node`'s nodeName."]
    #[serde(rename = "nodeName")]
    pub node_name: String,
    #[doc = "`Node`'s localName."]
    #[serde(rename = "localName")]
    pub local_name: String,
    #[doc = "`Node`'s nodeValue."]
    #[serde(rename = "nodeValue")]
    pub node_value: String,
    #[doc = "Child count for `Container` nodes."]
    #[serde(rename = "childNodeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub child_node_count: Option<i64>,
    #[doc = "Child nodes of this node when requested with children."]
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub children: Option<Vec<Node>>,
    #[doc = "Attributes of the `Element` node in the form of flat array `[name1, value1, name2, value2]`."]
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub attributes: Option<Vec<String>>,
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
    #[doc = "`DocumentType`'s publicId."]
    #[serde(rename = "publicId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub public_id: Option<String>,
    #[doc = "`DocumentType`'s systemId."]
    #[serde(rename = "systemId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub system_id: Option<String>,
    #[doc = "`DocumentType`'s internalSubset."]
    #[serde(rename = "internalSubset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_subset: Option<String>,
    #[doc = "`Document`'s XML version in case of XML documents."]
    #[serde(rename = "xmlVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub xml_version: Option<String>,
    #[doc = "`Attr`'s name."]
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[doc = "`Attr`'s value."]
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<String>,
    #[doc = "Pseudo element type for this node."]
    #[serde(rename = "pseudoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pseudo_type: Option<PseudoType>,
    #[doc = "Pseudo element identifier for this node. Only present if there is a\nvalid pseudoType."]
    #[serde(rename = "pseudoIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pseudo_identifier: Option<String>,
    #[doc = "Shadow root type."]
    #[serde(rename = "shadowRootType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shadow_root_type: Option<ShadowRootType>,
    #[doc = "Frame ID for frame owner elements."]
    #[serde(rename = "frameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_id: Option<super::super::page::types::FrameId>,
    #[doc = "Content document for frame owner elements."]
    #[serde(rename = "contentDocument")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub content_document: Option<Box<Node>>,
    #[doc = "Shadow root list for given element host."]
    #[serde(rename = "shadowRoots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shadow_roots: Option<Vec<Node>>,
    #[doc = "Content document fragment for template elements."]
    #[serde(rename = "templateContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub template_content: Option<Box<Node>>,
    #[doc = "Pseudo elements associated with this node."]
    #[serde(rename = "pseudoElements")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pseudo_elements: Option<Vec<Node>>,
    #[doc = "Distributed nodes for given insertion point."]
    #[serde(rename = "distributedNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub distributed_nodes: Option<Vec<BackendNode>>,
    #[doc = "Whether the node is SVG."]
    #[serde(rename = "isSVG")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_svg: Option<bool>,
    #[serde(rename = "compatibilityMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub compatibility_mode: Option<CompatibilityMode>,
    #[serde(rename = "assignedSlot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub assigned_slot: Option<BackendNode>,
    #[serde(rename = "isScrollable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_scrollable: Option<bool>,
    #[serde(rename = "affectedByStartingStyles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub affected_by_starting_styles: Option<bool>,
    #[serde(rename = "adoptedStyleSheets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub adopted_style_sheets: Option<Vec<StyleSheetId>>,
    #[serde(rename = "isAdRelated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_ad_related: Option<bool>,
}
impl Node {
    pub const IDENTIFIER: &'static str = "DOM.Node";
}
#[doc = "A structure to hold the top-level node of a detached tree and an array of its retained descendants.\n[DetachedElementInfo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-DetachedElementInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetachedElementInfo {
    #[serde(rename = "treeNode")]
    pub tree_node: Node,
    #[serde(rename = "retainedNodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub retained_node_ids: Vec<NodeId>,
}
impl DetachedElementInfo {
    pub fn new(tree_node: impl Into<Node>, retained_node_ids: Vec<NodeId>) -> Self {
        Self {
            tree_node: tree_node.into(),
            retained_node_ids,
        }
    }
}
impl DetachedElementInfo {
    pub const IDENTIFIER: &'static str = "DOM.DetachedElementInfo";
}
#[doc = "A structure holding an RGBA color.\n[RGBA](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-RGBA)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rgba {
    #[doc = "The red component, in the [0-255] range."]
    #[serde(rename = "r")]
    pub r: i64,
    #[doc = "The green component, in the [0-255] range."]
    #[serde(rename = "g")]
    pub g: i64,
    #[doc = "The blue component, in the [0-255] range."]
    #[serde(rename = "b")]
    pub b: i64,
    #[doc = "The alpha component, in the [0-1] range (default: 1)."]
    #[serde(rename = "a")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub a: Option<f64>,
}
impl Rgba {
    pub fn new(r: impl Into<i64>, g: impl Into<i64>, b: impl Into<i64>) -> Self {
        Self {
            r: r.into(),
            g: g.into(),
            b: b.into(),
            a: None,
        }
    }
}
impl Rgba {
    pub const IDENTIFIER: &'static str = "DOM.RGBA";
}
#[doc = "An array of quad vertices, x immediately followed by y for each point, points clock-wise.\n[Quad](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-Quad)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Quad(Vec<f64>);
impl Quad {
    pub fn new(val: impl Into<Vec<f64>>) -> Self {
        Quad(val.into())
    }
    pub fn inner(&self) -> &Vec<f64> {
        &self.0
    }
}
impl Quad {
    pub const IDENTIFIER: &'static str = "DOM.Quad";
}
#[doc = "Box model.\n[BoxModel](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-BoxModel)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoxModel {
    #[doc = "Content box"]
    #[serde(rename = "content")]
    pub content: Quad,
    #[doc = "Padding box"]
    #[serde(rename = "padding")]
    pub padding: Quad,
    #[doc = "Border box"]
    #[serde(rename = "border")]
    pub border: Quad,
    #[doc = "Margin box"]
    #[serde(rename = "margin")]
    pub margin: Quad,
    #[doc = "Node width"]
    #[serde(rename = "width")]
    pub width: i64,
    #[doc = "Node height"]
    #[serde(rename = "height")]
    pub height: i64,
    #[doc = "Shape outside coordinates"]
    #[serde(rename = "shapeOutside")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub shape_outside: Option<ShapeOutsideInfo>,
}
impl BoxModel {
    pub const IDENTIFIER: &'static str = "DOM.BoxModel";
}
#[doc = "CSS Shape Outside details.\n[ShapeOutsideInfo](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-ShapeOutsideInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShapeOutsideInfo {
    #[doc = "Shape bounds"]
    #[serde(rename = "bounds")]
    pub bounds: Quad,
    #[doc = "Shape coordinate details"]
    #[serde(rename = "shape")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub shape: Vec<serde_json::Value>,
    #[doc = "Margin shape bounds"]
    #[serde(rename = "marginShape")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub margin_shape: Vec<serde_json::Value>,
}
impl ShapeOutsideInfo {
    pub fn new(
        bounds: impl Into<Quad>,
        shape: Vec<serde_json::Value>,
        margin_shape: Vec<serde_json::Value>,
    ) -> Self {
        Self {
            bounds: bounds.into(),
            shape,
            margin_shape,
        }
    }
}
impl ShapeOutsideInfo {
    pub const IDENTIFIER: &'static str = "DOM.ShapeOutsideInfo";
}
#[doc = "Rectangle.\n[Rect](https://chromedevtools.github.io/devtools-protocol/tot/DOM/#type-Rect)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    #[doc = "X coordinate"]
    #[serde(rename = "x")]
    pub x: f64,
    #[doc = "Y coordinate"]
    #[serde(rename = "y")]
    pub y: f64,
    #[doc = "Rectangle width"]
    #[serde(rename = "width")]
    pub width: f64,
    #[doc = "Rectangle height"]
    #[serde(rename = "height")]
    pub height: f64,
}
impl Rect {
    pub fn new(
        x: impl Into<f64>,
        y: impl Into<f64>,
        width: impl Into<f64>,
        height: impl Into<f64>,
    ) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            width: width.into(),
            height: height.into(),
        }
    }
}
impl Rect {
    pub const IDENTIFIER: &'static str = "DOM.Rect";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssComputedStyleProperty {
    #[doc = "Computed style property name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Computed style property value."]
    #[serde(rename = "value")]
    pub value: String,
}
impl CssComputedStyleProperty {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
impl CssComputedStyleProperty {
    pub const IDENTIFIER: &'static str = "DOM.CSSComputedStyleProperty";
}
group_enum ! (Type { NodeId (NodeId) , BackendNodeId (BackendNodeId) , StyleSheetId (StyleSheetId) , BackendNode (BackendNode) , PseudoType (PseudoType) , ShadowRootType (ShadowRootType) , CompatibilityMode (CompatibilityMode) , PhysicalAxes (PhysicalAxes) , LogicalAxes (LogicalAxes) , ScrollOrientation (ScrollOrientation) , Node (Node) , DetachedElementInfo (DetachedElementInfo) , Rgba (Rgba) , Quad (Quad) , BoxModel (BoxModel) , ShapeOutsideInfo (ShapeOutsideInfo) , Rect (Rect) , CssComputedStyleProperty (CssComputedStyleProperty) });
