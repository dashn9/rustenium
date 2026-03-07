use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEventListenersResult {
    #[doc = "Array of relevant listeners."]
    #[serde(rename = "listeners")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub listeners: Vec<super::types::EventListener>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveDomBreakpointResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveEventListenerBreakpointResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveInstrumentationBreakpointResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveXhrBreakpointResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetBreakOnCspViolationResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDomBreakpointResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetEventListenerBreakpointResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetInstrumentationBreakpointResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetXhrBreakpointResult {}
