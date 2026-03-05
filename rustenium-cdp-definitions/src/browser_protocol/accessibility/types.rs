use serde::{Deserialize, Serialize};
#[doc = "Unique accessibility node identifier.\n[AXNodeId](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#type-AXNodeId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct AxNodeId(String);
impl AxNodeId {
    pub fn new(val: impl Into<String>) -> Self {
        AxNodeId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for AxNodeId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<AxNodeId> for String {
    fn from(el: AxNodeId) -> String {
        el.0
    }
}
impl From<String> for AxNodeId {
    fn from(expr: String) -> Self {
        AxNodeId(expr)
    }
}
impl std::borrow::Borrow<str> for AxNodeId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl AxNodeId {
    pub const IDENTIFIER: &'static str = "Accessibility.AXNodeId";
}
#[doc = "Enum of possible property types."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AxValueType {
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "tristate")]
    Tristate,
    #[serde(rename = "booleanOrUndefined")]
    BooleanOrUndefined,
    #[serde(rename = "idref")]
    Idref,
    #[serde(rename = "idrefList")]
    IdrefList,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "node")]
    Node,
    #[serde(rename = "nodeList")]
    NodeList,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "computedString")]
    ComputedString,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "tokenList")]
    TokenList,
    #[serde(rename = "domRelation")]
    DomRelation,
    #[serde(rename = "role")]
    Role,
    #[serde(rename = "internalRole")]
    InternalRole,
    #[serde(rename = "valueUndefined")]
    ValueUndefined,
}
#[doc = "Enum of possible property sources."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AxValueSourceType {
    #[serde(rename = "attribute")]
    Attribute,
    #[serde(rename = "implicit")]
    Implicit,
    #[serde(rename = "style")]
    Style,
    #[serde(rename = "contents")]
    Contents,
    #[serde(rename = "placeholder")]
    Placeholder,
    #[serde(rename = "relatedElement")]
    RelatedElement,
}
#[doc = "Enum of possible native property sources (as a subtype of a particular AXValueSourceType)."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AxValueNativeSourceType {
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "figcaption")]
    Figcaption,
    #[serde(rename = "label")]
    Label,
    #[serde(rename = "labelfor")]
    Labelfor,
    #[serde(rename = "labelwrapped")]
    Labelwrapped,
    #[serde(rename = "legend")]
    Legend,
    #[serde(rename = "rubyannotation")]
    Rubyannotation,
    #[serde(rename = "tablecaption")]
    Tablecaption,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "other")]
    Other,
}
#[doc = "A single source for a computed AX property.\n[AXValueSource](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#type-AXValueSource)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxValueSource {
    #[doc = "What type of source this is."]
    #[serde(rename = "type")]
    pub r#type: AxValueSourceType,
    #[doc = "The value of this property source."]
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<AxValue>,
    #[doc = "The name of the relevant attribute, if any."]
    #[serde(rename = "attribute")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub attribute: Option<String>,
    #[doc = "The value of the relevant attribute, if any."]
    #[serde(rename = "attributeValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub attribute_value: Option<AxValue>,
    #[doc = "Whether this source is superseded by a higher priority source."]
    #[serde(rename = "superseded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub superseded: Option<bool>,
    #[doc = "The native markup source for this value, e.g. a `<label>` element."]
    #[serde(rename = "nativeSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub native_source: Option<AxValueNativeSourceType>,
    #[doc = "The value, such as a node or node list, of the native source."]
    #[serde(rename = "nativeSourceValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub native_source_value: Option<AxValue>,
    #[doc = "Whether the value for this property is invalid."]
    #[serde(rename = "invalid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub invalid: Option<bool>,
    #[doc = "Reason for the value being invalid, if it is."]
    #[serde(rename = "invalidReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub invalid_reason: Option<String>,
}
impl AxValueSource {
    pub fn new(r#type: impl Into<AxValueSourceType>) -> Self {
        Self {
            r#type: r#type.into(),
            value: None,
            attribute: None,
            attribute_value: None,
            superseded: None,
            native_source: None,
            native_source_value: None,
            invalid: None,
            invalid_reason: None,
        }
    }
}
impl AxValueSource {
    pub const IDENTIFIER: &'static str = "Accessibility.AXValueSource";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxRelatedNode {
    #[doc = "The BackendNodeId of the related DOM node."]
    #[serde(rename = "backendDOMNodeId")]
    pub backend_dom_node_id: super::super::dom::types::BackendNodeId,
    #[doc = "The IDRef value provided, if any."]
    #[serde(rename = "idref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub idref: Option<String>,
    #[doc = "The text alternative of this node in the current context."]
    #[serde(rename = "text")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub text: Option<String>,
}
impl AxRelatedNode {
    pub fn new(backend_dom_node_id: impl Into<super::super::dom::types::BackendNodeId>) -> Self {
        Self {
            backend_dom_node_id: backend_dom_node_id.into(),
            idref: None,
            text: None,
        }
    }
}
impl AxRelatedNode {
    pub const IDENTIFIER: &'static str = "Accessibility.AXRelatedNode";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxProperty {
    #[doc = "The name of this property."]
    #[serde(rename = "name")]
    pub name: AxPropertyName,
    #[doc = "The value of this property."]
    #[serde(rename = "value")]
    pub value: AxValue,
}
impl AxProperty {
    pub fn new(name: impl Into<AxPropertyName>, value: impl Into<AxValue>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
impl AxProperty {
    pub const IDENTIFIER: &'static str = "Accessibility.AXProperty";
}
#[doc = "A single computed AX property.\n[AXValue](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#type-AXValue)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxValue {
    #[doc = "The type of this value."]
    #[serde(rename = "type")]
    pub r#type: AxValueType,
    #[doc = "The computed value of this property."]
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<serde_json::Value>,
    #[doc = "One or more related nodes, if applicable."]
    #[serde(rename = "relatedNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub related_nodes: Option<Vec<AxRelatedNode>>,
    #[doc = "The sources which contributed to the computation of this property."]
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sources: Option<Vec<AxValueSource>>,
}
impl AxValue {
    pub fn new(r#type: impl Into<AxValueType>) -> Self {
        Self {
            r#type: r#type.into(),
            value: None,
            related_nodes: None,
            sources: None,
        }
    }
}
impl AxValue {
    pub const IDENTIFIER: &'static str = "Accessibility.AXValue";
}
#[doc = "Values of AXProperty name:\n- from 'busy' to 'roledescription': states which apply to every AX node\n- from 'live' to 'root': attributes which apply to nodes in live regions\n- from 'autocomplete' to 'valuetext': attributes which apply to widgets\n- from 'checked' to 'selected': states which apply to widgets\n- from 'activedescendant' to 'owns': relationships between elements other than parent/child/sibling\n- from 'activeFullscreenElement' to 'uninteresting': reasons why this noode is hidden"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AxPropertyName {
    #[serde(rename = "actions")]
    Actions,
    #[serde(rename = "busy")]
    Busy,
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "editable")]
    Editable,
    #[serde(rename = "focusable")]
    Focusable,
    #[serde(rename = "focused")]
    Focused,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "hiddenRoot")]
    HiddenRoot,
    #[serde(rename = "invalid")]
    Invalid,
    #[serde(rename = "keyshortcuts")]
    Keyshortcuts,
    #[serde(rename = "settable")]
    Settable,
    #[serde(rename = "roledescription")]
    Roledescription,
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "atomic")]
    Atomic,
    #[serde(rename = "relevant")]
    Relevant,
    #[serde(rename = "root")]
    Root,
    #[serde(rename = "autocomplete")]
    Autocomplete,
    #[serde(rename = "hasPopup")]
    HasPopup,
    #[serde(rename = "level")]
    Level,
    #[serde(rename = "multiselectable")]
    Multiselectable,
    #[serde(rename = "orientation")]
    Orientation,
    #[serde(rename = "multiline")]
    Multiline,
    #[serde(rename = "readonly")]
    Readonly,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "valuemin")]
    Valuemin,
    #[serde(rename = "valuemax")]
    Valuemax,
    #[serde(rename = "valuetext")]
    Valuetext,
    #[serde(rename = "checked")]
    Checked,
    #[serde(rename = "expanded")]
    Expanded,
    #[serde(rename = "modal")]
    Modal,
    #[serde(rename = "pressed")]
    Pressed,
    #[serde(rename = "selected")]
    Selected,
    #[serde(rename = "activedescendant")]
    Activedescendant,
    #[serde(rename = "controls")]
    Controls,
    #[serde(rename = "describedby")]
    Describedby,
    #[serde(rename = "details")]
    Details,
    #[serde(rename = "errormessage")]
    Errormessage,
    #[serde(rename = "flowto")]
    Flowto,
    #[serde(rename = "labelledby")]
    Labelledby,
    #[serde(rename = "owns")]
    Owns,
    #[serde(rename = "url")]
    Url,
    #[doc = "LINT.IfChange(AXIgnoredReason)"]
    #[serde(rename = "activeFullscreenElement")]
    ActiveFullscreenElement,
    #[serde(rename = "activeModalDialog")]
    ActiveModalDialog,
    #[serde(rename = "activeAriaModalDialog")]
    ActiveAriaModalDialog,
    #[serde(rename = "ariaHiddenElement")]
    AriaHiddenElement,
    #[serde(rename = "ariaHiddenSubtree")]
    AriaHiddenSubtree,
    #[serde(rename = "emptyAlt")]
    EmptyAlt,
    #[serde(rename = "emptyText")]
    EmptyText,
    #[serde(rename = "inertElement")]
    InertElement,
    #[serde(rename = "inertSubtree")]
    InertSubtree,
    #[serde(rename = "labelContainer")]
    LabelContainer,
    #[serde(rename = "labelFor")]
    LabelFor,
    #[serde(rename = "notRendered")]
    NotRendered,
    #[serde(rename = "notVisible")]
    NotVisible,
    #[serde(rename = "presentationalRole")]
    PresentationalRole,
    #[serde(rename = "probablyPresentational")]
    ProbablyPresentational,
    #[serde(rename = "inactiveCarouselTabContent")]
    InactiveCarouselTabContent,
    #[serde(rename = "uninteresting")]
    Uninteresting,
}
#[doc = "LINT.ThenChange(//third_party/blink/renderer/modules/accessibility/ax_enums.cc:AXIgnoredReason)\nA node in the accessibility tree.\n[AXNode](https://chromedevtools.github.io/devtools-protocol/tot/Accessibility/#type-AXNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AxNode {
    #[doc = "Unique identifier for this node."]
    #[serde(rename = "nodeId")]
    pub node_id: AxNodeId,
    #[doc = "Whether this node is ignored for accessibility"]
    #[serde(rename = "ignored")]
    pub ignored: bool,
    #[doc = "Collection of reasons why this node is hidden."]
    #[serde(rename = "ignoredReasons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ignored_reasons: Option<Vec<AxProperty>>,
    #[doc = "This `Node`'s role, whether explicit or implicit."]
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub role: Option<AxValue>,
    #[doc = "This `Node`'s Chrome raw role."]
    #[serde(rename = "chromeRole")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub chrome_role: Option<AxValue>,
    #[doc = "The accessible name for this `Node`."]
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<AxValue>,
    #[doc = "The accessible description for this `Node`."]
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub description: Option<AxValue>,
    #[doc = "The value for this `Node`."]
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub value: Option<AxValue>,
    #[doc = "All other properties"]
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub properties: Option<Vec<AxProperty>>,
    #[doc = "ID for this node's parent."]
    #[serde(rename = "parentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_id: Option<AxNodeId>,
    #[doc = "IDs for each of this node's child nodes."]
    #[serde(rename = "childIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub child_ids: Option<Vec<AxNodeId>>,
    #[doc = "The backend ID for the associated DOM node, if any."]
    #[serde(rename = "backendDOMNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_dom_node_id: Option<super::super::dom::types::BackendNodeId>,
    #[doc = "The frame ID for the frame associated with this nodes document."]
    #[serde(rename = "frameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_id: Option<super::super::page::types::FrameId>,
}
impl AxNode {
    pub fn new(node_id: impl Into<AxNodeId>, ignored: impl Into<bool>) -> Self {
        Self {
            node_id: node_id.into(),
            ignored: ignored.into(),
            ignored_reasons: None,
            role: None,
            chrome_role: None,
            name: None,
            description: None,
            value: None,
            properties: None,
            parent_id: None,
            child_ids: None,
            backend_dom_node_id: None,
            frame_id: None,
        }
    }
}
impl AxNode {
    pub const IDENTIFIER: &'static str = "Accessibility.AXNode";
}
group_enum ! (AccessibilityTypes { AxNodeId (AxNodeId) , AxValueType (AxValueType) , AxValueSourceType (AxValueSourceType) , AxValueNativeSourceType (AxValueNativeSourceType) , AxValueSource (AxValueSource) , AxRelatedNode (AxRelatedNode) , AxProperty (AxProperty) , AxValue (AxValue) , AxPropertyName (AxPropertyName) , AxNode (AxNode) });
