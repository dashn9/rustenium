use serde::{Deserialize, Serialize};
#[doc = "Notification is issued every time when binding is called.\n[bindingCalled](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-bindingCalled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BindingCalled {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "payload")]
    pub payload: String,
    #[doc = "Identifier of the context where the call was made."]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: super::types::ExecutionContextId,
}
impl BindingCalled {
    pub const IDENTIFIER: &'static str = "Runtime.bindingCalled";
}
#[doc = "Issued when console API was called.\n[consoleAPICalled](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-consoleAPICalled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsoleApiCalled {
    #[doc = "Type of the call."]
    #[serde(rename = "type")]
    pub r#type: ConsoleApiCalledType,
    #[doc = "Call arguments."]
    #[serde(rename = "args")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<super::types::RemoteObject>,
    #[doc = "Identifier of the context where the call was made."]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: super::types::ExecutionContextId,
    #[doc = "Call timestamp."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::Timestamp,
    #[doc = "Stack trace captured when the call was made. The async stack chain is automatically reported for\nthe following call types: `assert`, `error`, `trace`, `warning`. For other types the async call\nchain can be retrieved using `Debugger.getStackTrace` and `stackTrace.parentId` field."]
    #[serde(rename = "stackTrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack_trace: Option<super::types::StackTrace>,
    #[doc = "Console context descriptor for calls on non-default console context (not console.*):\n'anonymous#unique-logger-id' for call on unnamed context, 'name#unique-logger-id' for call\non named context."]
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub context: Option<String>,
}
#[doc = "Type of the call."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConsoleApiCalledType {
    #[serde(rename = "log")]
    Log,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "dir")]
    Dir,
    #[serde(rename = "dirxml")]
    Dirxml,
    #[serde(rename = "table")]
    Table,
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "clear")]
    Clear,
    #[serde(rename = "startGroup")]
    StartGroup,
    #[serde(rename = "startGroupCollapsed")]
    StartGroupCollapsed,
    #[serde(rename = "endGroup")]
    EndGroup,
    #[serde(rename = "assert")]
    Assert,
    #[serde(rename = "profile")]
    Profile,
    #[serde(rename = "profileEnd")]
    ProfileEnd,
    #[serde(rename = "count")]
    Count,
    #[serde(rename = "timeEnd")]
    TimeEnd,
}
impl ConsoleApiCalled {
    pub const IDENTIFIER: &'static str = "Runtime.consoleAPICalled";
}
#[doc = "Issued when unhandled exception was revoked.\n[exceptionRevoked](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-exceptionRevoked)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExceptionRevoked {
    #[doc = "Reason describing why exception was revoked."]
    #[serde(rename = "reason")]
    pub reason: String,
    #[doc = "The id of revoked exception, as reported in `exceptionThrown`."]
    #[serde(rename = "exceptionId")]
    pub exception_id: i64,
}
impl ExceptionRevoked {
    pub const IDENTIFIER: &'static str = "Runtime.exceptionRevoked";
}
#[doc = "Issued when exception was thrown and unhandled.\n[exceptionThrown](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-exceptionThrown)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExceptionThrown {
    #[doc = "Timestamp of the exception."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::Timestamp,
    #[serde(rename = "exceptionDetails")]
    pub exception_details: super::types::ExceptionDetails,
}
impl ExceptionThrown {
    pub const IDENTIFIER: &'static str = "Runtime.exceptionThrown";
}
#[doc = "Issued when new execution context is created.\n[executionContextCreated](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-executionContextCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionContextCreated {
    #[doc = "A newly created execution context."]
    #[serde(rename = "context")]
    pub context: super::types::ExecutionContextDescription,
}
impl ExecutionContextCreated {
    pub const IDENTIFIER: &'static str = "Runtime.executionContextCreated";
}
#[doc = "Issued when execution context is destroyed.\n[executionContextDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-executionContextDestroyed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionContextDestroyed {
    #[doc = "Unique Id of the destroyed context"]
    #[serde(rename = "executionContextUniqueId")]
    pub execution_context_unique_id: String,
}
impl ExecutionContextDestroyed {
    pub const IDENTIFIER: &'static str = "Runtime.executionContextDestroyed";
}
#[doc = "Issued when all executionContexts were cleared in browser\n[executionContextsCleared](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-executionContextsCleared)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExecutionContextsCleared {}
impl ExecutionContextsCleared {
    pub const IDENTIFIER: &'static str = "Runtime.executionContextsCleared";
}
#[doc = "Issued when object should be inspected (for example, as a result of inspect() command line API\ncall).\n[inspectRequested](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-inspectRequested)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectRequested {
    #[serde(rename = "object")]
    pub object: super::types::RemoteObject,
    #[serde(rename = "hints")]
    pub hints: serde_json::Value,
    #[doc = "Identifier of the context where the call was made."]
    #[serde(rename = "executionContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub execution_context_id: Option<super::types::ExecutionContextId>,
}
impl InspectRequested {
    pub const IDENTIFIER: &'static str = "Runtime.inspectRequested";
}
group_enum ! (Event { BindingCalled (BindingCalled) , ConsoleApiCalled (ConsoleApiCalled) , ExceptionRevoked (ExceptionRevoked) , ExceptionThrown (ExceptionThrown) , ExecutionContextCreated (ExecutionContextCreated) , ExecutionContextDestroyed (ExecutionContextDestroyed) , ExecutionContextsCleared (ExecutionContextsCleared) , InspectRequested (InspectRequested) });
