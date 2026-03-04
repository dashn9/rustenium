use serde::{Deserialize, Serialize};
#[doc = "An unique ID for a graph object (AudioContext, AudioNode, AudioParam) in Web Audio API\n[GraphObjectId](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-GraphObjectId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct GraphObjectId(String);
impl GraphObjectId {
    pub fn new(val: impl Into<String>) -> Self {
        GraphObjectId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for GraphObjectId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<GraphObjectId> for String {
    fn from(el: GraphObjectId) -> String {
        el.0
    }
}
impl From<String> for GraphObjectId {
    fn from(expr: String) -> Self {
        GraphObjectId(expr)
    }
}
impl std::borrow::Borrow<str> for GraphObjectId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl GraphObjectId {
    pub const IDENTIFIER: &'static str = "WebAudio.GraphObjectId";
}
#[doc = "Enum of BaseAudioContext types"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContextType {
    #[serde(rename = "realtime")]
    Realtime,
    #[serde(rename = "offline")]
    Offline,
}
#[doc = "Enum of AudioContextState from the spec"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContextState {
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "interrupted")]
    Interrupted,
}
#[doc = "Enum of AudioNode types\n[NodeType](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-NodeType)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct NodeType(String);
impl NodeType {
    pub fn new(val: impl Into<String>) -> Self {
        NodeType(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for NodeType {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<NodeType> for String {
    fn from(el: NodeType) -> String {
        el.0
    }
}
impl From<String> for NodeType {
    fn from(expr: String) -> Self {
        NodeType(expr)
    }
}
impl NodeType {
    pub const IDENTIFIER: &'static str = "WebAudio.NodeType";
}
#[doc = "Enum of AudioNode::ChannelCountMode from the spec"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChannelCountMode {
    #[serde(rename = "clamped-max")]
    ClampedMax,
    #[serde(rename = "explicit")]
    Explicit,
    #[serde(rename = "max")]
    Max,
}
#[doc = "Enum of AudioNode::ChannelInterpretation from the spec"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChannelInterpretation {
    #[serde(rename = "discrete")]
    Discrete,
    #[serde(rename = "speakers")]
    Speakers,
}
#[doc = "Enum of AudioParam types\n[ParamType](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-ParamType)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct ParamType(String);
impl ParamType {
    pub fn new(val: impl Into<String>) -> Self {
        ParamType(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for ParamType {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<ParamType> for String {
    fn from(el: ParamType) -> String {
        el.0
    }
}
impl From<String> for ParamType {
    fn from(expr: String) -> Self {
        ParamType(expr)
    }
}
impl ParamType {
    pub const IDENTIFIER: &'static str = "WebAudio.ParamType";
}
#[doc = "Enum of AudioParam::AutomationRate from the spec"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AutomationRate {
    #[serde(rename = "a-rate")]
    ARate,
    #[serde(rename = "k-rate")]
    KRate,
}
#[doc = "Fields in AudioContext that change in real-time.\n[ContextRealtimeData](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-ContextRealtimeData)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextRealtimeData {
    #[doc = "The current context time in second in BaseAudioContext."]
    #[serde(rename = "currentTime")]
    pub current_time: f64,
    #[doc = "The time spent on rendering graph divided by render quantum duration,\nand multiplied by 100. 100 means the audio renderer reached the full\ncapacity and glitch may occur."]
    #[serde(rename = "renderCapacity")]
    pub render_capacity: f64,
    #[doc = "A running mean of callback interval."]
    #[serde(rename = "callbackIntervalMean")]
    pub callback_interval_mean: f64,
    #[doc = "A running variance of callback interval."]
    #[serde(rename = "callbackIntervalVariance")]
    pub callback_interval_variance: f64,
}
impl ContextRealtimeData {
    pub fn new(
        current_time: impl Into<f64>,
        render_capacity: impl Into<f64>,
        callback_interval_mean: impl Into<f64>,
        callback_interval_variance: impl Into<f64>,
    ) -> Self {
        Self {
            current_time: current_time.into(),
            render_capacity: render_capacity.into(),
            callback_interval_mean: callback_interval_mean.into(),
            callback_interval_variance: callback_interval_variance.into(),
        }
    }
}
impl ContextRealtimeData {
    pub const IDENTIFIER: &'static str = "WebAudio.ContextRealtimeData";
}
#[doc = "Protocol object for BaseAudioContext\n[BaseAudioContext](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-BaseAudioContext)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseAudioContext {
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId,
    #[serde(rename = "contextType")]
    pub context_type: ContextType,
    #[serde(rename = "contextState")]
    pub context_state: ContextState,
    #[serde(rename = "realtimeData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub realtime_data: Option<ContextRealtimeData>,
    #[doc = "Platform-dependent callback buffer size."]
    #[serde(rename = "callbackBufferSize")]
    pub callback_buffer_size: f64,
    #[doc = "Number of output channels supported by audio hardware in use."]
    #[serde(rename = "maxOutputChannelCount")]
    pub max_output_channel_count: f64,
    #[doc = "Context sample rate."]
    #[serde(rename = "sampleRate")]
    pub sample_rate: f64,
}
impl BaseAudioContext {
    pub const IDENTIFIER: &'static str = "WebAudio.BaseAudioContext";
}
#[doc = "Protocol object for AudioListener\n[AudioListener](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-AudioListener)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioListener {
    #[serde(rename = "listenerId")]
    pub listener_id: GraphObjectId,
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId,
}
impl AudioListener {
    pub fn new(
        listener_id: impl Into<GraphObjectId>,
        context_id: impl Into<GraphObjectId>,
    ) -> Self {
        Self {
            listener_id: listener_id.into(),
            context_id: context_id.into(),
        }
    }
}
impl AudioListener {
    pub const IDENTIFIER: &'static str = "WebAudio.AudioListener";
}
#[doc = "Protocol object for AudioNode\n[AudioNode](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-AudioNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioNode {
    #[serde(rename = "nodeId")]
    pub node_id: GraphObjectId,
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId,
    #[serde(rename = "nodeType")]
    pub node_type: NodeType,
    #[serde(rename = "numberOfInputs")]
    pub number_of_inputs: f64,
    #[serde(rename = "numberOfOutputs")]
    pub number_of_outputs: f64,
    #[serde(rename = "channelCount")]
    pub channel_count: f64,
    #[serde(rename = "channelCountMode")]
    pub channel_count_mode: ChannelCountMode,
    #[serde(rename = "channelInterpretation")]
    pub channel_interpretation: ChannelInterpretation,
}
impl AudioNode {
    pub const IDENTIFIER: &'static str = "WebAudio.AudioNode";
}
#[doc = "Protocol object for AudioParam\n[AudioParam](https://chromedevtools.github.io/devtools-protocol/tot/WebAudio/#type-AudioParam)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioParam {
    #[serde(rename = "paramId")]
    pub param_id: GraphObjectId,
    #[serde(rename = "nodeId")]
    pub node_id: GraphObjectId,
    #[serde(rename = "contextId")]
    pub context_id: GraphObjectId,
    #[serde(rename = "paramType")]
    pub param_type: ParamType,
    #[serde(rename = "rate")]
    pub rate: AutomationRate,
    #[serde(rename = "defaultValue")]
    pub default_value: f64,
    #[serde(rename = "minValue")]
    pub min_value: f64,
    #[serde(rename = "maxValue")]
    pub max_value: f64,
}
impl AudioParam {
    pub const IDENTIFIER: &'static str = "WebAudio.AudioParam";
}
group_enum ! (Type { GraphObjectId (GraphObjectId) , ContextType (ContextType) , ContextState (ContextState) , NodeType (NodeType) , ChannelCountMode (ChannelCountMode) , ChannelInterpretation (ChannelInterpretation) , ParamType (ParamType) , AutomationRate (AutomationRate) , ContextRealtimeData (ContextRealtimeData) , BaseAudioContext (BaseAudioContext) , AudioListener (AudioListener) , AudioNode (AudioNode) , AudioParam (AudioParam) });
