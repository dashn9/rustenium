use serde::{Deserialize, Serialize};
#[doc = "Sent when a performance timeline event is added. See reportPerformanceTimeline method.\n[timelineEventAdded](https://chromedevtools.github.io/devtools-protocol/tot/PerformanceTimeline/#event-timelineEventAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineEventAddedParams {
    #[serde(rename = "event")]
    pub event: super::types::TimelineEvent,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimelineEventAddedMethod {
    #[serde(rename = "PerformanceTimeline.timelineEventAdded")]
    TimelineEventAdded,
}
#[doc = "Sent when a performance timeline event is added. See reportPerformanceTimeline method.\n[timelineEventAdded](https://chromedevtools.github.io/devtools-protocol/tot/PerformanceTimeline/#event-timelineEventAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineEventAdded {
    pub method: TimelineEventAddedMethod,
    pub params: TimelineEventAddedParams,
}
impl TimelineEventAdded {
    pub const IDENTIFIER: &'static str = "PerformanceTimeline.timelineEventAdded";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (PerformanceTimelineEvents { TimelineEventAdded (TimelineEventAdded) } + identifiable);
