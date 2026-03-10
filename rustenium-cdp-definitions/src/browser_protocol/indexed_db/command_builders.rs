use super::commands::*;
impl ClearObjectStore {
    pub fn builder() -> ClearObjectStoreBuilder {
        <ClearObjectStoreBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ClearObjectStoreBuilder {
    security_origin: Option<String>,
    storage_key: Option<String>,
    storage_bucket: Option<crate::browser_protocol::storage::types::StorageBucket>,
    database_name: Option<String>,
    object_store_name: Option<String>,
}
impl ClearObjectStoreBuilder {
    pub fn security_origin(mut self, security_origin: impl Into<String>) -> Self {
        self.security_origin = Some(security_origin.into());
        self
    }
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn storage_bucket(
        mut self,
        storage_bucket: impl Into<crate::browser_protocol::storage::types::StorageBucket>,
    ) -> Self {
        self.storage_bucket = Some(storage_bucket.into());
        self
    }
    pub fn database_name(mut self, database_name: impl Into<String>) -> Self {
        self.database_name = Some(database_name.into());
        self
    }
    pub fn object_store_name(mut self, object_store_name: impl Into<String>) -> Self {
        self.object_store_name = Some(object_store_name.into());
        self
    }
    pub fn build(self) -> Result<ClearObjectStore, String> {
        Ok(ClearObjectStore {
            method: ClearObjectStoreMethod::ClearObjectStore,
            params: ClearObjectStoreParams {
                security_origin: self.security_origin,
                storage_key: self.storage_key,
                storage_bucket: self.storage_bucket,
                database_name: self.database_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(database_name))
                })?,
                object_store_name: self.object_store_name.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(object_store_name)
                    )
                })?,
            },
        })
    }
}
impl DeleteDatabase {
    pub fn builder() -> DeleteDatabaseBuilder {
        <DeleteDatabaseBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeleteDatabaseBuilder {
    security_origin: Option<String>,
    storage_key: Option<String>,
    storage_bucket: Option<crate::browser_protocol::storage::types::StorageBucket>,
    database_name: Option<String>,
}
impl DeleteDatabaseBuilder {
    pub fn security_origin(mut self, security_origin: impl Into<String>) -> Self {
        self.security_origin = Some(security_origin.into());
        self
    }
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn storage_bucket(
        mut self,
        storage_bucket: impl Into<crate::browser_protocol::storage::types::StorageBucket>,
    ) -> Self {
        self.storage_bucket = Some(storage_bucket.into());
        self
    }
    pub fn database_name(mut self, database_name: impl Into<String>) -> Self {
        self.database_name = Some(database_name.into());
        self
    }
    pub fn build(self) -> Result<DeleteDatabase, String> {
        Ok(DeleteDatabase {
            method: DeleteDatabaseMethod::DeleteDatabase,
            params: DeleteDatabaseParams {
                security_origin: self.security_origin,
                storage_key: self.storage_key,
                storage_bucket: self.storage_bucket,
                database_name: self.database_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(database_name))
                })?,
            },
        })
    }
}
impl DeleteObjectStoreEntries {
    pub fn builder() -> DeleteObjectStoreEntriesBuilder {
        <DeleteObjectStoreEntriesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DeleteObjectStoreEntriesBuilder {
    security_origin: Option<String>,
    storage_key: Option<String>,
    storage_bucket: Option<crate::browser_protocol::storage::types::StorageBucket>,
    database_name: Option<String>,
    object_store_name: Option<String>,
    key_range: Option<super::types::KeyRange>,
}
impl DeleteObjectStoreEntriesBuilder {
    pub fn security_origin(mut self, security_origin: impl Into<String>) -> Self {
        self.security_origin = Some(security_origin.into());
        self
    }
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn storage_bucket(
        mut self,
        storage_bucket: impl Into<crate::browser_protocol::storage::types::StorageBucket>,
    ) -> Self {
        self.storage_bucket = Some(storage_bucket.into());
        self
    }
    pub fn database_name(mut self, database_name: impl Into<String>) -> Self {
        self.database_name = Some(database_name.into());
        self
    }
    pub fn object_store_name(mut self, object_store_name: impl Into<String>) -> Self {
        self.object_store_name = Some(object_store_name.into());
        self
    }
    pub fn key_range(mut self, key_range: impl Into<super::types::KeyRange>) -> Self {
        self.key_range = Some(key_range.into());
        self
    }
    pub fn build(self) -> Result<DeleteObjectStoreEntries, String> {
        Ok(DeleteObjectStoreEntries {
            method: DeleteObjectStoreEntriesMethod::DeleteObjectStoreEntries,
            params: DeleteObjectStoreEntriesParams {
                security_origin: self.security_origin,
                storage_key: self.storage_key,
                storage_bucket: self.storage_bucket,
                database_name: self.database_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(database_name))
                })?,
                object_store_name: self.object_store_name.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(object_store_name)
                    )
                })?,
                key_range: self.key_range.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(key_range))
                })?,
            },
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct DisableBuilder;
impl DisableBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Disable {
        Disable {
            method: DisableMethod::Disable,
            params: DisableParams {},
        }
    }
}
impl Disable {
    pub fn builder() -> DisableBuilder {
        DisableBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct EnableBuilder;
impl EnableBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Enable {
        Enable {
            method: EnableMethod::Enable,
            params: EnableParams {},
        }
    }
}
impl Enable {
    pub fn builder() -> EnableBuilder {
        EnableBuilder
    }
}
impl RequestData {
    pub fn builder() -> RequestDataBuilder {
        <RequestDataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RequestDataBuilder {
    security_origin: Option<String>,
    storage_key: Option<String>,
    storage_bucket: Option<crate::browser_protocol::storage::types::StorageBucket>,
    database_name: Option<String>,
    object_store_name: Option<String>,
    index_name: Option<String>,
    skip_count: Option<i64>,
    page_size: Option<i64>,
    key_range: Option<super::types::KeyRange>,
}
impl RequestDataBuilder {
    pub fn security_origin(mut self, security_origin: impl Into<String>) -> Self {
        self.security_origin = Some(security_origin.into());
        self
    }
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn storage_bucket(
        mut self,
        storage_bucket: impl Into<crate::browser_protocol::storage::types::StorageBucket>,
    ) -> Self {
        self.storage_bucket = Some(storage_bucket.into());
        self
    }
    pub fn database_name(mut self, database_name: impl Into<String>) -> Self {
        self.database_name = Some(database_name.into());
        self
    }
    pub fn object_store_name(mut self, object_store_name: impl Into<String>) -> Self {
        self.object_store_name = Some(object_store_name.into());
        self
    }
    pub fn index_name(mut self, index_name: impl Into<String>) -> Self {
        self.index_name = Some(index_name.into());
        self
    }
    pub fn skip_count(mut self, skip_count: impl Into<i64>) -> Self {
        self.skip_count = Some(skip_count.into());
        self
    }
    pub fn page_size(mut self, page_size: impl Into<i64>) -> Self {
        self.page_size = Some(page_size.into());
        self
    }
    pub fn key_range(mut self, key_range: impl Into<super::types::KeyRange>) -> Self {
        self.key_range = Some(key_range.into());
        self
    }
    pub fn build(self) -> Result<RequestData, String> {
        Ok(RequestData {
            method: RequestDataMethod::RequestData,
            params: RequestDataParams {
                security_origin: self.security_origin,
                storage_key: self.storage_key,
                storage_bucket: self.storage_bucket,
                database_name: self.database_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(database_name))
                })?,
                object_store_name: self.object_store_name.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(object_store_name)
                    )
                })?,
                index_name: self.index_name,
                skip_count: self.skip_count.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(skip_count))
                })?,
                page_size: self.page_size.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(page_size))
                })?,
                key_range: self.key_range,
            },
        })
    }
}
impl GetMetadata {
    pub fn builder() -> GetMetadataBuilder {
        <GetMetadataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetMetadataBuilder {
    security_origin: Option<String>,
    storage_key: Option<String>,
    storage_bucket: Option<crate::browser_protocol::storage::types::StorageBucket>,
    database_name: Option<String>,
    object_store_name: Option<String>,
}
impl GetMetadataBuilder {
    pub fn security_origin(mut self, security_origin: impl Into<String>) -> Self {
        self.security_origin = Some(security_origin.into());
        self
    }
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn storage_bucket(
        mut self,
        storage_bucket: impl Into<crate::browser_protocol::storage::types::StorageBucket>,
    ) -> Self {
        self.storage_bucket = Some(storage_bucket.into());
        self
    }
    pub fn database_name(mut self, database_name: impl Into<String>) -> Self {
        self.database_name = Some(database_name.into());
        self
    }
    pub fn object_store_name(mut self, object_store_name: impl Into<String>) -> Self {
        self.object_store_name = Some(object_store_name.into());
        self
    }
    pub fn build(self) -> Result<GetMetadata, String> {
        Ok(GetMetadata {
            method: GetMetadataMethod::GetMetadata,
            params: GetMetadataParams {
                security_origin: self.security_origin,
                storage_key: self.storage_key,
                storage_bucket: self.storage_bucket,
                database_name: self.database_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(database_name))
                })?,
                object_store_name: self.object_store_name.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(object_store_name)
                    )
                })?,
            },
        })
    }
}
impl RequestDatabase {
    pub fn builder() -> RequestDatabaseBuilder {
        <RequestDatabaseBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RequestDatabaseBuilder {
    security_origin: Option<String>,
    storage_key: Option<String>,
    storage_bucket: Option<crate::browser_protocol::storage::types::StorageBucket>,
    database_name: Option<String>,
}
impl RequestDatabaseBuilder {
    pub fn security_origin(mut self, security_origin: impl Into<String>) -> Self {
        self.security_origin = Some(security_origin.into());
        self
    }
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn storage_bucket(
        mut self,
        storage_bucket: impl Into<crate::browser_protocol::storage::types::StorageBucket>,
    ) -> Self {
        self.storage_bucket = Some(storage_bucket.into());
        self
    }
    pub fn database_name(mut self, database_name: impl Into<String>) -> Self {
        self.database_name = Some(database_name.into());
        self
    }
    pub fn build(self) -> Result<RequestDatabase, String> {
        Ok(RequestDatabase {
            method: RequestDatabaseMethod::RequestDatabase,
            params: RequestDatabaseParams {
                security_origin: self.security_origin,
                storage_key: self.storage_key,
                storage_bucket: self.storage_bucket,
                database_name: self.database_name.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(database_name))
                })?,
            },
        })
    }
}
impl RequestDatabaseNames {
    pub fn builder() -> RequestDatabaseNamesBuilder {
        <RequestDatabaseNamesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct RequestDatabaseNamesBuilder {
    security_origin: Option<String>,
    storage_key: Option<String>,
    storage_bucket: Option<crate::browser_protocol::storage::types::StorageBucket>,
}
impl RequestDatabaseNamesBuilder {
    pub fn security_origin(mut self, security_origin: impl Into<String>) -> Self {
        self.security_origin = Some(security_origin.into());
        self
    }
    pub fn storage_key(mut self, storage_key: impl Into<String>) -> Self {
        self.storage_key = Some(storage_key.into());
        self
    }
    pub fn storage_bucket(
        mut self,
        storage_bucket: impl Into<crate::browser_protocol::storage::types::StorageBucket>,
    ) -> Self {
        self.storage_bucket = Some(storage_bucket.into());
        self
    }
    pub fn build(self) -> RequestDatabaseNames {
        RequestDatabaseNames {
            method: RequestDatabaseNamesMethod::RequestDatabaseNames,
            params: RequestDatabaseNamesParams {
                security_origin: self.security_origin,
                storage_key: self.storage_key,
                storage_bucket: self.storage_bucket,
            },
        }
    }
}
