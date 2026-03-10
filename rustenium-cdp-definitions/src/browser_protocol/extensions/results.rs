use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TriggerActionResult {}
impl TryFrom<serde_json::Value> for TriggerActionResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadUnpackedResult {
    #[doc = "Extension id."]
    #[serde(rename = "id")]
    pub id: String,
}
impl TryFrom<serde_json::Value> for LoadUnpackedResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetExtensionsResult {
    #[serde(rename = "extensions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extensions: Vec<super::types::ExtensionInfo>,
}
impl TryFrom<serde_json::Value> for GetExtensionsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UninstallResult {}
impl TryFrom<serde_json::Value> for UninstallResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStorageItemsResult {
    #[serde(rename = "data")]
    pub data: serde_json::Value,
}
impl TryFrom<serde_json::Value> for GetStorageItemsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveStorageItemsResult {}
impl TryFrom<serde_json::Value> for RemoveStorageItemsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearStorageItemsResult {}
impl TryFrom<serde_json::Value> for ClearStorageItemsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetStorageItemsResult {}
impl TryFrom<serde_json::Value> for SetStorageItemsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
