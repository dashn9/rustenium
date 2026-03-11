use serde::{Deserialize, Serialize};
#[doc = "Previously buffered events would be reported before method returns.\nSee also: timelineEventAdded\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/PerformanceTimeline/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {
    #[doc = "The types of event to report, as specified in\nhttps://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype\nThe specified filter overrides any previous filters, passing empty\nfilter disables recording.\nNote that not all types exposed to the web platform are currently supported."]
    #[serde(rename = "eventTypes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub event_types: Vec<String>,
}
impl EnableParams {
    pub fn new(event_types: Vec<String>) -> Self {
        Self { event_types }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "PerformanceTimeline.enable")]
    Enable,
}
#[doc = "Previously buffered events would be reported before method returns.\nSee also: timelineEventAdded\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/PerformanceTimeline/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "PerformanceTimeline.enable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
group_enum ! (PerformanceTimelineCommands { Enable (Enable) } + identifiable);
