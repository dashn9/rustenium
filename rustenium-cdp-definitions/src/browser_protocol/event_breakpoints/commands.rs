use serde::{Deserialize, Serialize};
#[doc = "Sets breakpoint on particular native event.\n[setInstrumentationBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/EventBreakpoints/#method-setInstrumentationBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInstrumentationBreakpointParams {
    #[doc = "Instrumentation name to stop on."]
    #[serde(rename = "eventName")]
    pub event_name: String,
}
impl SetInstrumentationBreakpointParams {
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            event_name: event_name.into(),
        }
    }
}
impl<T: Into<String>> From<T> for SetInstrumentationBreakpointParams {
    fn from(url: T) -> Self {
        SetInstrumentationBreakpointParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetInstrumentationBreakpointMethod {
    #[serde(rename = "EventBreakpoints.setInstrumentationBreakpoint")]
    SetInstrumentationBreakpoint,
}
impl SetInstrumentationBreakpointMethod {
    pub const IDENTIFIER: &'static str = "EventBreakpoints.setInstrumentationBreakpoint";
}
#[doc = "Sets breakpoint on particular native event.\n[setInstrumentationBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/EventBreakpoints/#method-setInstrumentationBreakpoint)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetInstrumentationBreakpoint {
    pub method: SetInstrumentationBreakpointMethod,
    pub params: SetInstrumentationBreakpointParams,
}
#[doc = "Removes breakpoint on particular native event.\n[removeInstrumentationBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/EventBreakpoints/#method-removeInstrumentationBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveInstrumentationBreakpointParams {
    #[doc = "Instrumentation name to stop on."]
    #[serde(rename = "eventName")]
    pub event_name: String,
}
impl RemoveInstrumentationBreakpointParams {
    pub fn new(event_name: impl Into<String>) -> Self {
        Self {
            event_name: event_name.into(),
        }
    }
}
impl<T: Into<String>> From<T> for RemoveInstrumentationBreakpointParams {
    fn from(url: T) -> Self {
        RemoveInstrumentationBreakpointParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveInstrumentationBreakpointMethod {
    #[serde(rename = "EventBreakpoints.removeInstrumentationBreakpoint")]
    RemoveInstrumentationBreakpoint,
}
impl RemoveInstrumentationBreakpointMethod {
    pub const IDENTIFIER: &'static str = "EventBreakpoints.removeInstrumentationBreakpoint";
}
#[doc = "Removes breakpoint on particular native event.\n[removeInstrumentationBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/EventBreakpoints/#method-removeInstrumentationBreakpoint)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RemoveInstrumentationBreakpoint {
    pub method: RemoveInstrumentationBreakpointMethod,
    pub params: RemoveInstrumentationBreakpointParams,
}
#[doc = "Removes all breakpoints\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/EventBreakpoints/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "EventBreakpoints.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "EventBreakpoints.disable";
}
#[doc = "Removes all breakpoints\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/EventBreakpoints/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
group_enum ! (Command { SetInstrumentationBreakpoint (SetInstrumentationBreakpoint) , RemoveInstrumentationBreakpoint (RemoveInstrumentationBreakpoint) , Disable (Disable) });
