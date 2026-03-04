use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableReturns {
    #[doc = "Unique identifier of the debugger."]
    #[serde(rename = "debuggerId")]
    pub debugger_id: super::super::runtime::types::UniqueDebuggerId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluateOnCallFrameReturns {
    #[doc = "Object wrapper for the evaluation result."]
    #[serde(rename = "result")]
    pub result: super::super::runtime::types::RemoteObject,
    #[doc = "Exception details."]
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<super::super::runtime::types::ExceptionDetails>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPossibleBreakpointsReturns {
    #[doc = "List of the possible breakpoint locations."]
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<super::types::BreakLocation>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScriptSourceReturns {
    #[doc = "Script source (empty in case of Wasm bytecode)."]
    #[serde(rename = "scriptSource")]
    pub script_source: String,
    #[doc = "Wasm bytecode."]
    #[serde(rename = "bytecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bytecode: Option<super::super::super::Binary>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisassembleWasmModuleReturns {
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
pub struct NextWasmDisassemblyChunkReturns {
    #[doc = "The next chunk of disassembly."]
    #[serde(rename = "chunk")]
    pub chunk: super::types::WasmDisassemblyChunk,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWasmBytecodeReturns {
    #[doc = "Script source."]
    #[serde(rename = "bytecode")]
    pub bytecode: super::super::super::Binary,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStackTraceReturns {
    #[serde(rename = "stackTrace")]
    pub stack_trace: super::super::runtime::types::StackTrace,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RestartFrameReturns {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchInContentReturns {
    #[doc = "List of search matches."]
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<super::types::SearchMatch>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakpointReturns {
    #[doc = "Id of the created breakpoint for further reference."]
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: super::types::BreakpointId,
    #[doc = "Location this breakpoint resolved into."]
    #[serde(rename = "actualLocation")]
    pub actual_location: super::types::Location,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInstrumentationBreakpointReturns {
    #[doc = "Id of the created breakpoint for further reference."]
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: super::types::BreakpointId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakpointByUrlReturns {
    #[doc = "Id of the created breakpoint for further reference."]
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: super::types::BreakpointId,
    #[doc = "List of the locations this breakpoint resolved into upon addition."]
    #[serde(rename = "locations")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub locations: Vec<super::types::Location>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBreakpointOnFunctionCallReturns {
    #[doc = "Id of the created breakpoint for further reference."]
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: super::types::BreakpointId,
}
#[doc = "Whether the operation was successful or not. Only `Ok` denotes a\nsuccessful live edit while the other enum variants denote why\nthe live edit failed."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetScriptSourceReturnsStatus {
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
pub struct SetScriptSourceReturns {
    #[doc = "Whether the operation was successful or not. Only `Ok` denotes a\nsuccessful live edit while the other enum variants denote why\nthe live edit failed."]
    #[serde(rename = "status")]
    pub status: SetScriptSourceReturnsStatus,
    #[doc = "Exception details if any. Only present when `status` is `CompileError`."]
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<super::super::runtime::types::ExceptionDetails>,
}
