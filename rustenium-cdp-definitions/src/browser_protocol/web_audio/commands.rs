use serde::{Deserialize, Serialize};
#[doc = "Enables the WebAudio domain and starts sending context lifetime events.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "WebAudio.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.enable";
}
#[doc = "Enables the WebAudio domain and starts sending context lifetime events.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
#[doc = "Disables the WebAudio domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "WebAudio.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.disable";
}
#[doc = "Disables the WebAudio domain.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
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
impl GetRealtimeDataMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.getRealtimeData";
}
#[doc = "Fetch the realtime data from the registered contexts.\n[getRealtimeData](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#method-getRealtimeData)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetRealtimeData {
    pub method: GetRealtimeDataMethod,
    pub params: GetRealtimeDataParams,
}
group_enum ! (Command { Enable (Enable) , Disable (Disable) , GetRealtimeData (GetRealtimeData) });
