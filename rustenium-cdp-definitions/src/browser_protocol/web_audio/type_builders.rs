use super::types::*;
impl ContextRealtimeData {
    pub fn builder() -> ContextRealtimeDataBuilder {
        <ContextRealtimeDataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ContextRealtimeDataBuilder {
    current_time: Option<f64>,
    render_capacity: Option<f64>,
    callback_interval_mean: Option<f64>,
    callback_interval_variance: Option<f64>,
}
impl ContextRealtimeDataBuilder {
    pub fn current_time(mut self, current_time: impl Into<f64>) -> Self {
        self.current_time = Some(current_time.into());
        self
    }
    pub fn render_capacity(mut self, render_capacity: impl Into<f64>) -> Self {
        self.render_capacity = Some(render_capacity.into());
        self
    }
    pub fn callback_interval_mean(mut self, callback_interval_mean: impl Into<f64>) -> Self {
        self.callback_interval_mean = Some(callback_interval_mean.into());
        self
    }
    pub fn callback_interval_variance(
        mut self,
        callback_interval_variance: impl Into<f64>,
    ) -> Self {
        self.callback_interval_variance = Some(callback_interval_variance.into());
        self
    }
    pub fn build(self) -> Result<ContextRealtimeData, String> {
        Ok(ContextRealtimeData {
            current_time: self.current_time.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(current_time))
            })?,
            render_capacity: self.render_capacity.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(render_capacity))
            })?,
            callback_interval_mean: self.callback_interval_mean.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(callback_interval_mean)
                )
            })?,
            callback_interval_variance: self.callback_interval_variance.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(callback_interval_variance)
                )
            })?,
        })
    }
}
impl BaseAudioContext {
    pub fn builder() -> BaseAudioContextBuilder {
        <BaseAudioContextBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BaseAudioContextBuilder {
    context_id: Option<GraphObjectId>,
    context_type: Option<ContextType>,
    context_state: Option<ContextState>,
    realtime_data: Option<ContextRealtimeData>,
    callback_buffer_size: Option<f64>,
    max_output_channel_count: Option<f64>,
    sample_rate: Option<f64>,
}
impl BaseAudioContextBuilder {
    pub fn context_id(mut self, context_id: impl Into<GraphObjectId>) -> Self {
        self.context_id = Some(context_id.into());
        self
    }
    pub fn context_type(mut self, context_type: impl Into<ContextType>) -> Self {
        self.context_type = Some(context_type.into());
        self
    }
    pub fn context_state(mut self, context_state: impl Into<ContextState>) -> Self {
        self.context_state = Some(context_state.into());
        self
    }
    pub fn realtime_data(mut self, realtime_data: impl Into<ContextRealtimeData>) -> Self {
        self.realtime_data = Some(realtime_data.into());
        self
    }
    pub fn callback_buffer_size(mut self, callback_buffer_size: impl Into<f64>) -> Self {
        self.callback_buffer_size = Some(callback_buffer_size.into());
        self
    }
    pub fn max_output_channel_count(mut self, max_output_channel_count: impl Into<f64>) -> Self {
        self.max_output_channel_count = Some(max_output_channel_count.into());
        self
    }
    pub fn sample_rate(mut self, sample_rate: impl Into<f64>) -> Self {
        self.sample_rate = Some(sample_rate.into());
        self
    }
    pub fn build(self) -> Result<BaseAudioContext, String> {
        Ok(BaseAudioContext {
            context_id: self
                .context_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context_id)))?,
            context_type: self.context_type.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(context_type))
            })?,
            context_state: self.context_state.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(context_state))
            })?,
            realtime_data: self.realtime_data,
            callback_buffer_size: self.callback_buffer_size.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(callback_buffer_size)
                )
            })?,
            max_output_channel_count: self.max_output_channel_count.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(max_output_channel_count)
                )
            })?,
            sample_rate: self
                .sample_rate
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(sample_rate)))?,
        })
    }
}
impl AudioListener {
    pub fn builder() -> AudioListenerBuilder {
        <AudioListenerBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AudioListenerBuilder {
    listener_id: Option<GraphObjectId>,
    context_id: Option<GraphObjectId>,
}
impl AudioListenerBuilder {
    pub fn listener_id(mut self, listener_id: impl Into<GraphObjectId>) -> Self {
        self.listener_id = Some(listener_id.into());
        self
    }
    pub fn context_id(mut self, context_id: impl Into<GraphObjectId>) -> Self {
        self.context_id = Some(context_id.into());
        self
    }
    pub fn build(self) -> Result<AudioListener, String> {
        Ok(AudioListener {
            listener_id: self
                .listener_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(listener_id)))?,
            context_id: self
                .context_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context_id)))?,
        })
    }
}
impl AudioNode {
    pub fn builder() -> AudioNodeBuilder {
        <AudioNodeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AudioNodeBuilder {
    node_id: Option<GraphObjectId>,
    context_id: Option<GraphObjectId>,
    node_type: Option<NodeType>,
    number_of_inputs: Option<f64>,
    number_of_outputs: Option<f64>,
    channel_count: Option<f64>,
    channel_count_mode: Option<ChannelCountMode>,
    channel_interpretation: Option<ChannelInterpretation>,
}
impl AudioNodeBuilder {
    pub fn node_id(mut self, node_id: impl Into<GraphObjectId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn context_id(mut self, context_id: impl Into<GraphObjectId>) -> Self {
        self.context_id = Some(context_id.into());
        self
    }
    pub fn node_type(mut self, node_type: impl Into<NodeType>) -> Self {
        self.node_type = Some(node_type.into());
        self
    }
    pub fn number_of_inputs(mut self, number_of_inputs: impl Into<f64>) -> Self {
        self.number_of_inputs = Some(number_of_inputs.into());
        self
    }
    pub fn number_of_outputs(mut self, number_of_outputs: impl Into<f64>) -> Self {
        self.number_of_outputs = Some(number_of_outputs.into());
        self
    }
    pub fn channel_count(mut self, channel_count: impl Into<f64>) -> Self {
        self.channel_count = Some(channel_count.into());
        self
    }
    pub fn channel_count_mode(mut self, channel_count_mode: impl Into<ChannelCountMode>) -> Self {
        self.channel_count_mode = Some(channel_count_mode.into());
        self
    }
    pub fn channel_interpretation(
        mut self,
        channel_interpretation: impl Into<ChannelInterpretation>,
    ) -> Self {
        self.channel_interpretation = Some(channel_interpretation.into());
        self
    }
    pub fn build(self) -> Result<AudioNode, String> {
        Ok(AudioNode {
            node_id: self
                .node_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            context_id: self
                .context_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context_id)))?,
            node_type: self
                .node_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_type)))?,
            number_of_inputs: self.number_of_inputs.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(number_of_inputs)
                )
            })?,
            number_of_outputs: self.number_of_outputs.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(number_of_outputs)
                )
            })?,
            channel_count: self.channel_count.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(channel_count))
            })?,
            channel_count_mode: self.channel_count_mode.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(channel_count_mode)
                )
            })?,
            channel_interpretation: self.channel_interpretation.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(channel_interpretation)
                )
            })?,
        })
    }
}
impl AudioParam {
    pub fn builder() -> AudioParamBuilder {
        <AudioParamBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AudioParamBuilder {
    param_id: Option<GraphObjectId>,
    node_id: Option<GraphObjectId>,
    context_id: Option<GraphObjectId>,
    param_type: Option<ParamType>,
    rate: Option<AutomationRate>,
    default_value: Option<f64>,
    min_value: Option<f64>,
    max_value: Option<f64>,
}
impl AudioParamBuilder {
    pub fn param_id(mut self, param_id: impl Into<GraphObjectId>) -> Self {
        self.param_id = Some(param_id.into());
        self
    }
    pub fn node_id(mut self, node_id: impl Into<GraphObjectId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn context_id(mut self, context_id: impl Into<GraphObjectId>) -> Self {
        self.context_id = Some(context_id.into());
        self
    }
    pub fn param_type(mut self, param_type: impl Into<ParamType>) -> Self {
        self.param_type = Some(param_type.into());
        self
    }
    pub fn rate(mut self, rate: impl Into<AutomationRate>) -> Self {
        self.rate = Some(rate.into());
        self
    }
    pub fn default_value(mut self, default_value: impl Into<f64>) -> Self {
        self.default_value = Some(default_value.into());
        self
    }
    pub fn min_value(mut self, min_value: impl Into<f64>) -> Self {
        self.min_value = Some(min_value.into());
        self
    }
    pub fn max_value(mut self, max_value: impl Into<f64>) -> Self {
        self.max_value = Some(max_value.into());
        self
    }
    pub fn build(self) -> Result<AudioParam, String> {
        Ok(AudioParam {
            param_id: self
                .param_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(param_id)))?,
            node_id: self
                .node_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(node_id)))?,
            context_id: self
                .context_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context_id)))?,
            param_type: self
                .param_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(param_type)))?,
            rate: self
                .rate
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(rate)))?,
            default_value: self.default_value.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(default_value))
            })?,
            min_value: self
                .min_value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(min_value)))?,
            max_value: self
                .max_value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(max_value)))?,
        })
    }
}
