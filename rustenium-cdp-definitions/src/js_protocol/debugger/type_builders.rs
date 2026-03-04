use super::types::*;
impl Location {
    pub fn builder() -> LocationBuilder {
        LocationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct LocationBuilder {
    script_id: Option<super::super::runtime::types::ScriptId>,
    line_number: Option<i64>,
    column_number: Option<i64>,
}
impl LocationBuilder {
    pub fn script_id(
        mut self,
        script_id: impl Into<super::super::runtime::types::ScriptId>,
    ) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn line_number(mut self, line_number: impl Into<i64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn column_number(mut self, column_number: impl Into<i64>) -> Self {
        self.column_number = Some(column_number.into());
        self
    }
    pub fn build(self) -> Result<Location, String> {
        Ok(Location {
            script_id: self
                .script_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(script_id)))?,
            line_number: self
                .line_number
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line_number)))?,
            column_number: self.column_number,
        })
    }
}
impl ScriptPosition {
    pub fn builder() -> ScriptPositionBuilder {
        ScriptPositionBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ScriptPositionBuilder {
    line_number: Option<i64>,
    column_number: Option<i64>,
}
impl ScriptPositionBuilder {
    pub fn line_number(mut self, line_number: impl Into<i64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn column_number(mut self, column_number: impl Into<i64>) -> Self {
        self.column_number = Some(column_number.into());
        self
    }
    pub fn build(self) -> Result<ScriptPosition, String> {
        Ok(ScriptPosition {
            line_number: self
                .line_number
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line_number)))?,
            column_number: self.column_number.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(column_number))
            })?,
        })
    }
}
impl LocationRange {
    pub fn builder() -> LocationRangeBuilder {
        LocationRangeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct LocationRangeBuilder {
    script_id: Option<super::super::runtime::types::ScriptId>,
    start: Option<ScriptPosition>,
    end: Option<ScriptPosition>,
}
impl LocationRangeBuilder {
    pub fn script_id(
        mut self,
        script_id: impl Into<super::super::runtime::types::ScriptId>,
    ) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn start(mut self, start: impl Into<ScriptPosition>) -> Self {
        self.start = Some(start.into());
        self
    }
    pub fn end(mut self, end: impl Into<ScriptPosition>) -> Self {
        self.end = Some(end.into());
        self
    }
    pub fn build(self) -> Result<LocationRange, String> {
        Ok(LocationRange {
            script_id: self
                .script_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(script_id)))?,
            start: self
                .start
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(start)))?,
            end: self
                .end
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(end)))?,
        })
    }
}
impl CallFrame {
    pub fn builder() -> CallFrameBuilder {
        CallFrameBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CallFrameBuilder {
    call_frame_id: Option<CallFrameId>,
    function_name: Option<String>,
    function_location: Option<Location>,
    location: Option<Location>,
    scope_chain: Option<Vec<Scope>>,
    this: Option<super::super::runtime::types::RemoteObject>,
    return_value: Option<super::super::runtime::types::RemoteObject>,
    can_be_restarted: Option<bool>,
}
impl CallFrameBuilder {
    pub fn call_frame_id(mut self, call_frame_id: impl Into<CallFrameId>) -> Self {
        self.call_frame_id = Some(call_frame_id.into());
        self
    }
    pub fn function_name(mut self, function_name: impl Into<String>) -> Self {
        self.function_name = Some(function_name.into());
        self
    }
    pub fn function_location(mut self, function_location: impl Into<Location>) -> Self {
        self.function_location = Some(function_location.into());
        self
    }
    pub fn location(mut self, location: impl Into<Location>) -> Self {
        self.location = Some(location.into());
        self
    }
    pub fn scope_chain(mut self, scope_chain: impl Into<Scope>) -> Self {
        let v = self.scope_chain.get_or_insert(Vec::new());
        v.push(scope_chain.into());
        self
    }
    pub fn scope_chains<I, S>(mut self, scope_chains: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Scope>,
    {
        let v = self.scope_chain.get_or_insert(Vec::new());
        for val in scope_chains {
            v.push(val.into());
        }
        self
    }
    pub fn this(mut self, this: impl Into<super::super::runtime::types::RemoteObject>) -> Self {
        self.this = Some(this.into());
        self
    }
    pub fn return_value(
        mut self,
        return_value: impl Into<super::super::runtime::types::RemoteObject>,
    ) -> Self {
        self.return_value = Some(return_value.into());
        self
    }
    pub fn can_be_restarted(mut self, can_be_restarted: impl Into<bool>) -> Self {
        self.can_be_restarted = Some(can_be_restarted.into());
        self
    }
    pub fn build(self) -> Result<CallFrame, String> {
        Ok(CallFrame {
            call_frame_id: self.call_frame_id.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(call_frame_id))
            })?,
            function_name: self.function_name.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(function_name))
            })?,
            function_location: self.function_location,
            location: self
                .location
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(location)))?,
            scope_chain: self
                .scope_chain
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(scope_chain)))?,
            this: self
                .this
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(this)))?,
            return_value: self.return_value,
            can_be_restarted: self.can_be_restarted,
        })
    }
}
impl Scope {
    pub fn builder() -> ScopeBuilder {
        ScopeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ScopeBuilder {
    r#type: Option<ScopeType>,
    object: Option<super::super::runtime::types::RemoteObject>,
    name: Option<String>,
    start_location: Option<Location>,
    end_location: Option<Location>,
}
impl ScopeBuilder {
    pub fn r#type(mut self, r#type: impl Into<ScopeType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn object(mut self, object: impl Into<super::super::runtime::types::RemoteObject>) -> Self {
        self.object = Some(object.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn start_location(mut self, start_location: impl Into<Location>) -> Self {
        self.start_location = Some(start_location.into());
        self
    }
    pub fn end_location(mut self, end_location: impl Into<Location>) -> Self {
        self.end_location = Some(end_location.into());
        self
    }
    pub fn build(self) -> Result<Scope, String> {
        Ok(Scope {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            object: self
                .object
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(object)))?,
            name: self.name,
            start_location: self.start_location,
            end_location: self.end_location,
        })
    }
}
impl SearchMatch {
    pub fn builder() -> SearchMatchBuilder {
        SearchMatchBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SearchMatchBuilder {
    line_number: Option<f64>,
    line_content: Option<String>,
}
impl SearchMatchBuilder {
    pub fn line_number(mut self, line_number: impl Into<f64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn line_content(mut self, line_content: impl Into<String>) -> Self {
        self.line_content = Some(line_content.into());
        self
    }
    pub fn build(self) -> Result<SearchMatch, String> {
        Ok(SearchMatch {
            line_number: self
                .line_number
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line_number)))?,
            line_content: self.line_content.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(line_content))
            })?,
        })
    }
}
impl BreakLocation {
    pub fn builder() -> BreakLocationBuilder {
        BreakLocationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct BreakLocationBuilder {
    script_id: Option<super::super::runtime::types::ScriptId>,
    line_number: Option<i64>,
    column_number: Option<i64>,
    r#type: Option<BreakLocationType>,
}
impl BreakLocationBuilder {
    pub fn script_id(
        mut self,
        script_id: impl Into<super::super::runtime::types::ScriptId>,
    ) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn line_number(mut self, line_number: impl Into<i64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn column_number(mut self, column_number: impl Into<i64>) -> Self {
        self.column_number = Some(column_number.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<BreakLocationType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<BreakLocation, String> {
        Ok(BreakLocation {
            script_id: self
                .script_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(script_id)))?,
            line_number: self
                .line_number
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line_number)))?,
            column_number: self.column_number,
            r#type: self.r#type,
        })
    }
}
impl WasmDisassemblyChunk {
    pub fn builder() -> WasmDisassemblyChunkBuilder {
        WasmDisassemblyChunkBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct WasmDisassemblyChunkBuilder {
    lines: Option<Vec<String>>,
    bytecode_offsets: Option<Vec<i64>>,
}
impl WasmDisassemblyChunkBuilder {
    pub fn line(mut self, line: impl Into<String>) -> Self {
        let v = self.lines.get_or_insert(Vec::new());
        v.push(line.into());
        self
    }
    pub fn lines<I, S>(mut self, lines: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.lines.get_or_insert(Vec::new());
        for val in lines {
            v.push(val.into());
        }
        self
    }
    pub fn bytecode_offset(mut self, bytecode_offset: impl Into<i64>) -> Self {
        let v = self.bytecode_offsets.get_or_insert(Vec::new());
        v.push(bytecode_offset.into());
        self
    }
    pub fn bytecode_offsets<I, S>(mut self, bytecode_offsets: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<i64>,
    {
        let v = self.bytecode_offsets.get_or_insert(Vec::new());
        for val in bytecode_offsets {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<WasmDisassemblyChunk, String> {
        Ok(WasmDisassemblyChunk {
            lines: self
                .lines
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(lines)))?,
            bytecode_offsets: self.bytecode_offsets.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(bytecode_offsets)
                )
            })?,
        })
    }
}
impl DebugSymbols {
    pub fn builder() -> DebugSymbolsBuilder {
        DebugSymbolsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DebugSymbolsBuilder {
    r#type: Option<DebugSymbolsType>,
    external_url: Option<String>,
}
impl DebugSymbolsBuilder {
    pub fn r#type(mut self, r#type: impl Into<DebugSymbolsType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn external_url(mut self, external_url: impl Into<String>) -> Self {
        self.external_url = Some(external_url.into());
        self
    }
    pub fn build(self) -> Result<DebugSymbols, String> {
        Ok(DebugSymbols {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            external_url: self.external_url,
        })
    }
}
impl ResolvedBreakpoint {
    pub fn builder() -> ResolvedBreakpointBuilder {
        ResolvedBreakpointBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ResolvedBreakpointBuilder {
    breakpoint_id: Option<BreakpointId>,
    location: Option<Location>,
}
impl ResolvedBreakpointBuilder {
    pub fn breakpoint_id(mut self, breakpoint_id: impl Into<BreakpointId>) -> Self {
        self.breakpoint_id = Some(breakpoint_id.into());
        self
    }
    pub fn location(mut self, location: impl Into<Location>) -> Self {
        self.location = Some(location.into());
        self
    }
    pub fn build(self) -> Result<ResolvedBreakpoint, String> {
        Ok(ResolvedBreakpoint {
            breakpoint_id: self.breakpoint_id.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(breakpoint_id))
            })?,
            location: self
                .location
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(location)))?,
        })
    }
}
