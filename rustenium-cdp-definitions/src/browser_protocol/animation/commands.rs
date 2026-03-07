use serde::{Deserialize, Serialize};
#[doc = "Disables animation domain notifications.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Animation.disable")]
    Disable,
}
#[doc = "Disables animation domain notifications.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "Animation.disable";
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables animation domain notifications.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Animation.enable")]
    Enable,
}
#[doc = "Enables animation domain notifications.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "Animation.enable";
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Returns the current time of the an animation.\n[getCurrentTime](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-getCurrentTime)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCurrentTimeParams {
    #[doc = "Id of animation."]
    #[serde(rename = "id")]
    pub id: String,
}
impl GetCurrentTimeParams {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}
impl<T: Into<String>> From<T> for GetCurrentTimeParams {
    fn from(url: T) -> Self {
        GetCurrentTimeParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetCurrentTimeMethod {
    #[serde(rename = "Animation.getCurrentTime")]
    GetCurrentTime,
}
#[doc = "Returns the current time of the an animation.\n[getCurrentTime](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-getCurrentTime)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetCurrentTime {
    pub method: GetCurrentTimeMethod,
    pub params: GetCurrentTimeParams,
}
impl GetCurrentTime {
    pub const IDENTIFIER: &'static str = "Animation.getCurrentTime";
}
impl crate::CommandResult for GetCurrentTime {
    type Result = super::results::GetCurrentTimeResult;
}
#[doc = "Gets the playback rate of the document timeline.\n[getPlaybackRate](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-getPlaybackRate)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlaybackRateParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetPlaybackRateMethod {
    #[serde(rename = "Animation.getPlaybackRate")]
    GetPlaybackRate,
}
#[doc = "Gets the playback rate of the document timeline.\n[getPlaybackRate](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-getPlaybackRate)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetPlaybackRate {
    pub method: GetPlaybackRateMethod,
    pub params: GetPlaybackRateParams,
}
impl GetPlaybackRate {
    pub const IDENTIFIER: &'static str = "Animation.getPlaybackRate";
}
impl crate::CommandResult for GetPlaybackRate {
    type Result = super::results::GetPlaybackRateResult;
}
#[doc = "Releases a set of animations to no longer be manipulated.\n[releaseAnimations](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-releaseAnimations)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseAnimationsParams {
    #[doc = "List of animation ids to seek."]
    #[serde(rename = "animations")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub animations: Vec<String>,
}
impl ReleaseAnimationsParams {
    pub fn new(animations: Vec<String>) -> Self {
        Self { animations }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReleaseAnimationsMethod {
    #[serde(rename = "Animation.releaseAnimations")]
    ReleaseAnimations,
}
#[doc = "Releases a set of animations to no longer be manipulated.\n[releaseAnimations](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-releaseAnimations)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ReleaseAnimations {
    pub method: ReleaseAnimationsMethod,
    pub params: ReleaseAnimationsParams,
}
impl ReleaseAnimations {
    pub const IDENTIFIER: &'static str = "Animation.releaseAnimations";
}
impl crate::CommandResult for ReleaseAnimations {
    type Result = super::results::ReleaseAnimationsResult;
}
#[doc = "Gets the remote object of the Animation.\n[resolveAnimation](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-resolveAnimation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveAnimationParams {
    #[doc = "Animation id."]
    #[serde(rename = "animationId")]
    pub animation_id: String,
}
impl ResolveAnimationParams {
    pub fn new(animation_id: impl Into<String>) -> Self {
        Self {
            animation_id: animation_id.into(),
        }
    }
}
impl<T: Into<String>> From<T> for ResolveAnimationParams {
    fn from(url: T) -> Self {
        ResolveAnimationParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResolveAnimationMethod {
    #[serde(rename = "Animation.resolveAnimation")]
    ResolveAnimation,
}
#[doc = "Gets the remote object of the Animation.\n[resolveAnimation](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-resolveAnimation)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ResolveAnimation {
    pub method: ResolveAnimationMethod,
    pub params: ResolveAnimationParams,
}
impl ResolveAnimation {
    pub const IDENTIFIER: &'static str = "Animation.resolveAnimation";
}
impl crate::CommandResult for ResolveAnimation {
    type Result = super::results::ResolveAnimationResult;
}
#[doc = "Seek a set of animations to a particular time within each animation.\n[seekAnimations](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-seekAnimations)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SeekAnimationsParams {
    #[doc = "List of animation ids to seek."]
    #[serde(rename = "animations")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub animations: Vec<String>,
    #[doc = "Set the current time of each animation."]
    #[serde(rename = "currentTime")]
    pub current_time: f64,
}
impl SeekAnimationsParams {
    pub fn new(animations: Vec<String>, current_time: impl Into<f64>) -> Self {
        Self {
            animations,
            current_time: current_time.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SeekAnimationsMethod {
    #[serde(rename = "Animation.seekAnimations")]
    SeekAnimations,
}
#[doc = "Seek a set of animations to a particular time within each animation.\n[seekAnimations](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-seekAnimations)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SeekAnimations {
    pub method: SeekAnimationsMethod,
    pub params: SeekAnimationsParams,
}
impl SeekAnimations {
    pub const IDENTIFIER: &'static str = "Animation.seekAnimations";
}
impl crate::CommandResult for SeekAnimations {
    type Result = super::results::SeekAnimationsResult;
}
#[doc = "Sets the paused state of a set of animations.\n[setPaused](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-setPaused)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPausedParams {
    #[doc = "Animations to set the pause state of."]
    #[serde(rename = "animations")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub animations: Vec<String>,
    #[doc = "Paused state to set to."]
    #[serde(rename = "paused")]
    pub paused: bool,
}
impl SetPausedParams {
    pub fn new(animations: Vec<String>, paused: impl Into<bool>) -> Self {
        Self {
            animations,
            paused: paused.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetPausedMethod {
    #[serde(rename = "Animation.setPaused")]
    SetPaused,
}
#[doc = "Sets the paused state of a set of animations.\n[setPaused](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-setPaused)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetPaused {
    pub method: SetPausedMethod,
    pub params: SetPausedParams,
}
impl SetPaused {
    pub const IDENTIFIER: &'static str = "Animation.setPaused";
}
impl crate::CommandResult for SetPaused {
    type Result = super::results::SetPausedResult;
}
#[doc = "Sets the playback rate of the document timeline.\n[setPlaybackRate](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-setPlaybackRate)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPlaybackRateParams {
    #[doc = "Playback rate for animations on page"]
    #[serde(rename = "playbackRate")]
    pub playback_rate: f64,
}
impl SetPlaybackRateParams {
    pub fn new(playback_rate: impl Into<f64>) -> Self {
        Self {
            playback_rate: playback_rate.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetPlaybackRateMethod {
    #[serde(rename = "Animation.setPlaybackRate")]
    SetPlaybackRate,
}
#[doc = "Sets the playback rate of the document timeline.\n[setPlaybackRate](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-setPlaybackRate)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetPlaybackRate {
    pub method: SetPlaybackRateMethod,
    pub params: SetPlaybackRateParams,
}
impl SetPlaybackRate {
    pub const IDENTIFIER: &'static str = "Animation.setPlaybackRate";
}
impl crate::CommandResult for SetPlaybackRate {
    type Result = super::results::SetPlaybackRateResult;
}
#[doc = "Sets the timing of an animation node.\n[setTiming](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-setTiming)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetTimingParams {
    #[doc = "Animation id."]
    #[serde(rename = "animationId")]
    pub animation_id: String,
    #[doc = "Duration of the animation."]
    #[serde(rename = "duration")]
    pub duration: f64,
    #[doc = "Delay of the animation."]
    #[serde(rename = "delay")]
    pub delay: f64,
}
impl SetTimingParams {
    pub fn new(
        animation_id: impl Into<String>,
        duration: impl Into<f64>,
        delay: impl Into<f64>,
    ) -> Self {
        Self {
            animation_id: animation_id.into(),
            duration: duration.into(),
            delay: delay.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetTimingMethod {
    #[serde(rename = "Animation.setTiming")]
    SetTiming,
}
#[doc = "Sets the timing of an animation node.\n[setTiming](https://chromedevtools.github.io/devtools-protocol/tot/Animation/#method-setTiming)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetTiming {
    pub method: SetTimingMethod,
    pub params: SetTimingParams,
}
impl SetTiming {
    pub const IDENTIFIER: &'static str = "Animation.setTiming";
}
impl crate::CommandResult for SetTiming {
    type Result = super::results::SetTimingResult;
}
group_enum ! (AnimationCommands { Disable (Disable) , Enable (Enable) , GetCurrentTime (GetCurrentTime) , GetPlaybackRate (GetPlaybackRate) , ReleaseAnimations (ReleaseAnimations) , ResolveAnimation (ResolveAnimation) , SeekAnimations (SeekAnimations) , SetPaused (SetPaused) , SetPlaybackRate (SetPlaybackRate) , SetTiming (SetTiming) });
