use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwaitPromiseReturns {
    #[doc = "Promise result. Will contain rejected value if promise was rejected."]
    #[serde(rename = "result")]
    pub result: super::types::RemoteObject,
    #[doc = "Exception details if stack strace is available."]
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<super::types::ExceptionDetails>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallFunctionOnReturns {
    #[doc = "Call result."]
    #[serde(rename = "result")]
    pub result: super::types::RemoteObject,
    #[doc = "Exception details."]
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<super::types::ExceptionDetails>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CompileScriptReturns {
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvaluateReturns {
    #[doc = "Evaluation result."]
    #[serde(rename = "result")]
    pub result: super::types::RemoteObject,
    #[doc = "Exception details."]
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<super::types::ExceptionDetails>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetIsolateIdReturns {
    #[doc = "The isolate id."]
    #[serde(rename = "id")]
    pub id: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHeapUsageReturns {
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPropertiesReturns {
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalLexicalScopeNamesReturns {
    #[serde(rename = "names")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub names: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryObjectsReturns {
    #[doc = "Array with objects."]
    #[serde(rename = "objects")]
    pub objects: super::types::RemoteObject,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunScriptReturns {
    #[doc = "Run result."]
    #[serde(rename = "result")]
    pub result: super::types::RemoteObject,
    #[doc = "Exception details."]
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<super::types::ExceptionDetails>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetExceptionDetailsReturns {
    #[serde(rename = "exceptionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub exception_details: Option<super::types::ExceptionDetails>,
}
