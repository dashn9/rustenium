use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCurrentTimeReturns {
    #[doc = "Current time of the page."]
    #[serde(rename = "currentTime")]
    pub current_time: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPlaybackRateReturns {
    #[doc = "Playback rate for animations on page."]
    #[serde(rename = "playbackRate")]
    pub playback_rate: f64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolveAnimationReturns {
    #[doc = "Corresponding remote object."]
    #[serde(rename = "remoteObject")]
    pub remote_object: super::super::super::js_protocol::runtime::types::RemoteObject,
}
