use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsoleProfileFinished {
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "Location of console.profileEnd()."]
    #[serde(rename = "location")]
    pub location: super::super::debugger::types::Location,
    #[serde(rename = "profile")]
    pub profile: super::types::Profile,
    #[doc = "Profile title passed as an argument to console.profile()."]
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,
}
impl ConsoleProfileFinished {
    pub const IDENTIFIER: &'static str = "Profiler.consoleProfileFinished";
}
#[doc = "Sent when new profile recording is started using console.profile() call.\n[consoleProfileStarted](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#event-consoleProfileStarted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsoleProfileStarted {
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "Location of console.profile()."]
    #[serde(rename = "location")]
    pub location: super::super::debugger::types::Location,
    #[doc = "Profile title passed as an argument to console.profile()."]
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,
}
impl ConsoleProfileStarted {
    pub const IDENTIFIER: &'static str = "Profiler.consoleProfileStarted";
}
#[doc = "Reports coverage delta since the last poll (either from an event like this, or from\n`takePreciseCoverage` for the current isolate. May only be sent if precise code\ncoverage has been started. This event can be trigged by the embedder to, for example,\ntrigger collection of coverage data immediately at a certain point in time.\n[preciseCoverageDeltaUpdate](https://chromedevtools.github.io/devtools-protocol/tot/Profiler/#event-preciseCoverageDeltaUpdate)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PreciseCoverageDeltaUpdate {
    #[doc = "Monotonically increasing time (in seconds) when the coverage update was taken in the backend."]
    #[serde(rename = "timestamp")]
    pub timestamp: f64,
    #[doc = "Identifier for distinguishing coverage events."]
    #[serde(rename = "occasion")]
    pub occasion: String,
    #[doc = "Coverage data for the current isolate."]
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<super::types::ScriptCoverage>,
}
impl PreciseCoverageDeltaUpdate {
    pub const IDENTIFIER: &'static str = "Profiler.preciseCoverageDeltaUpdate";
}
group_enum ! (ProfilerEvents { ConsoleProfileFinished (ConsoleProfileFinished) , ConsoleProfileStarted (ConsoleProfileStarted) , PreciseCoverageDeltaUpdate (PreciseCoverageDeltaUpdate) });
