use serde::{Deserialize, Serialize};
#[doc = "Notifies that a new BaseAudioContext has been created.\n[contextCreated](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-contextCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextCreatedParams {
    #[serde(rename = "context")]
    pub context: super::types::BaseAudioContext,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContextCreatedMethod {
    #[serde(rename = "WebAudio.contextCreated")]
    ContextCreated,
}
impl ContextCreatedMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.contextCreated";
}
#[doc = "Notifies that a new BaseAudioContext has been created.\n[contextCreated](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-contextCreated)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ContextCreated {
    pub method: ContextCreatedMethod,
    pub params: ContextCreatedParams,
}
#[doc = "Notifies that an existing BaseAudioContext will be destroyed.\n[contextWillBeDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-contextWillBeDestroyed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextWillBeDestroyedParams {
    #[serde(rename = "contextId")]
    pub context_id: super::types::GraphObjectId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContextWillBeDestroyedMethod {
    #[serde(rename = "WebAudio.contextWillBeDestroyed")]
    ContextWillBeDestroyed,
}
impl ContextWillBeDestroyedMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.contextWillBeDestroyed";
}
#[doc = "Notifies that an existing BaseAudioContext will be destroyed.\n[contextWillBeDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-contextWillBeDestroyed)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ContextWillBeDestroyed {
    pub method: ContextWillBeDestroyedMethod,
    pub params: ContextWillBeDestroyedParams,
}
#[doc = "Notifies that existing BaseAudioContext has changed some properties (id stays the same)..\n[contextChanged](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-contextChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextChangedParams {
    #[serde(rename = "context")]
    pub context: super::types::BaseAudioContext,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContextChangedMethod {
    #[serde(rename = "WebAudio.contextChanged")]
    ContextChanged,
}
impl ContextChangedMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.contextChanged";
}
#[doc = "Notifies that existing BaseAudioContext has changed some properties (id stays the same)..\n[contextChanged](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-contextChanged)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ContextChanged {
    pub method: ContextChangedMethod,
    pub params: ContextChangedParams,
}
#[doc = "Notifies that the construction of an AudioListener has finished.\n[audioListenerCreated](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioListenerCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioListenerCreatedParams {
    #[serde(rename = "listener")]
    pub listener: super::types::AudioListener,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioListenerCreatedMethod {
    #[serde(rename = "WebAudio.audioListenerCreated")]
    AudioListenerCreated,
}
impl AudioListenerCreatedMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.audioListenerCreated";
}
#[doc = "Notifies that the construction of an AudioListener has finished.\n[audioListenerCreated](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioListenerCreated)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AudioListenerCreated {
    pub method: AudioListenerCreatedMethod,
    pub params: AudioListenerCreatedParams,
}
#[doc = "Notifies that a new AudioListener has been created.\n[audioListenerWillBeDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioListenerWillBeDestroyed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioListenerWillBeDestroyedParams {
    #[serde(rename = "contextId")]
    pub context_id: super::types::GraphObjectId,
    #[serde(rename = "listenerId")]
    pub listener_id: super::types::GraphObjectId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioListenerWillBeDestroyedMethod {
    #[serde(rename = "WebAudio.audioListenerWillBeDestroyed")]
    AudioListenerWillBeDestroyed,
}
impl AudioListenerWillBeDestroyedMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.audioListenerWillBeDestroyed";
}
#[doc = "Notifies that a new AudioListener has been created.\n[audioListenerWillBeDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioListenerWillBeDestroyed)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AudioListenerWillBeDestroyed {
    pub method: AudioListenerWillBeDestroyedMethod,
    pub params: AudioListenerWillBeDestroyedParams,
}
#[doc = "Notifies that a new AudioNode has been created.\n[audioNodeCreated](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioNodeCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioNodeCreatedParams {
    #[serde(rename = "node")]
    pub node: super::types::AudioNode,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioNodeCreatedMethod {
    #[serde(rename = "WebAudio.audioNodeCreated")]
    AudioNodeCreated,
}
impl AudioNodeCreatedMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.audioNodeCreated";
}
#[doc = "Notifies that a new AudioNode has been created.\n[audioNodeCreated](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioNodeCreated)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AudioNodeCreated {
    pub method: AudioNodeCreatedMethod,
    pub params: AudioNodeCreatedParams,
}
#[doc = "Notifies that an existing AudioNode has been destroyed.\n[audioNodeWillBeDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioNodeWillBeDestroyed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioNodeWillBeDestroyedParams {
    #[serde(rename = "contextId")]
    pub context_id: super::types::GraphObjectId,
    #[serde(rename = "nodeId")]
    pub node_id: super::types::GraphObjectId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioNodeWillBeDestroyedMethod {
    #[serde(rename = "WebAudio.audioNodeWillBeDestroyed")]
    AudioNodeWillBeDestroyed,
}
impl AudioNodeWillBeDestroyedMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.audioNodeWillBeDestroyed";
}
#[doc = "Notifies that an existing AudioNode has been destroyed.\n[audioNodeWillBeDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioNodeWillBeDestroyed)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AudioNodeWillBeDestroyed {
    pub method: AudioNodeWillBeDestroyedMethod,
    pub params: AudioNodeWillBeDestroyedParams,
}
#[doc = "Notifies that a new AudioParam has been created.\n[audioParamCreated](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioParamCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioParamCreatedParams {
    #[serde(rename = "param")]
    pub param: super::types::AudioParam,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioParamCreatedMethod {
    #[serde(rename = "WebAudio.audioParamCreated")]
    AudioParamCreated,
}
impl AudioParamCreatedMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.audioParamCreated";
}
#[doc = "Notifies that a new AudioParam has been created.\n[audioParamCreated](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioParamCreated)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AudioParamCreated {
    pub method: AudioParamCreatedMethod,
    pub params: AudioParamCreatedParams,
}
#[doc = "Notifies that an existing AudioParam has been destroyed.\n[audioParamWillBeDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioParamWillBeDestroyed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioParamWillBeDestroyedParams {
    #[serde(rename = "contextId")]
    pub context_id: super::types::GraphObjectId,
    #[serde(rename = "nodeId")]
    pub node_id: super::types::GraphObjectId,
    #[serde(rename = "paramId")]
    pub param_id: super::types::GraphObjectId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AudioParamWillBeDestroyedMethod {
    #[serde(rename = "WebAudio.audioParamWillBeDestroyed")]
    AudioParamWillBeDestroyed,
}
impl AudioParamWillBeDestroyedMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.audioParamWillBeDestroyed";
}
#[doc = "Notifies that an existing AudioParam has been destroyed.\n[audioParamWillBeDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioParamWillBeDestroyed)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AudioParamWillBeDestroyed {
    pub method: AudioParamWillBeDestroyedMethod,
    pub params: AudioParamWillBeDestroyedParams,
}
#[doc = "Notifies that two AudioNodes are connected.\n[nodesConnected](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-nodesConnected)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodesConnectedParams {
    #[serde(rename = "contextId")]
    pub context_id: super::types::GraphObjectId,
    #[serde(rename = "sourceId")]
    pub source_id: super::types::GraphObjectId,
    #[serde(rename = "destinationId")]
    pub destination_id: super::types::GraphObjectId,
    #[serde(rename = "sourceOutputIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_output_index: Option<f64>,
    #[serde(rename = "destinationInputIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub destination_input_index: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodesConnectedMethod {
    #[serde(rename = "WebAudio.nodesConnected")]
    NodesConnected,
}
impl NodesConnectedMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.nodesConnected";
}
#[doc = "Notifies that two AudioNodes are connected.\n[nodesConnected](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-nodesConnected)"]
#[derive(Debug, Clone, PartialEq)]
pub struct NodesConnected {
    pub method: NodesConnectedMethod,
    pub params: NodesConnectedParams,
}
#[doc = "Notifies that AudioNodes are disconnected. The destination can be null, and it means all the outgoing connections from the source are disconnected.\n[nodesDisconnected](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-nodesDisconnected)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodesDisconnectedParams {
    #[serde(rename = "contextId")]
    pub context_id: super::types::GraphObjectId,
    #[serde(rename = "sourceId")]
    pub source_id: super::types::GraphObjectId,
    #[serde(rename = "destinationId")]
    pub destination_id: super::types::GraphObjectId,
    #[serde(rename = "sourceOutputIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_output_index: Option<f64>,
    #[serde(rename = "destinationInputIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub destination_input_index: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodesDisconnectedMethod {
    #[serde(rename = "WebAudio.nodesDisconnected")]
    NodesDisconnected,
}
impl NodesDisconnectedMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.nodesDisconnected";
}
#[doc = "Notifies that AudioNodes are disconnected. The destination can be null, and it means all the outgoing connections from the source are disconnected.\n[nodesDisconnected](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-nodesDisconnected)"]
#[derive(Debug, Clone, PartialEq)]
pub struct NodesDisconnected {
    pub method: NodesDisconnectedMethod,
    pub params: NodesDisconnectedParams,
}
#[doc = "Notifies that an AudioNode is connected to an AudioParam.\n[nodeParamConnected](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-nodeParamConnected)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeParamConnectedParams {
    #[serde(rename = "contextId")]
    pub context_id: super::types::GraphObjectId,
    #[serde(rename = "sourceId")]
    pub source_id: super::types::GraphObjectId,
    #[serde(rename = "destinationId")]
    pub destination_id: super::types::GraphObjectId,
    #[serde(rename = "sourceOutputIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_output_index: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeParamConnectedMethod {
    #[serde(rename = "WebAudio.nodeParamConnected")]
    NodeParamConnected,
}
impl NodeParamConnectedMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.nodeParamConnected";
}
#[doc = "Notifies that an AudioNode is connected to an AudioParam.\n[nodeParamConnected](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-nodeParamConnected)"]
#[derive(Debug, Clone, PartialEq)]
pub struct NodeParamConnected {
    pub method: NodeParamConnectedMethod,
    pub params: NodeParamConnectedParams,
}
#[doc = "Notifies that an AudioNode is disconnected to an AudioParam.\n[nodeParamDisconnected](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-nodeParamDisconnected)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeParamDisconnectedParams {
    #[serde(rename = "contextId")]
    pub context_id: super::types::GraphObjectId,
    #[serde(rename = "sourceId")]
    pub source_id: super::types::GraphObjectId,
    #[serde(rename = "destinationId")]
    pub destination_id: super::types::GraphObjectId,
    #[serde(rename = "sourceOutputIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_output_index: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeParamDisconnectedMethod {
    #[serde(rename = "WebAudio.nodeParamDisconnected")]
    NodeParamDisconnected,
}
impl NodeParamDisconnectedMethod {
    pub const IDENTIFIER: &'static str = "WebAudio.nodeParamDisconnected";
}
#[doc = "Notifies that an AudioNode is disconnected to an AudioParam.\n[nodeParamDisconnected](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-nodeParamDisconnected)"]
#[derive(Debug, Clone, PartialEq)]
pub struct NodeParamDisconnected {
    pub method: NodeParamDisconnectedMethod,
    pub params: NodeParamDisconnectedParams,
}
group_enum ! (WebAudioEvents { ContextCreated (ContextCreated) , ContextWillBeDestroyed (ContextWillBeDestroyed) , ContextChanged (ContextChanged) , AudioListenerCreated (AudioListenerCreated) , AudioListenerWillBeDestroyed (AudioListenerWillBeDestroyed) , AudioNodeCreated (AudioNodeCreated) , AudioNodeWillBeDestroyed (AudioNodeWillBeDestroyed) , AudioParamCreated (AudioParamCreated) , AudioParamWillBeDestroyed (AudioParamWillBeDestroyed) , NodesConnected (NodesConnected) , NodesDisconnected (NodesDisconnected) , NodeParamConnected (NodeParamConnected) , NodeParamDisconnected (NodeParamDisconnected) });
