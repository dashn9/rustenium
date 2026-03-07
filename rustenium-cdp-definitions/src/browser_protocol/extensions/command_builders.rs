use super::commands::*;
impl TriggerAction {
    pub fn builder() -> TriggerActionBuilder {
        <TriggerActionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct TriggerActionBuilder {
    id: Option<String>,
    target_id: Option<String>,
}
impl TriggerActionBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn target_id(mut self, target_id: impl Into<String>) -> Self {
        self.target_id = Some(target_id.into());
        self
    }
    pub fn build(self) -> Result<TriggerAction, String> {
        Ok(TriggerAction {
            method: TriggerActionMethod::TriggerAction,
            params: TriggerActionParams {
                id: self
                    .id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
                target_id: self.target_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(target_id))
                })?,
            },
        })
    }
}
impl LoadUnpacked {
    pub fn builder() -> LoadUnpackedBuilder {
        <LoadUnpackedBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct LoadUnpackedBuilder {
    path: Option<String>,
    enable_in_incognito: Option<bool>,
}
impl LoadUnpackedBuilder {
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
    pub fn enable_in_incognito(mut self, enable_in_incognito: impl Into<bool>) -> Self {
        self.enable_in_incognito = Some(enable_in_incognito.into());
        self
    }
    pub fn build(self) -> Result<LoadUnpacked, String> {
        Ok(LoadUnpacked {
            method: LoadUnpackedMethod::LoadUnpacked,
            params: LoadUnpackedParams {
                path: self
                    .path
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(path)))?,
                enable_in_incognito: self.enable_in_incognito,
            },
        })
    }
}
impl Uninstall {
    pub fn builder() -> UninstallBuilder {
        <UninstallBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UninstallBuilder {
    id: Option<String>,
}
impl UninstallBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn build(self) -> Result<Uninstall, String> {
        Ok(Uninstall {
            method: UninstallMethod::Uninstall,
            params: UninstallParams {
                id: self
                    .id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            },
        })
    }
}
impl GetStorageItems {
    pub fn builder() -> GetStorageItemsBuilder {
        <GetStorageItemsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetStorageItemsBuilder {
    id: Option<String>,
    storage_area: Option<super::types::StorageArea>,
    keys: Option<Vec<String>>,
}
impl GetStorageItemsBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn storage_area(mut self, storage_area: impl Into<super::types::StorageArea>) -> Self {
        self.storage_area = Some(storage_area.into());
        self
    }
    pub fn key(mut self, key: impl Into<String>) -> Self {
        let v = self.keys.get_or_insert(Vec::new());
        v.push(key.into());
        self
    }
    pub fn keys<I, S>(mut self, keys: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.keys.get_or_insert(Vec::new());
        for val in keys {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<GetStorageItems, String> {
        Ok(GetStorageItems {
            method: GetStorageItemsMethod::GetStorageItems,
            params: GetStorageItemsParams {
                id: self
                    .id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
                storage_area: self.storage_area.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_area))
                })?,
                keys: self.keys,
            },
        })
    }
}
impl RemoveStorageItems {
    pub fn builder() -> RemoveStorageItemsBuilder {
        <RemoveStorageItemsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveStorageItemsBuilder {
    id: Option<String>,
    storage_area: Option<super::types::StorageArea>,
    keys: Option<Vec<String>>,
}
impl RemoveStorageItemsBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn storage_area(mut self, storage_area: impl Into<super::types::StorageArea>) -> Self {
        self.storage_area = Some(storage_area.into());
        self
    }
    pub fn key(mut self, key: impl Into<String>) -> Self {
        let v = self.keys.get_or_insert(Vec::new());
        v.push(key.into());
        self
    }
    pub fn keys<I, S>(mut self, keys: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.keys.get_or_insert(Vec::new());
        for val in keys {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<RemoveStorageItems, String> {
        Ok(RemoveStorageItems {
            method: RemoveStorageItemsMethod::RemoveStorageItems,
            params: RemoveStorageItemsParams {
                id: self
                    .id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
                storage_area: self.storage_area.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_area))
                })?,
                keys: self
                    .keys
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(keys)))?,
            },
        })
    }
}
impl ClearStorageItems {
    pub fn builder() -> ClearStorageItemsBuilder {
        <ClearStorageItemsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ClearStorageItemsBuilder {
    id: Option<String>,
    storage_area: Option<super::types::StorageArea>,
}
impl ClearStorageItemsBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn storage_area(mut self, storage_area: impl Into<super::types::StorageArea>) -> Self {
        self.storage_area = Some(storage_area.into());
        self
    }
    pub fn build(self) -> Result<ClearStorageItems, String> {
        Ok(ClearStorageItems {
            method: ClearStorageItemsMethod::ClearStorageItems,
            params: ClearStorageItemsParams {
                id: self
                    .id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
                storage_area: self.storage_area.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_area))
                })?,
            },
        })
    }
}
impl SetStorageItems {
    pub fn builder() -> SetStorageItemsBuilder {
        <SetStorageItemsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetStorageItemsBuilder {
    id: Option<String>,
    storage_area: Option<super::types::StorageArea>,
    values: Option<serde_json::Value>,
}
impl SetStorageItemsBuilder {
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn storage_area(mut self, storage_area: impl Into<super::types::StorageArea>) -> Self {
        self.storage_area = Some(storage_area.into());
        self
    }
    pub fn values(mut self, values: impl Into<serde_json::Value>) -> Self {
        self.values = Some(values.into());
        self
    }
    pub fn build(self) -> Result<SetStorageItems, String> {
        Ok(SetStorageItems {
            method: SetStorageItemsMethod::SetStorageItems,
            params: SetStorageItemsParams {
                id: self
                    .id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
                storage_area: self.storage_area.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(storage_area))
                })?,
                values: self
                    .values
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(values)))?,
            },
        })
    }
}
