use serde::{Deserialize, Serialize};
#[doc = "Add handler to promise with given promise object id.\n[awaitPromise](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-awaitPromise)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwaitPromiseParams {
    #[doc = "Identifier of the promise."]
    #[serde(rename = "promiseObjectId")]
    pub promise_object_id: super::types::RemoteObjectId,
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
}
impl AwaitPromiseParams {
    pub fn new(promise_object_id: impl Into<super::types::RemoteObjectId>) -> Self {
        Self {
            promise_object_id: promise_object_id.into(),
            return_by_value: None,
            generate_preview: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AwaitPromiseMethod {
    #[serde(rename = "Runtime.awaitPromise")]
    AwaitPromise,
}
impl AwaitPromiseMethod {
    pub const IDENTIFIER: &'static str = "Runtime.awaitPromise";
}
#[doc = "Add handler to promise with given promise object id.\n[awaitPromise](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-awaitPromise)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AwaitPromise {
    pub method: AwaitPromiseMethod,
    pub params: AwaitPromiseParams,
}
#[doc = "Calls function with given declaration on the given object. Object group of the result is\ninherited from the target object.\n[callFunctionOn](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-callFunctionOn)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallFunctionOnParams {
    #[doc = "Declaration of the function to call."]
    #[serde(rename = "functionDeclaration")]
    pub function_declaration: String,
    #[doc = "Identifier of the object to call function on. Either objectId or executionContextId should\nbe specified."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<super::types::RemoteObjectId>,
    #[doc = "Call arguments. All call arguments must belong to the same JavaScript world as the target\nobject."]
    #[serde(rename = "arguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub arguments: Option<Vec<super::types::CallArgument>>,
    #[doc = "In silent mode exceptions thrown during evaluation are not reported and do not pause\nexecution. Overrides `setPauseOnException` state."]
    #[serde(rename = "silent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub silent: Option<bool>,
    #[doc = "Whether the result is expected to be a JSON object which should be sent by value.\nCan be overriden by `serializationOptions`."]
    #[serde(rename = "returnByValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub return_by_value: Option<bool>,
    #[doc = "Whether preview should be generated for the result."]
    #[serde(rename = "generatePreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub generate_preview: Option<bool>,
    #[doc = "Whether execution should be treated as initiated by user in the UI."]
    #[serde(rename = "userGesture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_gesture: Option<bool>,
    #[doc = "Whether execution should `await` for resulting value and return once awaited promise is\nresolved."]
    #[serde(rename = "awaitPromise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub await_promise: Option<bool>,
    #[doc = "Specifies execution context which global object will be used to call function on. Either\nexecutionContextId or objectId should be specified."]
    #[serde(rename = "executionContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub execution_context_id: Option<super::types::ExecutionContextId>,
    #[doc = "Symbolic group name that can be used to release multiple objects. If objectGroup is not\nspecified and objectId is, objectGroup will be inherited from object."]
    #[serde(rename = "objectGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_group: Option<String>,
    #[doc = "Whether to throw an exception if side effect cannot be ruled out during evaluation."]
    #[serde(rename = "throwOnSideEffect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub throw_on_side_effect: Option<bool>,
    #[doc = "An alternative way to specify the execution context to call function on.\nCompared to contextId that may be reused across processes, this is guaranteed to be\nsystem-unique, so it can be used to prevent accidental function call\nin context different than intended (e.g. as a result of navigation across process\nboundaries).\nThis is mutually exclusive with `executionContextId`."]
    #[serde(rename = "uniqueContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unique_context_id: Option<String>,
    #[doc = "Specifies the result serialization. If provided, overrides\n`generatePreview` and `returnByValue`."]
    #[serde(rename = "serializationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub serialization_options: Option<super::types::SerializationOptions>,
}
impl CallFunctionOnParams {
    pub fn new(function_declaration: impl Into<String>) -> Self {
        Self {
            function_declaration: function_declaration.into(),
            object_id: None,
            arguments: None,
            silent: None,
            return_by_value: None,
            generate_preview: None,
            user_gesture: None,
            await_promise: None,
            execution_context_id: None,
            object_group: None,
            throw_on_side_effect: None,
            unique_context_id: None,
            serialization_options: None,
        }
    }
}
impl<T: Into<String>> From<T> for CallFunctionOnParams {
    fn from(url: T) -> Self {
        CallFunctionOnParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CallFunctionOnMethod {
    #[serde(rename = "Runtime.callFunctionOn")]
    CallFunctionOn,
}
impl CallFunctionOnMethod {
    pub const IDENTIFIER: &'static str = "Runtime.callFunctionOn";
}
#[doc = "Calls function with given declaration on the given object. Object group of the result is\ninherited from the target object.\n[callFunctionOn](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-callFunctionOn)"]
#[derive(Debug, Clone, PartialEq)]
pub struct CallFunctionOn {
    pub method: CallFunctionOnMethod,
    pub params: CallFunctionOnParams,
}
#[doc = "Compiles expression.\n[compileScript](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-compileScript)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompileScriptParams {
    #[doc = "Expression to compile."]
    #[serde(rename = "expression")]
    pub expression: String,
    #[doc = "Source url to be set for the script."]
    #[serde(rename = "sourceURL")]
    pub source_url: String,
    #[doc = "Specifies whether the compiled script should be persisted."]
    #[serde(rename = "persistScript")]
    pub persist_script: bool,
    #[doc = "Specifies in which execution context to perform script run. If the parameter is omitted the\nevaluation will be performed in the context of the inspected page."]
    #[serde(rename = "executionContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub execution_context_id: Option<super::types::ExecutionContextId>,
}
impl CompileScriptParams {
    pub fn new(
        expression: impl Into<String>,
        source_url: impl Into<String>,
        persist_script: impl Into<bool>,
    ) -> Self {
        Self {
            expression: expression.into(),
            source_url: source_url.into(),
            persist_script: persist_script.into(),
            execution_context_id: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompileScriptMethod {
    #[serde(rename = "Runtime.compileScript")]
    CompileScript,
}
impl CompileScriptMethod {
    pub const IDENTIFIER: &'static str = "Runtime.compileScript";
}
#[doc = "Compiles expression.\n[compileScript](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-compileScript)"]
#[derive(Debug, Clone, PartialEq)]
pub struct CompileScript {
    pub method: CompileScriptMethod,
    pub params: CompileScriptParams,
}
#[doc = "Disables reporting of execution contexts creation.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Runtime.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "Runtime.disable";
}
#[doc = "Disables reporting of execution contexts creation.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
#[doc = "Discards collected exceptions and console API calls.\n[discardConsoleEntries](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-discardConsoleEntries)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DiscardConsoleEntriesParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DiscardConsoleEntriesMethod {
    #[serde(rename = "Runtime.discardConsoleEntries")]
    DiscardConsoleEntries,
}
impl DiscardConsoleEntriesMethod {
    pub const IDENTIFIER: &'static str = "Runtime.discardConsoleEntries";
}
#[doc = "Discards collected exceptions and console API calls.\n[discardConsoleEntries](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-discardConsoleEntries)"]
#[derive(Debug, Clone, PartialEq)]
pub struct DiscardConsoleEntries {
    pub method: DiscardConsoleEntriesMethod,
    pub params: DiscardConsoleEntriesParams,
}
#[doc = "Enables reporting of execution contexts creation by means of `executionContextCreated` event.\nWhen the reporting gets enabled the event will be sent immediately for each existing execution\ncontext.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Runtime.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "Runtime.enable";
}
#[doc = "Enables reporting of execution contexts creation by means of `executionContextCreated` event.\nWhen the reporting gets enabled the event will be sent immediately for each existing execution\ncontext.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
#[doc = "Evaluates expression on global object.\n[evaluate](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-evaluate)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluateParams {
    #[doc = "Expression to evaluate."]
    #[serde(rename = "expression")]
    pub expression: String,
    #[doc = "Symbolic group name that can be used to release multiple objects."]
    #[serde(rename = "objectGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_group: Option<String>,
    #[doc = "Determines whether Command Line API should be available during the evaluation."]
    #[serde(rename = "includeCommandLineAPI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_command_line_api: Option<bool>,
    #[doc = "In silent mode exceptions thrown during evaluation are not reported and do not pause\nexecution. Overrides `setPauseOnException` state."]
    #[serde(rename = "silent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub silent: Option<bool>,
    #[doc = "Specifies in which execution context to perform evaluation. If the parameter is omitted the\nevaluation will be performed in the context of the inspected page.\nThis is mutually exclusive with `uniqueContextId`, which offers an\nalternative way to identify the execution context that is more reliable\nin a multi-process environment."]
    #[serde(rename = "contextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub context_id: Option<super::types::ExecutionContextId>,
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
    #[doc = "Whether execution should be treated as initiated by user in the UI."]
    #[serde(rename = "userGesture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_gesture: Option<bool>,
    #[doc = "Whether execution should `await` for resulting value and return once awaited promise is\nresolved."]
    #[serde(rename = "awaitPromise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub await_promise: Option<bool>,
    #[doc = "Whether to throw an exception if side effect cannot be ruled out during evaluation.\nThis implies `disableBreaks` below."]
    #[serde(rename = "throwOnSideEffect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub throw_on_side_effect: Option<bool>,
    #[doc = "Terminate execution after timing out (number of milliseconds)."]
    #[serde(rename = "timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub timeout: Option<super::types::TimeDelta>,
    #[doc = "Disable breakpoints during execution."]
    #[serde(rename = "disableBreaks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub disable_breaks: Option<bool>,
    #[doc = "Setting this flag to true enables `let` re-declaration and top-level `await`.\nNote that `let` variables can only be re-declared if they originate from\n`replMode` themselves."]
    #[serde(rename = "replMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub repl_mode: Option<bool>,
    #[doc = "The Content Security Policy (CSP) for the target might block 'unsafe-eval'\nwhich includes eval(), Function(), setTimeout() and setInterval()\nwhen called with non-callable arguments. This flag bypasses CSP for this\nevaluation and allows unsafe-eval. Defaults to true."]
    #[serde(rename = "allowUnsafeEvalBlockedByCSP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub allow_unsafe_eval_blocked_by_csp: Option<bool>,
    #[doc = "An alternative way to specify the execution context to evaluate in.\nCompared to contextId that may be reused across processes, this is guaranteed to be\nsystem-unique, so it can be used to prevent accidental evaluation of the expression\nin context different than intended (e.g. as a result of navigation across process\nboundaries).\nThis is mutually exclusive with `contextId`."]
    #[serde(rename = "uniqueContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unique_context_id: Option<String>,
    #[doc = "Specifies the result serialization. If provided, overrides\n`generatePreview` and `returnByValue`."]
    #[serde(rename = "serializationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub serialization_options: Option<super::types::SerializationOptions>,
}
impl EvaluateParams {
    pub fn new(expression: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),
            object_group: None,
            include_command_line_api: None,
            silent: None,
            context_id: None,
            return_by_value: None,
            generate_preview: None,
            user_gesture: None,
            await_promise: None,
            throw_on_side_effect: None,
            timeout: None,
            disable_breaks: None,
            repl_mode: None,
            allow_unsafe_eval_blocked_by_csp: None,
            unique_context_id: None,
            serialization_options: None,
        }
    }
}
impl<T: Into<String>> From<T> for EvaluateParams {
    fn from(url: T) -> Self {
        EvaluateParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EvaluateMethod {
    #[serde(rename = "Runtime.evaluate")]
    Evaluate,
}
impl EvaluateMethod {
    pub const IDENTIFIER: &'static str = "Runtime.evaluate";
}
#[doc = "Evaluates expression on global object.\n[evaluate](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-evaluate)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Evaluate {
    pub method: EvaluateMethod,
    pub params: EvaluateParams,
}
#[doc = "Returns the isolate id.\n[getIsolateId](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-getIsolateId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetIsolateIdParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetIsolateIdMethod {
    #[serde(rename = "Runtime.getIsolateId")]
    GetIsolateId,
}
impl GetIsolateIdMethod {
    pub const IDENTIFIER: &'static str = "Runtime.getIsolateId";
}
#[doc = "Returns the isolate id.\n[getIsolateId](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-getIsolateId)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetIsolateId {
    pub method: GetIsolateIdMethod,
    pub params: GetIsolateIdParams,
}
#[doc = "Returns the JavaScript heap usage.\nIt is the total usage of the corresponding isolate not scoped to a particular Runtime.\n[getHeapUsage](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-getHeapUsage)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetHeapUsageParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetHeapUsageMethod {
    #[serde(rename = "Runtime.getHeapUsage")]
    GetHeapUsage,
}
impl GetHeapUsageMethod {
    pub const IDENTIFIER: &'static str = "Runtime.getHeapUsage";
}
#[doc = "Returns the JavaScript heap usage.\nIt is the total usage of the corresponding isolate not scoped to a particular Runtime.\n[getHeapUsage](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-getHeapUsage)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetHeapUsage {
    pub method: GetHeapUsageMethod,
    pub params: GetHeapUsageParams,
}
#[doc = "Returns properties of a given object. Object group of the result is inherited from the target\nobject.\n[getProperties](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-getProperties)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPropertiesParams {
    #[doc = "Identifier of the object to return properties for."]
    #[serde(rename = "objectId")]
    pub object_id: super::types::RemoteObjectId,
    #[doc = "If true, returns properties belonging only to the element itself, not to its prototype\nchain."]
    #[serde(rename = "ownProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub own_properties: Option<bool>,
    #[doc = "If true, returns accessor properties (with getter/setter) only; internal properties are not\nreturned either."]
    #[serde(rename = "accessorPropertiesOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub accessor_properties_only: Option<bool>,
    #[doc = "Whether preview should be generated for the results."]
    #[serde(rename = "generatePreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub generate_preview: Option<bool>,
    #[doc = "If true, returns non-indexed properties only."]
    #[serde(rename = "nonIndexedPropertiesOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub non_indexed_properties_only: Option<bool>,
}
impl GetPropertiesParams {
    pub fn new(object_id: impl Into<super::types::RemoteObjectId>) -> Self {
        Self {
            object_id: object_id.into(),
            own_properties: None,
            accessor_properties_only: None,
            generate_preview: None,
            non_indexed_properties_only: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetPropertiesMethod {
    #[serde(rename = "Runtime.getProperties")]
    GetProperties,
}
impl GetPropertiesMethod {
    pub const IDENTIFIER: &'static str = "Runtime.getProperties";
}
#[doc = "Returns properties of a given object. Object group of the result is inherited from the target\nobject.\n[getProperties](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-getProperties)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetProperties {
    pub method: GetPropertiesMethod,
    pub params: GetPropertiesParams,
}
#[doc = "Returns all let, const and class variables from global scope.\n[globalLexicalScopeNames](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-globalLexicalScopeNames)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GlobalLexicalScopeNamesParams {
    #[doc = "Specifies in which execution context to lookup global scope variables."]
    #[serde(rename = "executionContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub execution_context_id: Option<super::types::ExecutionContextId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GlobalLexicalScopeNamesMethod {
    #[serde(rename = "Runtime.globalLexicalScopeNames")]
    GlobalLexicalScopeNames,
}
impl GlobalLexicalScopeNamesMethod {
    pub const IDENTIFIER: &'static str = "Runtime.globalLexicalScopeNames";
}
#[doc = "Returns all let, const and class variables from global scope.\n[globalLexicalScopeNames](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-globalLexicalScopeNames)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GlobalLexicalScopeNames {
    pub method: GlobalLexicalScopeNamesMethod,
    pub params: GlobalLexicalScopeNamesParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryObjectsParams {
    #[doc = "Identifier of the prototype to return objects for."]
    #[serde(rename = "prototypeObjectId")]
    pub prototype_object_id: super::types::RemoteObjectId,
    #[doc = "Symbolic group name that can be used to release the results."]
    #[serde(rename = "objectGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_group: Option<String>,
}
impl QueryObjectsParams {
    pub fn new(prototype_object_id: impl Into<super::types::RemoteObjectId>) -> Self {
        Self {
            prototype_object_id: prototype_object_id.into(),
            object_group: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QueryObjectsMethod {
    #[serde(rename = "Runtime.queryObjects")]
    QueryObjects,
}
impl QueryObjectsMethod {
    pub const IDENTIFIER: &'static str = "Runtime.queryObjects";
}
#[derive(Debug, Clone, PartialEq)]
pub struct QueryObjects {
    pub method: QueryObjectsMethod,
    pub params: QueryObjectsParams,
}
#[doc = "Releases remote object with given id.\n[releaseObject](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-releaseObject)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseObjectParams {
    #[doc = "Identifier of the object to release."]
    #[serde(rename = "objectId")]
    pub object_id: super::types::RemoteObjectId,
}
impl ReleaseObjectParams {
    pub fn new(object_id: impl Into<super::types::RemoteObjectId>) -> Self {
        Self {
            object_id: object_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReleaseObjectMethod {
    #[serde(rename = "Runtime.releaseObject")]
    ReleaseObject,
}
impl ReleaseObjectMethod {
    pub const IDENTIFIER: &'static str = "Runtime.releaseObject";
}
#[doc = "Releases remote object with given id.\n[releaseObject](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-releaseObject)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ReleaseObject {
    pub method: ReleaseObjectMethod,
    pub params: ReleaseObjectParams,
}
#[doc = "Releases all remote objects that belong to a given group.\n[releaseObjectGroup](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-releaseObjectGroup)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReleaseObjectGroupParams {
    #[doc = "Symbolic object group name."]
    #[serde(rename = "objectGroup")]
    pub object_group: String,
}
impl ReleaseObjectGroupParams {
    pub fn new(object_group: impl Into<String>) -> Self {
        Self {
            object_group: object_group.into(),
        }
    }
}
impl<T: Into<String>> From<T> for ReleaseObjectGroupParams {
    fn from(url: T) -> Self {
        ReleaseObjectGroupParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReleaseObjectGroupMethod {
    #[serde(rename = "Runtime.releaseObjectGroup")]
    ReleaseObjectGroup,
}
impl ReleaseObjectGroupMethod {
    pub const IDENTIFIER: &'static str = "Runtime.releaseObjectGroup";
}
#[doc = "Releases all remote objects that belong to a given group.\n[releaseObjectGroup](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-releaseObjectGroup)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ReleaseObjectGroup {
    pub method: ReleaseObjectGroupMethod,
    pub params: ReleaseObjectGroupParams,
}
#[doc = "Tells inspected instance to run if it was waiting for debugger to attach.\n[runIfWaitingForDebugger](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-runIfWaitingForDebugger)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RunIfWaitingForDebuggerParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RunIfWaitingForDebuggerMethod {
    #[serde(rename = "Runtime.runIfWaitingForDebugger")]
    RunIfWaitingForDebugger,
}
impl RunIfWaitingForDebuggerMethod {
    pub const IDENTIFIER: &'static str = "Runtime.runIfWaitingForDebugger";
}
#[doc = "Tells inspected instance to run if it was waiting for debugger to attach.\n[runIfWaitingForDebugger](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-runIfWaitingForDebugger)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RunIfWaitingForDebugger {
    pub method: RunIfWaitingForDebuggerMethod,
    pub params: RunIfWaitingForDebuggerParams,
}
#[doc = "Runs script with given id in a given context.\n[runScript](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-runScript)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunScriptParams {
    #[doc = "Id of the script to run."]
    #[serde(rename = "scriptId")]
    pub script_id: super::types::ScriptId,
    #[doc = "Specifies in which execution context to perform script run. If the parameter is omitted the\nevaluation will be performed in the context of the inspected page."]
    #[serde(rename = "executionContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub execution_context_id: Option<super::types::ExecutionContextId>,
    #[doc = "Symbolic group name that can be used to release multiple objects."]
    #[serde(rename = "objectGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_group: Option<String>,
    #[doc = "In silent mode exceptions thrown during evaluation are not reported and do not pause\nexecution. Overrides `setPauseOnException` state."]
    #[serde(rename = "silent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub silent: Option<bool>,
    #[doc = "Determines whether Command Line API should be available during the evaluation."]
    #[serde(rename = "includeCommandLineAPI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_command_line_api: Option<bool>,
    #[doc = "Whether the result is expected to be a JSON object which should be sent by value."]
    #[serde(rename = "returnByValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub return_by_value: Option<bool>,
    #[doc = "Whether preview should be generated for the result."]
    #[serde(rename = "generatePreview")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub generate_preview: Option<bool>,
    #[doc = "Whether execution should `await` for resulting value and return once awaited promise is\nresolved."]
    #[serde(rename = "awaitPromise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub await_promise: Option<bool>,
}
impl RunScriptParams {
    pub fn new(script_id: impl Into<super::types::ScriptId>) -> Self {
        Self {
            script_id: script_id.into(),
            execution_context_id: None,
            object_group: None,
            silent: None,
            include_command_line_api: None,
            return_by_value: None,
            generate_preview: None,
            await_promise: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RunScriptMethod {
    #[serde(rename = "Runtime.runScript")]
    RunScript,
}
impl RunScriptMethod {
    pub const IDENTIFIER: &'static str = "Runtime.runScript";
}
#[doc = "Runs script with given id in a given context.\n[runScript](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-runScript)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RunScript {
    pub method: RunScriptMethod,
    pub params: RunScriptParams,
}
#[doc = "Enables or disables async call stacks tracking.\n[setAsyncCallStackDepth](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-setAsyncCallStackDepth)"]
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
    #[serde(rename = "Runtime.setAsyncCallStackDepth")]
    SetAsyncCallStackDepth,
}
impl SetAsyncCallStackDepthMethod {
    pub const IDENTIFIER: &'static str = "Runtime.setAsyncCallStackDepth";
}
#[doc = "Enables or disables async call stacks tracking.\n[setAsyncCallStackDepth](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-setAsyncCallStackDepth)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetAsyncCallStackDepth {
    pub method: SetAsyncCallStackDepthMethod,
    pub params: SetAsyncCallStackDepthParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCustomObjectFormatterEnabledParams {
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
impl SetCustomObjectFormatterEnabledParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self {
            enabled: enabled.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetCustomObjectFormatterEnabledMethod {
    #[serde(rename = "Runtime.setCustomObjectFormatterEnabled")]
    SetCustomObjectFormatterEnabled,
}
impl SetCustomObjectFormatterEnabledMethod {
    pub const IDENTIFIER: &'static str = "Runtime.setCustomObjectFormatterEnabled";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetCustomObjectFormatterEnabled {
    pub method: SetCustomObjectFormatterEnabledMethod,
    pub params: SetCustomObjectFormatterEnabledParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetMaxCallStackSizeToCaptureParams {
    #[serde(rename = "size")]
    pub size: i64,
}
impl SetMaxCallStackSizeToCaptureParams {
    pub fn new(size: impl Into<i64>) -> Self {
        Self { size: size.into() }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetMaxCallStackSizeToCaptureMethod {
    #[serde(rename = "Runtime.setMaxCallStackSizeToCapture")]
    SetMaxCallStackSizeToCapture,
}
impl SetMaxCallStackSizeToCaptureMethod {
    pub const IDENTIFIER: &'static str = "Runtime.setMaxCallStackSizeToCapture";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetMaxCallStackSizeToCapture {
    pub method: SetMaxCallStackSizeToCaptureMethod,
    pub params: SetMaxCallStackSizeToCaptureParams,
}
#[doc = "Terminate current or next JavaScript execution.\nWill cancel the termination when the outer-most script execution ends.\n[terminateExecution](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-terminateExecution)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TerminateExecutionParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TerminateExecutionMethod {
    #[serde(rename = "Runtime.terminateExecution")]
    TerminateExecution,
}
impl TerminateExecutionMethod {
    pub const IDENTIFIER: &'static str = "Runtime.terminateExecution";
}
#[doc = "Terminate current or next JavaScript execution.\nWill cancel the termination when the outer-most script execution ends.\n[terminateExecution](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-terminateExecution)"]
#[derive(Debug, Clone, PartialEq)]
pub struct TerminateExecution {
    pub method: TerminateExecutionMethod,
    pub params: TerminateExecutionParams,
}
#[doc = "If executionContextId is empty, adds binding with the given name on the\nglobal objects of all inspected contexts, including those created later,\nbindings survive reloads.\nBinding function takes exactly one argument, this argument should be string,\nin case of any other input, function throws an exception.\nEach binding function call produces Runtime.bindingCalled notification.\n[addBinding](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-addBinding)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddBindingParams {
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "If specified, the binding is exposed to the executionContext with\nmatching name, even for contexts created after the binding is added.\nSee also `ExecutionContext.name` and `worldName` parameter to\n`Page.addScriptToEvaluateOnNewDocument`.\nThis parameter is mutually exclusive with `executionContextId`."]
    #[serde(rename = "executionContextName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub execution_context_name: Option<String>,
}
impl AddBindingParams {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            execution_context_name: None,
        }
    }
}
impl<T: Into<String>> From<T> for AddBindingParams {
    fn from(url: T) -> Self {
        AddBindingParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddBindingMethod {
    #[serde(rename = "Runtime.addBinding")]
    AddBinding,
}
impl AddBindingMethod {
    pub const IDENTIFIER: &'static str = "Runtime.addBinding";
}
#[doc = "If executionContextId is empty, adds binding with the given name on the\nglobal objects of all inspected contexts, including those created later,\nbindings survive reloads.\nBinding function takes exactly one argument, this argument should be string,\nin case of any other input, function throws an exception.\nEach binding function call produces Runtime.bindingCalled notification.\n[addBinding](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-addBinding)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AddBinding {
    pub method: AddBindingMethod,
    pub params: AddBindingParams,
}
#[doc = "This method does not remove binding function from global object but\nunsubscribes current runtime agent from Runtime.bindingCalled notifications.\n[removeBinding](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-removeBinding)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveBindingParams {
    #[serde(rename = "name")]
    pub name: String,
}
impl RemoveBindingParams {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
impl<T: Into<String>> From<T> for RemoveBindingParams {
    fn from(url: T) -> Self {
        RemoveBindingParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveBindingMethod {
    #[serde(rename = "Runtime.removeBinding")]
    RemoveBinding,
}
impl RemoveBindingMethod {
    pub const IDENTIFIER: &'static str = "Runtime.removeBinding";
}
#[doc = "This method does not remove binding function from global object but\nunsubscribes current runtime agent from Runtime.bindingCalled notifications.\n[removeBinding](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-removeBinding)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RemoveBinding {
    pub method: RemoveBindingMethod,
    pub params: RemoveBindingParams,
}
#[doc = "This method tries to lookup and populate exception details for a\nJavaScript Error object.\nNote that the stackTrace portion of the resulting exceptionDetails will\nonly be populated if the Runtime domain was enabled at the time when the\nError was thrown.\n[getExceptionDetails](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-getExceptionDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetExceptionDetailsParams {
    #[doc = "The error object for which to resolve the exception details."]
    #[serde(rename = "errorObjectId")]
    pub error_object_id: super::types::RemoteObjectId,
}
impl GetExceptionDetailsParams {
    pub fn new(error_object_id: impl Into<super::types::RemoteObjectId>) -> Self {
        Self {
            error_object_id: error_object_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetExceptionDetailsMethod {
    #[serde(rename = "Runtime.getExceptionDetails")]
    GetExceptionDetails,
}
impl GetExceptionDetailsMethod {
    pub const IDENTIFIER: &'static str = "Runtime.getExceptionDetails";
}
#[doc = "This method tries to lookup and populate exception details for a\nJavaScript Error object.\nNote that the stackTrace portion of the resulting exceptionDetails will\nonly be populated if the Runtime domain was enabled at the time when the\nError was thrown.\n[getExceptionDetails](https://chromedevtools.github.io/devtools-protocol/tot/Runtime/#method-getExceptionDetails)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetExceptionDetails {
    pub method: GetExceptionDetailsMethod,
    pub params: GetExceptionDetailsParams,
}
group_enum ! (RuntimeCommands { AwaitPromise (AwaitPromise) , CallFunctionOn (CallFunctionOn) , CompileScript (CompileScript) , Disable (Disable) , DiscardConsoleEntries (DiscardConsoleEntries) , Enable (Enable) , Evaluate (Evaluate) , GetIsolateId (GetIsolateId) , GetHeapUsage (GetHeapUsage) , GetProperties (GetProperties) , GlobalLexicalScopeNames (GlobalLexicalScopeNames) , QueryObjects (QueryObjects) , ReleaseObject (ReleaseObject) , ReleaseObjectGroup (ReleaseObjectGroup) , RunIfWaitingForDebugger (RunIfWaitingForDebugger) , RunScript (RunScript) , SetAsyncCallStackDepth (SetAsyncCallStackDepth) , SetCustomObjectFormatterEnabled (SetCustomObjectFormatterEnabled) , SetMaxCallStackSizeToCapture (SetMaxCallStackSizeToCapture) , TerminateExecution (TerminateExecution) , AddBinding (AddBinding) , RemoveBinding (RemoveBinding) , GetExceptionDetails (GetExceptionDetails) });
