use serde::{Deserialize, Serialize};
#[doc = "Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a\nscreenshot from the resulting frame. Requires that the target was created with enabled\nBeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also\nhttps://goo.gle/chrome-headless-rendering for more background.\n[beginFrame](https://chromedevtools.github.io/devtools-protocol/tot/HeadlessExperimental/#method-beginFrame)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BeginFrameParams {
    #[doc = "Timestamp of this BeginFrame in Renderer TimeTicks (milliseconds of uptime). If not set,\nthe current time will be used."]
    #[serde(rename = "frameTimeTicks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_time_ticks: Option<f64>,
    #[doc = "The interval between BeginFrames that is reported to the compositor, in milliseconds.\nDefaults to a 60 frames/second interval, i.e. about 16.666 milliseconds."]
    #[serde(rename = "interval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub interval: Option<f64>,
    #[doc = "Whether updates should not be committed and drawn onto the display. False by default. If\ntrue, only side effects of the BeginFrame will be run, such as layout and animations, but\nany visual updates may not be visible on the display or in screenshots."]
    #[serde(rename = "noDisplayUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub no_display_updates: Option<bool>,
    #[doc = "If set, a screenshot of the frame will be captured and returned in the response. Otherwise,\nno screenshot will be captured. Note that capturing a screenshot can fail, for example,\nduring renderer initialization. In such a case, no screenshot data will be returned."]
    #[serde(rename = "screenshot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub screenshot: Option<super::types::ScreenshotParams>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BeginFrameMethod {
    #[serde(rename = "HeadlessExperimental.beginFrame")]
    BeginFrame,
}
impl BeginFrameMethod {
    pub const IDENTIFIER: &'static str = "HeadlessExperimental.beginFrame";
}
#[doc = "Sends a BeginFrame to the target and returns when the frame was completed. Optionally captures a\nscreenshot from the resulting frame. Requires that the target was created with enabled\nBeginFrameControl. Designed for use with --run-all-compositor-stages-before-draw, see also\nhttps://goo.gle/chrome-headless-rendering for more background.\n[beginFrame](https://chromedevtools.github.io/devtools-protocol/tot/HeadlessExperimental/#method-beginFrame)"]
#[derive(Debug, Clone, PartialEq)]
pub struct BeginFrame {
    pub method: BeginFrameMethod,
    pub params: BeginFrameParams,
}
impl super::super::super::CommandResult for BeginFrame {
    type Result = super::results::BeginFrameResult;
}
group_enum ! (HeadlessExperimentalCommands { BeginFrame (BeginFrame) });
