use serde::{Deserialize, Serialize};
#[doc = "DOM breakpoint type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DomBreakpointType {
    #[serde(rename = "subtree-modified")]
    SubtreeModified,
    #[serde(rename = "attribute-modified")]
    AttributeModified,
    #[serde(rename = "node-removed")]
    NodeRemoved,
}
#[doc = "CSP Violation type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CspViolationType {
    #[serde(rename = "trustedtype-sink-violation")]
    TrustedtypeSinkViolation,
    #[serde(rename = "trustedtype-policy-violation")]
    TrustedtypePolicyViolation,
}
#[doc = "Object event listener.\n[EventListener](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#type-EventListener)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventListener {
    #[doc = "`EventListener`'s type."]
    #[serde(rename = "type")]
    pub r#type: String,
    #[doc = "`EventListener`'s useCapture."]
    #[serde(rename = "useCapture")]
    pub use_capture: bool,
    #[doc = "`EventListener`'s passive flag."]
    #[serde(rename = "passive")]
    pub passive: bool,
    #[doc = "`EventListener`'s once flag."]
    #[serde(rename = "once")]
    pub once: bool,
    #[doc = "Script id of the handler code."]
    #[serde(rename = "scriptId")]
    pub script_id: super::super::super::js_protocol::runtime::types::ScriptId,
    #[doc = "Line number in the script (0-based)."]
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[doc = "Column number in the script (0-based)."]
    #[serde(rename = "columnNumber")]
    pub column_number: i64,
    #[doc = "Event handler function value."]
    #[serde(rename = "handler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub handler: Option<super::super::super::js_protocol::runtime::types::RemoteObject>,
    #[doc = "Event original handler function value."]
    #[serde(rename = "originalHandler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub original_handler: Option<super::super::super::js_protocol::runtime::types::RemoteObject>,
    #[doc = "Node the listener is added to (if any)."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<super::super::dom::types::BackendNodeId>,
}
impl EventListener {
    pub const IDENTIFIER: &'static str = "DOMDebugger.EventListener";
}
group_enum ! (DomDebuggerTypes { DomBreakpointType (DomBreakpointType) , CspViolationType (CspViolationType) , EventListener (EventListener) });
