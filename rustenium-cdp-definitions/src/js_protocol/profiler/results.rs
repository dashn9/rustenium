use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBestEffortCoverageResult {
    #[doc = "Coverage data for the current isolate."]
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<super::types::ScriptCoverage>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSamplingIntervalResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartPreciseCoverageResult {
    #[doc = "Monotonically increasing time (in seconds) when the coverage update was taken in the backend."]
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopResult {
    #[doc = "Recorded profile."]
    #[serde(rename = "profile")]
    pub profile: super::types::Profile,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopPreciseCoverageResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakePreciseCoverageResult {
    #[doc = "Coverage data for the current isolate."]
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<super::types::ScriptCoverage>,
    #[doc = "Monotonically increasing time (in seconds) when the coverage update was taken in the backend."]
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
}
