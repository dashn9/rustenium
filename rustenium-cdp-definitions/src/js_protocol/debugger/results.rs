use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ContinueToLocationResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableResult {
    #[doc = "Unique identifier of the debugger."]
    #[serde(rename = "debuggerId")]
    pub debugger_id: crate::js_protocol::runtime::types::UniqueDebuggerId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluateOnCallFrameResult {
    #[doc = "Object wrapper for the evaluation result."]
    #[serde(rename = "result")]
    pub result: crate::js_protocol::runtime::types::RemoteObject,
    #[doc = "Exception details."]
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<crate::js_protocol::runtime::types::ExceptionDetails>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPossibleBreakpointsResult {
    #[doc = "List of the possible breakpoint locations."]
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<super::types::BreakLocation>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScriptSourceResult {
    #[doc = "Script source (empty in case of Wasm bytecode)."]
    #[serde(rename = "scriptSource")]
    pub script_source: String,
    #[doc = "Wasm bytecode."]
    #[serde(rename = "bytecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bytecode: Option<crate::Binary>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisassembleWasmModuleResult {
    #[doc = "For large modules, return a stream from which additional chunks of\ndisassembly can be read successively."]
    #[serde(rename = "streamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stream_id: Option<String>,
    #[doc = "The total number of lines in the disassembly text."]
    #[serde(rename = "totalNumberOfLines")]
    pub total_number_of_lines: i64,
    #[doc = "The offsets of all function bodies, in the format [start1, end1,\nstart2, end2, ...] where all ends are exclusive."]
    #[serde(rename = "functionBodyOffsets")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub function_body_offsets: Vec<i64>,
    #[doc = "The first chunk of disassembly."]
    #[serde(rename = "chunk")]
    pub chunk: super::types::WasmDisassemblyChunk,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NextWasmDisassemblyChunkResult {
    #[doc = "The next chunk of disassembly."]
    #[serde(rename = "chunk")]
    pub chunk: super::types::WasmDisassemblyChunk,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWasmBytecodeResult {
    #[doc = "Script source."]
    #[serde(rename = "bytecode")]
    pub bytecode: crate::Binary,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStackTraceResult {
    #[serde(rename = "stackTrace")]
    pub stack_trace: crate::js_protocol::runtime::types::StackTrace,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PauseResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PauseOnAsyncCallResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveBreakpointResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RestartFrameResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResumeResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchInContentResult {
    #[doc = "List of search matches."]
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<super::types::SearchMatch>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAsyncCallStackDepthResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetBlackboxExecutionContextsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetBlackboxPatternsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetBlackboxedRangesResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakpointResult {
    #[doc = "Id of the created breakpoint for further reference."]
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: super::types::BreakpointId,
    #[doc = "Location this breakpoint resolved into."]
    #[serde(rename = "actualLocation")]
    pub actual_location: super::types::Location,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInstrumentationBreakpointResult {
    #[doc = "Id of the created breakpoint for further reference."]
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: super::types::BreakpointId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakpointByUrlResult {
    #[doc = "Id of the created breakpoint for further reference."]
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: super::types::BreakpointId,
    #[doc = "List of the locations this breakpoint resolved into upon addition."]
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<super::types::Location>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakpointOnFunctionCallResult {
    #[doc = "Id of the created breakpoint for further reference."]
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: super::types::BreakpointId,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetBreakpointsActiveResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPauseOnExceptionsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetReturnValueResult {}
#[doc = "Whether the operation was successful or not. Only `Ok` denotes a\nsuccessful live edit while the other enum variants denote why\nthe live edit failed."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetScriptSourceResultStatus {
    #[serde(rename = "Ok")]
    Ok,
    #[serde(rename = "CompileError")]
    CompileError,
    #[serde(rename = "BlockedByActiveGenerator")]
    BlockedByActiveGenerator,
    #[serde(rename = "BlockedByActiveFunction")]
    BlockedByActiveFunction,
    #[serde(rename = "BlockedByTopLevelEsModuleChange")]
    BlockedByTopLevelEsModuleChange,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetScriptSourceResult {
    #[doc = "Whether the operation was successful or not. Only `Ok` denotes a\nsuccessful live edit while the other enum variants denote why\nthe live edit failed."]
    #[serde(rename = "status")]
    pub status: SetScriptSourceResultStatus,
    #[doc = "Exception details if any. Only present when `status` is `CompileError`."]
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<crate::js_protocol::runtime::types::ExceptionDetails>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSkipAllPausesResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetVariableValueResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StepIntoResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StepOutResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StepOverResult {}
