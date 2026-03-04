use serde::{Deserialize, Serialize};
#[doc = "Sent when a performance timeline event is added. See reportPerformanceTimeline method.\n[timelineEventAdded](https://chromedevtools.github.io/devtools-protocol/tot/PerformanceTimeline/#event-timelineEventAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineEventAdded {
    #[serde(rename = "event")]
    pub event: super::types::TimelineEvent,
}
impl TimelineEventAdded {
    pub const IDENTIFIER: &'static str = "PerformanceTimeline.timelineEventAdded";
}
group_enum ! (Event { TimelineEventAdded (TimelineEventAdded) });
