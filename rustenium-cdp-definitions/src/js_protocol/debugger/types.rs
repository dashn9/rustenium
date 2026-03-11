use serde::{Deserialize, Serialize};
#[doc = "Breakpoint identifier.\n[BreakpointId](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#type-BreakpointId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct BreakpointId(String);
impl BreakpointId {
    pub fn new(val: impl Into<String>) -> Self {
        BreakpointId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for BreakpointId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<BreakpointId> for String {
    fn from(el: BreakpointId) -> String {
        el.0
    }
}
impl From<String> for BreakpointId {
    fn from(expr: String) -> Self {
        BreakpointId(expr)
    }
}
impl std::borrow::Borrow<str> for BreakpointId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl BreakpointId {
    pub const IDENTIFIER: &'static str = "Debugger.BreakpointId";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Call frame identifier.\n[CallFrameId](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#type-CallFrameId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct CallFrameId(String);
impl CallFrameId {
    pub fn new(val: impl Into<String>) -> Self {
        CallFrameId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for CallFrameId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<CallFrameId> for String {
    fn from(el: CallFrameId) -> String {
        el.0
    }
}
impl From<String> for CallFrameId {
    fn from(expr: String) -> Self {
        CallFrameId(expr)
    }
}
impl std::borrow::Borrow<str> for CallFrameId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl CallFrameId {
    pub const IDENTIFIER: &'static str = "Debugger.CallFrameId";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Location in the source code.\n[Location](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#type-Location)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    #[doc = "Script identifier as reported in the `Debugger.scriptParsed`."]
    #[serde(rename = "scriptId")]
    pub script_id: crate::js_protocol::runtime::types::ScriptId,
    #[doc = "Line number in the script (0-based)."]
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[doc = "Column number in the script (0-based)."]
    #[serde(rename = "columnNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub column_number: Option<i64>,
}
impl Location {
    pub fn new(
        script_id: impl Into<crate::js_protocol::runtime::types::ScriptId>,
        line_number: impl Into<i64>,
    ) -> Self {
        Self {
            script_id: script_id.into(),
            line_number: line_number.into(),
            column_number: None,
        }
    }
}
impl Location {
    pub const IDENTIFIER: &'static str = "Debugger.Location";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Location in the source code.\n[ScriptPosition](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#type-ScriptPosition)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScriptPosition {
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[serde(rename = "columnNumber")]
    pub column_number: i64,
}
impl ScriptPosition {
    pub fn new(line_number: impl Into<i64>, column_number: impl Into<i64>) -> Self {
        Self {
            line_number: line_number.into(),
            column_number: column_number.into(),
        }
    }
}
impl ScriptPosition {
    pub const IDENTIFIER: &'static str = "Debugger.ScriptPosition";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Location range within one script.\n[LocationRange](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#type-LocationRange)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationRange {
    #[serde(rename = "scriptId")]
    pub script_id: crate::js_protocol::runtime::types::ScriptId,
    #[serde(rename = "start")]
    pub start: ScriptPosition,
    #[serde(rename = "end")]
    pub end: ScriptPosition,
}
impl LocationRange {
    pub fn new(
        script_id: impl Into<crate::js_protocol::runtime::types::ScriptId>,
        start: impl Into<ScriptPosition>,
        end: impl Into<ScriptPosition>,
    ) -> Self {
        Self {
            script_id: script_id.into(),
            start: start.into(),
            end: end.into(),
        }
    }
}
impl LocationRange {
    pub const IDENTIFIER: &'static str = "Debugger.LocationRange";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "JavaScript call frame. Array of call frames form the call stack.\n[CallFrame](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#type-CallFrame)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallFrame {
    #[doc = "Call frame identifier. This identifier is only valid while the virtual machine is paused."]
    #[serde(rename = "callFrameId")]
    pub call_frame_id: CallFrameId,
    #[doc = "Name of the JavaScript function called on this call frame."]
    #[serde(rename = "functionName")]
    pub function_name: String,
    #[doc = "Location in the source code."]
    #[serde(rename = "functionLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub function_location: Option<Location>,
    #[doc = "Location in the source code."]
    #[serde(rename = "location")]
    pub location: Location,
    #[doc = "Scope chain for this call frame."]
    #[serde(rename = "scopeChain")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scope_chain: Vec<Scope>,
    #[doc = "`this` object for this call frame."]
    #[serde(rename = "this")]
    pub this: crate::js_protocol::runtime::types::RemoteObject,
    #[doc = "The value being returned, if the function is at return point."]
    #[serde(rename = "returnValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub return_value: Option<crate::js_protocol::runtime::types::RemoteObject>,
    #[doc = "Valid only while the VM is paused and indicates whether this frame\ncan be restarted or not. Note that a `true` value here does not\nguarantee that Debugger#restartFrame with this CallFrameId will be\nsuccessful, but it is very likely."]
    #[serde(rename = "canBeRestarted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub can_be_restarted: Option<bool>,
}
impl CallFrame {
    pub const IDENTIFIER: &'static str = "Debugger.CallFrame";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Scope description.\n[Scope](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#type-Scope)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Scope {
    #[doc = "Scope type."]
    #[serde(rename = "type")]
    pub r#type: ScopeType,
    #[doc = "Object representing the scope. For `global` and `with` scopes it represents the actual\nobject; for the rest of the scopes, it is artificial transient object enumerating scope\nvariables as its properties."]
    #[serde(rename = "object")]
    pub object: crate::js_protocol::runtime::types::RemoteObject,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[doc = "Location in the source code where scope starts"]
    #[serde(rename = "startLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub start_location: Option<Location>,
    #[doc = "Location in the source code where scope ends"]
    #[serde(rename = "endLocation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub end_location: Option<Location>,
}
#[doc = "Scope type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScopeType {
    #[serde(rename = "global")]
    Global,
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "with")]
    With,
    #[serde(rename = "closure")]
    Closure,
    #[serde(rename = "catch")]
    Catch,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "eval")]
    Eval,
    #[serde(rename = "module")]
    Module,
    #[serde(rename = "wasm-expression-stack")]
    WasmExpressionStack,
}
impl Scope {
    pub fn new(
        r#type: impl Into<ScopeType>,
        object: impl Into<crate::js_protocol::runtime::types::RemoteObject>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            object: object.into(),
            name: None,
            start_location: None,
            end_location: None,
        }
    }
}
impl Scope {
    pub const IDENTIFIER: &'static str = "Debugger.Scope";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Search match for resource.\n[SearchMatch](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#type-SearchMatch)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchMatch {
    #[doc = "Line number in resource content."]
    #[serde(rename = "lineNumber")]
    pub line_number: f64,
    #[doc = "Line with match content."]
    #[serde(rename = "lineContent")]
    pub line_content: String,
}
impl SearchMatch {
    pub fn new(line_number: impl Into<f64>, line_content: impl Into<String>) -> Self {
        Self {
            line_number: line_number.into(),
            line_content: line_content.into(),
        }
    }
}
impl SearchMatch {
    pub const IDENTIFIER: &'static str = "Debugger.SearchMatch";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BreakLocation {
    #[doc = "Script identifier as reported in the `Debugger.scriptParsed`."]
    #[serde(rename = "scriptId")]
    pub script_id: crate::js_protocol::runtime::types::ScriptId,
    #[doc = "Line number in the script (0-based)."]
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[doc = "Column number in the script (0-based)."]
    #[serde(rename = "columnNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub column_number: Option<i64>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub r#type: Option<BreakLocationType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BreakLocationType {
    #[serde(rename = "debuggerStatement")]
    DebuggerStatement,
    #[serde(rename = "call")]
    Call,
    #[serde(rename = "return")]
    Return,
}
impl BreakLocation {
    pub fn new(
        script_id: impl Into<crate::js_protocol::runtime::types::ScriptId>,
        line_number: impl Into<i64>,
    ) -> Self {
        Self {
            script_id: script_id.into(),
            line_number: line_number.into(),
            column_number: None,
            r#type: None,
        }
    }
}
impl BreakLocation {
    pub const IDENTIFIER: &'static str = "Debugger.BreakLocation";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WasmDisassemblyChunk {
    #[doc = "The next chunk of disassembled lines."]
    #[serde(rename = "lines")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lines: Vec<String>,
    #[doc = "The bytecode offsets describing the start of each line."]
    #[serde(rename = "bytecodeOffsets")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bytecode_offsets: Vec<i64>,
}
impl WasmDisassemblyChunk {
    pub fn new(lines: Vec<String>, bytecode_offsets: Vec<i64>) -> Self {
        Self {
            lines,
            bytecode_offsets,
        }
    }
}
impl WasmDisassemblyChunk {
    pub const IDENTIFIER: &'static str = "Debugger.WasmDisassemblyChunk";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Enum of possible script languages."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScriptLanguage {
    #[serde(rename = "JavaScript")]
    JavaScript,
    #[serde(rename = "WebAssembly")]
    WebAssembly,
}
#[doc = "Debug symbols available for a wasm script.\n[DebugSymbols](https://chromedevtools.github.io/devtools-protocol/tot/Debugger/#type-DebugSymbols)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DebugSymbols {
    #[doc = "Type of the debug symbols."]
    #[serde(rename = "type")]
    pub r#type: DebugSymbolsType,
    #[doc = "URL of the external symbol source."]
    #[serde(rename = "externalURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub external_url: Option<String>,
}
#[doc = "Type of the debug symbols."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DebugSymbolsType {
    #[serde(rename = "SourceMap")]
    SourceMap,
    #[serde(rename = "EmbeddedDWARF")]
    EmbeddedDwarf,
    #[serde(rename = "ExternalDWARF")]
    ExternalDwarf,
}
impl DebugSymbols {
    pub fn new(r#type: impl Into<DebugSymbolsType>) -> Self {
        Self {
            r#type: r#type.into(),
            external_url: None,
        }
    }
}
impl DebugSymbols {
    pub const IDENTIFIER: &'static str = "Debugger.DebugSymbols";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedBreakpoint {
    #[doc = "Breakpoint unique identifier."]
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: BreakpointId,
    #[doc = "Actual breakpoint location."]
    #[serde(rename = "location")]
    pub location: Location,
}
impl ResolvedBreakpoint {
    pub fn new(breakpoint_id: impl Into<BreakpointId>, location: impl Into<Location>) -> Self {
        Self {
            breakpoint_id: breakpoint_id.into(),
            location: location.into(),
        }
    }
}
impl ResolvedBreakpoint {
    pub const IDENTIFIER: &'static str = "Debugger.ResolvedBreakpoint";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (DebuggerTypes { BreakpointId (BreakpointId) , CallFrameId (CallFrameId) , Location (Location) , ScriptPosition (ScriptPosition) , LocationRange (LocationRange) , CallFrame (CallFrame) , Scope (Scope) , SearchMatch (SearchMatch) , BreakLocation (BreakLocation) , WasmDisassemblyChunk (WasmDisassemblyChunk) , ScriptLanguage (ScriptLanguage) , DebugSymbols (DebugSymbols) , ResolvedBreakpoint (ResolvedBreakpoint) });
