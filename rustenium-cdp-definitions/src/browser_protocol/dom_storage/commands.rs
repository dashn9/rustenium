use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearParams {
    #[serde(rename = "storageId")]
    pub storage_id: super::types::StorageId,
}
impl ClearParams {
    pub fn new(storage_id: impl Into<super::types::StorageId>) -> Self {
        Self {
            storage_id: storage_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearMethod {
    #[serde(rename = "DOMStorage.clear")]
    Clear,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Clear {
    pub method: ClearMethod,
    pub params: ClearParams,
}
impl Clear {
    pub const IDENTIFIER: &'static str = "DOMStorage.clear";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Clear {
    type Result = super::results::ClearResult;
}
#[doc = "Disables storage tracking, prevents storage events from being sent to the client.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/DOMStorage/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "DOMStorage.disable")]
    Disable,
}
#[doc = "Disables storage tracking, prevents storage events from being sent to the client.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/DOMStorage/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "DOMStorage.disable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables storage tracking, storage events will now be delivered to the client.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/DOMStorage/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "DOMStorage.enable")]
    Enable,
}
#[doc = "Enables storage tracking, storage events will now be delivered to the client.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/DOMStorage/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "DOMStorage.enable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomStorageItemsParams {
    #[serde(rename = "storageId")]
    pub storage_id: super::types::StorageId,
}
impl GetDomStorageItemsParams {
    pub fn new(storage_id: impl Into<super::types::StorageId>) -> Self {
        Self {
            storage_id: storage_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetDomStorageItemsMethod {
    #[serde(rename = "DOMStorage.getDOMStorageItems")]
    GetDomStorageItems,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetDomStorageItems {
    pub method: GetDomStorageItemsMethod,
    pub params: GetDomStorageItemsParams,
}
impl GetDomStorageItems {
    pub const IDENTIFIER: &'static str = "DOMStorage.getDOMStorageItems";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetDomStorageItems {
    type Result = super::results::GetDomStorageItemsResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveDomStorageItemParams {
    #[serde(rename = "storageId")]
    pub storage_id: super::types::StorageId,
    #[serde(rename = "key")]
    pub key: String,
}
impl RemoveDomStorageItemParams {
    pub fn new(storage_id: impl Into<super::types::StorageId>, key: impl Into<String>) -> Self {
        Self {
            storage_id: storage_id.into(),
            key: key.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveDomStorageItemMethod {
    #[serde(rename = "DOMStorage.removeDOMStorageItem")]
    RemoveDomStorageItem,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveDomStorageItem {
    pub method: RemoveDomStorageItemMethod,
    pub params: RemoveDomStorageItemParams,
}
impl RemoveDomStorageItem {
    pub const IDENTIFIER: &'static str = "DOMStorage.removeDOMStorageItem";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for RemoveDomStorageItem {
    type Result = super::results::RemoveDomStorageItemResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDomStorageItemParams {
    #[serde(rename = "storageId")]
    pub storage_id: super::types::StorageId,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl SetDomStorageItemParams {
    pub fn new(
        storage_id: impl Into<super::types::StorageId>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            storage_id: storage_id.into(),
            key: key.into(),
            value: value.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDomStorageItemMethod {
    #[serde(rename = "DOMStorage.setDOMStorageItem")]
    SetDomStorageItem,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDomStorageItem {
    pub method: SetDomStorageItemMethod,
    pub params: SetDomStorageItemParams,
}
impl SetDomStorageItem {
    pub const IDENTIFIER: &'static str = "DOMStorage.setDOMStorageItem";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetDomStorageItem {
    type Result = super::results::SetDomStorageItemResult;
}
group_enum ! (DomStorageCommands { Clear (Clear) , Disable (Disable) , Enable (Enable) , GetDomStorageItems (GetDomStorageItems) , RemoveDomStorageItem (RemoveDomStorageItem) , SetDomStorageItem (SetDomStorageItem) });
