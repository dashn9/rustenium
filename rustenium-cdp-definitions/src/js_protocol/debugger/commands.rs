use serde::{Deserialize, Serialize};
#[doc = "Continues execution until specific location is reached.\n[continueToLocation](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-continueToLocation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueToLocationParams {
    #[doc = "Location to continue to."]
    #[serde(rename = "location")]
    pub location: super::types::Location,
    #[serde(rename = "targetCallFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub target_call_frames: Option<ContinueToLocationTargetCallFrames>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContinueToLocationTargetCallFrames {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "current")]
    Current,
}
impl ContinueToLocationParams {
    pub fn new(location: impl Into<super::types::Location>) -> Self {
        Self {
            location: location.into(),
            target_call_frames: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContinueToLocationMethod {
    #[serde(rename = "Debugger.continueToLocation")]
    ContinueToLocation,
}
#[doc = "Continues execution until specific location is reached.\n[continueToLocation](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-continueToLocation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContinueToLocation {
    pub method: ContinueToLocationMethod,
    pub params: ContinueToLocationParams,
}
impl ContinueToLocation {
    pub const IDENTIFIER: &'static str = "Debugger.continueToLocation";
}
impl crate::CommandResult for ContinueToLocation {
    type Result = super::results::ContinueToLocationResult;
}
#[doc = "Disables debugger for given page.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Debugger.disable")]
    Disable,
}
#[doc = "Disables debugger for given page.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "Debugger.disable";
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables debugger for the given page. Clients should not assume that the debugging has been\nenabled until the result for this command is received.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {
    #[doc = "The maximum size in bytes of collected scripts (not referenced by other heap objects)\nthe debugger can hold. Puts no limit if parameter is omitted."]
    #[serde(rename = "maxScriptsCacheSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_scripts_cache_size: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Debugger.enable")]
    Enable,
}
#[doc = "Enables debugger for the given page. Clients should not assume that the debugging has been\nenabled until the result for this command is received.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "Debugger.enable";
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Evaluates expression on a given call frame.\n[evaluateOnCallFrame](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-evaluateOnCallFrame)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluateOnCallFrameParams {
    #[doc = "Call frame identifier to evaluate on."]
    #[serde(rename = "callFrameId")]
    pub call_frame_id: super::types::CallFrameId,
    #[doc = "Expression to evaluate."]
    #[serde(rename = "expression")]
    pub expression: String,
    #[doc = "String object group name to put result into (allows rapid releasing resulting object handles\nusing `releaseObjectGroup`)."]
    #[serde(rename = "objectGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_group: Option<String>,
    #[doc = "Specifies whether command line API should be available to the evaluated expression, defaults\nto false."]
    #[serde(rename = "includeCommandLineAPI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_command_line_api: Option<bool>,
    #[doc = "In silent mode exceptions thrown during evaluation are not reported and do not pause\nexecution. Overrides `setPauseOnException` state."]
    #[serde(rename = "silent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub silent: Option<bool>,
    #[doc = "Whether the result is expected to be a JSON object that should be sent by value."]
    #[serde(rename = "returnByValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub return_by_value: Option<bool>,
    #[doc = "Whether preview should be generated for the result."]
    #[serde(rename = "generatePreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub generate_preview: Option<bool>,
    #[doc = "Whether to throw an exception if side effect cannot be ruled out during evaluation."]
    #[serde(rename = "throwOnSideEffect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub throw_on_side_effect: Option<bool>,
    #[doc = "Terminate execution after timing out (number of milliseconds)."]
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub timeout: Option<crate::js_protocol::runtime::types::TimeDelta>,
}
impl EvaluateOnCallFrameParams {
    pub fn new(
        call_frame_id: impl Into<super::types::CallFrameId>,
        expression: impl Into<String>,
    ) -> Self {
        Self {
            call_frame_id: call_frame_id.into(),
            expression: expression.into(),
            object_group: None,
            include_command_line_api: None,
            silent: None,
            return_by_value: None,
            generate_preview: None,
            throw_on_side_effect: None,
            timeout: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EvaluateOnCallFrameMethod {
    #[serde(rename = "Debugger.evaluateOnCallFrame")]
    EvaluateOnCallFrame,
}
#[doc = "Evaluates expression on a given call frame.\n[evaluateOnCallFrame](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-evaluateOnCallFrame)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluateOnCallFrame {
    pub method: EvaluateOnCallFrameMethod,
    pub params: EvaluateOnCallFrameParams,
}
impl EvaluateOnCallFrame {
    pub const IDENTIFIER: &'static str = "Debugger.evaluateOnCallFrame";
}
impl crate::CommandResult for EvaluateOnCallFrame {
    type Result = super::results::EvaluateOnCallFrameResult;
}
#[doc = "Returns possible locations for breakpoint. scriptId in start and end range locations should be\nthe same.\n[getPossibleBreakpoints](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-getPossibleBreakpoints)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPossibleBreakpointsParams {
    #[doc = "Start of range to search possible breakpoint locations in."]
    #[serde(rename = "start")]
    pub start: super::types::Location,
    #[doc = "End of range to search possible breakpoint locations in (excluding). When not specified, end\nof scripts is used as end of range."]
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub end: Option<super::types::Location>,
    #[doc = "Only consider locations which are in the same (non-nested) function as start."]
    #[serde(rename = "restrictToFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub restrict_to_function: Option<bool>,
}
impl GetPossibleBreakpointsParams {
    pub fn new(start: impl Into<super::types::Location>) -> Self {
        Self {
            start: start.into(),
            end: None,
            restrict_to_function: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetPossibleBreakpointsMethod {
    #[serde(rename = "Debugger.getPossibleBreakpoints")]
    GetPossibleBreakpoints,
}
#[doc = "Returns possible locations for breakpoint. scriptId in start and end range locations should be\nthe same.\n[getPossibleBreakpoints](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-getPossibleBreakpoints)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPossibleBreakpoints {
    pub method: GetPossibleBreakpointsMethod,
    pub params: GetPossibleBreakpointsParams,
}
impl GetPossibleBreakpoints {
    pub const IDENTIFIER: &'static str = "Debugger.getPossibleBreakpoints";
}
impl crate::CommandResult for GetPossibleBreakpoints {
    type Result = super::results::GetPossibleBreakpointsResult;
}
#[doc = "Returns source for the script with given id.\n[getScriptSource](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-getScriptSource)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScriptSourceParams {
    #[doc = "Id of the script to get source for."]
    #[serde(rename = "scriptId")]
    pub script_id: crate::js_protocol::runtime::types::ScriptId,
}
impl GetScriptSourceParams {
    pub fn new(script_id: impl Into<crate::js_protocol::runtime::types::ScriptId>) -> Self {
        Self {
            script_id: script_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetScriptSourceMethod {
    #[serde(rename = "Debugger.getScriptSource")]
    GetScriptSource,
}
#[doc = "Returns source for the script with given id.\n[getScriptSource](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-getScriptSource)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScriptSource {
    pub method: GetScriptSourceMethod,
    pub params: GetScriptSourceParams,
}
impl GetScriptSource {
    pub const IDENTIFIER: &'static str = "Debugger.getScriptSource";
}
impl crate::CommandResult for GetScriptSource {
    type Result = super::results::GetScriptSourceResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisassembleWasmModuleParams {
    #[doc = "Id of the script to disassemble"]
    #[serde(rename = "scriptId")]
    pub script_id: crate::js_protocol::runtime::types::ScriptId,
}
impl DisassembleWasmModuleParams {
    pub fn new(script_id: impl Into<crate::js_protocol::runtime::types::ScriptId>) -> Self {
        Self {
            script_id: script_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisassembleWasmModuleMethod {
    #[serde(rename = "Debugger.disassembleWasmModule")]
    DisassembleWasmModule,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisassembleWasmModule {
    pub method: DisassembleWasmModuleMethod,
    pub params: DisassembleWasmModuleParams,
}
impl DisassembleWasmModule {
    pub const IDENTIFIER: &'static str = "Debugger.disassembleWasmModule";
}
impl crate::CommandResult for DisassembleWasmModule {
    type Result = super::results::DisassembleWasmModuleResult;
}
#[doc = "Disassemble the next chunk of lines for the module corresponding to the\nstream. If disassembly is complete, this API will invalidate the streamId\nand return an empty chunk. Any subsequent calls for the now invalid stream\nwill return errors.\n[nextWasmDisassemblyChunk](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-nextWasmDisassemblyChunk)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NextWasmDisassemblyChunkParams {
    #[serde(rename = "streamId")]
    pub stream_id: String,
}
impl NextWasmDisassemblyChunkParams {
    pub fn new(stream_id: impl Into<String>) -> Self {
        Self {
            stream_id: stream_id.into(),
        }
    }
}
impl<T: Into<String>> From<T> for NextWasmDisassemblyChunkParams {
    fn from(url: T) -> Self {
        NextWasmDisassemblyChunkParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NextWasmDisassemblyChunkMethod {
    #[serde(rename = "Debugger.nextWasmDisassemblyChunk")]
    NextWasmDisassemblyChunk,
}
#[doc = "Disassemble the next chunk of lines for the module corresponding to the\nstream. If disassembly is complete, this API will invalidate the streamId\nand return an empty chunk. Any subsequent calls for the now invalid stream\nwill return errors.\n[nextWasmDisassemblyChunk](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-nextWasmDisassemblyChunk)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NextWasmDisassemblyChunk {
    pub method: NextWasmDisassemblyChunkMethod,
    pub params: NextWasmDisassemblyChunkParams,
}
impl NextWasmDisassemblyChunk {
    pub const IDENTIFIER: &'static str = "Debugger.nextWasmDisassemblyChunk";
}
impl crate::CommandResult for NextWasmDisassemblyChunk {
    type Result = super::results::NextWasmDisassemblyChunkResult;
}
#[doc = "Returns stack trace with given `stackTraceId`.\n[getStackTrace](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-getStackTrace)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStackTraceParams {
    #[serde(rename = "stackTraceId")]
    pub stack_trace_id: crate::js_protocol::runtime::types::StackTraceId,
}
impl GetStackTraceParams {
    pub fn new(
        stack_trace_id: impl Into<crate::js_protocol::runtime::types::StackTraceId>,
    ) -> Self {
        Self {
            stack_trace_id: stack_trace_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetStackTraceMethod {
    #[serde(rename = "Debugger.getStackTrace")]
    GetStackTrace,
}
#[doc = "Returns stack trace with given `stackTraceId`.\n[getStackTrace](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-getStackTrace)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStackTrace {
    pub method: GetStackTraceMethod,
    pub params: GetStackTraceParams,
}
impl GetStackTrace {
    pub const IDENTIFIER: &'static str = "Debugger.getStackTrace";
}
impl crate::CommandResult for GetStackTrace {
    type Result = super::results::GetStackTraceResult;
}
#[doc = "Stops on the next JavaScript statement.\n[pause](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-pause)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PauseParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PauseMethod {
    #[serde(rename = "Debugger.pause")]
    Pause,
}
#[doc = "Stops on the next JavaScript statement.\n[pause](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-pause)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pause {
    pub method: PauseMethod,
    pub params: PauseParams,
}
impl Pause {
    pub const IDENTIFIER: &'static str = "Debugger.pause";
}
impl crate::CommandResult for Pause {
    type Result = super::results::PauseResult;
}
#[doc = "Removes JavaScript breakpoint.\n[removeBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-removeBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveBreakpointParams {
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: super::types::BreakpointId,
}
impl RemoveBreakpointParams {
    pub fn new(breakpoint_id: impl Into<super::types::BreakpointId>) -> Self {
        Self {
            breakpoint_id: breakpoint_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveBreakpointMethod {
    #[serde(rename = "Debugger.removeBreakpoint")]
    RemoveBreakpoint,
}
#[doc = "Removes JavaScript breakpoint.\n[removeBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-removeBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveBreakpoint {
    pub method: RemoveBreakpointMethod,
    pub params: RemoveBreakpointParams,
}
impl RemoveBreakpoint {
    pub const IDENTIFIER: &'static str = "Debugger.removeBreakpoint";
}
impl crate::CommandResult for RemoveBreakpoint {
    type Result = super::results::RemoveBreakpointResult;
}
#[doc = "Restarts particular call frame from the beginning. The old, deprecated\nbehavior of `restartFrame` is to stay paused and allow further CDP commands\nafter a restart was scheduled. This can cause problems with restarting, so\nwe now continue execution immediatly after it has been scheduled until we\nreach the beginning of the restarted frame.\n\nTo stay back-wards compatible, `restartFrame` now expects a `mode`\nparameter to be present. If the `mode` parameter is missing, `restartFrame`\nerrors out.\n\nThe various return values are deprecated and `callFrames` is always empty.\nUse the call frames from the `Debugger#paused` events instead, that fires\nonce V8 pauses at the beginning of the restarted function.\n[restartFrame](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-restartFrame)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestartFrameParams {
    #[doc = "Call frame identifier to evaluate on."]
    #[serde(rename = "callFrameId")]
    pub call_frame_id: super::types::CallFrameId,
    #[doc = "The `mode` parameter must be present and set to 'StepInto', otherwise\n`restartFrame` will error out."]
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub mode: Option<RestartFrameMode>,
}
#[doc = "The `mode` parameter must be present and set to 'StepInto', otherwise\n`restartFrame` will error out."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RestartFrameMode {
    #[doc = "Pause at the beginning of the restarted function"]
    #[serde(rename = "StepInto")]
    StepInto,
}
impl RestartFrameParams {
    pub fn new(call_frame_id: impl Into<super::types::CallFrameId>) -> Self {
        Self {
            call_frame_id: call_frame_id.into(),
            mode: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RestartFrameMethod {
    #[serde(rename = "Debugger.restartFrame")]
    RestartFrame,
}
#[doc = "Restarts particular call frame from the beginning. The old, deprecated\nbehavior of `restartFrame` is to stay paused and allow further CDP commands\nafter a restart was scheduled. This can cause problems with restarting, so\nwe now continue execution immediatly after it has been scheduled until we\nreach the beginning of the restarted frame.\n\nTo stay back-wards compatible, `restartFrame` now expects a `mode`\nparameter to be present. If the `mode` parameter is missing, `restartFrame`\nerrors out.\n\nThe various return values are deprecated and `callFrames` is always empty.\nUse the call frames from the `Debugger#paused` events instead, that fires\nonce V8 pauses at the beginning of the restarted function.\n[restartFrame](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-restartFrame)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RestartFrame {
    pub method: RestartFrameMethod,
    pub params: RestartFrameParams,
}
impl RestartFrame {
    pub const IDENTIFIER: &'static str = "Debugger.restartFrame";
}
impl crate::CommandResult for RestartFrame {
    type Result = super::results::RestartFrameResult;
}
#[doc = "Resumes JavaScript execution.\n[resume](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-resume)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResumeParams {
    #[doc = "Set to true to terminate execution upon resuming execution. In contrast\nto Runtime.terminateExecution, this will allows to execute further\nJavaScript (i.e. via evaluation) until execution of the paused code\nis actually resumed, at which point termination is triggered.\nIf execution is currently not paused, this parameter has no effect."]
    #[serde(rename = "terminateOnResume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub terminate_on_resume: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResumeMethod {
    #[serde(rename = "Debugger.resume")]
    Resume,
}
#[doc = "Resumes JavaScript execution.\n[resume](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-resume)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resume {
    pub method: ResumeMethod,
    pub params: ResumeParams,
}
impl Resume {
    pub const IDENTIFIER: &'static str = "Debugger.resume";
}
impl crate::CommandResult for Resume {
    type Result = super::results::ResumeResult;
}
#[doc = "Searches for given string in script content.\n[searchInContent](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-searchInContent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchInContentParams {
    #[doc = "Id of the script to search in."]
    #[serde(rename = "scriptId")]
    pub script_id: crate::js_protocol::runtime::types::ScriptId,
    #[doc = "String to search for."]
    #[serde(rename = "query")]
    pub query: String,
    #[doc = "If true, search is case sensitive."]
    #[serde(rename = "caseSensitive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub case_sensitive: Option<bool>,
    #[doc = "If true, treats string parameter as regex."]
    #[serde(rename = "isRegex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_regex: Option<bool>,
}
impl SearchInContentParams {
    pub fn new(
        script_id: impl Into<crate::js_protocol::runtime::types::ScriptId>,
        query: impl Into<String>,
    ) -> Self {
        Self {
            script_id: script_id.into(),
            query: query.into(),
            case_sensitive: None,
            is_regex: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SearchInContentMethod {
    #[serde(rename = "Debugger.searchInContent")]
    SearchInContent,
}
#[doc = "Searches for given string in script content.\n[searchInContent](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-searchInContent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchInContent {
    pub method: SearchInContentMethod,
    pub params: SearchInContentParams,
}
impl SearchInContent {
    pub const IDENTIFIER: &'static str = "Debugger.searchInContent";
}
impl crate::CommandResult for SearchInContent {
    type Result = super::results::SearchInContentResult;
}
#[doc = "Enables or disables async call stacks tracking.\n[setAsyncCallStackDepth](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setAsyncCallStackDepth)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAsyncCallStackDepthParams {
    #[doc = "Maximum depth of async call stacks. Setting to `0` will effectively disable collecting async\ncall stacks (default)."]
    #[serde(rename = "maxDepth")]
    pub max_depth: i64,
}
impl SetAsyncCallStackDepthParams {
    pub fn new(max_depth: impl Into<i64>) -> Self {
        Self {
            max_depth: max_depth.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetAsyncCallStackDepthMethod {
    #[serde(rename = "Debugger.setAsyncCallStackDepth")]
    SetAsyncCallStackDepth,
}
#[doc = "Enables or disables async call stacks tracking.\n[setAsyncCallStackDepth](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setAsyncCallStackDepth)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAsyncCallStackDepth {
    pub method: SetAsyncCallStackDepthMethod,
    pub params: SetAsyncCallStackDepthParams,
}
impl SetAsyncCallStackDepth {
    pub const IDENTIFIER: &'static str = "Debugger.setAsyncCallStackDepth";
}
impl crate::CommandResult for SetAsyncCallStackDepth {
    type Result = super::results::SetAsyncCallStackDepthResult;
}
#[doc = "Replace previous blackbox execution contexts with passed ones. Forces backend to skip\nstepping/pausing in scripts in these execution contexts. VM will try to leave blackboxed script by\nperforming 'step in' several times, finally resorting to 'step out' if unsuccessful.\n[setBlackboxExecutionContexts](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setBlackboxExecutionContexts)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBlackboxExecutionContextsParams {
    #[doc = "Array of execution context unique ids for the debugger to ignore."]
    #[serde(rename = "uniqueIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub unique_ids: Vec<String>,
}
impl SetBlackboxExecutionContextsParams {
    pub fn new(unique_ids: Vec<String>) -> Self {
        Self { unique_ids }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetBlackboxExecutionContextsMethod {
    #[serde(rename = "Debugger.setBlackboxExecutionContexts")]
    SetBlackboxExecutionContexts,
}
#[doc = "Replace previous blackbox execution contexts with passed ones. Forces backend to skip\nstepping/pausing in scripts in these execution contexts. VM will try to leave blackboxed script by\nperforming 'step in' several times, finally resorting to 'step out' if unsuccessful.\n[setBlackboxExecutionContexts](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setBlackboxExecutionContexts)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBlackboxExecutionContexts {
    pub method: SetBlackboxExecutionContextsMethod,
    pub params: SetBlackboxExecutionContextsParams,
}
impl SetBlackboxExecutionContexts {
    pub const IDENTIFIER: &'static str = "Debugger.setBlackboxExecutionContexts";
}
impl crate::CommandResult for SetBlackboxExecutionContexts {
    type Result = super::results::SetBlackboxExecutionContextsResult;
}
#[doc = "Replace previous blackbox patterns with passed ones. Forces backend to skip stepping/pausing in\nscripts with url matching one of the patterns. VM will try to leave blackboxed script by\nperforming 'step in' several times, finally resorting to 'step out' if unsuccessful.\n[setBlackboxPatterns](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setBlackboxPatterns)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBlackboxPatternsParams {
    #[doc = "Array of regexps that will be used to check script url for blackbox state."]
    #[serde(rename = "patterns")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub patterns: Vec<String>,
    #[doc = "If true, also ignore scripts with no source url."]
    #[serde(rename = "skipAnonymous")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub skip_anonymous: Option<bool>,
}
impl SetBlackboxPatternsParams {
    pub fn new(patterns: Vec<String>) -> Self {
        Self {
            patterns,
            skip_anonymous: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetBlackboxPatternsMethod {
    #[serde(rename = "Debugger.setBlackboxPatterns")]
    SetBlackboxPatterns,
}
#[doc = "Replace previous blackbox patterns with passed ones. Forces backend to skip stepping/pausing in\nscripts with url matching one of the patterns. VM will try to leave blackboxed script by\nperforming 'step in' several times, finally resorting to 'step out' if unsuccessful.\n[setBlackboxPatterns](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setBlackboxPatterns)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBlackboxPatterns {
    pub method: SetBlackboxPatternsMethod,
    pub params: SetBlackboxPatternsParams,
}
impl SetBlackboxPatterns {
    pub const IDENTIFIER: &'static str = "Debugger.setBlackboxPatterns";
}
impl crate::CommandResult for SetBlackboxPatterns {
    type Result = super::results::SetBlackboxPatternsResult;
}
#[doc = "Makes backend skip steps in the script in blackboxed ranges. VM will try leave blacklisted\nscripts by performing 'step in' several times, finally resorting to 'step out' if unsuccessful.\nPositions array contains positions where blackbox state is changed. First interval isn't\nblackboxed. Array should be sorted.\n[setBlackboxedRanges](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setBlackboxedRanges)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBlackboxedRangesParams {
    #[doc = "Id of the script."]
    #[serde(rename = "scriptId")]
    pub script_id: crate::js_protocol::runtime::types::ScriptId,
    #[serde(rename = "positions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub positions: Vec<super::types::ScriptPosition>,
}
impl SetBlackboxedRangesParams {
    pub fn new(
        script_id: impl Into<crate::js_protocol::runtime::types::ScriptId>,
        positions: Vec<super::types::ScriptPosition>,
    ) -> Self {
        Self {
            script_id: script_id.into(),
            positions,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetBlackboxedRangesMethod {
    #[serde(rename = "Debugger.setBlackboxedRanges")]
    SetBlackboxedRanges,
}
#[doc = "Makes backend skip steps in the script in blackboxed ranges. VM will try leave blacklisted\nscripts by performing 'step in' several times, finally resorting to 'step out' if unsuccessful.\nPositions array contains positions where blackbox state is changed. First interval isn't\nblackboxed. Array should be sorted.\n[setBlackboxedRanges](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setBlackboxedRanges)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBlackboxedRanges {
    pub method: SetBlackboxedRangesMethod,
    pub params: SetBlackboxedRangesParams,
}
impl SetBlackboxedRanges {
    pub const IDENTIFIER: &'static str = "Debugger.setBlackboxedRanges";
}
impl crate::CommandResult for SetBlackboxedRanges {
    type Result = super::results::SetBlackboxedRangesResult;
}
#[doc = "Sets JavaScript breakpoint at a given location.\n[setBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakpointParams {
    #[doc = "Location to set breakpoint in."]
    #[serde(rename = "location")]
    pub location: super::types::Location,
    #[doc = "Expression to use as a breakpoint condition. When specified, debugger will only stop on the\nbreakpoint if this expression evaluates to true."]
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub condition: Option<String>,
}
impl SetBreakpointParams {
    pub fn new(location: impl Into<super::types::Location>) -> Self {
        Self {
            location: location.into(),
            condition: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetBreakpointMethod {
    #[serde(rename = "Debugger.setBreakpoint")]
    SetBreakpoint,
}
#[doc = "Sets JavaScript breakpoint at a given location.\n[setBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakpoint {
    pub method: SetBreakpointMethod,
    pub params: SetBreakpointParams,
}
impl SetBreakpoint {
    pub const IDENTIFIER: &'static str = "Debugger.setBreakpoint";
}
impl crate::CommandResult for SetBreakpoint {
    type Result = super::results::SetBreakpointResult;
}
#[doc = "Sets instrumentation breakpoint.\n[setInstrumentationBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setInstrumentationBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInstrumentationBreakpointParams {
    #[doc = "Instrumentation name."]
    #[serde(rename = "instrumentation")]
    pub instrumentation: SetInstrumentationBreakpointInstrumentation,
}
#[doc = "Instrumentation name."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetInstrumentationBreakpointInstrumentation {
    #[serde(rename = "beforeScriptExecution")]
    BeforeScriptExecution,
    #[serde(rename = "beforeScriptWithSourceMapExecution")]
    BeforeScriptWithSourceMapExecution,
}
impl SetInstrumentationBreakpointParams {
    pub fn new(instrumentation: impl Into<SetInstrumentationBreakpointInstrumentation>) -> Self {
        Self {
            instrumentation: instrumentation.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetInstrumentationBreakpointMethod {
    #[serde(rename = "Debugger.setInstrumentationBreakpoint")]
    SetInstrumentationBreakpoint,
}
#[doc = "Sets instrumentation breakpoint.\n[setInstrumentationBreakpoint](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setInstrumentationBreakpoint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInstrumentationBreakpoint {
    pub method: SetInstrumentationBreakpointMethod,
    pub params: SetInstrumentationBreakpointParams,
}
impl SetInstrumentationBreakpoint {
    pub const IDENTIFIER: &'static str = "Debugger.setInstrumentationBreakpoint";
}
impl crate::CommandResult for SetInstrumentationBreakpoint {
    type Result = super::results::SetInstrumentationBreakpointResult;
}
#[doc = "Sets JavaScript breakpoint at given location specified either by URL or URL regex. Once this\ncommand is issued, all existing parsed scripts will have breakpoints resolved and returned in\n`locations` property. Further matching script parsing will result in subsequent\n`breakpointResolved` events issued. This logical breakpoint will survive page reloads.\n[setBreakpointByUrl](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setBreakpointByUrl)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakpointByUrlParams {
    #[doc = "Line number to set breakpoint at."]
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[doc = "URL of the resources to set breakpoint on."]
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
    #[doc = "Regex pattern for the URLs of the resources to set breakpoints on. Either `url` or\n`urlRegex` must be specified."]
    #[serde(rename = "urlRegex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url_regex: Option<String>,
    #[doc = "Script hash of the resources to set breakpoint on."]
    #[serde(rename = "scriptHash")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub script_hash: Option<String>,
    #[doc = "Offset in the line to set breakpoint at."]
    #[serde(rename = "columnNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub column_number: Option<i64>,
    #[doc = "Expression to use as a breakpoint condition. When specified, debugger will only stop on the\nbreakpoint if this expression evaluates to true."]
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub condition: Option<String>,
}
impl SetBreakpointByUrlParams {
    pub fn new(line_number: impl Into<i64>) -> Self {
        Self {
            line_number: line_number.into(),
            url: None,
            url_regex: None,
            script_hash: None,
            column_number: None,
            condition: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetBreakpointByUrlMethod {
    #[serde(rename = "Debugger.setBreakpointByUrl")]
    SetBreakpointByUrl,
}
#[doc = "Sets JavaScript breakpoint at given location specified either by URL or URL regex. Once this\ncommand is issued, all existing parsed scripts will have breakpoints resolved and returned in\n`locations` property. Further matching script parsing will result in subsequent\n`breakpointResolved` events issued. This logical breakpoint will survive page reloads.\n[setBreakpointByUrl](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setBreakpointByUrl)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakpointByUrl {
    pub method: SetBreakpointByUrlMethod,
    pub params: SetBreakpointByUrlParams,
}
impl SetBreakpointByUrl {
    pub const IDENTIFIER: &'static str = "Debugger.setBreakpointByUrl";
}
impl crate::CommandResult for SetBreakpointByUrl {
    type Result = super::results::SetBreakpointByUrlResult;
}
#[doc = "Sets JavaScript breakpoint before each call to the given function.\nIf another function was created from the same source as a given one,\ncalling it will also trigger the breakpoint.\n[setBreakpointOnFunctionCall](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setBreakpointOnFunctionCall)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakpointOnFunctionCallParams {
    #[doc = "Function object id."]
    #[serde(rename = "objectId")]
    pub object_id: crate::js_protocol::runtime::types::RemoteObjectId,
    #[doc = "Expression to use as a breakpoint condition. When specified, debugger will\nstop on the breakpoint if this expression evaluates to true."]
    #[serde(rename = "condition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub condition: Option<String>,
}
impl SetBreakpointOnFunctionCallParams {
    pub fn new(object_id: impl Into<crate::js_protocol::runtime::types::RemoteObjectId>) -> Self {
        Self {
            object_id: object_id.into(),
            condition: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetBreakpointOnFunctionCallMethod {
    #[serde(rename = "Debugger.setBreakpointOnFunctionCall")]
    SetBreakpointOnFunctionCall,
}
#[doc = "Sets JavaScript breakpoint before each call to the given function.\nIf another function was created from the same source as a given one,\ncalling it will also trigger the breakpoint.\n[setBreakpointOnFunctionCall](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setBreakpointOnFunctionCall)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakpointOnFunctionCall {
    pub method: SetBreakpointOnFunctionCallMethod,
    pub params: SetBreakpointOnFunctionCallParams,
}
impl SetBreakpointOnFunctionCall {
    pub const IDENTIFIER: &'static str = "Debugger.setBreakpointOnFunctionCall";
}
impl crate::CommandResult for SetBreakpointOnFunctionCall {
    type Result = super::results::SetBreakpointOnFunctionCallResult;
}
#[doc = "Activates / deactivates all breakpoints on the page.\n[setBreakpointsActive](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setBreakpointsActive)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakpointsActiveParams {
    #[doc = "New value for breakpoints active state."]
    #[serde(rename = "active")]
    pub active: bool,
}
impl SetBreakpointsActiveParams {
    pub fn new(active: impl Into<bool>) -> Self {
        Self {
            active: active.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetBreakpointsActiveMethod {
    #[serde(rename = "Debugger.setBreakpointsActive")]
    SetBreakpointsActive,
}
#[doc = "Activates / deactivates all breakpoints on the page.\n[setBreakpointsActive](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setBreakpointsActive)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakpointsActive {
    pub method: SetBreakpointsActiveMethod,
    pub params: SetBreakpointsActiveParams,
}
impl SetBreakpointsActive {
    pub const IDENTIFIER: &'static str = "Debugger.setBreakpointsActive";
}
impl crate::CommandResult for SetBreakpointsActive {
    type Result = super::results::SetBreakpointsActiveResult;
}
#[doc = "Defines pause on exceptions state. Can be set to stop on all exceptions, uncaught exceptions,\nor caught exceptions, no exceptions. Initial pause on exceptions state is `none`.\n[setPauseOnExceptions](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setPauseOnExceptions)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPauseOnExceptionsParams {
    #[doc = "Pause on exceptions mode."]
    #[serde(rename = "state")]
    pub state: SetPauseOnExceptionsState,
}
#[doc = "Pause on exceptions mode."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetPauseOnExceptionsState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "caught")]
    Caught,
    #[serde(rename = "uncaught")]
    Uncaught,
    #[serde(rename = "all")]
    All,
}
impl SetPauseOnExceptionsParams {
    pub fn new(state: impl Into<SetPauseOnExceptionsState>) -> Self {
        Self {
            state: state.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetPauseOnExceptionsMethod {
    #[serde(rename = "Debugger.setPauseOnExceptions")]
    SetPauseOnExceptions,
}
#[doc = "Defines pause on exceptions state. Can be set to stop on all exceptions, uncaught exceptions,\nor caught exceptions, no exceptions. Initial pause on exceptions state is `none`.\n[setPauseOnExceptions](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setPauseOnExceptions)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPauseOnExceptions {
    pub method: SetPauseOnExceptionsMethod,
    pub params: SetPauseOnExceptionsParams,
}
impl SetPauseOnExceptions {
    pub const IDENTIFIER: &'static str = "Debugger.setPauseOnExceptions";
}
impl crate::CommandResult for SetPauseOnExceptions {
    type Result = super::results::SetPauseOnExceptionsResult;
}
#[doc = "Changes return value in top frame. Available only at return break position.\n[setReturnValue](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setReturnValue)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetReturnValueParams {
    #[doc = "New return value."]
    #[serde(rename = "newValue")]
    pub new_value: crate::js_protocol::runtime::types::CallArgument,
}
impl SetReturnValueParams {
    pub fn new(new_value: impl Into<crate::js_protocol::runtime::types::CallArgument>) -> Self {
        Self {
            new_value: new_value.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetReturnValueMethod {
    #[serde(rename = "Debugger.setReturnValue")]
    SetReturnValue,
}
#[doc = "Changes return value in top frame. Available only at return break position.\n[setReturnValue](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setReturnValue)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetReturnValue {
    pub method: SetReturnValueMethod,
    pub params: SetReturnValueParams,
}
impl SetReturnValue {
    pub const IDENTIFIER: &'static str = "Debugger.setReturnValue";
}
impl crate::CommandResult for SetReturnValue {
    type Result = super::results::SetReturnValueResult;
}
#[doc = "Edits JavaScript source live.\n\nIn general, functions that are currently on the stack can not be edited with\na single exception: If the edited function is the top-most stack frame and\nthat is the only activation of that function on the stack. In this case\nthe live edit will be successful and a `Debugger.restartFrame` for the\ntop-most function is automatically triggered.\n[setScriptSource](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setScriptSource)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetScriptSourceParams {
    #[doc = "Id of the script to edit."]
    #[serde(rename = "scriptId")]
    pub script_id: crate::js_protocol::runtime::types::ScriptId,
    #[doc = "New content of the script."]
    #[serde(rename = "scriptSource")]
    pub script_source: String,
    #[doc = "If true the change will not actually be applied. Dry run may be used to get result\ndescription without actually modifying the code."]
    #[serde(rename = "dryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dry_run: Option<bool>,
    #[doc = "If true, then `scriptSource` is allowed to change the function on top of the stack\nas long as the top-most stack frame is the only activation of that function."]
    #[serde(rename = "allowTopFrameEditing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub allow_top_frame_editing: Option<bool>,
}
impl SetScriptSourceParams {
    pub fn new(
        script_id: impl Into<crate::js_protocol::runtime::types::ScriptId>,
        script_source: impl Into<String>,
    ) -> Self {
        Self {
            script_id: script_id.into(),
            script_source: script_source.into(),
            dry_run: None,
            allow_top_frame_editing: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetScriptSourceMethod {
    #[serde(rename = "Debugger.setScriptSource")]
    SetScriptSource,
}
#[doc = "Edits JavaScript source live.\n\nIn general, functions that are currently on the stack can not be edited with\na single exception: If the edited function is the top-most stack frame and\nthat is the only activation of that function on the stack. In this case\nthe live edit will be successful and a `Debugger.restartFrame` for the\ntop-most function is automatically triggered.\n[setScriptSource](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setScriptSource)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetScriptSource {
    pub method: SetScriptSourceMethod,
    pub params: SetScriptSourceParams,
}
impl SetScriptSource {
    pub const IDENTIFIER: &'static str = "Debugger.setScriptSource";
}
impl crate::CommandResult for SetScriptSource {
    type Result = super::results::SetScriptSourceResult;
}
#[doc = "Makes page not interrupt on any pauses (breakpoint, exception, dom exception etc).\n[setSkipAllPauses](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setSkipAllPauses)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSkipAllPausesParams {
    #[doc = "New value for skip pauses state."]
    #[serde(rename = "skip")]
    pub skip: bool,
}
impl SetSkipAllPausesParams {
    pub fn new(skip: impl Into<bool>) -> Self {
        Self { skip: skip.into() }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetSkipAllPausesMethod {
    #[serde(rename = "Debugger.setSkipAllPauses")]
    SetSkipAllPauses,
}
#[doc = "Makes page not interrupt on any pauses (breakpoint, exception, dom exception etc).\n[setSkipAllPauses](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setSkipAllPauses)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSkipAllPauses {
    pub method: SetSkipAllPausesMethod,
    pub params: SetSkipAllPausesParams,
}
impl SetSkipAllPauses {
    pub const IDENTIFIER: &'static str = "Debugger.setSkipAllPauses";
}
impl crate::CommandResult for SetSkipAllPauses {
    type Result = super::results::SetSkipAllPausesResult;
}
#[doc = "Changes value of variable in a callframe. Object-based scopes are not supported and must be\nmutated manually.\n[setVariableValue](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setVariableValue)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetVariableValueParams {
    #[doc = "0-based number of scope as was listed in scope chain. Only 'local', 'closure' and 'catch'\nscope types are allowed. Other scopes could be manipulated manually."]
    #[serde(rename = "scopeNumber")]
    pub scope_number: i64,
    #[doc = "Variable name."]
    #[serde(rename = "variableName")]
    pub variable_name: String,
    #[doc = "New variable value."]
    #[serde(rename = "newValue")]
    pub new_value: crate::js_protocol::runtime::types::CallArgument,
    #[doc = "Id of callframe that holds variable."]
    #[serde(rename = "callFrameId")]
    pub call_frame_id: super::types::CallFrameId,
}
impl SetVariableValueParams {
    pub fn new(
        scope_number: impl Into<i64>,
        variable_name: impl Into<String>,
        new_value: impl Into<crate::js_protocol::runtime::types::CallArgument>,
        call_frame_id: impl Into<super::types::CallFrameId>,
    ) -> Self {
        Self {
            scope_number: scope_number.into(),
            variable_name: variable_name.into(),
            new_value: new_value.into(),
            call_frame_id: call_frame_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetVariableValueMethod {
    #[serde(rename = "Debugger.setVariableValue")]
    SetVariableValue,
}
#[doc = "Changes value of variable in a callframe. Object-based scopes are not supported and must be\nmutated manually.\n[setVariableValue](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-setVariableValue)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetVariableValue {
    pub method: SetVariableValueMethod,
    pub params: SetVariableValueParams,
}
impl SetVariableValue {
    pub const IDENTIFIER: &'static str = "Debugger.setVariableValue";
}
impl crate::CommandResult for SetVariableValue {
    type Result = super::results::SetVariableValueResult;
}
#[doc = "Steps into the function call.\n[stepInto](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-stepInto)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StepIntoParams {
    #[doc = "Debugger will pause on the execution of the first async task which was scheduled\nbefore next pause."]
    #[serde(rename = "breakOnAsyncCall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub break_on_async_call: Option<bool>,
    #[doc = "The skipList specifies location ranges that should be skipped on step into."]
    #[serde(rename = "skipList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub skip_list: Option<Vec<super::types::LocationRange>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StepIntoMethod {
    #[serde(rename = "Debugger.stepInto")]
    StepInto,
}
#[doc = "Steps into the function call.\n[stepInto](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-stepInto)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepInto {
    pub method: StepIntoMethod,
    pub params: StepIntoParams,
}
impl StepInto {
    pub const IDENTIFIER: &'static str = "Debugger.stepInto";
}
impl crate::CommandResult for StepInto {
    type Result = super::results::StepIntoResult;
}
#[doc = "Steps out of the function call.\n[stepOut](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-stepOut)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepOutParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StepOutMethod {
    #[serde(rename = "Debugger.stepOut")]
    StepOut,
}
#[doc = "Steps out of the function call.\n[stepOut](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-stepOut)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepOut {
    pub method: StepOutMethod,
    pub params: StepOutParams,
}
impl StepOut {
    pub const IDENTIFIER: &'static str = "Debugger.stepOut";
}
impl crate::CommandResult for StepOut {
    type Result = super::results::StepOutResult;
}
#[doc = "Steps over the statement.\n[stepOver](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-stepOver)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StepOverParams {
    #[doc = "The skipList specifies location ranges that should be skipped on step over."]
    #[serde(rename = "skipList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub skip_list: Option<Vec<super::types::LocationRange>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StepOverMethod {
    #[serde(rename = "Debugger.stepOver")]
    StepOver,
}
#[doc = "Steps over the statement.\n[stepOver](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#method-stepOver)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StepOver {
    pub method: StepOverMethod,
    pub params: StepOverParams,
}
impl StepOver {
    pub const IDENTIFIER: &'static str = "Debugger.stepOver";
}
impl crate::CommandResult for StepOver {
    type Result = super::results::StepOverResult;
}
group_enum ! (DebuggerCommands { ContinueToLocation (ContinueToLocation) , Disable (Disable) , Enable (Enable) , EvaluateOnCallFrame (EvaluateOnCallFrame) , GetPossibleBreakpoints (GetPossibleBreakpoints) , GetScriptSource (GetScriptSource) , DisassembleWasmModule (DisassembleWasmModule) , NextWasmDisassemblyChunk (NextWasmDisassemblyChunk) , GetStackTrace (GetStackTrace) , Pause (Pause) , RemoveBreakpoint (RemoveBreakpoint) , RestartFrame (RestartFrame) , Resume (Resume) , SearchInContent (SearchInContent) , SetAsyncCallStackDepth (SetAsyncCallStackDepth) , SetBlackboxExecutionContexts (SetBlackboxExecutionContexts) , SetBlackboxPatterns (SetBlackboxPatterns) , SetBlackboxedRanges (SetBlackboxedRanges) , SetBreakpoint (SetBreakpoint) , SetInstrumentationBreakpoint (SetInstrumentationBreakpoint) , SetBreakpointByUrl (SetBreakpointByUrl) , SetBreakpointOnFunctionCall (SetBreakpointOnFunctionCall) , SetBreakpointsActive (SetBreakpointsActive) , SetPauseOnExceptions (SetPauseOnExceptions) , SetReturnValue (SetReturnValue) , SetScriptSource (SetScriptSource) , SetSkipAllPauses (SetSkipAllPauses) , SetVariableValue (SetVariableValue) , StepInto (StepInto) , StepOut (StepOut) , StepOver (StepOver) });
