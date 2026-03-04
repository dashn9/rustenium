use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanEmulateReturns {
    #[doc = "True if emulation is supported."]
    #[serde(rename = "result")]
    pub result: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOverriddenSensorInformationReturns {
    #[serde(rename = "requestedSamplingFrequency")]
    pub requested_sampling_frequency: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetVirtualTimePolicyReturns {
    #[doc = "Absolute timestamp at which virtual time was first enabled (up time in milliseconds)."]
    #[serde(rename = "virtualTimeTicksBase")]
    pub virtual_time_ticks_base: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScreenInfosReturns {
    #[serde(rename = "screenInfos")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub screen_infos: Vec<super::types::ScreenInfo>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddScreenReturns {
    #[serde(rename = "screenInfo")]
    pub screen_info: super::types::ScreenInfo,
}
