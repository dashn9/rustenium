use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Profiler.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "Profiler.disable";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Profiler.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "Profiler.enable";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
#[doc = "Collect coverage data for the current isolate. The coverage data may be incomplete due to\ngarbage collection.\n[getBestEffortCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-getBestEffortCoverage)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetBestEffortCoverageParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetBestEffortCoverageMethod {
    #[serde(rename = "Profiler.getBestEffortCoverage")]
    GetBestEffortCoverage,
}
impl GetBestEffortCoverageMethod {
    pub const IDENTIFIER: &'static str = "Profiler.getBestEffortCoverage";
}
#[doc = "Collect coverage data for the current isolate. The coverage data may be incomplete due to\ngarbage collection.\n[getBestEffortCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-getBestEffortCoverage)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetBestEffortCoverage {
    pub method: GetBestEffortCoverageMethod,
    pub params: GetBestEffortCoverageParams,
}
#[doc = "Changes CPU profiler sampling interval. Must be called before CPU profiles recording started.\n[setSamplingInterval](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-setSamplingInterval)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSamplingIntervalParams {
    #[doc = "New sampling interval in microseconds."]
    #[serde(rename = "interval")]
    pub interval: i64,
}
impl SetSamplingIntervalParams {
    pub fn new(interval: impl Into<i64>) -> Self {
        Self {
            interval: interval.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetSamplingIntervalMethod {
    #[serde(rename = "Profiler.setSamplingInterval")]
    SetSamplingInterval,
}
impl SetSamplingIntervalMethod {
    pub const IDENTIFIER: &'static str = "Profiler.setSamplingInterval";
}
#[doc = "Changes CPU profiler sampling interval. Must be called before CPU profiles recording started.\n[setSamplingInterval](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-setSamplingInterval)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetSamplingInterval {
    pub method: SetSamplingIntervalMethod,
    pub params: SetSamplingIntervalParams,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartMethod {
    #[serde(rename = "Profiler.start")]
    Start,
}
impl StartMethod {
    pub const IDENTIFIER: &'static str = "Profiler.start";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Start {
    pub method: StartMethod,
    pub params: StartParams,
}
#[doc = "Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code\ncoverage may be incomplete. Enabling prevents running optimized code and resets execution\ncounters.\n[startPreciseCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-startPreciseCoverage)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartPreciseCoverageParams {
    #[doc = "Collect accurate call counts beyond simple 'covered' or 'not covered'."]
    #[serde(rename = "callCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub call_count: Option<bool>,
    #[doc = "Collect block-based coverage."]
    #[serde(rename = "detailed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub detailed: Option<bool>,
    #[doc = "Allow the backend to send updates on its own initiative"]
    #[serde(rename = "allowTriggeredUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub allow_triggered_updates: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartPreciseCoverageMethod {
    #[serde(rename = "Profiler.startPreciseCoverage")]
    StartPreciseCoverage,
}
impl StartPreciseCoverageMethod {
    pub const IDENTIFIER: &'static str = "Profiler.startPreciseCoverage";
}
#[doc = "Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code\ncoverage may be incomplete. Enabling prevents running optimized code and resets execution\ncounters.\n[startPreciseCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-startPreciseCoverage)"]
#[derive(Debug, Clone, PartialEq)]
pub struct StartPreciseCoverage {
    pub method: StartPreciseCoverageMethod,
    pub params: StartPreciseCoverageParams,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopMethod {
    #[serde(rename = "Profiler.stop")]
    Stop,
}
impl StopMethod {
    pub const IDENTIFIER: &'static str = "Profiler.stop";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Stop {
    pub method: StopMethod,
    pub params: StopParams,
}
#[doc = "Disable precise code coverage. Disabling releases unnecessary execution count records and allows\nexecuting optimized code.\n[stopPreciseCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-stopPreciseCoverage)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopPreciseCoverageParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopPreciseCoverageMethod {
    #[serde(rename = "Profiler.stopPreciseCoverage")]
    StopPreciseCoverage,
}
impl StopPreciseCoverageMethod {
    pub const IDENTIFIER: &'static str = "Profiler.stopPreciseCoverage";
}
#[doc = "Disable precise code coverage. Disabling releases unnecessary execution count records and allows\nexecuting optimized code.\n[stopPreciseCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-stopPreciseCoverage)"]
#[derive(Debug, Clone, PartialEq)]
pub struct StopPreciseCoverage {
    pub method: StopPreciseCoverageMethod,
    pub params: StopPreciseCoverageParams,
}
#[doc = "Collect coverage data for the current isolate, and resets execution counters. Precise code\ncoverage needs to have started.\n[takePreciseCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-takePreciseCoverage)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TakePreciseCoverageParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TakePreciseCoverageMethod {
    #[serde(rename = "Profiler.takePreciseCoverage")]
    TakePreciseCoverage,
}
impl TakePreciseCoverageMethod {
    pub const IDENTIFIER: &'static str = "Profiler.takePreciseCoverage";
}
#[doc = "Collect coverage data for the current isolate, and resets execution counters. Precise code\ncoverage needs to have started.\n[takePreciseCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-takePreciseCoverage)"]
#[derive(Debug, Clone, PartialEq)]
pub struct TakePreciseCoverage {
    pub method: TakePreciseCoverageMethod,
    pub params: TakePreciseCoverageParams,
}
group_enum ! (ProfilerCommands { Disable (Disable) , Enable (Enable) , GetBestEffortCoverage (GetBestEffortCoverage) , SetSamplingInterval (SetSamplingInterval) , Start (Start) , StartPreciseCoverage (StartPreciseCoverage) , Stop (Stop) , StopPreciseCoverage (StopPreciseCoverage) , TakePreciseCoverage (TakePreciseCoverage) });
