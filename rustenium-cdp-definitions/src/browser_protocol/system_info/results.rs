use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInfoResult {
    #[doc = "Information about the GPUs on the system."]
    #[serde(rename = "gpu")]
    pub gpu: super::types::GpuInfo,
    #[doc = "A platform-dependent description of the model of the machine. On Mac OS, this is, for\nexample, 'MacBookPro'. Will be the empty string if not supported."]
    #[serde(rename = "modelName")]
    pub model_name: String,
    #[doc = "A platform-dependent description of the version of the machine. On Mac OS, this is, for\nexample, '10.1'. Will be the empty string if not supported."]
    #[serde(rename = "modelVersion")]
    pub model_version: String,
    #[doc = "The command line string used to launch the browser. Will be the empty string if not\nsupported."]
    #[serde(rename = "commandLine")]
    pub command_line: String,
}
impl TryFrom<serde_json::Value> for GetInfoResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFeatureStateResult {
    #[serde(rename = "featureEnabled")]
    pub feature_enabled: bool,
}
impl TryFrom<serde_json::Value> for GetFeatureStateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetProcessInfoResult {
    #[doc = "An array of process info blocks."]
    #[serde(rename = "processInfo")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub process_info: Vec<super::types::ProcessInfo>,
}
impl TryFrom<serde_json::Value> for GetProcessInfoResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
