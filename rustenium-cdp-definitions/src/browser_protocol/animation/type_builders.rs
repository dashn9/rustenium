use super::types::*;
impl Animation {
    pub fn builder() -> AnimationBuilder {
        AnimationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AnimationBuilder {
    id: Option<String>,
    name: Option<String>,
    paused_state: Option<bool>,
    play_state: Option<String>,
    playback_rate: Option<f64>,
    start_time: Option<f64>,
    current_time: Option<f64>,
    r#type: Option<AnimationType>,
    source: Option<AnimationEffect>,
    css_id: Option<String>,
    view_or_scroll_timeline: Option<ViewOrScrollTimeline>,
}
impl AnimationBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn paused_state(mut self, paused_state: impl Into<bool>) -> Self {
        self.paused_state = Some(paused_state.into());
        self
    }
    pub fn play_state(mut self, play_state: impl Into<String>) -> Self {
        self.play_state = Some(play_state.into());
        self
    }
    pub fn playback_rate(mut self, playback_rate: impl Into<f64>) -> Self {
        self.playback_rate = Some(playback_rate.into());
        self
    }
    pub fn start_time(mut self, start_time: impl Into<f64>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }
    pub fn current_time(mut self, current_time: impl Into<f64>) -> Self {
        self.current_time = Some(current_time.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<AnimationType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn source(mut self, source: impl Into<AnimationEffect>) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn css_id(mut self, css_id: impl Into<String>) -> Self {
        self.css_id = Some(css_id.into());
        self
    }
    pub fn view_or_scroll_timeline(
        mut self,
        view_or_scroll_timeline: impl Into<ViewOrScrollTimeline>,
    ) -> Self {
        self.view_or_scroll_timeline = Some(view_or_scroll_timeline.into());
        self
    }
    pub fn build(self) -> Result<Animation, String> {
        Ok(Animation {
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            paused_state: self.paused_state.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(paused_state))
            })?,
            play_state: self
                .play_state
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(play_state)))?,
            playback_rate: self.playback_rate.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(playback_rate))
            })?,
            start_time: self
                .start_time
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(start_time)))?,
            current_time: self.current_time.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(current_time))
            })?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            source: self.source,
            css_id: self.css_id,
            view_or_scroll_timeline: self.view_or_scroll_timeline,
        })
    }
}
impl ViewOrScrollTimeline {
    pub fn builder() -> ViewOrScrollTimelineBuilder {
        ViewOrScrollTimelineBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ViewOrScrollTimelineBuilder {
    source_node_id: Option<super::super::dom::types::BackendNodeId>,
    start_offset: Option<f64>,
    end_offset: Option<f64>,
    subject_node_id: Option<super::super::dom::types::BackendNodeId>,
    axis: Option<super::super::dom::types::ScrollOrientation>,
}
impl ViewOrScrollTimelineBuilder {
    pub fn source_node_id(
        mut self,
        source_node_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.source_node_id = Some(source_node_id.into());
        self
    }
    pub fn start_offset(mut self, start_offset: impl Into<f64>) -> Self {
        self.start_offset = Some(start_offset.into());
        self
    }
    pub fn end_offset(mut self, end_offset: impl Into<f64>) -> Self {
        self.end_offset = Some(end_offset.into());
        self
    }
    pub fn subject_node_id(
        mut self,
        subject_node_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.subject_node_id = Some(subject_node_id.into());
        self
    }
    pub fn axis(mut self, axis: impl Into<super::super::dom::types::ScrollOrientation>) -> Self {
        self.axis = Some(axis.into());
        self
    }
    pub fn build(self) -> Result<ViewOrScrollTimeline, String> {
        Ok(ViewOrScrollTimeline {
            source_node_id: self.source_node_id,
            start_offset: self.start_offset,
            end_offset: self.end_offset,
            subject_node_id: self.subject_node_id,
            axis: self
                .axis
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(axis)))?,
        })
    }
}
impl AnimationEffect {
    pub fn builder() -> AnimationEffectBuilder {
        AnimationEffectBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AnimationEffectBuilder {
    delay: Option<f64>,
    end_delay: Option<f64>,
    iteration_start: Option<f64>,
    iterations: Option<f64>,
    duration: Option<f64>,
    direction: Option<String>,
    fill: Option<String>,
    backend_node_id: Option<super::super::dom::types::BackendNodeId>,
    keyframes_rule: Option<KeyframesRule>,
    easing: Option<String>,
}
impl AnimationEffectBuilder {
    pub fn delay(mut self, delay: impl Into<f64>) -> Self {
        self.delay = Some(delay.into());
        self
    }
    pub fn end_delay(mut self, end_delay: impl Into<f64>) -> Self {
        self.end_delay = Some(end_delay.into());
        self
    }
    pub fn iteration_start(mut self, iteration_start: impl Into<f64>) -> Self {
        self.iteration_start = Some(iteration_start.into());
        self
    }
    pub fn iterations(mut self, iterations: impl Into<f64>) -> Self {
        self.iterations = Some(iterations.into());
        self
    }
    pub fn duration(mut self, duration: impl Into<f64>) -> Self {
        self.duration = Some(duration.into());
        self
    }
    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
    }
    pub fn fill(mut self, fill: impl Into<String>) -> Self {
        self.fill = Some(fill.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<super::super::dom::types::BackendNodeId>,
    ) -> Self {
        self.backend_node_id = Some(backend_node_id.into());
        self
    }
    pub fn keyframes_rule(mut self, keyframes_rule: impl Into<KeyframesRule>) -> Self {
        self.keyframes_rule = Some(keyframes_rule.into());
        self
    }
    pub fn easing(mut self, easing: impl Into<String>) -> Self {
        self.easing = Some(easing.into());
        self
    }
    pub fn build(self) -> Result<AnimationEffect, String> {
        Ok(AnimationEffect {
            delay: self
                .delay
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(delay)))?,
            end_delay: self
                .end_delay
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(end_delay)))?,
            iteration_start: self.iteration_start.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(iteration_start))
            })?,
            iterations: self.iterations,
            duration: self
                .duration
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(duration)))?,
            direction: self
                .direction
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(direction)))?,
            fill: self
                .fill
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(fill)))?,
            backend_node_id: self.backend_node_id,
            keyframes_rule: self.keyframes_rule,
            easing: self
                .easing
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(easing)))?,
        })
    }
}
impl KeyframesRule {
    pub fn builder() -> KeyframesRuleBuilder {
        KeyframesRuleBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct KeyframesRuleBuilder {
    name: Option<String>,
    keyframes: Option<Vec<KeyframeStyle>>,
}
impl KeyframesRuleBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn keyframe(mut self, keyframe: impl Into<KeyframeStyle>) -> Self {
        let v = self.keyframes.get_or_insert(Vec::new());
        v.push(keyframe.into());
        self
    }
    pub fn keyframes<I, S>(mut self, keyframes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<KeyframeStyle>,
    {
        let v = self.keyframes.get_or_insert(Vec::new());
        for val in keyframes {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<KeyframesRule, String> {
        Ok(KeyframesRule {
            name: self.name,
            keyframes: self
                .keyframes
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(keyframes)))?,
        })
    }
}
impl KeyframeStyle {
    pub fn builder() -> KeyframeStyleBuilder {
        KeyframeStyleBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct KeyframeStyleBuilder {
    offset: Option<String>,
    easing: Option<String>,
}
impl KeyframeStyleBuilder {
    pub fn offset(mut self, offset: impl Into<String>) -> Self {
        self.offset = Some(offset.into());
        self
    }
    pub fn easing(mut self, easing: impl Into<String>) -> Self {
        self.easing = Some(easing.into());
        self
    }
    pub fn build(self) -> Result<KeyframeStyle, String> {
        Ok(KeyframeStyle {
            offset: self
                .offset
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(offset)))?,
            easing: self
                .easing
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(easing)))?,
        })
    }
}
