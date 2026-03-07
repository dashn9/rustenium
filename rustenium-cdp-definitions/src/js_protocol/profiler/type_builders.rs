use super::types::*;
impl ProfileNode {
    pub fn builder() -> ProfileNodeBuilder {
        <ProfileNodeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ProfileNodeBuilder {
    id: Option<i64>,
    call_frame: Option<crate::js_protocol::runtime::types::CallFrame>,
    hit_count: Option<i64>,
    children: Option<Vec<i64>>,
    deopt_reason: Option<String>,
    position_ticks: Option<Vec<PositionTickInfo>>,
}
impl ProfileNodeBuilder {
    pub fn id(mut self, id: impl Into<i64>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn call_frame(
        mut self,
        call_frame: impl Into<crate::js_protocol::runtime::types::CallFrame>,
    ) -> Self {
        self.call_frame = Some(call_frame.into());
        self
    }
    pub fn hit_count(mut self, hit_count: impl Into<i64>) -> Self {
        self.hit_count = Some(hit_count.into());
        self
    }
    pub fn children(mut self, children: impl Into<i64>) -> Self {
        let v = self.children.get_or_insert(Vec::new());
        v.push(children.into());
        self
    }
    pub fn childrens<I, S>(mut self, childrens: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.children.get_or_insert(Vec::new());
        for val in childrens {
            v.push(val.into());
        }
        self
    }
    pub fn deopt_reason(mut self, deopt_reason: impl Into<String>) -> Self {
        self.deopt_reason = Some(deopt_reason.into());
        self
    }
    pub fn position_tick(mut self, position_tick: impl Into<PositionTickInfo>) -> Self {
        let v = self.position_ticks.get_or_insert(Vec::new());
        v.push(position_tick.into());
        self
    }
    pub fn position_ticks<I, S>(mut self, position_ticks: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<PositionTickInfo>,
    {
        let v = self.position_ticks.get_or_insert(Vec::new());
        for val in position_ticks {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<ProfileNode, String> {
        Ok(ProfileNode {
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            call_frame: self
                .call_frame
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(call_frame)))?,
            hit_count: self.hit_count,
            children: self.children,
            deopt_reason: self.deopt_reason,
            position_ticks: self.position_ticks,
        })
    }
}
impl Profile {
    pub fn builder() -> ProfileBuilder {
        <ProfileBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ProfileBuilder {
    nodes: Option<Vec<ProfileNode>>,
    start_time: Option<f64>,
    end_time: Option<f64>,
    samples: Option<Vec<i64>>,
    time_deltas: Option<Vec<i64>>,
}
impl ProfileBuilder {
    pub fn node(mut self, node: impl Into<ProfileNode>) -> Self {
        let v = self.nodes.get_or_insert(Vec::new());
        v.push(node.into());
        self
    }
    pub fn nodes<I, S>(mut self, nodes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<ProfileNode>,
    {
        let v = self.nodes.get_or_insert(Vec::new());
        for val in nodes {
            v.push(val.into());
        }
        self
    }
    pub fn start_time(mut self, start_time: impl Into<f64>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }
    pub fn end_time(mut self, end_time: impl Into<f64>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }
    pub fn sample(mut self, sample: impl Into<i64>) -> Self {
        let v = self.samples.get_or_insert(Vec::new());
        v.push(sample.into());
        self
    }
    pub fn samples<I, S>(mut self, samples: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.samples.get_or_insert(Vec::new());
        for val in samples {
            v.push(val.into());
        }
        self
    }
    pub fn time_delta(mut self, time_delta: impl Into<i64>) -> Self {
        let v = self.time_deltas.get_or_insert(Vec::new());
        v.push(time_delta.into());
        self
    }
    pub fn time_deltas<I, S>(mut self, time_deltas: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.time_deltas.get_or_insert(Vec::new());
        for val in time_deltas {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<Profile, String> {
        Ok(Profile {
            nodes: self
                .nodes
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(nodes)))?,
            start_time: self
                .start_time
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(start_time)))?,
            end_time: self
                .end_time
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(end_time)))?,
            samples: self.samples,
            time_deltas: self.time_deltas,
        })
    }
}
impl PositionTickInfo {
    pub fn builder() -> PositionTickInfoBuilder {
        <PositionTickInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PositionTickInfoBuilder {
    line: Option<i64>,
    ticks: Option<i64>,
}
impl PositionTickInfoBuilder {
    pub fn line(mut self, line: impl Into<i64>) -> Self {
        self.line = Some(line.into());
        self
    }
    pub fn ticks(mut self, ticks: impl Into<i64>) -> Self {
        self.ticks = Some(ticks.into());
        self
    }
    pub fn build(self) -> Result<PositionTickInfo, String> {
        Ok(PositionTickInfo {
            line: self
                .line
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line)))?,
            ticks: self
                .ticks
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(ticks)))?,
        })
    }
}
impl CoverageRange {
    pub fn builder() -> CoverageRangeBuilder {
        <CoverageRangeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CoverageRangeBuilder {
    start_offset: Option<i64>,
    end_offset: Option<i64>,
    count: Option<i64>,
}
impl CoverageRangeBuilder {
    pub fn start_offset(mut self, start_offset: impl Into<i64>) -> Self {
        self.start_offset = Some(start_offset.into());
        self
    }
    pub fn end_offset(mut self, end_offset: impl Into<i64>) -> Self {
        self.end_offset = Some(end_offset.into());
        self
    }
    pub fn count(mut self, count: impl Into<i64>) -> Self {
        self.count = Some(count.into());
        self
    }
    pub fn build(self) -> Result<CoverageRange, String> {
        Ok(CoverageRange {
            start_offset: self.start_offset.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(start_offset))
            })?,
            end_offset: self
                .end_offset
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(end_offset)))?,
            count: self
                .count
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(count)))?,
        })
    }
}
impl FunctionCoverage {
    pub fn builder() -> FunctionCoverageBuilder {
        <FunctionCoverageBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct FunctionCoverageBuilder {
    function_name: Option<String>,
    ranges: Option<Vec<CoverageRange>>,
    is_block_coverage: Option<bool>,
}
impl FunctionCoverageBuilder {
    pub fn function_name(mut self, function_name: impl Into<String>) -> Self {
        self.function_name = Some(function_name.into());
        self
    }
    pub fn range(mut self, range: impl Into<CoverageRange>) -> Self {
        let v = self.ranges.get_or_insert(Vec::new());
        v.push(range.into());
        self
    }
    pub fn ranges<I, S>(mut self, ranges: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<CoverageRange>,
    {
        let v = self.ranges.get_or_insert(Vec::new());
        for val in ranges {
            v.push(val.into());
        }
        self
    }
    pub fn is_block_coverage(mut self, is_block_coverage: impl Into<bool>) -> Self {
        self.is_block_coverage = Some(is_block_coverage.into());
        self
    }
    pub fn build(self) -> Result<FunctionCoverage, String> {
        Ok(FunctionCoverage {
            function_name: self.function_name.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(function_name))
            })?,
            ranges: self
                .ranges
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(ranges)))?,
            is_block_coverage: self.is_block_coverage.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(is_block_coverage)
                )
            })?,
        })
    }
}
impl ScriptCoverage {
    pub fn builder() -> ScriptCoverageBuilder {
        <ScriptCoverageBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ScriptCoverageBuilder {
    script_id: Option<crate::js_protocol::runtime::types::ScriptId>,
    url: Option<String>,
    functions: Option<Vec<FunctionCoverage>>,
}
impl ScriptCoverageBuilder {
    pub fn script_id(
        mut self,
        script_id: impl Into<crate::js_protocol::runtime::types::ScriptId>,
    ) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn function(mut self, function: impl Into<FunctionCoverage>) -> Self {
        let v = self.functions.get_or_insert(Vec::new());
        v.push(function.into());
        self
    }
    pub fn functions<I, S>(mut self, functions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<FunctionCoverage>,
    {
        let v = self.functions.get_or_insert(Vec::new());
        for val in functions {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<ScriptCoverage, String> {
        Ok(ScriptCoverage {
            script_id: self
                .script_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(script_id)))?,
            url: self
                .url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            functions: self
                .functions
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(functions)))?,
        })
    }
}
