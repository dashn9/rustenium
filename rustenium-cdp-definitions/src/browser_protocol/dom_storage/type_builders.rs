use super::types::*;
impl StorageId {
    pub fn builder() -> StorageIdBuilder {
        StorageIdBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct StorageIdBuilder {
    security_origin: Option<String>,
    storage_key: Option<SerializedStorageKey>,
    is_local_storage: Option<bool>,
}
impl StorageIdBuilder {
    pub fn security_origin(mut self, security_origin: impl Into<String>) -> Self {
        self.security_origin = Some(security_origin.into());
        self
    }
    pub fn storage_key(mut self, storage_key: impl Into<SerializedStorageKey>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn is_local_storage(mut self, is_local_storage: impl Into<bool>) -> Self {
        self.is_local_storage = Some(is_local_storage.into());
        self
    }
    pub fn build(self) -> Result<StorageId, String> {
        Ok(StorageId {
            security_origin: self.security_origin,
            storage_key: self.storage_key,
            is_local_storage: self.is_local_storage.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(is_local_storage)
                )
            })?,
        })
    }
}
