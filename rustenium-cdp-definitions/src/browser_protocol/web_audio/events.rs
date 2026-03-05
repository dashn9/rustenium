use serde::{Deserialize, Serialize};
#[doc = "Notifies that a new BaseAudioContext has been created.\n[contextCreated](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-contextCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextCreated {
    #[serde(rename = "context")]
    pub context: super::types::BaseAudioContext,
}
impl ContextCreated {
    pub const IDENTIFIER: &'static str = "WebAudio.contextCreated";
}
#[doc = "Notifies that an existing BaseAudioContext will be destroyed.\n[contextWillBeDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-contextWillBeDestroyed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextWillBeDestroyed {
    #[serde(rename = "contextId")]
    pub context_id: super::types::GraphObjectId,
}
impl ContextWillBeDestroyed {
    pub const IDENTIFIER: &'static str = "WebAudio.contextWillBeDestroyed";
}
#[doc = "Notifies that existing BaseAudioContext has changed some properties (id stays the same)..\n[contextChanged](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-contextChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextChanged {
    #[serde(rename = "context")]
    pub context: super::types::BaseAudioContext,
}
impl ContextChanged {
    pub const IDENTIFIER: &'static str = "WebAudio.contextChanged";
}
#[doc = "Notifies that the construction of an AudioListener has finished.\n[audioListenerCreated](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioListenerCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioListenerCreated {
    #[serde(rename = "listener")]
    pub listener: super::types::AudioListener,
}
impl AudioListenerCreated {
    pub const IDENTIFIER: &'static str = "WebAudio.audioListenerCreated";
}
#[doc = "Notifies that a new AudioListener has been created.\n[audioListenerWillBeDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioListenerWillBeDestroyed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioListenerWillBeDestroyed {
    #[serde(rename = "contextId")]
    pub context_id: super::types::GraphObjectId,
    #[serde(rename = "listenerId")]
    pub listener_id: super::types::GraphObjectId,
}
impl AudioListenerWillBeDestroyed {
    pub const IDENTIFIER: &'static str = "WebAudio.audioListenerWillBeDestroyed";
}
#[doc = "Notifies that a new AudioNode has been created.\n[audioNodeCreated](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioNodeCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioNodeCreated {
    #[serde(rename = "node")]
    pub node: super::types::AudioNode,
}
impl AudioNodeCreated {
    pub const IDENTIFIER: &'static str = "WebAudio.audioNodeCreated";
}
#[doc = "Notifies that an existing AudioNode has been destroyed.\n[audioNodeWillBeDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioNodeWillBeDestroyed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioNodeWillBeDestroyed {
    #[serde(rename = "contextId")]
    pub context_id: super::types::GraphObjectId,
    #[serde(rename = "nodeId")]
    pub node_id: super::types::GraphObjectId,
}
impl AudioNodeWillBeDestroyed {
    pub const IDENTIFIER: &'static str = "WebAudio.audioNodeWillBeDestroyed";
}
#[doc = "Notifies that a new AudioParam has been created.\n[audioParamCreated](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioParamCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioParamCreated {
    #[serde(rename = "param")]
    pub param: super::types::AudioParam,
}
impl AudioParamCreated {
    pub const IDENTIFIER: &'static str = "WebAudio.audioParamCreated";
}
#[doc = "Notifies that an existing AudioParam has been destroyed.\n[audioParamWillBeDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-audioParamWillBeDestroyed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioParamWillBeDestroyed {
    #[serde(rename = "contextId")]
    pub context_id: super::types::GraphObjectId,
    #[serde(rename = "nodeId")]
    pub node_id: super::types::GraphObjectId,
    #[serde(rename = "paramId")]
    pub param_id: super::types::GraphObjectId,
}
impl AudioParamWillBeDestroyed {
    pub const IDENTIFIER: &'static str = "WebAudio.audioParamWillBeDestroyed";
}
#[doc = "Notifies that two AudioNodes are connected.\n[nodesConnected](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-nodesConnected)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodesConnected {
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
impl NodesConnected {
    pub const IDENTIFIER: &'static str = "WebAudio.nodesConnected";
}
#[doc = "Notifies that AudioNodes are disconnected. The destination can be null, and it means all the outgoing connections from the source are disconnected.\n[nodesDisconnected](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-nodesDisconnected)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodesDisconnected {
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
impl NodesDisconnected {
    pub const IDENTIFIER: &'static str = "WebAudio.nodesDisconnected";
}
#[doc = "Notifies that an AudioNode is connected to an AudioParam.\n[nodeParamConnected](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-nodeParamConnected)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeParamConnected {
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
impl NodeParamConnected {
    pub const IDENTIFIER: &'static str = "WebAudio.nodeParamConnected";
}
#[doc = "Notifies that an AudioNode is disconnected to an AudioParam.\n[nodeParamDisconnected](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#event-nodeParamDisconnected)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeParamDisconnected {
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
impl NodeParamDisconnected {
    pub const IDENTIFIER: &'static str = "WebAudio.nodeParamDisconnected";
}
group_enum ! (WebAudioEvents { ContextCreated (ContextCreated) , ContextWillBeDestroyed (ContextWillBeDestroyed) , ContextChanged (ContextChanged) , AudioListenerCreated (AudioListenerCreated) , AudioListenerWillBeDestroyed (AudioListenerWillBeDestroyed) , AudioNodeCreated (AudioNodeCreated) , AudioNodeWillBeDestroyed (AudioNodeWillBeDestroyed) , AudioParamCreated (AudioParamCreated) , AudioParamWillBeDestroyed (AudioParamWillBeDestroyed) , NodesConnected (NodesConnected) , NodesDisconnected (NodesDisconnected) , NodeParamConnected (NodeParamConnected) , NodeParamDisconnected (NodeParamDisconnected) });
