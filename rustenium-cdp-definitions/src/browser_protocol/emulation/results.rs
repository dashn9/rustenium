use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanEmulateResult {
    #[doc = "True if emulation is supported."]
    #[serde(rename = "result")]
    pub result: bool,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearDeviceMetricsOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearGeolocationOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResetPageScaleFactorResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetFocusEmulationEnabledResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAutoDarkModeOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetCpuThrottlingRateResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDefaultBackgroundColorOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSafeAreaInsetsOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDeviceMetricsOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDevicePostureOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearDevicePostureOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDisplayFeaturesOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearDisplayFeaturesOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetScrollbarsHiddenResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDocumentCookieDisabledResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetEmitTouchEventsForMouseResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetEmulatedMediaResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetEmulatedVisionDeficiencyResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetEmulatedOsTextScaleResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetGeolocationOverrideResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOverriddenSensorInformationResult {
    #[serde(rename = "requestedSamplingFrequency")]
    pub requested_sampling_frequency: f64,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSensorOverrideEnabledResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSensorOverrideReadingsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPressureSourceOverrideEnabledResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPressureStateOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPressureDataOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetIdleOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearIdleOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetNavigatorOverridesResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPageScaleFactorResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetScriptExecutionDisabledResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetTouchEmulationEnabledResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetVirtualTimePolicyResult {
    #[doc = "Absolute timestamp at which virtual time was first enabled (up time in milliseconds)."]
    #[serde(rename = "virtualTimeTicksBase")]
    pub virtual_time_ticks_base: f64,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetLocaleOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetTimezoneOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetVisibleSizeResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDisabledImageTypesResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDataSaverOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetHardwareConcurrencyOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetUserAgentOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAutomationOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSmallViewportHeightDifferenceOverrideResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScreenInfosResult {
    #[serde(rename = "screenInfos")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub screen_infos: Vec<super::types::ScreenInfo>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddScreenResult {
    #[serde(rename = "screenInfo")]
    pub screen_info: super::types::ScreenInfo,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveScreenResult {}
