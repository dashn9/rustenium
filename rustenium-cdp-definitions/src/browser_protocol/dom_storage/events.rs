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
impl DomStorageItemAddedMethod {
    pub const IDENTIFIER: &'static str = "DOMStorage.domStorageItemAdded";
}
#[derive(Debug, Clone, PartialEq)]
pub struct DomStorageItemAdded {
    pub method: DomStorageItemAddedMethod,
    pub params: DomStorageItemAddedParams,
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
impl DomStorageItemRemovedMethod {
    pub const IDENTIFIER: &'static str = "DOMStorage.domStorageItemRemoved";
}
#[derive(Debug, Clone, PartialEq)]
pub struct DomStorageItemRemoved {
    pub method: DomStorageItemRemovedMethod,
    pub params: DomStorageItemRemovedParams,
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
impl DomStorageItemUpdatedMethod {
    pub const IDENTIFIER: &'static str = "DOMStorage.domStorageItemUpdated";
}
#[derive(Debug, Clone, PartialEq)]
pub struct DomStorageItemUpdated {
    pub method: DomStorageItemUpdatedMethod,
    pub params: DomStorageItemUpdatedParams,
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
impl DomStorageItemsClearedMethod {
    pub const IDENTIFIER: &'static str = "DOMStorage.domStorageItemsCleared";
}
#[derive(Debug, Clone, PartialEq)]
pub struct DomStorageItemsCleared {
    pub method: DomStorageItemsClearedMethod,
    pub params: DomStorageItemsClearedParams,
}
group_enum ! (DomStorageEvents { DomStorageItemAdded (DomStorageItemAdded) , DomStorageItemRemoved (DomStorageItemRemoved) , DomStorageItemUpdated (DomStorageItemUpdated) , DomStorageItemsCleared (DomStorageItemsCleared) });
