use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Profiler.disable")]
    Disable,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "Profiler.disable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Profiler.enable")]
    Enable,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "Profiler.enable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Collect coverage data for the current isolate. The coverage data may be incomplete due to\ngarbage collection.\n[getBestEffortCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-getBestEffortCoverage)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBestEffortCoverageParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetBestEffortCoverageMethod {
    #[serde(rename = "Profiler.getBestEffortCoverage")]
    GetBestEffortCoverage,
}
#[doc = "Collect coverage data for the current isolate. The coverage data may be incomplete due to\ngarbage collection.\n[getBestEffortCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-getBestEffortCoverage)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBestEffortCoverage {
    pub method: GetBestEffortCoverageMethod,
    pub params: GetBestEffortCoverageParams,
}
impl GetBestEffortCoverage {
    pub const IDENTIFIER: &'static str = "Profiler.getBestEffortCoverage";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetBestEffortCoverage {
    type Result = super::results::GetBestEffortCoverageResult;
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
#[doc = "Changes CPU profiler sampling interval. Must be called before CPU profiles recording started.\n[setSamplingInterval](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-setSamplingInterval)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSamplingInterval {
    pub method: SetSamplingIntervalMethod,
    pub params: SetSamplingIntervalParams,
}
impl SetSamplingInterval {
    pub const IDENTIFIER: &'static str = "Profiler.setSamplingInterval";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetSamplingInterval {
    type Result = super::results::SetSamplingIntervalResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartMethod {
    #[serde(rename = "Profiler.start")]
    Start,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Start {
    pub method: StartMethod,
    pub params: StartParams,
}
impl Start {
    pub const IDENTIFIER: &'static str = "Profiler.start";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Start {
    type Result = super::results::StartResult;
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
#[doc = "Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code\ncoverage may be incomplete. Enabling prevents running optimized code and resets execution\ncounters.\n[startPreciseCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-startPreciseCoverage)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartPreciseCoverage {
    pub method: StartPreciseCoverageMethod,
    pub params: StartPreciseCoverageParams,
}
impl StartPreciseCoverage {
    pub const IDENTIFIER: &'static str = "Profiler.startPreciseCoverage";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StartPreciseCoverage {
    type Result = super::results::StartPreciseCoverageResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopMethod {
    #[serde(rename = "Profiler.stop")]
    Stop,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stop {
    pub method: StopMethod,
    pub params: StopParams,
}
impl Stop {
    pub const IDENTIFIER: &'static str = "Profiler.stop";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Stop {
    type Result = super::results::StopResult;
}
#[doc = "Disable precise code coverage. Disabling releases unnecessary execution count records and allows\nexecuting optimized code.\n[stopPreciseCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-stopPreciseCoverage)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopPreciseCoverageParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopPreciseCoverageMethod {
    #[serde(rename = "Profiler.stopPreciseCoverage")]
    StopPreciseCoverage,
}
#[doc = "Disable precise code coverage. Disabling releases unnecessary execution count records and allows\nexecuting optimized code.\n[stopPreciseCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-stopPreciseCoverage)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopPreciseCoverage {
    pub method: StopPreciseCoverageMethod,
    pub params: StopPreciseCoverageParams,
}
impl StopPreciseCoverage {
    pub const IDENTIFIER: &'static str = "Profiler.stopPreciseCoverage";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StopPreciseCoverage {
    type Result = super::results::StopPreciseCoverageResult;
}
#[doc = "Collect coverage data for the current isolate, and resets execution counters. Precise code\ncoverage needs to have started.\n[takePreciseCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-takePreciseCoverage)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakePreciseCoverageParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TakePreciseCoverageMethod {
    #[serde(rename = "Profiler.takePreciseCoverage")]
    TakePreciseCoverage,
}
#[doc = "Collect coverage data for the current isolate, and resets execution counters. Precise code\ncoverage needs to have started.\n[takePreciseCoverage](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#method-takePreciseCoverage)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TakePreciseCoverage {
    pub method: TakePreciseCoverageMethod,
    pub params: TakePreciseCoverageParams,
}
impl TakePreciseCoverage {
    pub const IDENTIFIER: &'static str = "Profiler.takePreciseCoverage";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for TakePreciseCoverage {
    type Result = super::results::TakePreciseCoverageResult;
}
group_enum ! (ProfilerCommands { Disable (Disable) , Enable (Enable) , GetBestEffortCoverage (GetBestEffortCoverage) , SetSamplingInterval (SetSamplingInterval) , Start (Start) , StartPreciseCoverage (StartPreciseCoverage) , Stop (Stop) , StopPreciseCoverage (StopPreciseCoverage) , TakePreciseCoverage (TakePreciseCoverage) });
