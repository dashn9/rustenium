use serde::{Deserialize, Serialize};
#[doc = "Retruns current DOM object counters.\n[getDOMCounters](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-getDOMCounters)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetDomCountersParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetDomCountersMethod {
    #[serde(rename = "Memory.getDOMCounters")]
    GetDomCounters,
}
impl GetDomCountersMethod {
    pub const IDENTIFIER: &'static str = "Memory.getDOMCounters";
}
#[doc = "Retruns current DOM object counters.\n[getDOMCounters](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-getDOMCounters)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetDomCounters {
    pub method: GetDomCountersMethod,
    pub params: GetDomCountersParams,
}
impl super::super::super::CommandResult for GetDomCounters {
    type Result = super::results::GetDomCountersResult;
}
#[doc = "Retruns DOM object counters after preparing renderer for leak detection.\n[getDOMCountersForLeakDetection](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-getDOMCountersForLeakDetection)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetDomCountersForLeakDetectionParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetDomCountersForLeakDetectionMethod {
    #[serde(rename = "Memory.getDOMCountersForLeakDetection")]
    GetDomCountersForLeakDetection,
}
impl GetDomCountersForLeakDetectionMethod {
    pub const IDENTIFIER: &'static str = "Memory.getDOMCountersForLeakDetection";
}
#[doc = "Retruns DOM object counters after preparing renderer for leak detection.\n[getDOMCountersForLeakDetection](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-getDOMCountersForLeakDetection)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetDomCountersForLeakDetection {
    pub method: GetDomCountersForLeakDetectionMethod,
    pub params: GetDomCountersForLeakDetectionParams,
}
impl super::super::super::CommandResult for GetDomCountersForLeakDetection {
    type Result = super::results::GetDomCountersForLeakDetectionResult;
}
#[doc = "Prepares for leak detection by terminating workers, stopping spellcheckers,\ndropping non-essential internal caches, running garbage collections, etc.\n[prepareForLeakDetection](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-prepareForLeakDetection)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PrepareForLeakDetectionParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PrepareForLeakDetectionMethod {
    #[serde(rename = "Memory.prepareForLeakDetection")]
    PrepareForLeakDetection,
}
impl PrepareForLeakDetectionMethod {
    pub const IDENTIFIER: &'static str = "Memory.prepareForLeakDetection";
}
#[doc = "Prepares for leak detection by terminating workers, stopping spellcheckers,\ndropping non-essential internal caches, running garbage collections, etc.\n[prepareForLeakDetection](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-prepareForLeakDetection)"]
#[derive(Debug, Clone, PartialEq)]
pub struct PrepareForLeakDetection {
    pub method: PrepareForLeakDetectionMethod,
    pub params: PrepareForLeakDetectionParams,
}
impl super::super::super::CommandResult for PrepareForLeakDetection {
    type Result = super::results::PrepareForLeakDetectionResult;
}
#[doc = "Simulate OomIntervention by purging V8 memory.\n[forciblyPurgeJavaScriptMemory](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-forciblyPurgeJavaScriptMemory)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ForciblyPurgeJavaScriptMemoryParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ForciblyPurgeJavaScriptMemoryMethod {
    #[serde(rename = "Memory.forciblyPurgeJavaScriptMemory")]
    ForciblyPurgeJavaScriptMemory,
}
impl ForciblyPurgeJavaScriptMemoryMethod {
    pub const IDENTIFIER: &'static str = "Memory.forciblyPurgeJavaScriptMemory";
}
#[doc = "Simulate OomIntervention by purging V8 memory.\n[forciblyPurgeJavaScriptMemory](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-forciblyPurgeJavaScriptMemory)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ForciblyPurgeJavaScriptMemory {
    pub method: ForciblyPurgeJavaScriptMemoryMethod,
    pub params: ForciblyPurgeJavaScriptMemoryParams,
}
impl super::super::super::CommandResult for ForciblyPurgeJavaScriptMemory {
    type Result = super::results::ForciblyPurgeJavaScriptMemoryResult;
}
#[doc = "Enable/disable suppressing memory pressure notifications in all processes.\n[setPressureNotificationsSuppressed](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-setPressureNotificationsSuppressed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPressureNotificationsSuppressedParams {
    #[doc = "If true, memory pressure notifications will be suppressed."]
    #[serde(rename = "suppressed")]
    pub suppressed: bool,
}
impl SetPressureNotificationsSuppressedParams {
    pub fn new(suppressed: impl Into<bool>) -> Self {
        Self {
            suppressed: suppressed.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetPressureNotificationsSuppressedMethod {
    #[serde(rename = "Memory.setPressureNotificationsSuppressed")]
    SetPressureNotificationsSuppressed,
}
impl SetPressureNotificationsSuppressedMethod {
    pub const IDENTIFIER: &'static str = "Memory.setPressureNotificationsSuppressed";
}
#[doc = "Enable/disable suppressing memory pressure notifications in all processes.\n[setPressureNotificationsSuppressed](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-setPressureNotificationsSuppressed)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetPressureNotificationsSuppressed {
    pub method: SetPressureNotificationsSuppressedMethod,
    pub params: SetPressureNotificationsSuppressedParams,
}
impl super::super::super::CommandResult for SetPressureNotificationsSuppressed {
    type Result = super::results::SetPressureNotificationsSuppressedResult;
}
#[doc = "Simulate a memory pressure notification in all processes.\n[simulatePressureNotification](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-simulatePressureNotification)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulatePressureNotificationParams {
    #[doc = "Memory pressure level of the notification."]
    #[serde(rename = "level")]
    pub level: super::types::PressureLevel,
}
impl SimulatePressureNotificationParams {
    pub fn new(level: impl Into<super::types::PressureLevel>) -> Self {
        Self {
            level: level.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SimulatePressureNotificationMethod {
    #[serde(rename = "Memory.simulatePressureNotification")]
    SimulatePressureNotification,
}
impl SimulatePressureNotificationMethod {
    pub const IDENTIFIER: &'static str = "Memory.simulatePressureNotification";
}
#[doc = "Simulate a memory pressure notification in all processes.\n[simulatePressureNotification](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-simulatePressureNotification)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SimulatePressureNotification {
    pub method: SimulatePressureNotificationMethod,
    pub params: SimulatePressureNotificationParams,
}
impl super::super::super::CommandResult for SimulatePressureNotification {
    type Result = super::results::SimulatePressureNotificationResult;
}
#[doc = "Start collecting native memory profile.\n[startSampling](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-startSampling)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartSamplingParams {
    #[doc = "Average number of bytes between samples."]
    #[serde(rename = "samplingInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sampling_interval: Option<i64>,
    #[doc = "Do not randomize intervals between samples."]
    #[serde(rename = "suppressRandomness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub suppress_randomness: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartSamplingMethod {
    #[serde(rename = "Memory.startSampling")]
    StartSampling,
}
impl StartSamplingMethod {
    pub const IDENTIFIER: &'static str = "Memory.startSampling";
}
#[doc = "Start collecting native memory profile.\n[startSampling](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-startSampling)"]
#[derive(Debug, Clone, PartialEq)]
pub struct StartSampling {
    pub method: StartSamplingMethod,
    pub params: StartSamplingParams,
}
impl super::super::super::CommandResult for StartSampling {
    type Result = super::results::StartSamplingResult;
}
#[doc = "Stop collecting native memory profile.\n[stopSampling](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-stopSampling)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopSamplingParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopSamplingMethod {
    #[serde(rename = "Memory.stopSampling")]
    StopSampling,
}
impl StopSamplingMethod {
    pub const IDENTIFIER: &'static str = "Memory.stopSampling";
}
#[doc = "Stop collecting native memory profile.\n[stopSampling](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-stopSampling)"]
#[derive(Debug, Clone, PartialEq)]
pub struct StopSampling {
    pub method: StopSamplingMethod,
    pub params: StopSamplingParams,
}
impl super::super::super::CommandResult for StopSampling {
    type Result = super::results::StopSamplingResult;
}
#[doc = "Retrieve native memory allocations profile\ncollected since renderer process startup.\n[getAllTimeSamplingProfile](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-getAllTimeSamplingProfile)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetAllTimeSamplingProfileParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetAllTimeSamplingProfileMethod {
    #[serde(rename = "Memory.getAllTimeSamplingProfile")]
    GetAllTimeSamplingProfile,
}
impl GetAllTimeSamplingProfileMethod {
    pub const IDENTIFIER: &'static str = "Memory.getAllTimeSamplingProfile";
}
#[doc = "Retrieve native memory allocations profile\ncollected since renderer process startup.\n[getAllTimeSamplingProfile](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-getAllTimeSamplingProfile)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetAllTimeSamplingProfile {
    pub method: GetAllTimeSamplingProfileMethod,
    pub params: GetAllTimeSamplingProfileParams,
}
impl super::super::super::CommandResult for GetAllTimeSamplingProfile {
    type Result = super::results::GetAllTimeSamplingProfileResult;
}
#[doc = "Retrieve native memory allocations profile\ncollected since browser process startup.\n[getBrowserSamplingProfile](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-getBrowserSamplingProfile)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetBrowserSamplingProfileParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetBrowserSamplingProfileMethod {
    #[serde(rename = "Memory.getBrowserSamplingProfile")]
    GetBrowserSamplingProfile,
}
impl GetBrowserSamplingProfileMethod {
    pub const IDENTIFIER: &'static str = "Memory.getBrowserSamplingProfile";
}
#[doc = "Retrieve native memory allocations profile\ncollected since browser process startup.\n[getBrowserSamplingProfile](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-getBrowserSamplingProfile)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetBrowserSamplingProfile {
    pub method: GetBrowserSamplingProfileMethod,
    pub params: GetBrowserSamplingProfileParams,
}
impl super::super::super::CommandResult for GetBrowserSamplingProfile {
    type Result = super::results::GetBrowserSamplingProfileResult;
}
#[doc = "Retrieve native memory allocations profile collected since last\n`startSampling` call.\n[getSamplingProfile](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-getSamplingProfile)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetSamplingProfileParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetSamplingProfileMethod {
    #[serde(rename = "Memory.getSamplingProfile")]
    GetSamplingProfile,
}
impl GetSamplingProfileMethod {
    pub const IDENTIFIER: &'static str = "Memory.getSamplingProfile";
}
#[doc = "Retrieve native memory allocations profile collected since last\n`startSampling` call.\n[getSamplingProfile](https://chromedevtools.github.io/devtools-protocol/tot/Memory/#method-getSamplingProfile)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetSamplingProfile {
    pub method: GetSamplingProfileMethod,
    pub params: GetSamplingProfileParams,
}
impl super::super::super::CommandResult for GetSamplingProfile {
    type Result = super::results::GetSamplingProfileResult;
}
group_enum ! (MemoryCommands { GetDomCounters (GetDomCounters) , GetDomCountersForLeakDetection (GetDomCountersForLeakDetection) , PrepareForLeakDetection (PrepareForLeakDetection) , ForciblyPurgeJavaScriptMemory (ForciblyPurgeJavaScriptMemory) , SetPressureNotificationsSuppressed (SetPressureNotificationsSuppressed) , SimulatePressureNotification (SimulatePressureNotification) , StartSampling (StartSampling) , StopSampling (StopSampling) , GetAllTimeSamplingProfile (GetAllTimeSamplingProfile) , GetBrowserSamplingProfile (GetBrowserSamplingProfile) , GetSamplingProfile (GetSamplingProfile) });
