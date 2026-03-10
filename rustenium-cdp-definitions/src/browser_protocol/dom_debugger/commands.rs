use serde::{Deserialize, Serialize};
#[doc = "Returns event listeners of the given object.\n[getEventListeners](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-getEventListeners)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEventListenersParams {
    #[doc = "Identifier of the object to return listeners for."]
    #[serde(rename = "objectId")]
    pub object_id: crate::js_protocol::runtime::types::RemoteObjectId,
    #[doc = "The maximum depth at which Node children should be retrieved, defaults to 1. Use -1 for the\nentire subtree or provide an integer larger than 0."]
    #[serde(rename = "depth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub depth: Option<i64>,
    #[doc = "Whether or not iframes and shadow roots should be traversed when returning the subtree\n(default is false). Reports listeners for all contexts if pierce is enabled."]
    #[serde(rename = "pierce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pierce: Option<bool>,
}
impl GetEventListenersParams {
    pub fn new(object_id: impl Into<crate::js_protocol::runtime::types::RemoteObjectId>) -> Self {
        Self {
            object_id: object_id.into(),
            depth: None,
            pierce: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetEventListenersMethod {
    #[serde(rename = "DOMDebugger.getEventListeners")]
    GetEventListeners,
}
#[doc = "Returns event listeners of the given object.\n[getEventListeners](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-getEventListeners)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEventListeners {
    pub method: GetEventListenersMethod,
    pub params: GetEventListenersParams,
}
impl GetEventListeners {
    pub const IDENTIFIER: &'static str = "DOMDebugger.getEventListeners";
}
impl crate::CommandResult for GetEventListeners {
    type Result = super::results::GetEventListenersResult;
}
#[doc = "Removes DOM breakpoint that was set using `setDOMBreakpoint`.\n[removeDOMBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-removeDOMBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveDomBreakpointParams {
    #[doc = "Identifier of the node to remove breakpoint from."]
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
    #[doc = "Type of the breakpoint to remove."]
    #[serde(rename = "type")]
    pub r#type: super::types::DomBreakpointType,
}
impl RemoveDomBreakpointParams {
    pub fn new(
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
        r#type: impl Into<super::types::DomBreakpointType>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            r#type: r#type.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveDomBreakpointMethod {
    #[serde(rename = "DOMDebugger.removeDOMBreakpoint")]
    RemoveDomBreakpoint,
}
#[doc = "Removes DOM breakpoint that was set using `setDOMBreakpoint`.\n[removeDOMBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-removeDOMBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveDomBreakpoint {
    pub method: RemoveDomBreakpointMethod,
    pub params: RemoveDomBreakpointParams,
}
impl RemoveDomBreakpoint {
    pub const IDENTIFIER: &'static str = "DOMDebugger.removeDOMBreakpoint";
}
impl crate::CommandResult for RemoveDomBreakpoint {
    type Result = super::results::RemoveDomBreakpointResult;
}
#[doc = "Removes breakpoint on particular DOM event.\n[removeEventListenerBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-removeEventListenerBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveEventListenerBreakpointParams {
    #[doc = "Event name."]
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[doc = "EventTarget interface name."]
    #[serde(rename = "targetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub target_name: Option<String>,
}
impl RemoveEventListenerBreakpointParams {
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            event_name: event_name.into(),
            target_name: None,
        }
    }
}
impl<T: Into<String>> From<T> for RemoveEventListenerBreakpointParams {
    fn from(url: T) -> Self {
        RemoveEventListenerBreakpointParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveEventListenerBreakpointMethod {
    #[serde(rename = "DOMDebugger.removeEventListenerBreakpoint")]
    RemoveEventListenerBreakpoint,
}
#[doc = "Removes breakpoint on particular DOM event.\n[removeEventListenerBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-removeEventListenerBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveEventListenerBreakpoint {
    pub method: RemoveEventListenerBreakpointMethod,
    pub params: RemoveEventListenerBreakpointParams,
}
impl RemoveEventListenerBreakpoint {
    pub const IDENTIFIER: &'static str = "DOMDebugger.removeEventListenerBreakpoint";
}
impl crate::CommandResult for RemoveEventListenerBreakpoint {
    type Result = super::results::RemoveEventListenerBreakpointResult;
}
#[doc = "Removes breakpoint from XMLHttpRequest.\n[removeXHRBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-removeXHRBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveXhrBreakpointParams {
    #[doc = "Resource URL substring."]
    #[serde(rename = "url")]
    pub url: String,
}
impl RemoveXhrBreakpointParams {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}
impl<T: Into<String>> From<T> for RemoveXhrBreakpointParams {
    fn from(url: T) -> Self {
        RemoveXhrBreakpointParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveXhrBreakpointMethod {
    #[serde(rename = "DOMDebugger.removeXHRBreakpoint")]
    RemoveXhrBreakpoint,
}
#[doc = "Removes breakpoint from XMLHttpRequest.\n[removeXHRBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-removeXHRBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveXhrBreakpoint {
    pub method: RemoveXhrBreakpointMethod,
    pub params: RemoveXhrBreakpointParams,
}
impl RemoveXhrBreakpoint {
    pub const IDENTIFIER: &'static str = "DOMDebugger.removeXHRBreakpoint";
}
impl crate::CommandResult for RemoveXhrBreakpoint {
    type Result = super::results::RemoveXhrBreakpointResult;
}
#[doc = "Sets breakpoint on particular CSP violations.\n[setBreakOnCSPViolation](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-setBreakOnCSPViolation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakOnCspViolationParams {
    #[doc = "CSP Violations to stop upon."]
    #[serde(rename = "violationTypes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub violation_types: Vec<super::types::CspViolationType>,
}
impl SetBreakOnCspViolationParams {
    pub fn new(violation_types: Vec<super::types::CspViolationType>) -> Self {
        Self { violation_types }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetBreakOnCspViolationMethod {
    #[serde(rename = "DOMDebugger.setBreakOnCSPViolation")]
    SetBreakOnCspViolation,
}
#[doc = "Sets breakpoint on particular CSP violations.\n[setBreakOnCSPViolation](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-setBreakOnCSPViolation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakOnCspViolation {
    pub method: SetBreakOnCspViolationMethod,
    pub params: SetBreakOnCspViolationParams,
}
impl SetBreakOnCspViolation {
    pub const IDENTIFIER: &'static str = "DOMDebugger.setBreakOnCSPViolation";
}
impl crate::CommandResult for SetBreakOnCspViolation {
    type Result = super::results::SetBreakOnCspViolationResult;
}
#[doc = "Sets breakpoint on particular operation with DOM.\n[setDOMBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-setDOMBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDomBreakpointParams {
    #[doc = "Identifier of the node to set breakpoint on."]
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
    #[doc = "Type of the operation to stop upon."]
    #[serde(rename = "type")]
    pub r#type: super::types::DomBreakpointType,
}
impl SetDomBreakpointParams {
    pub fn new(
        node_id: impl Into<crate::browser_protocol::dom::types::NodeId>,
        r#type: impl Into<super::types::DomBreakpointType>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            r#type: r#type.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDomBreakpointMethod {
    #[serde(rename = "DOMDebugger.setDOMBreakpoint")]
    SetDomBreakpoint,
}
#[doc = "Sets breakpoint on particular operation with DOM.\n[setDOMBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-setDOMBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDomBreakpoint {
    pub method: SetDomBreakpointMethod,
    pub params: SetDomBreakpointParams,
}
impl SetDomBreakpoint {
    pub const IDENTIFIER: &'static str = "DOMDebugger.setDOMBreakpoint";
}
impl crate::CommandResult for SetDomBreakpoint {
    type Result = super::results::SetDomBreakpointResult;
}
#[doc = "Sets breakpoint on particular DOM event.\n[setEventListenerBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-setEventListenerBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEventListenerBreakpointParams {
    #[doc = "DOM Event name to stop on (any DOM event will do)."]
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[doc = "EventTarget interface name to stop on. If equal to `\"*\"` or not provided, will stop on any\nEventTarget."]
    #[serde(rename = "targetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub target_name: Option<String>,
}
impl SetEventListenerBreakpointParams {
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            event_name: event_name.into(),
            target_name: None,
        }
    }
}
impl<T: Into<String>> From<T> for SetEventListenerBreakpointParams {
    fn from(url: T) -> Self {
        SetEventListenerBreakpointParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetEventListenerBreakpointMethod {
    #[serde(rename = "DOMDebugger.setEventListenerBreakpoint")]
    SetEventListenerBreakpoint,
}
#[doc = "Sets breakpoint on particular DOM event.\n[setEventListenerBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-setEventListenerBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEventListenerBreakpoint {
    pub method: SetEventListenerBreakpointMethod,
    pub params: SetEventListenerBreakpointParams,
}
impl SetEventListenerBreakpoint {
    pub const IDENTIFIER: &'static str = "DOMDebugger.setEventListenerBreakpoint";
}
impl crate::CommandResult for SetEventListenerBreakpoint {
    type Result = super::results::SetEventListenerBreakpointResult;
}
#[doc = "Sets breakpoint on XMLHttpRequest.\n[setXHRBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-setXHRBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetXhrBreakpointParams {
    #[doc = "Resource URL substring. All XHRs having this substring in the URL will get stopped upon."]
    #[serde(rename = "url")]
    pub url: String,
}
impl SetXhrBreakpointParams {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}
impl<T: Into<String>> From<T> for SetXhrBreakpointParams {
    fn from(url: T) -> Self {
        SetXhrBreakpointParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetXhrBreakpointMethod {
    #[serde(rename = "DOMDebugger.setXHRBreakpoint")]
    SetXhrBreakpoint,
}
#[doc = "Sets breakpoint on XMLHttpRequest.\n[setXHRBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/DOMDebugger/#method-setXHRBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetXhrBreakpoint {
    pub method: SetXhrBreakpointMethod,
    pub params: SetXhrBreakpointParams,
}
impl SetXhrBreakpoint {
    pub const IDENTIFIER: &'static str = "DOMDebugger.setXHRBreakpoint";
}
impl crate::CommandResult for SetXhrBreakpoint {
    type Result = super::results::SetXhrBreakpointResult;
}
group_enum ! (DomDebuggerCommands { GetEventListeners (GetEventListeners) , RemoveDomBreakpoint (RemoveDomBreakpoint) , RemoveEventListenerBreakpoint (RemoveEventListenerBreakpoint) , RemoveXhrBreakpoint (RemoveXhrBreakpoint) , SetBreakOnCspViolation (SetBreakOnCspViolation) , SetDomBreakpoint (SetDomBreakpoint) , SetEventListenerBreakpoint (SetEventListenerBreakpoint) , SetXhrBreakpoint (SetXhrBreakpoint) });
