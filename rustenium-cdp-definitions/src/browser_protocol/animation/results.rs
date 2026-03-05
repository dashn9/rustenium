use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ReleaseAnimationsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SeekAnimationsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPausedResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPlaybackRateResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetTimingResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCurrentTimeResult {
    #[doc = "Current time of the page."]
    #[serde(rename = "currentTime")]
    pub current_time: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlaybackRateResult {
    #[doc = "Playback rate for animations on page."]
    #[serde(rename = "playbackRate")]
    pub playback_rate: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveAnimationResult {
    #[doc = "Corresponding remote object."]
    #[serde(rename = "remoteObject")]
    pub remote_object: super::super::super::js_protocol::runtime::types::RemoteObject,
}
