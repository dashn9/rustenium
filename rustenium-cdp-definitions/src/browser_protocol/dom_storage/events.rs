use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomStorageItemAddedParams {
    #[serde(rename = "storageId")]
    pub storage_id: super::types::StorageId,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "newValue")]
    pub new_value: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomStorageItemAddedMethod {
    #[serde(rename = "DOMStorage.domStorageItemAdded")]
    DomStorageItemAdded,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomStorageItemAdded {
    pub method: DomStorageItemAddedMethod,
    pub params: DomStorageItemAddedParams,
}
impl DomStorageItemAdded {
    pub const IDENTIFIER: &'static str = "DOMStorage.domStorageItemAdded";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomStorageItemRemovedParams {
    #[serde(rename = "storageId")]
    pub storage_id: super::types::StorageId,
    #[serde(rename = "key")]
    pub key: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomStorageItemRemovedMethod {
    #[serde(rename = "DOMStorage.domStorageItemRemoved")]
    DomStorageItemRemoved,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomStorageItemRemoved {
    pub method: DomStorageItemRemovedMethod,
    pub params: DomStorageItemRemovedParams,
}
impl DomStorageItemRemoved {
    pub const IDENTIFIER: &'static str = "DOMStorage.domStorageItemRemoved";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomStorageItemUpdatedParams {
    #[serde(rename = "storageId")]
    pub storage_id: super::types::StorageId,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "oldValue")]
    pub old_value: String,
    #[serde(rename = "newValue")]
    pub new_value: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomStorageItemUpdatedMethod {
    #[serde(rename = "DOMStorage.domStorageItemUpdated")]
    DomStorageItemUpdated,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomStorageItemUpdated {
    pub method: DomStorageItemUpdatedMethod,
    pub params: DomStorageItemUpdatedParams,
}
impl DomStorageItemUpdated {
    pub const IDENTIFIER: &'static str = "DOMStorage.domStorageItemUpdated";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomStorageItemsClearedParams {
    #[serde(rename = "storageId")]
    pub storage_id: super::types::StorageId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomStorageItemsClearedMethod {
    #[serde(rename = "DOMStorage.domStorageItemsCleared")]
    DomStorageItemsCleared,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomStorageItemsCleared {
    pub method: DomStorageItemsClearedMethod,
    pub params: DomStorageItemsClearedParams,
}
impl DomStorageItemsCleared {
    pub const IDENTIFIER: &'static str = "DOMStorage.domStorageItemsCleared";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (DomStorageEvents { DomStorageItemAdded (DomStorageItemAdded) , DomStorageItemRemoved (DomStorageItemRemoved) , DomStorageItemUpdated (DomStorageItemUpdated) , DomStorageItemsCleared (DomStorageItemsCleared) });
