use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwaitPromiseResult {
    #[doc = "Promise result. Will contain rejected value if promise was rejected."]
    #[serde(rename = "result")]
    pub result: super::types::RemoteObject,
    #[doc = "Exception details if stack strace is available."]
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<super::types::ExceptionDetails>,
}
impl TryFrom<serde_json::Value> for AwaitPromiseResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallFunctionOnResult {
    #[doc = "Call result."]
    #[serde(rename = "result")]
    pub result: super::types::RemoteObject,
    #[doc = "Exception details."]
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<super::types::ExceptionDetails>,
}
impl TryFrom<serde_json::Value> for CallFunctionOnResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CompileScriptResult {
    #[doc = "Id of the script."]
    #[serde(rename = "scriptId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub script_id: Option<super::types::ScriptId>,
    #[doc = "Exception details."]
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<super::types::ExceptionDetails>,
}
impl TryFrom<serde_json::Value> for CompileScriptResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
impl TryFrom<serde_json::Value> for DisableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DiscardConsoleEntriesResult {}
impl TryFrom<serde_json::Value> for DiscardConsoleEntriesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
impl TryFrom<serde_json::Value> for EnableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluateResult {
    #[doc = "Evaluation result."]
    #[serde(rename = "result")]
    pub result: super::types::RemoteObject,
    #[doc = "Exception details."]
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<super::types::ExceptionDetails>,
}
impl TryFrom<serde_json::Value> for EvaluateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetIsolateIdResult {
    #[doc = "The isolate id."]
    #[serde(rename = "id")]
    pub id: String,
}
impl TryFrom<serde_json::Value> for GetIsolateIdResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHeapUsageResult {
    #[doc = "Used JavaScript heap size in bytes."]
    #[serde(rename = "usedSize")]
    pub used_size: f64,
    #[doc = "Allocated JavaScript heap size in bytes."]
    #[serde(rename = "totalSize")]
    pub total_size: f64,
    #[doc = "Used size in bytes in the embedder's garbage-collected heap."]
    #[serde(rename = "embedderHeapUsedSize")]
    pub embedder_heap_used_size: f64,
    #[doc = "Size in bytes of backing storage for array buffers and external strings."]
    #[serde(rename = "backingStorageSize")]
    pub backing_storage_size: f64,
}
impl TryFrom<serde_json::Value> for GetHeapUsageResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPropertiesResult {
    #[doc = "Object properties."]
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<super::types::PropertyDescriptor>,
    #[doc = "Internal object properties (only of the element itself)."]
    #[serde(rename = "internalProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub internal_properties: Option<Vec<super::types::InternalPropertyDescriptor>>,
    #[doc = "Object private properties."]
    #[serde(rename = "privateProperties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub private_properties: Option<Vec<super::types::PrivatePropertyDescriptor>>,
    #[doc = "Exception details."]
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<super::types::ExceptionDetails>,
}
impl TryFrom<serde_json::Value> for GetPropertiesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalLexicalScopeNamesResult {
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
}
impl TryFrom<serde_json::Value> for GlobalLexicalScopeNamesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryObjectsResult {
    #[doc = "Array with objects."]
    #[serde(rename = "objects")]
    pub objects: super::types::RemoteObject,
}
impl TryFrom<serde_json::Value> for QueryObjectsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ReleaseObjectResult {}
impl TryFrom<serde_json::Value> for ReleaseObjectResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ReleaseObjectGroupResult {}
impl TryFrom<serde_json::Value> for ReleaseObjectGroupResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RunIfWaitingForDebuggerResult {}
impl TryFrom<serde_json::Value> for RunIfWaitingForDebuggerResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunScriptResult {
    #[doc = "Run result."]
    #[serde(rename = "result")]
    pub result: super::types::RemoteObject,
    #[doc = "Exception details."]
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<super::types::ExceptionDetails>,
}
impl TryFrom<serde_json::Value> for RunScriptResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAsyncCallStackDepthResult {}
impl TryFrom<serde_json::Value> for SetAsyncCallStackDepthResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetCustomObjectFormatterEnabledResult {}
impl TryFrom<serde_json::Value> for SetCustomObjectFormatterEnabledResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetMaxCallStackSizeToCaptureResult {}
impl TryFrom<serde_json::Value> for SetMaxCallStackSizeToCaptureResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TerminateExecutionResult {}
impl TryFrom<serde_json::Value> for TerminateExecutionResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddBindingResult {}
impl TryFrom<serde_json::Value> for AddBindingResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveBindingResult {}
impl TryFrom<serde_json::Value> for RemoveBindingResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetExceptionDetailsResult {
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<super::types::ExceptionDetails>,
}
impl TryFrom<serde_json::Value> for GetExceptionDetailsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
