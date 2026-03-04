use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomStorageItemAdded {
    #[serde(rename = "storageId")]
    pub storage_id: super::types::StorageId,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "newValue")]
    pub new_value: String,
}
impl DomStorageItemAdded {
    pub const IDENTIFIER: &'static str = "DOMStorage.domStorageItemAdded";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomStorageItemRemoved {
    #[serde(rename = "storageId")]
    pub storage_id: super::types::StorageId,
    #[serde(rename = "key")]
    pub key: String,
}
impl DomStorageItemRemoved {
    pub const IDENTIFIER: &'static str = "DOMStorage.domStorageItemRemoved";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomStorageItemUpdated {
    #[serde(rename = "storageId")]
    pub storage_id: super::types::StorageId,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "oldValue")]
    pub old_value: String,
    #[serde(rename = "newValue")]
    pub new_value: String,
}
impl DomStorageItemUpdated {
    pub const IDENTIFIER: &'static str = "DOMStorage.domStorageItemUpdated";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomStorageItemsCleared {
    #[serde(rename = "storageId")]
    pub storage_id: super::types::StorageId,
}
impl DomStorageItemsCleared {
    pub const IDENTIFIER: &'static str = "DOMStorage.domStorageItemsCleared";
}
group_enum ! (Event { DomStorageItemAdded (DomStorageItemAdded) , DomStorageItemRemoved (DomStorageItemRemoved) , DomStorageItemUpdated (DomStorageItemUpdated) , DomStorageItemsCleared (DomStorageItemsCleared) });
