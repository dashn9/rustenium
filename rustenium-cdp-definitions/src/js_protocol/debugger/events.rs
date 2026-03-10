use serde::{Deserialize, Serialize};
#[doc = "Fired when the virtual machine stopped on breakpoint or exception or any other stop criteria.\n[paused](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#event-paused)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PausedParams {
    #[doc = "Call stack the virtual machine stopped on."]
    #[serde(rename = "callFrames")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub call_frames: Vec<super::types::CallFrame>,
    #[doc = "Pause reason."]
    #[serde(rename = "reason")]
    pub reason: PausedReason,
    #[doc = "Object containing break-specific auxiliary properties."]
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<serde_json::Value>,
    #[doc = "Hit breakpoints IDs"]
    #[serde(rename = "hitBreakpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub hit_breakpoints: Option<Vec<String>>,
    #[doc = "Async stack trace, if any."]
    #[serde(rename = "asyncStackTrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub async_stack_trace: Option<crate::js_protocol::runtime::types::StackTrace>,
    #[doc = "Async stack trace, if any."]
    #[serde(rename = "asyncStackTraceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub async_stack_trace_id: Option<crate::js_protocol::runtime::types::StackTraceId>,
}
#[doc = "Pause reason."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PausedReason {
    #[serde(rename = "ambiguous")]
    Ambiguous,
    #[serde(rename = "assert")]
    Assert,
    #[serde(rename = "CSPViolation")]
    CspViolation,
    #[serde(rename = "debugCommand")]
    DebugCommand,
    #[serde(rename = "DOM")]
    Dom,
    #[serde(rename = "EventListener")]
    EventListener,
    #[serde(rename = "exception")]
    Exception,
    #[serde(rename = "instrumentation")]
    Instrumentation,
    #[serde(rename = "OOM")]
    Oom,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "promiseRejection")]
    PromiseRejection,
    #[serde(rename = "XHR")]
    Xhr,
    #[serde(rename = "step")]
    Step,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PausedMethod {
    #[serde(rename = "Debugger.paused")]
    Paused,
}
#[doc = "Fired when the virtual machine stopped on breakpoint or exception or any other stop criteria.\n[paused](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#event-paused)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Paused {
    pub method: PausedMethod,
    pub params: PausedParams,
}
impl Paused {
    pub const IDENTIFIER: &'static str = "Debugger.paused";
}
#[doc = "Fired when the virtual machine resumed execution.\n[resumed](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#event-resumed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResumedParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResumedMethod {
    #[serde(rename = "Debugger.resumed")]
    Resumed,
}
#[doc = "Fired when the virtual machine resumed execution.\n[resumed](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#event-resumed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resumed {
    pub method: ResumedMethod,
    pub params: ResumedParams,
}
impl Resumed {
    pub const IDENTIFIER: &'static str = "Debugger.resumed";
}
#[doc = "Fired when virtual machine fails to parse the script.\n[scriptFailedToParse](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#event-scriptFailedToParse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptFailedToParseParams {
    #[doc = "Identifier of the script parsed."]
    #[serde(rename = "scriptId")]
    pub script_id: crate::js_protocol::runtime::types::ScriptId,
    #[doc = "URL or name of the script parsed (if any)."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Line offset of the script within the resource with given URL (for script tags)."]
    #[serde(rename = "startLine")]
    pub start_line: i64,
    #[doc = "Column offset of the script within the resource with given URL."]
    #[serde(rename = "startColumn")]
    pub start_column: i64,
    #[doc = "Last line of the script."]
    #[serde(rename = "endLine")]
    pub end_line: i64,
    #[doc = "Length of the last line of the script."]
    #[serde(rename = "endColumn")]
    pub end_column: i64,
    #[doc = "Specifies script creation context."]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: crate::js_protocol::runtime::types::ExecutionContextId,
    #[doc = "Content hash of the script, SHA-256."]
    #[serde(rename = "hash")]
    pub hash: String,
    #[doc = "For Wasm modules, the content of the `build_id` custom section. For JavaScript the `debugId` magic comment."]
    #[serde(rename = "buildId")]
    pub build_id: String,
    #[doc = "Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}"]
    #[serde(rename = "executionContextAuxData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub execution_context_aux_data: Option<serde_json::Value>,
    #[doc = "URL of source map associated with script (if any)."]
    #[serde(rename = "sourceMapURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_map_url: Option<String>,
    #[doc = "True, if this script has sourceURL."]
    #[serde(rename = "hasSourceURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub has_source_url: Option<bool>,
    #[doc = "True, if this script is ES6 module."]
    #[serde(rename = "isModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_module: Option<bool>,
    #[doc = "This script length."]
    #[serde(rename = "length")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub length: Option<i64>,
    #[doc = "JavaScript top stack frame of where the script parsed event was triggered if available."]
    #[serde(rename = "stackTrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack_trace: Option<crate::js_protocol::runtime::types::StackTrace>,
    #[doc = "If the scriptLanguage is WebAssembly, the code section offset in the module."]
    #[serde(rename = "codeOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub code_offset: Option<i64>,
    #[doc = "The language of the script."]
    #[serde(rename = "scriptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub script_language: Option<super::types::ScriptLanguage>,
    #[doc = "The name the embedder supplied for this script."]
    #[serde(rename = "embedderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub embedder_name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScriptFailedToParseMethod {
    #[serde(rename = "Debugger.scriptFailedToParse")]
    ScriptFailedToParse,
}
#[doc = "Fired when virtual machine fails to parse the script.\n[scriptFailedToParse](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#event-scriptFailedToParse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptFailedToParse {
    pub method: ScriptFailedToParseMethod,
    pub params: ScriptFailedToParseParams,
}
impl ScriptFailedToParse {
    pub const IDENTIFIER: &'static str = "Debugger.scriptFailedToParse";
}
#[doc = "Fired when virtual machine parses script. This event is also fired for all known and uncollected\nscripts upon enabling debugger.\n[scriptParsed](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#event-scriptParsed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptParsedParams {
    #[doc = "Identifier of the script parsed."]
    #[serde(rename = "scriptId")]
    pub script_id: crate::js_protocol::runtime::types::ScriptId,
    #[doc = "URL or name of the script parsed (if any)."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Line offset of the script within the resource with given URL (for script tags)."]
    #[serde(rename = "startLine")]
    pub start_line: i64,
    #[doc = "Column offset of the script within the resource with given URL."]
    #[serde(rename = "startColumn")]
    pub start_column: i64,
    #[doc = "Last line of the script."]
    #[serde(rename = "endLine")]
    pub end_line: i64,
    #[doc = "Length of the last line of the script."]
    #[serde(rename = "endColumn")]
    pub end_column: i64,
    #[doc = "Specifies script creation context."]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: crate::js_protocol::runtime::types::ExecutionContextId,
    #[doc = "Content hash of the script, SHA-256."]
    #[serde(rename = "hash")]
    pub hash: String,
    #[doc = "For Wasm modules, the content of the `build_id` custom section. For JavaScript the `debugId` magic comment."]
    #[serde(rename = "buildId")]
    pub build_id: String,
    #[doc = "Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}"]
    #[serde(rename = "executionContextAuxData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub execution_context_aux_data: Option<serde_json::Value>,
    #[doc = "True, if this script is generated as a result of the live edit operation."]
    #[serde(rename = "isLiveEdit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_live_edit: Option<bool>,
    #[doc = "URL of source map associated with script (if any)."]
    #[serde(rename = "sourceMapURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source_map_url: Option<String>,
    #[doc = "True, if this script has sourceURL."]
    #[serde(rename = "hasSourceURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub has_source_url: Option<bool>,
    #[doc = "True, if this script is ES6 module."]
    #[serde(rename = "isModule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_module: Option<bool>,
    #[doc = "This script length."]
    #[serde(rename = "length")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub length: Option<i64>,
    #[doc = "JavaScript top stack frame of where the script parsed event was triggered if available."]
    #[serde(rename = "stackTrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack_trace: Option<crate::js_protocol::runtime::types::StackTrace>,
    #[doc = "If the scriptLanguage is WebAssembly, the code section offset in the module."]
    #[serde(rename = "codeOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub code_offset: Option<i64>,
    #[doc = "The language of the script."]
    #[serde(rename = "scriptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub script_language: Option<super::types::ScriptLanguage>,
    #[doc = "If the scriptLanguage is WebAssembly, the source of debug symbols for the module."]
    #[serde(rename = "debugSymbols")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub debug_symbols: Option<Vec<super::types::DebugSymbols>>,
    #[doc = "The name the embedder supplied for this script."]
    #[serde(rename = "embedderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub embedder_name: Option<String>,
    #[doc = "The list of set breakpoints in this script if calls to `setBreakpointByUrl`\nmatches this script's URL or hash. Clients that use this list can ignore the\n`breakpointResolved` event. They are equivalent."]
    #[serde(rename = "resolvedBreakpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub resolved_breakpoints: Option<Vec<super::types::ResolvedBreakpoint>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScriptParsedMethod {
    #[serde(rename = "Debugger.scriptParsed")]
    ScriptParsed,
}
#[doc = "Fired when virtual machine parses script. This event is also fired for all known and uncollected\nscripts upon enabling debugger.\n[scriptParsed](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#event-scriptParsed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptParsed {
    pub method: ScriptParsedMethod,
    pub params: ScriptParsedParams,
}
impl ScriptParsed {
    pub const IDENTIFIER: &'static str = "Debugger.scriptParsed";
}
group_enum ! (DebuggerEvents { Paused (Paused) , Resumed (Resumed) , ScriptFailedToParse (ScriptFailedToParse) , ScriptParsed (ScriptParsed) });
