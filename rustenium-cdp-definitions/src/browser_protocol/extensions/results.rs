use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadUnpackedReturns {
    #[doc = "Extension id."]
    #[serde(rename = "id")]
    pub id: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetExtensionsReturns {
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<super::types::ExtensionInfo>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStorageItemsReturns {
    #[serde(rename = "data")]
    pub data: serde_json::Value,
}
