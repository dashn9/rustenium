use serde::{Deserialize, Serialize};
#[doc = "See https://github.com/WICG/LargestContentfulPaint and largest_contentful_paint.idl\n[LargestContentfulPaint](https://chromedevtools.github.io/devtools-protocol/tot/PerformanceTimeline/#type-LargestContentfulPaint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LargestContentfulPaint {
    #[serde(rename = "renderTime")]
    pub render_time: super::super::network::types::TimeSinceEpoch,
    #[serde(rename = "loadTime")]
    pub load_time: super::super::network::types::TimeSinceEpoch,
    #[doc = "The number of pixels being painted."]
    #[serde(rename = "size")]
    pub size: f64,
    #[doc = "The id attribute of the element, if available."]
    #[serde(rename = "elementId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub element_id: Option<String>,
    #[doc = "The URL of the image (may be trimmed)."]
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<super::super::dom::types::BackendNodeId>,
}
impl LargestContentfulPaint {
    pub fn new(
        render_time: impl Into<super::super::network::types::TimeSinceEpoch>,
        load_time: impl Into<super::super::network::types::TimeSinceEpoch>,
        size: impl Into<f64>,
    ) -> Self {
        Self {
            render_time: render_time.into(),
            load_time: load_time.into(),
            size: size.into(),
            element_id: None,
            url: None,
            node_id: None,
        }
    }
}
impl LargestContentfulPaint {
    pub const IDENTIFIER: &'static str = "PerformanceTimeline.LargestContentfulPaint";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutShiftAttribution {
    #[serde(rename = "previousRect")]
    pub previous_rect: super::super::dom::types::Rect,
    #[serde(rename = "currentRect")]
    pub current_rect: super::super::dom::types::Rect,
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<super::super::dom::types::BackendNodeId>,
}
impl LayoutShiftAttribution {
    pub fn new(
        previous_rect: impl Into<super::super::dom::types::Rect>,
        current_rect: impl Into<super::super::dom::types::Rect>,
    ) -> Self {
        Self {
            previous_rect: previous_rect.into(),
            current_rect: current_rect.into(),
            node_id: None,
        }
    }
}
impl LayoutShiftAttribution {
    pub const IDENTIFIER: &'static str = "PerformanceTimeline.LayoutShiftAttribution";
}
#[doc = "See https://wicg.github.io/layout-instability/#sec-layout-shift and layout_shift.idl\n[LayoutShift](https://chromedevtools.github.io/devtools-protocol/tot/PerformanceTimeline/#type-LayoutShift)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LayoutShift {
    #[doc = "Score increment produced by this event."]
    #[serde(rename = "value")]
    pub value: f64,
    #[serde(rename = "hadRecentInput")]
    pub had_recent_input: bool,
    #[serde(rename = "lastInputTime")]
    pub last_input_time: super::super::network::types::TimeSinceEpoch,
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<LayoutShiftAttribution>,
}
impl LayoutShift {
    pub fn new(
        value: impl Into<f64>,
        had_recent_input: impl Into<bool>,
        last_input_time: impl Into<super::super::network::types::TimeSinceEpoch>,
        sources: Vec<LayoutShiftAttribution>,
    ) -> Self {
        Self {
            value: value.into(),
            had_recent_input: had_recent_input.into(),
            last_input_time: last_input_time.into(),
            sources,
        }
    }
}
impl LayoutShift {
    pub const IDENTIFIER: &'static str = "PerformanceTimeline.LayoutShift";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimelineEvent {
    #[doc = "Identifies the frame that this event is related to. Empty for non-frame targets."]
    #[serde(rename = "frameId")]
    pub frame_id: super::super::page::types::FrameId,
    #[doc = "The event type, as specified in https://w3c.github.io/performance-timeline/#dom-performanceentry-entrytype\nThis determines which of the optional \"details\" fields is present."]
    #[serde(rename = "type")]
    pub r#type: String,
    #[doc = "Name may be empty depending on the type."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Time in seconds since Epoch, monotonically increasing within document lifetime."]
    #[serde(rename = "time")]
    pub time: super::super::network::types::TimeSinceEpoch,
    #[doc = "Event duration, if applicable."]
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub duration: Option<f64>,
    #[serde(rename = "lcpDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub lcp_details: Option<LargestContentfulPaint>,
    #[serde(rename = "layoutShiftDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub layout_shift_details: Option<LayoutShift>,
}
impl TimelineEvent {
    pub fn new(
        frame_id: impl Into<super::super::page::types::FrameId>,
        r#type: impl Into<String>,
        name: impl Into<String>,
        time: impl Into<super::super::network::types::TimeSinceEpoch>,
    ) -> Self {
        Self {
            frame_id: frame_id.into(),
            r#type: r#type.into(),
            name: name.into(),
            time: time.into(),
            duration: None,
            lcp_details: None,
            layout_shift_details: None,
        }
    }
}
impl TimelineEvent {
    pub const IDENTIFIER: &'static str = "PerformanceTimeline.TimelineEvent";
}
group_enum ! (Type { LargestContentfulPaint (LargestContentfulPaint) , LayoutShiftAttribution (LayoutShiftAttribution) , LayoutShift (LayoutShift) , TimelineEvent (TimelineEvent) });
