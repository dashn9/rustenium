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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomCountersForLeakDetectionResult {
    #[doc = "DOM object counters."]
    #[serde(rename = "counters")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub counters: Vec<super::types::DomCounter>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PrepareForLeakDetectionResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ForciblyPurgeJavaScriptMemoryResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPressureNotificationsSuppressedResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SimulatePressureNotificationResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartSamplingResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopSamplingResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAllTimeSamplingProfileResult {
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingProfile,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBrowserSamplingProfileResult {
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingProfile,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSamplingProfileResult {
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingProfile,
}
