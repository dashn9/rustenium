use super::commands::*;
impl ContinueToLocation {
    pub fn builder() -> ContinueToLocationBuilder {
        ContinueToLocationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ContinueToLocationBuilder {
    location: Option<super::types::Location>,
    target_call_frames: Option<ContinueToLocationTargetCallFrames>,
}
impl ContinueToLocationBuilder {
    pub fn location(mut self, location: impl Into<super::types::Location>) -> Self {
        self.location = Some(location.into());
        self
    }
    pub fn target_call_frames(
        mut self,
        target_call_frames: impl Into<ContinueToLocationTargetCallFrames>,
    ) -> Self {
        self.target_call_frames = Some(target_call_frames.into());
        self
    }
    pub fn build(self) -> Result<ContinueToLocation, String> {
        Ok(ContinueToLocation {
            method: ContinueToLocationMethod::ContinueToLocation,
            params: ContinueToLocationParams {
                location: self.location.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(location))
                })?,
                target_call_frames: self.target_call_frames,
            },
        })
    }
}
impl Enable {
    pub fn builder() -> EnableBuilder {
        EnableBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct EnableBuilder {
    max_scripts_cache_size: Option<f64>,
}
impl EnableBuilder {
    pub fn max_scripts_cache_size(mut self, max_scripts_cache_size: impl Into<f64>) -> Self {
        self.max_scripts_cache_size = Some(max_scripts_cache_size.into());
        self
    }
    pub fn build(self) -> Enable {
        Enable {
            method: EnableMethod::Enable,
            params: EnableParams {
                max_scripts_cache_size: self.max_scripts_cache_size,
            },
        }
    }
}
impl EvaluateOnCallFrame {
    pub fn builder() -> EvaluateOnCallFrameBuilder {
        EvaluateOnCallFrameBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct EvaluateOnCallFrameBuilder {
    call_frame_id: Option<super::types::CallFrameId>,
    expression: Option<String>,
    object_group: Option<String>,
    include_command_line_api: Option<bool>,
    silent: Option<bool>,
    return_by_value: Option<bool>,
    generate_preview: Option<bool>,
    throw_on_side_effect: Option<bool>,
    timeout: Option<super::super::runtime::types::TimeDelta>,
}
impl EvaluateOnCallFrameBuilder {
    pub fn call_frame_id(mut self, call_frame_id: impl Into<super::types::CallFrameId>) -> Self {
        self.call_frame_id = Some(call_frame_id.into());
        self
    }
    pub fn expression(mut self, expression: impl Into<String>) -> Self {
        self.expression = Some(expression.into());
        self
    }
    pub fn object_group(mut self, object_group: impl Into<String>) -> Self {
        self.object_group = Some(object_group.into());
        self
    }
    pub fn include_command_line_api(mut self, include_command_line_api: impl Into<bool>) -> Self {
        self.include_command_line_api = Some(include_command_line_api.into());
        self
    }
    pub fn silent(mut self, silent: impl Into<bool>) -> Self {
        self.silent = Some(silent.into());
        self
    }
    pub fn return_by_value(mut self, return_by_value: impl Into<bool>) -> Self {
        self.return_by_value = Some(return_by_value.into());
        self
    }
    pub fn generate_preview(mut self, generate_preview: impl Into<bool>) -> Self {
        self.generate_preview = Some(generate_preview.into());
        self
    }
    pub fn throw_on_side_effect(mut self, throw_on_side_effect: impl Into<bool>) -> Self {
        self.throw_on_side_effect = Some(throw_on_side_effect.into());
        self
    }
    pub fn timeout(mut self, timeout: impl Into<super::super::runtime::types::TimeDelta>) -> Self {
        self.timeout = Some(timeout.into());
        self
    }
    pub fn build(self) -> Result<EvaluateOnCallFrame, String> {
        Ok(EvaluateOnCallFrame {
            method: EvaluateOnCallFrameMethod::EvaluateOnCallFrame,
            params: EvaluateOnCallFrameParams {
                call_frame_id: self.call_frame_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(call_frame_id))
                })?,
                expression: self.expression.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(expression))
                })?,
                object_group: self.object_group,
                include_command_line_api: self.include_command_line_api,
                silent: self.silent,
                return_by_value: self.return_by_value,
                generate_preview: self.generate_preview,
                throw_on_side_effect: self.throw_on_side_effect,
                timeout: self.timeout,
            },
        })
    }
}
impl GetPossibleBreakpoints {
    pub fn builder() -> GetPossibleBreakpointsBuilder {
        GetPossibleBreakpointsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetPossibleBreakpointsBuilder {
    start: Option<super::types::Location>,
    end: Option<super::types::Location>,
    restrict_to_function: Option<bool>,
}
impl GetPossibleBreakpointsBuilder {
    pub fn start(mut self, start: impl Into<super::types::Location>) -> Self {
        self.start = Some(start.into());
        self
    }
    pub fn end(mut self, end: impl Into<super::types::Location>) -> Self {
        self.end = Some(end.into());
        self
    }
    pub fn restrict_to_function(mut self, restrict_to_function: impl Into<bool>) -> Self {
        self.restrict_to_function = Some(restrict_to_function.into());
        self
    }
    pub fn build(self) -> Result<GetPossibleBreakpoints, String> {
        Ok(GetPossibleBreakpoints {
            method: GetPossibleBreakpointsMethod::GetPossibleBreakpoints,
            params: GetPossibleBreakpointsParams {
                start: self
                    .start
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(start)))?,
                end: self.end,
                restrict_to_function: self.restrict_to_function,
            },
        })
    }
}
impl GetScriptSource {
    pub fn builder() -> GetScriptSourceBuilder {
        GetScriptSourceBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetScriptSourceBuilder {
    script_id: Option<super::super::runtime::types::ScriptId>,
}
impl GetScriptSourceBuilder {
    pub fn script_id(
        mut self,
        script_id: impl Into<super::super::runtime::types::ScriptId>,
    ) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn build(self) -> Result<GetScriptSource, String> {
        Ok(GetScriptSource {
            method: GetScriptSourceMethod::GetScriptSource,
            params: GetScriptSourceParams {
                script_id: self.script_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(script_id))
                })?,
            },
        })
    }
}
impl DisassembleWasmModule {
    pub fn builder() -> DisassembleWasmModuleBuilder {
        DisassembleWasmModuleBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DisassembleWasmModuleBuilder {
    script_id: Option<super::super::runtime::types::ScriptId>,
}
impl DisassembleWasmModuleBuilder {
    pub fn script_id(
        mut self,
        script_id: impl Into<super::super::runtime::types::ScriptId>,
    ) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn build(self) -> Result<DisassembleWasmModule, String> {
        Ok(DisassembleWasmModule {
            method: DisassembleWasmModuleMethod::DisassembleWasmModule,
            params: DisassembleWasmModuleParams {
                script_id: self.script_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(script_id))
                })?,
            },
        })
    }
}
impl NextWasmDisassemblyChunk {
    pub fn builder() -> NextWasmDisassemblyChunkBuilder {
        NextWasmDisassemblyChunkBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct NextWasmDisassemblyChunkBuilder {
    stream_id: Option<String>,
}
impl NextWasmDisassemblyChunkBuilder {
    pub fn stream_id(mut self, stream_id: impl Into<String>) -> Self {
        self.stream_id = Some(stream_id.into());
        self
    }
    pub fn build(self) -> Result<NextWasmDisassemblyChunk, String> {
        Ok(NextWasmDisassemblyChunk {
            method: NextWasmDisassemblyChunkMethod::NextWasmDisassemblyChunk,
            params: NextWasmDisassemblyChunkParams {
                stream_id: self.stream_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(stream_id))
                })?,
            },
        })
    }
}
impl GetStackTrace {
    pub fn builder() -> GetStackTraceBuilder {
        GetStackTraceBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetStackTraceBuilder {
    stack_trace_id: Option<super::super::runtime::types::StackTraceId>,
}
impl GetStackTraceBuilder {
    pub fn stack_trace_id(
        mut self,
        stack_trace_id: impl Into<super::super::runtime::types::StackTraceId>,
    ) -> Self {
        self.stack_trace_id = Some(stack_trace_id.into());
        self
    }
    pub fn build(self) -> Result<GetStackTrace, String> {
        Ok(GetStackTrace {
            method: GetStackTraceMethod::GetStackTrace,
            params: GetStackTraceParams {
                stack_trace_id: self.stack_trace_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(stack_trace_id))
                })?,
            },
        })
    }
}
impl RemoveBreakpoint {
    pub fn builder() -> RemoveBreakpointBuilder {
        RemoveBreakpointBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveBreakpointBuilder {
    breakpoint_id: Option<super::types::BreakpointId>,
}
impl RemoveBreakpointBuilder {
    pub fn breakpoint_id(mut self, breakpoint_id: impl Into<super::types::BreakpointId>) -> Self {
        self.breakpoint_id = Some(breakpoint_id.into());
        self
    }
    pub fn build(self) -> Result<RemoveBreakpoint, String> {
        Ok(RemoveBreakpoint {
            method: RemoveBreakpointMethod::RemoveBreakpoint,
            params: RemoveBreakpointParams {
                breakpoint_id: self.breakpoint_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(breakpoint_id))
                })?,
            },
        })
    }
}
impl RestartFrame {
    pub fn builder() -> RestartFrameBuilder {
        RestartFrameBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RestartFrameBuilder {
    call_frame_id: Option<super::types::CallFrameId>,
    mode: Option<RestartFrameMode>,
}
impl RestartFrameBuilder {
    pub fn call_frame_id(mut self, call_frame_id: impl Into<super::types::CallFrameId>) -> Self {
        self.call_frame_id = Some(call_frame_id.into());
        self
    }
    pub fn mode(mut self, mode: impl Into<RestartFrameMode>) -> Self {
        self.mode = Some(mode.into());
        self
    }
    pub fn build(self) -> Result<RestartFrame, String> {
        Ok(RestartFrame {
            method: RestartFrameMethod::RestartFrame,
            params: RestartFrameParams {
                call_frame_id: self.call_frame_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(call_frame_id))
                })?,
                mode: self.mode,
            },
        })
    }
}
impl Resume {
    pub fn builder() -> ResumeBuilder {
        ResumeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ResumeBuilder {
    terminate_on_resume: Option<bool>,
}
impl ResumeBuilder {
    pub fn terminate_on_resume(mut self, terminate_on_resume: impl Into<bool>) -> Self {
        self.terminate_on_resume = Some(terminate_on_resume.into());
        self
    }
    pub fn build(self) -> Resume {
        Resume {
            method: ResumeMethod::Resume,
            params: ResumeParams {
                terminate_on_resume: self.terminate_on_resume,
            },
        }
    }
}
impl SearchInContent {
    pub fn builder() -> SearchInContentBuilder {
        SearchInContentBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SearchInContentBuilder {
    script_id: Option<super::super::runtime::types::ScriptId>,
    query: Option<String>,
    case_sensitive: Option<bool>,
    is_regex: Option<bool>,
}
impl SearchInContentBuilder {
    pub fn script_id(
        mut self,
        script_id: impl Into<super::super::runtime::types::ScriptId>,
    ) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }
    pub fn case_sensitive(mut self, case_sensitive: impl Into<bool>) -> Self {
        self.case_sensitive = Some(case_sensitive.into());
        self
    }
    pub fn is_regex(mut self, is_regex: impl Into<bool>) -> Self {
        self.is_regex = Some(is_regex.into());
        self
    }
    pub fn build(self) -> Result<SearchInContent, String> {
        Ok(SearchInContent {
            method: SearchInContentMethod::SearchInContent,
            params: SearchInContentParams {
                script_id: self.script_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(script_id))
                })?,
                query: self
                    .query
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(query)))?,
                case_sensitive: self.case_sensitive,
                is_regex: self.is_regex,
            },
        })
    }
}
impl SetAsyncCallStackDepth {
    pub fn builder() -> SetAsyncCallStackDepthBuilder {
        SetAsyncCallStackDepthBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetAsyncCallStackDepthBuilder {
    max_depth: Option<i64>,
}
impl SetAsyncCallStackDepthBuilder {
    pub fn max_depth(mut self, max_depth: impl Into<i64>) -> Self {
        self.max_depth = Some(max_depth.into());
        self
    }
    pub fn build(self) -> Result<SetAsyncCallStackDepth, String> {
        Ok(SetAsyncCallStackDepth {
            method: SetAsyncCallStackDepthMethod::SetAsyncCallStackDepth,
            params: SetAsyncCallStackDepthParams {
                max_depth: self.max_depth.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(max_depth))
                })?,
            },
        })
    }
}
impl SetBlackboxExecutionContexts {
    pub fn builder() -> SetBlackboxExecutionContextsBuilder {
        SetBlackboxExecutionContextsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetBlackboxExecutionContextsBuilder {
    unique_ids: Option<Vec<String>>,
}
impl SetBlackboxExecutionContextsBuilder {
    pub fn unique_id(mut self, unique_id: impl Into<String>) -> Self {
        let v = self.unique_ids.get_or_insert(Vec::new());
        v.push(unique_id.into());
        self
    }
    pub fn unique_ids<I, S>(mut self, unique_ids: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.unique_ids.get_or_insert(Vec::new());
        for val in unique_ids {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetBlackboxExecutionContexts, String> {
        Ok(SetBlackboxExecutionContexts {
            method: SetBlackboxExecutionContextsMethod::SetBlackboxExecutionContexts,
            params: SetBlackboxExecutionContextsParams {
                unique_ids: self.unique_ids.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(unique_ids))
                })?,
            },
        })
    }
}
impl SetBlackboxPatterns {
    pub fn builder() -> SetBlackboxPatternsBuilder {
        SetBlackboxPatternsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetBlackboxPatternsBuilder {
    patterns: Option<Vec<String>>,
    skip_anonymous: Option<bool>,
}
impl SetBlackboxPatternsBuilder {
    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        let v = self.patterns.get_or_insert(Vec::new());
        v.push(pattern.into());
        self
    }
    pub fn patterns<I, S>(mut self, patterns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.patterns.get_or_insert(Vec::new());
        for val in patterns {
            v.push(val.into());
        }
        self
    }
    pub fn skip_anonymous(mut self, skip_anonymous: impl Into<bool>) -> Self {
        self.skip_anonymous = Some(skip_anonymous.into());
        self
    }
    pub fn build(self) -> Result<SetBlackboxPatterns, String> {
        Ok(SetBlackboxPatterns {
            method: SetBlackboxPatternsMethod::SetBlackboxPatterns,
            params: SetBlackboxPatternsParams {
                patterns: self.patterns.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(patterns))
                })?,
                skip_anonymous: self.skip_anonymous,
            },
        })
    }
}
impl SetBlackboxedRanges {
    pub fn builder() -> SetBlackboxedRangesBuilder {
        SetBlackboxedRangesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetBlackboxedRangesBuilder {
    script_id: Option<super::super::runtime::types::ScriptId>,
    positions: Option<Vec<super::types::ScriptPosition>>,
}
impl SetBlackboxedRangesBuilder {
    pub fn script_id(
        mut self,
        script_id: impl Into<super::super::runtime::types::ScriptId>,
    ) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn position(mut self, position: impl Into<super::types::ScriptPosition>) -> Self {
        let v = self.positions.get_or_insert(Vec::new());
        v.push(position.into());
        self
    }
    pub fn positions<I, S>(mut self, positions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::ScriptPosition>,
    {
        let v = self.positions.get_or_insert(Vec::new());
        for val in positions {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetBlackboxedRanges, String> {
        Ok(SetBlackboxedRanges {
            method: SetBlackboxedRangesMethod::SetBlackboxedRanges,
            params: SetBlackboxedRangesParams {
                script_id: self.script_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(script_id))
                })?,
                positions: self.positions.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(positions))
                })?,
            },
        })
    }
}
impl SetBreakpoint {
    pub fn builder() -> SetBreakpointBuilder {
        SetBreakpointBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetBreakpointBuilder {
    location: Option<super::types::Location>,
    condition: Option<String>,
}
impl SetBreakpointBuilder {
    pub fn location(mut self, location: impl Into<super::types::Location>) -> Self {
        self.location = Some(location.into());
        self
    }
    pub fn condition(mut self, condition: impl Into<String>) -> Self {
        self.condition = Some(condition.into());
        self
    }
    pub fn build(self) -> Result<SetBreakpoint, String> {
        Ok(SetBreakpoint {
            method: SetBreakpointMethod::SetBreakpoint,
            params: SetBreakpointParams {
                location: self.location.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(location))
                })?,
                condition: self.condition,
            },
        })
    }
}
impl SetInstrumentationBreakpoint {
    pub fn builder() -> SetInstrumentationBreakpointBuilder {
        SetInstrumentationBreakpointBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetInstrumentationBreakpointBuilder {
    instrumentation: Option<SetInstrumentationBreakpointInstrumentation>,
}
impl SetInstrumentationBreakpointBuilder {
    pub fn instrumentation(
        mut self,
        instrumentation: impl Into<SetInstrumentationBreakpointInstrumentation>,
    ) -> Self {
        self.instrumentation = Some(instrumentation.into());
        self
    }
    pub fn build(self) -> Result<SetInstrumentationBreakpoint, String> {
        Ok(SetInstrumentationBreakpoint {
            method: SetInstrumentationBreakpointMethod::SetInstrumentationBreakpoint,
            params: SetInstrumentationBreakpointParams {
                instrumentation: self.instrumentation.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(instrumentation))
                })?,
            },
        })
    }
}
impl SetBreakpointByUrl {
    pub fn builder() -> SetBreakpointByUrlBuilder {
        SetBreakpointByUrlBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetBreakpointByUrlBuilder {
    line_number: Option<i64>,
    url: Option<String>,
    url_regex: Option<String>,
    script_hash: Option<String>,
    column_number: Option<i64>,
    condition: Option<String>,
}
impl SetBreakpointByUrlBuilder {
    pub fn line_number(mut self, line_number: impl Into<i64>) -> Self {
        self.line_number = Some(line_number.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn url_regex(mut self, url_regex: impl Into<String>) -> Self {
        self.url_regex = Some(url_regex.into());
        self
    }
    pub fn script_hash(mut self, script_hash: impl Into<String>) -> Self {
        self.script_hash = Some(script_hash.into());
        self
    }
    pub fn column_number(mut self, column_number: impl Into<i64>) -> Self {
        self.column_number = Some(column_number.into());
        self
    }
    pub fn condition(mut self, condition: impl Into<String>) -> Self {
        self.condition = Some(condition.into());
        self
    }
    pub fn build(self) -> Result<SetBreakpointByUrl, String> {
        Ok(SetBreakpointByUrl {
            method: SetBreakpointByUrlMethod::SetBreakpointByUrl,
            params: SetBreakpointByUrlParams {
                line_number: self.line_number.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(line_number))
                })?,
                url: self.url,
                url_regex: self.url_regex,
                script_hash: self.script_hash,
                column_number: self.column_number,
                condition: self.condition,
            },
        })
    }
}
impl SetBreakpointOnFunctionCall {
    pub fn builder() -> SetBreakpointOnFunctionCallBuilder {
        SetBreakpointOnFunctionCallBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetBreakpointOnFunctionCallBuilder {
    object_id: Option<super::super::runtime::types::RemoteObjectId>,
    condition: Option<String>,
}
impl SetBreakpointOnFunctionCallBuilder {
    pub fn object_id(
        mut self,
        object_id: impl Into<super::super::runtime::types::RemoteObjectId>,
    ) -> Self {
        self.object_id = Some(object_id.into());
        self
    }
    pub fn condition(mut self, condition: impl Into<String>) -> Self {
        self.condition = Some(condition.into());
        self
    }
    pub fn build(self) -> Result<SetBreakpointOnFunctionCall, String> {
        Ok(SetBreakpointOnFunctionCall {
            method: SetBreakpointOnFunctionCallMethod::SetBreakpointOnFunctionCall,
            params: SetBreakpointOnFunctionCallParams {
                object_id: self.object_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(object_id))
                })?,
                condition: self.condition,
            },
        })
    }
}
impl SetBreakpointsActive {
    pub fn builder() -> SetBreakpointsActiveBuilder {
        SetBreakpointsActiveBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetBreakpointsActiveBuilder {
    active: Option<bool>,
}
impl SetBreakpointsActiveBuilder {
    pub fn active(mut self, active: impl Into<bool>) -> Self {
        self.active = Some(active.into());
        self
    }
    pub fn build(self) -> Result<SetBreakpointsActive, String> {
        Ok(SetBreakpointsActive {
            method: SetBreakpointsActiveMethod::SetBreakpointsActive,
            params: SetBreakpointsActiveParams {
                active: self
                    .active
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(active)))?,
            },
        })
    }
}
impl SetPauseOnExceptions {
    pub fn builder() -> SetPauseOnExceptionsBuilder {
        SetPauseOnExceptionsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetPauseOnExceptionsBuilder {
    state: Option<SetPauseOnExceptionsState>,
}
impl SetPauseOnExceptionsBuilder {
    pub fn state(mut self, state: impl Into<SetPauseOnExceptionsState>) -> Self {
        self.state = Some(state.into());
        self
    }
    pub fn build(self) -> Result<SetPauseOnExceptions, String> {
        Ok(SetPauseOnExceptions {
            method: SetPauseOnExceptionsMethod::SetPauseOnExceptions,
            params: SetPauseOnExceptionsParams {
                state: self
                    .state
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(state)))?,
            },
        })
    }
}
impl SetReturnValue {
    pub fn builder() -> SetReturnValueBuilder {
        SetReturnValueBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetReturnValueBuilder {
    new_value: Option<super::super::runtime::types::CallArgument>,
}
impl SetReturnValueBuilder {
    pub fn new_value(
        mut self,
        new_value: impl Into<super::super::runtime::types::CallArgument>,
    ) -> Self {
        self.new_value = Some(new_value.into());
        self
    }
    pub fn build(self) -> Result<SetReturnValue, String> {
        Ok(SetReturnValue {
            method: SetReturnValueMethod::SetReturnValue,
            params: SetReturnValueParams {
                new_value: self.new_value.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(new_value))
                })?,
            },
        })
    }
}
impl SetScriptSource {
    pub fn builder() -> SetScriptSourceBuilder {
        SetScriptSourceBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetScriptSourceBuilder {
    script_id: Option<super::super::runtime::types::ScriptId>,
    script_source: Option<String>,
    dry_run: Option<bool>,
    allow_top_frame_editing: Option<bool>,
}
impl SetScriptSourceBuilder {
    pub fn script_id(
        mut self,
        script_id: impl Into<super::super::runtime::types::ScriptId>,
    ) -> Self {
        self.script_id = Some(script_id.into());
        self
    }
    pub fn script_source(mut self, script_source: impl Into<String>) -> Self {
        self.script_source = Some(script_source.into());
        self
    }
    pub fn dry_run(mut self, dry_run: impl Into<bool>) -> Self {
        self.dry_run = Some(dry_run.into());
        self
    }
    pub fn allow_top_frame_editing(mut self, allow_top_frame_editing: impl Into<bool>) -> Self {
        self.allow_top_frame_editing = Some(allow_top_frame_editing.into());
        self
    }
    pub fn build(self) -> Result<SetScriptSource, String> {
        Ok(SetScriptSource {
            method: SetScriptSourceMethod::SetScriptSource,
            params: SetScriptSourceParams {
                script_id: self.script_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(script_id))
                })?,
                script_source: self.script_source.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(script_source))
                })?,
                dry_run: self.dry_run,
                allow_top_frame_editing: self.allow_top_frame_editing,
            },
        })
    }
}
impl SetSkipAllPauses {
    pub fn builder() -> SetSkipAllPausesBuilder {
        SetSkipAllPausesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetSkipAllPausesBuilder {
    skip: Option<bool>,
}
impl SetSkipAllPausesBuilder {
    pub fn skip(mut self, skip: impl Into<bool>) -> Self {
        self.skip = Some(skip.into());
        self
    }
    pub fn build(self) -> Result<SetSkipAllPauses, String> {
        Ok(SetSkipAllPauses {
            method: SetSkipAllPausesMethod::SetSkipAllPauses,
            params: SetSkipAllPausesParams {
                skip: self
                    .skip
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(skip)))?,
            },
        })
    }
}
impl SetVariableValue {
    pub fn builder() -> SetVariableValueBuilder {
        SetVariableValueBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetVariableValueBuilder {
    scope_number: Option<i64>,
    variable_name: Option<String>,
    new_value: Option<super::super::runtime::types::CallArgument>,
    call_frame_id: Option<super::types::CallFrameId>,
}
impl SetVariableValueBuilder {
    pub fn scope_number(mut self, scope_number: impl Into<i64>) -> Self {
        self.scope_number = Some(scope_number.into());
        self
    }
    pub fn variable_name(mut self, variable_name: impl Into<String>) -> Self {
        self.variable_name = Some(variable_name.into());
        self
    }
    pub fn new_value(
        mut self,
        new_value: impl Into<super::super::runtime::types::CallArgument>,
    ) -> Self {
        self.new_value = Some(new_value.into());
        self
    }
    pub fn call_frame_id(mut self, call_frame_id: impl Into<super::types::CallFrameId>) -> Self {
        self.call_frame_id = Some(call_frame_id.into());
        self
    }
    pub fn build(self) -> Result<SetVariableValue, String> {
        Ok(SetVariableValue {
            method: SetVariableValueMethod::SetVariableValue,
            params: SetVariableValueParams {
                scope_number: self.scope_number.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(scope_number))
                })?,
                variable_name: self.variable_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(variable_name))
                })?,
                new_value: self.new_value.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(new_value))
                })?,
                call_frame_id: self.call_frame_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(call_frame_id))
                })?,
            },
        })
    }
}
impl StepInto {
    pub fn builder() -> StepIntoBuilder {
        StepIntoBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct StepIntoBuilder {
    break_on_async_call: Option<bool>,
    skip_list: Option<Vec<super::types::LocationRange>>,
}
impl StepIntoBuilder {
    pub fn break_on_async_call(mut self, break_on_async_call: impl Into<bool>) -> Self {
        self.break_on_async_call = Some(break_on_async_call.into());
        self
    }
    pub fn skip_list(mut self, skip_list: impl Into<super::types::LocationRange>) -> Self {
        let v = self.skip_list.get_or_insert(Vec::new());
        v.push(skip_list.into());
        self
    }
    pub fn skip_lists<I, S>(mut self, skip_lists: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::LocationRange>,
    {
        let v = self.skip_list.get_or_insert(Vec::new());
        for val in skip_lists {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> StepInto {
        StepInto {
            method: StepIntoMethod::StepInto,
            params: StepIntoParams {
                break_on_async_call: self.break_on_async_call,
                skip_list: self.skip_list,
            },
        }
    }
}
impl StepOver {
    pub fn builder() -> StepOverBuilder {
        StepOverBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct StepOverBuilder {
    skip_list: Option<Vec<super::types::LocationRange>>,
}
impl StepOverBuilder {
    pub fn skip_list(mut self, skip_list: impl Into<super::types::LocationRange>) -> Self {
        let v = self.skip_list.get_or_insert(Vec::new());
        v.push(skip_list.into());
        self
    }
    pub fn skip_lists<I, S>(mut self, skip_lists: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::LocationRange>,
    {
        let v = self.skip_list.get_or_insert(Vec::new());
        for val in skip_lists {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> StepOver {
        StepOver {
            method: StepOverMethod::StepOver,
            params: StepOverParams {
                skip_list: self.skip_list,
            },
        }
    }
}
