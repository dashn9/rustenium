use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetEventListenersResult {
    #[doc = "Array of relevant listeners."]
    #[serde(rename = "listeners")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub listeners: Vec<super::types::EventListener>,
}
impl TryFrom<serde_json::Value> for GetEventListenersResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveDomBreakpointResult {}
impl TryFrom<serde_json::Value> for RemoveDomBreakpointResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveEventListenerBreakpointResult {}
impl TryFrom<serde_json::Value> for RemoveEventListenerBreakpointResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveInstrumentationBreakpointResult {}
impl TryFrom<serde_json::Value> for RemoveInstrumentationBreakpointResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveXhrBreakpointResult {}
impl TryFrom<serde_json::Value> for RemoveXhrBreakpointResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetBreakOnCspViolationResult {}
impl TryFrom<serde_json::Value> for SetBreakOnCspViolationResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDomBreakpointResult {}
impl TryFrom<serde_json::Value> for SetDomBreakpointResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetEventListenerBreakpointResult {}
impl TryFrom<serde_json::Value> for SetEventListenerBreakpointResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetInstrumentationBreakpointResult {}
impl TryFrom<serde_json::Value> for SetInstrumentationBreakpointResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetXhrBreakpointResult {}
impl TryFrom<serde_json::Value> for SetXhrBreakpointResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
