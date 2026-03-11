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
#[doc = "Sets breakpoint on particular native event.\n[setInstrumentationBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/EventBreakpoints/#method-setInstrumentationBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInstrumentationBreakpoint {
    pub method: SetInstrumentationBreakpointMethod,
    pub params: SetInstrumentationBreakpointParams,
}
impl SetInstrumentationBreakpoint {
    pub const IDENTIFIER: &'static str = "EventBreakpoints.setInstrumentationBreakpoint";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetInstrumentationBreakpoint {
    type Result = super::results::SetInstrumentationBreakpointResult;
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
#[doc = "Removes breakpoint on particular native event.\n[removeInstrumentationBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/EventBreakpoints/#method-removeInstrumentationBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveInstrumentationBreakpoint {
    pub method: RemoveInstrumentationBreakpointMethod,
    pub params: RemoveInstrumentationBreakpointParams,
}
impl RemoveInstrumentationBreakpoint {
    pub const IDENTIFIER: &'static str = "EventBreakpoints.removeInstrumentationBreakpoint";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for RemoveInstrumentationBreakpoint {
    type Result = super::results::RemoveInstrumentationBreakpointResult;
}
#[doc = "Removes all breakpoints\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/EventBreakpoints/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "EventBreakpoints.disable")]
    Disable,
}
#[doc = "Removes all breakpoints\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/EventBreakpoints/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "EventBreakpoints.disable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
group_enum ! (EventBreakpointsCommands { SetInstrumentationBreakpoint (SetInstrumentationBreakpoint) , RemoveInstrumentationBreakpoint (RemoveInstrumentationBreakpoint) , Disable (Disable) });
