use super::commands::*;
impl Clear {
    pub fn builder() -> ClearBuilder {
        ClearBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ClearBuilder {
    storage_id: Option<super::types::StorageId>,
}
impl ClearBuilder {
    pub fn storage_id(mut self, storage_id: impl Into<super::types::StorageId>) -> Self {
        self.storage_id = Some(storage_id.into());
        self
    }
    pub fn build(self) -> Result<Clear, String> {
        Ok(Clear {
            method: ClearMethod::Clear,
            params: ClearParams {
                storage_id: self.storage_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_id))
                })?,
            },
        })
    }
}
impl GetDomStorageItems {
    pub fn builder() -> GetDomStorageItemsBuilder {
        GetDomStorageItemsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetDomStorageItemsBuilder {
    storage_id: Option<super::types::StorageId>,
}
impl GetDomStorageItemsBuilder {
    pub fn storage_id(mut self, storage_id: impl Into<super::types::StorageId>) -> Self {
        self.storage_id = Some(storage_id.into());
        self
    }
    pub fn build(self) -> Result<GetDomStorageItems, String> {
        Ok(GetDomStorageItems {
            method: GetDomStorageItemsMethod::GetDomStorageItems,
            params: GetDomStorageItemsParams {
                storage_id: self.storage_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_id))
                })?,
            },
        })
    }
}
impl RemoveDomStorageItem {
    pub fn builder() -> RemoveDomStorageItemBuilder {
        RemoveDomStorageItemBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveDomStorageItemBuilder {
    storage_id: Option<super::types::StorageId>,
    key: Option<String>,
}
impl RemoveDomStorageItemBuilder {
    pub fn storage_id(mut self, storage_id: impl Into<super::types::StorageId>) -> Self {
        self.storage_id = Some(storage_id.into());
        self
    }
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn build(self) -> Result<RemoveDomStorageItem, String> {
        Ok(RemoveDomStorageItem {
            method: RemoveDomStorageItemMethod::RemoveDomStorageItem,
            params: RemoveDomStorageItemParams {
                storage_id: self.storage_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_id))
                })?,
                key: self
                    .key
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key)))?,
            },
        })
    }
}
impl SetDomStorageItem {
    pub fn builder() -> SetDomStorageItemBuilder {
        SetDomStorageItemBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDomStorageItemBuilder {
    storage_id: Option<super::types::StorageId>,
    key: Option<String>,
    value: Option<String>,
}
impl SetDomStorageItemBuilder {
    pub fn storage_id(mut self, storage_id: impl Into<super::types::StorageId>) -> Self {
        self.storage_id = Some(storage_id.into());
        self
    }
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<SetDomStorageItem, String> {
        Ok(SetDomStorageItem {
            method: SetDomStorageItemMethod::SetDomStorageItem,
            params: SetDomStorageItemParams {
                storage_id: self.storage_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_id))
                })?,
                key: self
                    .key
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(key)))?,
                value: self
                    .value
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            },
        })
    }
}
