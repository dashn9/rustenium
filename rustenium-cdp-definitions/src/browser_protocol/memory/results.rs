use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomCountersReturns {
    #[serde(rename = "documents")]
    pub documents: i64,
    #[serde(rename = "nodes")]
    pub nodes: i64,
    #[serde(rename = "jsEventListeners")]
    pub js_event_listeners: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomCountersForLeakDetectionReturns {
    #[doc = "DOM object counters."]
    #[serde(rename = "counters")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub counters: Vec<super::types::DomCounter>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAllTimeSamplingProfileReturns {
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingProfile,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBrowserSamplingProfileReturns {
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingProfile,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSamplingProfileReturns {
    #[serde(rename = "profile")]
    pub profile: super::types::SamplingProfile,
}
