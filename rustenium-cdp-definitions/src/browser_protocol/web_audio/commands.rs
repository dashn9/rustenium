use serde::{Deserialize, Serialize};
#[doc = "Enables the WebAudio domain and starts sending context lifetime events.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "WebAudio.enable")]
    Enable,
}
#[doc = "Enables the WebAudio domain and starts sending context lifetime events.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "WebAudio.enable";
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Disables the WebAudio domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "WebAudio.disable")]
    Disable,
}
#[doc = "Disables the WebAudio domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "WebAudio.disable";
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Fetch the realtime data from the registered contexts.\n[getRealtimeData](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#method-getRealtimeData)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRealtimeDataParams {
    #[serde(rename = "contextId")]
    pub context_id: super::types::GraphObjectId,
}
impl GetRealtimeDataParams {
    pub fn new(context_id: impl Into<super::types::GraphObjectId>) -> Self {
        Self {
            context_id: context_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetRealtimeDataMethod {
    #[serde(rename = "WebAudio.getRealtimeData")]
    GetRealtimeData,
}
#[doc = "Fetch the realtime data from the registered contexts.\n[getRealtimeData](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#method-getRealtimeData)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRealtimeData {
    pub method: GetRealtimeDataMethod,
    pub params: GetRealtimeDataParams,
}
impl GetRealtimeData {
    pub const IDENTIFIER: &'static str = "WebAudio.getRealtimeData";
}
impl crate::CommandResult for GetRealtimeData {
    type Result = super::results::GetRealtimeDataResult;
}
group_enum ! (WebAudioCommands { Enable (Enable) , Disable (Disable) , GetRealtimeData (GetRealtimeData) });
