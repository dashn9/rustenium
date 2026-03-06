use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallResult {
    #[serde(rename = "extension")]
    pub extension: super::types::Extension,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UninstallResult {
    #[serde(flatten)]
    #[serde(skip_serializing_if = "std::collections::HashMap::is_empty")]
    #[serde(default)]
    pub extensible: std::collections::HashMap<String, serde_json::Value>,
}
