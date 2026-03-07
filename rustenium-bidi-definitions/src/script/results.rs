use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EvaluateResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddPreloadScriptResult {
    #[serde(rename = "script")]
    pub script: super::types::PreloadScript,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisownResult {
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CallFunctionResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRealmsResult {
    #[serde(rename = "realms")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub realms: Vec<super::types::RealmInfo>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemovePreloadScriptResult {}
