use serde::{Deserialize, Serialize};
#[doc = "Notification is issued every time when binding is called.\n[bindingCalled](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-bindingCalled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BindingCalledParams {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "payload")]
    pub payload: String,
    #[doc = "Identifier of the context where the call was made."]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: super::types::ExecutionContextId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BindingCalledMethod {
    #[serde(rename = "Runtime.bindingCalled")]
    BindingCalled,
}
#[doc = "Notification is issued every time when binding is called.\n[bindingCalled](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-bindingCalled)"]
#[derive(Debug, Clone, PartialEq)]
pub struct BindingCalled {
    pub method: BindingCalledMethod,
    pub params: BindingCalledParams,
}
impl BindingCalled {
    pub const IDENTIFIER: &'static str = "Runtime.bindingCalled";
}
#[doc = "Issued when console API was called.\n[consoleAPICalled](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-consoleAPICalled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsoleApiCalledParams {
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ConsoleApiCalledMethod {
    #[serde(rename = "Runtime.consoleAPICalled")]
    ConsoleApiCalled,
}
#[doc = "Issued when console API was called.\n[consoleAPICalled](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-consoleAPICalled)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ConsoleApiCalled {
    pub method: ConsoleApiCalledMethod,
    pub params: ConsoleApiCalledParams,
}
impl ConsoleApiCalled {
    pub const IDENTIFIER: &'static str = "Runtime.consoleAPICalled";
}
#[doc = "Issued when unhandled exception was revoked.\n[exceptionRevoked](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-exceptionRevoked)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExceptionRevokedParams {
    #[doc = "Reason describing why exception was revoked."]
    #[serde(rename = "reason")]
    pub reason: String,
    #[doc = "The id of revoked exception, as reported in `exceptionThrown`."]
    #[serde(rename = "exceptionId")]
    pub exception_id: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExceptionRevokedMethod {
    #[serde(rename = "Runtime.exceptionRevoked")]
    ExceptionRevoked,
}
#[doc = "Issued when unhandled exception was revoked.\n[exceptionRevoked](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-exceptionRevoked)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ExceptionRevoked {
    pub method: ExceptionRevokedMethod,
    pub params: ExceptionRevokedParams,
}
impl ExceptionRevoked {
    pub const IDENTIFIER: &'static str = "Runtime.exceptionRevoked";
}
#[doc = "Issued when exception was thrown and unhandled.\n[exceptionThrown](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-exceptionThrown)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExceptionThrownParams {
    #[doc = "Timestamp of the exception."]
    #[serde(rename = "timestamp")]
    pub timestamp: super::types::Timestamp,
    #[serde(rename = "exceptionDetails")]
    pub exception_details: super::types::ExceptionDetails,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExceptionThrownMethod {
    #[serde(rename = "Runtime.exceptionThrown")]
    ExceptionThrown,
}
#[doc = "Issued when exception was thrown and unhandled.\n[exceptionThrown](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-exceptionThrown)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ExceptionThrown {
    pub method: ExceptionThrownMethod,
    pub params: ExceptionThrownParams,
}
impl ExceptionThrown {
    pub const IDENTIFIER: &'static str = "Runtime.exceptionThrown";
}
#[doc = "Issued when new execution context is created.\n[executionContextCreated](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-executionContextCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionContextCreatedParams {
    #[doc = "A newly created execution context."]
    #[serde(rename = "context")]
    pub context: super::types::ExecutionContextDescription,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionContextCreatedMethod {
    #[serde(rename = "Runtime.executionContextCreated")]
    ExecutionContextCreated,
}
#[doc = "Issued when new execution context is created.\n[executionContextCreated](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-executionContextCreated)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionContextCreated {
    pub method: ExecutionContextCreatedMethod,
    pub params: ExecutionContextCreatedParams,
}
impl ExecutionContextCreated {
    pub const IDENTIFIER: &'static str = "Runtime.executionContextCreated";
}
#[doc = "Issued when execution context is destroyed.\n[executionContextDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-executionContextDestroyed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionContextDestroyedParams {
    #[doc = "Unique Id of the destroyed context"]
    #[serde(rename = "executionContextUniqueId")]
    pub execution_context_unique_id: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionContextDestroyedMethod {
    #[serde(rename = "Runtime.executionContextDestroyed")]
    ExecutionContextDestroyed,
}
#[doc = "Issued when execution context is destroyed.\n[executionContextDestroyed](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-executionContextDestroyed)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionContextDestroyed {
    pub method: ExecutionContextDestroyedMethod,
    pub params: ExecutionContextDestroyedParams,
}
impl ExecutionContextDestroyed {
    pub const IDENTIFIER: &'static str = "Runtime.executionContextDestroyed";
}
#[doc = "Issued when all executionContexts were cleared in browser\n[executionContextsCleared](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-executionContextsCleared)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecutionContextsClearedParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecutionContextsClearedMethod {
    #[serde(rename = "Runtime.executionContextsCleared")]
    ExecutionContextsCleared,
}
#[doc = "Issued when all executionContexts were cleared in browser\n[executionContextsCleared](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-executionContextsCleared)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionContextsCleared {
    pub method: ExecutionContextsClearedMethod,
    pub params: ExecutionContextsClearedParams,
}
impl ExecutionContextsCleared {
    pub const IDENTIFIER: &'static str = "Runtime.executionContextsCleared";
}
#[doc = "Issued when object should be inspected (for example, as a result of inspect() command line API\ncall).\n[inspectRequested](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-inspectRequested)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectRequestedParams {
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InspectRequestedMethod {
    #[serde(rename = "Runtime.inspectRequested")]
    InspectRequested,
}
#[doc = "Issued when object should be inspected (for example, as a result of inspect() command line API\ncall).\n[inspectRequested](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#event-inspectRequested)"]
#[derive(Debug, Clone, PartialEq)]
pub struct InspectRequested {
    pub method: InspectRequestedMethod,
    pub params: InspectRequestedParams,
}
impl InspectRequested {
    pub const IDENTIFIER: &'static str = "Runtime.inspectRequested";
}
group_enum ! (RuntimeEvents { BindingCalled (BindingCalled) , ConsoleApiCalled (ConsoleApiCalled) , ExceptionRevoked (ExceptionRevoked) , ExceptionThrown (ExceptionThrown) , ExecutionContextCreated (ExecutionContextCreated) , ExecutionContextDestroyed (ExecutionContextDestroyed) , ExecutionContextsCleared (ExecutionContextsCleared) , InspectRequested (InspectRequested) });
