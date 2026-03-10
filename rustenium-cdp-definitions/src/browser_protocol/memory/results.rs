use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomCountersResult {
    #[serde(rename = "documents")]
    pub documents: i64,
    #[serde(rename = "nodes")]
    pub nodes: i64,
    #[serde(rename = "jsEventListeners")]
    pub js_event_listeners: i64,
}
impl TryFrom<serde_json::Value> for GetDomCountersResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomCountersForLeakDetectionResult {
    #[doc = "DOM object counters."]
    #[serde(rename = "counters")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub counters: Vec<super::types::DomCounter>,
}
impl TryFrom<serde_json::Value> for GetDomCountersForLeakDetectionResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PrepareForLeakDetectionResult {}
impl TryFrom<serde_json::Value> for PrepareForLeakDetectionResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ForciblyPurgeJavaScriptMemoryResult {}
impl TryFrom<serde_json::Value> for ForciblyPurgeJavaScriptMemoryResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPressureNotificationsSuppressedResult {}
impl TryFrom<serde_json::Value> for SetPressureNotificationsSuppressedResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulatePressureNotificationResult {}
impl TryFrom<serde_json::Value> for SimulatePressureNotificationResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartSamplingResult {}
impl TryFrom<serde_json::Value> for StartSamplingResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopSamplingResult {}
impl TryFrom<serde_json::Value> for StopSamplingResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAllTimeSamplingProfileResult {
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingProfile,
}
impl TryFrom<serde_json::Value> for GetAllTimeSamplingProfileResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBrowserSamplingProfileResult {
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingProfile,
}
impl TryFrom<serde_json::Value> for GetBrowserSamplingProfileResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSamplingProfileResult {
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingProfile,
}
impl TryFrom<serde_json::Value> for GetSamplingProfileResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
