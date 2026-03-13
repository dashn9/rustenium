use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EvaluateResult {
    EvaluateResultSuccess(super::types::EvaluateResultSuccess),
    EvaluateResultException(super::types::EvaluateResultException),
}
impl From<super::types::EvaluateResultSuccess> for EvaluateResult {
    fn from(val: super::types::EvaluateResultSuccess) -> Self {
        EvaluateResult::EvaluateResultSuccess(val)
    }
}
impl TryFrom<EvaluateResult> for super::types::EvaluateResultSuccess {
    type Error = EvaluateResult;
    fn try_from(e: EvaluateResult) -> Result<Self, Self::Error> {
        match e {
            EvaluateResult::EvaluateResultSuccess(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<super::types::EvaluateResultException> for EvaluateResult {
    fn from(val: super::types::EvaluateResultException) -> Self {
        EvaluateResult::EvaluateResultException(val)
    }
}
impl TryFrom<EvaluateResult> for super::types::EvaluateResultException {
    type Error = EvaluateResult;
    fn try_from(e: EvaluateResult) -> Result<Self, Self::Error> {
        match e {
            EvaluateResult::EvaluateResultException(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl TryFrom<serde_json::Value> for EvaluateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddPreloadScriptResult {
    #[serde(rename = "script")]
    pub script: super::types::PreloadScript,
}
impl TryFrom<serde_json::Value> for AddPreloadScriptResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisownResult {
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl TryFrom<serde_json::Value> for DisownResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
pub type CallFunctionResult = EvaluateResult;
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRealmsResult {
    #[serde(rename = "realms")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub realms: Vec<super::types::RealmInfo>,
}
impl TryFrom<serde_json::Value> for GetRealmsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemovePreloadScriptResult {
    #[serde(flatten)]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
impl TryFrom<serde_json::Value> for RemovePreloadScriptResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
