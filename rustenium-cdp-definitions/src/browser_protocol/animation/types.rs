use serde::{Deserialize, Serialize};
#[doc = "Animation instance.\n[Animation](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-Animation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Animation {
    #[doc = "`Animation`'s id."]
    #[serde(rename = "id")]
    pub id: String,
    #[doc = "`Animation`'s name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "`Animation`'s internal paused state."]
    #[serde(rename = "pausedState")]
    pub paused_state: bool,
    #[doc = "`Animation`'s play state."]
    #[serde(rename = "playState")]
    pub play_state: String,
    #[doc = "`Animation`'s playback rate."]
    #[serde(rename = "playbackRate")]
    pub playback_rate: f64,
    #[doc = "`Animation`'s start time.\nMilliseconds for time based animations and\npercentage [0 - 100] for scroll driven animations\n(i.e. when viewOrScrollTimeline exists)."]
    #[serde(rename = "startTime")]
    pub start_time: f64,
    #[doc = "`Animation`'s current time."]
    #[serde(rename = "currentTime")]
    pub current_time: f64,
    #[doc = "Animation type of `Animation`."]
    #[serde(rename = "type")]
    pub r#type: AnimationType,
    #[doc = "`Animation`'s source animation node."]
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source: Option<AnimationEffect>,
    #[doc = "A unique ID for `Animation` representing the sources that triggered this CSS\nanimation/transition."]
    #[serde(rename = "cssId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub css_id: Option<String>,
    #[doc = "View or scroll timeline"]
    #[serde(rename = "viewOrScrollTimeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub view_or_scroll_timeline: Option<ViewOrScrollTimeline>,
}
#[doc = "Animation type of `Animation`."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AnimationType {
    #[serde(rename = "CSSTransition")]
    CssTransition,
    #[serde(rename = "CSSAnimation")]
    CssAnimation,
    #[serde(rename = "WebAnimation")]
    WebAnimation,
}
impl Animation {
    pub const IDENTIFIER: &'static str = "Animation.Animation";
}
#[doc = "Timeline instance\n[ViewOrScrollTimeline](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-ViewOrScrollTimeline)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewOrScrollTimeline {
    #[doc = "Scroll container node"]
    #[serde(rename = "sourceNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_node_id: Option<super::super::dom::types::BackendNodeId>,
    #[doc = "Represents the starting scroll position of the timeline\nas a length offset in pixels from scroll origin."]
    #[serde(rename = "startOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub start_offset: Option<f64>,
    #[doc = "Represents the ending scroll position of the timeline\nas a length offset in pixels from scroll origin."]
    #[serde(rename = "endOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub end_offset: Option<f64>,
    #[doc = "The element whose principal box's visibility in the\nscrollport defined the progress of the timeline.\nDoes not exist for animations with ScrollTimeline"]
    #[serde(rename = "subjectNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub subject_node_id: Option<super::super::dom::types::BackendNodeId>,
    #[doc = "Orientation of the scroll"]
    #[serde(rename = "axis")]
    pub axis: super::super::dom::types::ScrollOrientation,
}
impl ViewOrScrollTimeline {
    pub fn new(axis: impl Into<super::super::dom::types::ScrollOrientation>) -> Self {
        Self {
            axis: axis.into(),
            source_node_id: None,
            start_offset: None,
            end_offset: None,
            subject_node_id: None,
        }
    }
}
impl ViewOrScrollTimeline {
    pub const IDENTIFIER: &'static str = "Animation.ViewOrScrollTimeline";
}
#[doc = "AnimationEffect instance\n[AnimationEffect](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-AnimationEffect)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationEffect {
    #[doc = "`AnimationEffect`'s delay."]
    #[serde(rename = "delay")]
    pub delay: f64,
    #[doc = "`AnimationEffect`'s end delay."]
    #[serde(rename = "endDelay")]
    pub end_delay: f64,
    #[doc = "`AnimationEffect`'s iteration start."]
    #[serde(rename = "iterationStart")]
    pub iteration_start: f64,
    #[doc = "`AnimationEffect`'s iterations. Omitted if the value is infinite."]
    #[serde(rename = "iterations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub iterations: Option<f64>,
    #[doc = "`AnimationEffect`'s iteration duration.\nMilliseconds for time based animations and\npercentage [0 - 100] for scroll driven animations\n(i.e. when viewOrScrollTimeline exists)."]
    #[serde(rename = "duration")]
    pub duration: f64,
    #[doc = "`AnimationEffect`'s playback direction."]
    #[serde(rename = "direction")]
    pub direction: String,
    #[doc = "`AnimationEffect`'s fill mode."]
    #[serde(rename = "fill")]
    pub fill: String,
    #[doc = "`AnimationEffect`'s target node."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<super::super::dom::types::BackendNodeId>,
    #[doc = "`AnimationEffect`'s keyframes."]
    #[serde(rename = "keyframesRule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub keyframes_rule: Option<KeyframesRule>,
    #[doc = "`AnimationEffect`'s timing function."]
    #[serde(rename = "easing")]
    pub easing: String,
}
impl AnimationEffect {
    pub const IDENTIFIER: &'static str = "Animation.AnimationEffect";
}
#[doc = "Keyframes Rule\n[KeyframesRule](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-KeyframesRule)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyframesRule {
    #[doc = "CSS keyframed animation's name."]
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[doc = "List of animation keyframes."]
    #[serde(rename = "keyframes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub keyframes: Vec<KeyframeStyle>,
}
impl KeyframesRule {
    pub fn new(keyframes: Vec<KeyframeStyle>) -> Self {
        Self {
            keyframes,
            name: None,
        }
    }
}
impl KeyframesRule {
    pub const IDENTIFIER: &'static str = "Animation.KeyframesRule";
}
#[doc = "Keyframe Style\n[KeyframeStyle](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#type-KeyframeStyle)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyframeStyle {
    #[doc = "Keyframe's time offset."]
    #[serde(rename = "offset")]
    pub offset: String,
    #[doc = "`AnimationEffect`'s timing function."]
    #[serde(rename = "easing")]
    pub easing: String,
}
impl KeyframeStyle {
    pub fn new(offset: impl Into<String>, easing: impl Into<String>) -> Self {
        Self {
            offset: offset.into(),
            easing: easing.into(),
        }
    }
}
impl KeyframeStyle {
    pub const IDENTIFIER: &'static str = "Animation.KeyframeStyle";
}
group_enum ! (Type { Animation (Animation) , ViewOrScrollTimeline (ViewOrScrollTimeline) , AnimationEffect (AnimationEffect) , KeyframesRule (KeyframesRule) , KeyframeStyle (KeyframeStyle) });
