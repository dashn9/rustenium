use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
impl TryFrom<serde_json::Value> for DisableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
impl TryFrom<serde_json::Value> for EnableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBestEffortCoverageResult {
    #[doc = "Coverage data for the current isolate."]
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<super::types::ScriptCoverage>,
}
impl TryFrom<serde_json::Value> for GetBestEffortCoverageResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSamplingIntervalResult {}
impl TryFrom<serde_json::Value> for SetSamplingIntervalResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartResult {}
impl TryFrom<serde_json::Value> for StartResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartPreciseCoverageResult {
    #[doc = "Monotonically increasing time (in seconds) when the coverage update was taken in the backend."]
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
}
impl TryFrom<serde_json::Value> for StartPreciseCoverageResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopResult {
    #[doc = "Recorded profile."]
    #[serde(rename = "profile")]
    pub profile: super::types::Profile,
}
impl TryFrom<serde_json::Value> for StopResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopPreciseCoverageResult {}
impl TryFrom<serde_json::Value> for StopPreciseCoverageResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
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
impl TryFrom<serde_json::Value> for TakePreciseCoverageResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
