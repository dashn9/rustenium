use super::types::*;
impl LargestContentfulPaint {
    pub fn builder() -> LargestContentfulPaintBuilder {
        LargestContentfulPaintBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct LargestContentfulPaintBuilder {
    render_time: Option<super::super::network::types::TimeSinceEpoch>,
    load_time: Option<super::super::network::types::TimeSinceEpoch>,
    size: Option<f64>,
    element_id: Option<String>,
    url: Option<String>,
    node_id: Option<super::super::dom::types::BackendNodeId>,
}
impl LargestContentfulPaintBuilder {
    pub fn render_time(
        mut self,
        render_time: impl Into<super::super::network::types::TimeSinceEpoch>,
    ) -> Self {
        self.render_time = Some(render_time.into());
        self
    }
    pub fn load_time(
        mut self,
        load_time: impl Into<super::super::network::types::TimeSinceEpoch>,
    ) -> Self {
        self.load_time = Some(load_time.into());
        self
    }
    pub fn size(mut self, size: impl Into<f64>) -> Self {
        self.size = Some(size.into());
        self
    }
    pub fn element_id(mut self, element_id: impl Into<String>) -> Self {
        self.element_id = Some(element_id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::BackendNodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<LargestContentfulPaint, String> {
        Ok(LargestContentfulPaint {
            render_time: self
                .render_time
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(render_time)))?,
            load_time: self
                .load_time
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(load_time)))?,
            size: self
                .size
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(size)))?,
            element_id: self.element_id,
            url: self.url,
            node_id: self.node_id,
        })
    }
}
impl LayoutShiftAttribution {
    pub fn builder() -> LayoutShiftAttributionBuilder {
        LayoutShiftAttributionBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct LayoutShiftAttributionBuilder {
    previous_rect: Option<super::super::dom::types::Rect>,
    current_rect: Option<super::super::dom::types::Rect>,
    node_id: Option<super::super::dom::types::BackendNodeId>,
}
impl LayoutShiftAttributionBuilder {
    pub fn previous_rect(
        mut self,
        previous_rect: impl Into<super::super::dom::types::Rect>,
    ) -> Self {
        self.previous_rect = Some(previous_rect.into());
        self
    }
    pub fn current_rect(mut self, current_rect: impl Into<super::super::dom::types::Rect>) -> Self {
        self.current_rect = Some(current_rect.into());
        self
    }
    pub fn node_id(mut self, node_id: impl Into<super::super::dom::types::BackendNodeId>) -> Self {
        self.node_id = Some(node_id.into());
        self
    }
    pub fn build(self) -> Result<LayoutShiftAttribution, String> {
        Ok(LayoutShiftAttribution {
            previous_rect: self.previous_rect.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(previous_rect))
            })?,
            current_rect: self.current_rect.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(current_rect))
            })?,
            node_id: self.node_id,
        })
    }
}
impl LayoutShift {
    pub fn builder() -> LayoutShiftBuilder {
        LayoutShiftBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct LayoutShiftBuilder {
    value: Option<f64>,
    had_recent_input: Option<bool>,
    last_input_time: Option<super::super::network::types::TimeSinceEpoch>,
    sources: Option<Vec<LayoutShiftAttribution>>,
}
impl LayoutShiftBuilder {
    pub fn value(mut self, value: impl Into<f64>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn had_recent_input(mut self, had_recent_input: impl Into<bool>) -> Self {
        self.had_recent_input = Some(had_recent_input.into());
        self
    }
    pub fn last_input_time(
        mut self,
        last_input_time: impl Into<super::super::network::types::TimeSinceEpoch>,
    ) -> Self {
        self.last_input_time = Some(last_input_time.into());
        self
    }
    pub fn source(mut self, source: impl Into<LayoutShiftAttribution>) -> Self {
        let v = self.sources.get_or_insert(Vec::new());
        v.push(source.into());
        self
    }
    pub fn sources<I, S>(mut self, sources: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<LayoutShiftAttribution>,
    {
        let v = self.sources.get_or_insert(Vec::new());
        for val in sources {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<LayoutShift, String> {
        Ok(LayoutShift {
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            had_recent_input: self.had_recent_input.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(had_recent_input)
                )
            })?,
            last_input_time: self.last_input_time.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(last_input_time))
            })?,
            sources: self
                .sources
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(sources)))?,
        })
    }
}
impl TimelineEvent {
    pub fn builder() -> TimelineEventBuilder {
        TimelineEventBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct TimelineEventBuilder {
    frame_id: Option<super::super::page::types::FrameId>,
    r#type: Option<String>,
    name: Option<String>,
    time: Option<super::super::network::types::TimeSinceEpoch>,
    duration: Option<f64>,
    lcp_details: Option<LargestContentfulPaint>,
    layout_shift_details: Option<LayoutShift>,
}
impl TimelineEventBuilder {
    pub fn frame_id(mut self, frame_id: impl Into<super::super::page::types::FrameId>) -> Self {
        self.frame_id = Some(frame_id.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn time(mut self, time: impl Into<super::super::network::types::TimeSinceEpoch>) -> Self {
        self.time = Some(time.into());
        self
    }
    pub fn duration(mut self, duration: impl Into<f64>) -> Self {
        self.duration = Some(duration.into());
        self
    }
    pub fn lcp_details(mut self, lcp_details: impl Into<LargestContentfulPaint>) -> Self {
        self.lcp_details = Some(lcp_details.into());
        self
    }
    pub fn layout_shift_details(mut self, layout_shift_details: impl Into<LayoutShift>) -> Self {
        self.layout_shift_details = Some(layout_shift_details.into());
        self
    }
    pub fn build(self) -> Result<TimelineEvent, String> {
        Ok(TimelineEvent {
            frame_id: self
                .frame_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(frame_id)))?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            time: self
                .time
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(time)))?,
            duration: self.duration,
            lcp_details: self.lcp_details,
            layout_shift_details: self.layout_shift_details,
        })
    }
}
