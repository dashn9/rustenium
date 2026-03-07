use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TriggerActionResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadUnpackedResult {
    #[doc = "Extension id."]
    #[serde(rename = "id")]
    pub id: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetExtensionsResult {
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<super::types::ExtensionInfo>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UninstallResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStorageItemsResult {
    #[serde(rename = "data")]
    pub data: serde_json::Value,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveStorageItemsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearStorageItemsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetStorageItemsResult {}
