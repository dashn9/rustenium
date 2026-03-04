use serde::{Deserialize, Serialize};
#[doc = "Clears all entries from an object store.\n[clearObjectStore](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-clearObjectStore)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearObjectStoreParams {
    #[doc = "At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.\nSecurity origin."]
    #[serde(rename = "securityOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub security_origin: Option<String>,
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_key: Option<String>,
    #[doc = "Storage bucket. If not specified, it uses the default bucket."]
    #[serde(rename = "storageBucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_bucket: Option<super::super::storage::types::StorageBucket>,
    #[doc = "Database name."]
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[doc = "Object store name."]
    #[serde(rename = "objectStoreName")]
    pub object_store_name: String,
}
impl ClearObjectStoreParams {
    pub fn new(database_name: impl Into<String>, object_store_name: impl Into<String>) -> Self {
        Self {
            database_name: database_name.into(),
            object_store_name: object_store_name.into(),
            security_origin: None,
            storage_key: None,
            storage_bucket: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearObjectStoreMethod {
    #[serde(rename = "IndexedDB.clearObjectStore")]
    ClearObjectStore,
}
impl ClearObjectStoreMethod {
    pub const IDENTIFIER: &'static str = "IndexedDB.clearObjectStore";
}
#[doc = "Clears all entries from an object store.\n[clearObjectStore](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-clearObjectStore)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearObjectStore {
    pub method: ClearObjectStoreMethod,
    pub params: ClearObjectStoreParams,
}
#[doc = "Deletes a database.\n[deleteDatabase](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-deleteDatabase)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteDatabaseParams {
    #[doc = "At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.\nSecurity origin."]
    #[serde(rename = "securityOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub security_origin: Option<String>,
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_key: Option<String>,
    #[doc = "Storage bucket. If not specified, it uses the default bucket."]
    #[serde(rename = "storageBucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_bucket: Option<super::super::storage::types::StorageBucket>,
    #[doc = "Database name."]
    #[serde(rename = "databaseName")]
    pub database_name: String,
}
impl DeleteDatabaseParams {
    pub fn new(database_name: impl Into<String>) -> Self {
        Self {
            database_name: database_name.into(),
            security_origin: None,
            storage_key: None,
            storage_bucket: None,
        }
    }
}
impl<T: Into<String>> From<T> for DeleteDatabaseParams {
    fn from(url: T) -> Self {
        DeleteDatabaseParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteDatabaseMethod {
    #[serde(rename = "IndexedDB.deleteDatabase")]
    DeleteDatabase,
}
impl DeleteDatabaseMethod {
    pub const IDENTIFIER: &'static str = "IndexedDB.deleteDatabase";
}
#[doc = "Deletes a database.\n[deleteDatabase](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-deleteDatabase)"]
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteDatabase {
    pub method: DeleteDatabaseMethod,
    pub params: DeleteDatabaseParams,
}
#[doc = "Delete a range of entries from an object store\n[deleteObjectStoreEntries](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-deleteObjectStoreEntries)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteObjectStoreEntriesParams {
    #[doc = "At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.\nSecurity origin."]
    #[serde(rename = "securityOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub security_origin: Option<String>,
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_key: Option<String>,
    #[doc = "Storage bucket. If not specified, it uses the default bucket."]
    #[serde(rename = "storageBucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_bucket: Option<super::super::storage::types::StorageBucket>,
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[serde(rename = "objectStoreName")]
    pub object_store_name: String,
    #[doc = "Range of entry keys to delete"]
    #[serde(rename = "keyRange")]
    pub key_range: super::types::KeyRange,
}
impl DeleteObjectStoreEntriesParams {
    pub fn new(
        database_name: impl Into<String>,
        object_store_name: impl Into<String>,
        key_range: impl Into<super::types::KeyRange>,
    ) -> Self {
        Self {
            database_name: database_name.into(),
            object_store_name: object_store_name.into(),
            key_range: key_range.into(),
            security_origin: None,
            storage_key: None,
            storage_bucket: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteObjectStoreEntriesMethod {
    #[serde(rename = "IndexedDB.deleteObjectStoreEntries")]
    DeleteObjectStoreEntries,
}
impl DeleteObjectStoreEntriesMethod {
    pub const IDENTIFIER: &'static str = "IndexedDB.deleteObjectStoreEntries";
}
#[doc = "Delete a range of entries from an object store\n[deleteObjectStoreEntries](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-deleteObjectStoreEntries)"]
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteObjectStoreEntries {
    pub method: DeleteObjectStoreEntriesMethod,
    pub params: DeleteObjectStoreEntriesParams,
}
#[doc = "Disables events from backend.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "IndexedDB.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "IndexedDB.disable";
}
#[doc = "Disables events from backend.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
#[doc = "Enables events from backend.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "IndexedDB.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "IndexedDB.enable";
}
#[doc = "Enables events from backend.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
#[doc = "Requests data from object store or index.\n[requestData](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-requestData)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestDataParams {
    #[doc = "At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.\nSecurity origin."]
    #[serde(rename = "securityOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub security_origin: Option<String>,
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_key: Option<String>,
    #[doc = "Storage bucket. If not specified, it uses the default bucket."]
    #[serde(rename = "storageBucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_bucket: Option<super::super::storage::types::StorageBucket>,
    #[doc = "Database name."]
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[doc = "Object store name."]
    #[serde(rename = "objectStoreName")]
    pub object_store_name: String,
    #[doc = "Index name. If not specified, it performs an object store data request."]
    #[serde(rename = "indexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub index_name: Option<String>,
    #[doc = "Number of records to skip."]
    #[serde(rename = "skipCount")]
    pub skip_count: i64,
    #[doc = "Number of records to fetch."]
    #[serde(rename = "pageSize")]
    pub page_size: i64,
    #[doc = "Key range."]
    #[serde(rename = "keyRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub key_range: Option<super::types::KeyRange>,
}
impl RequestDataParams {
    pub fn new(
        database_name: impl Into<String>,
        object_store_name: impl Into<String>,
        skip_count: impl Into<i64>,
        page_size: impl Into<i64>,
    ) -> Self {
        Self {
            database_name: database_name.into(),
            object_store_name: object_store_name.into(),
            skip_count: skip_count.into(),
            page_size: page_size.into(),
            security_origin: None,
            storage_key: None,
            storage_bucket: None,
            index_name: None,
            key_range: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RequestDataMethod {
    #[serde(rename = "IndexedDB.requestData")]
    RequestData,
}
impl RequestDataMethod {
    pub const IDENTIFIER: &'static str = "IndexedDB.requestData";
}
#[doc = "Requests data from object store or index.\n[requestData](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-requestData)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestData {
    pub method: RequestDataMethod,
    pub params: RequestDataParams,
}
#[doc = "Gets metadata of an object store.\n[getMetadata](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-getMetadata)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetMetadataParams {
    #[doc = "At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.\nSecurity origin."]
    #[serde(rename = "securityOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub security_origin: Option<String>,
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_key: Option<String>,
    #[doc = "Storage bucket. If not specified, it uses the default bucket."]
    #[serde(rename = "storageBucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_bucket: Option<super::super::storage::types::StorageBucket>,
    #[doc = "Database name."]
    #[serde(rename = "databaseName")]
    pub database_name: String,
    #[doc = "Object store name."]
    #[serde(rename = "objectStoreName")]
    pub object_store_name: String,
}
impl GetMetadataParams {
    pub fn new(database_name: impl Into<String>, object_store_name: impl Into<String>) -> Self {
        Self {
            database_name: database_name.into(),
            object_store_name: object_store_name.into(),
            security_origin: None,
            storage_key: None,
            storage_bucket: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetMetadataMethod {
    #[serde(rename = "IndexedDB.getMetadata")]
    GetMetadata,
}
impl GetMetadataMethod {
    pub const IDENTIFIER: &'static str = "IndexedDB.getMetadata";
}
#[doc = "Gets metadata of an object store.\n[getMetadata](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-getMetadata)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetMetadata {
    pub method: GetMetadataMethod,
    pub params: GetMetadataParams,
}
#[doc = "Requests database with given name in given frame.\n[requestDatabase](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-requestDatabase)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestDatabaseParams {
    #[doc = "At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.\nSecurity origin."]
    #[serde(rename = "securityOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub security_origin: Option<String>,
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_key: Option<String>,
    #[doc = "Storage bucket. If not specified, it uses the default bucket."]
    #[serde(rename = "storageBucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_bucket: Option<super::super::storage::types::StorageBucket>,
    #[doc = "Database name."]
    #[serde(rename = "databaseName")]
    pub database_name: String,
}
impl RequestDatabaseParams {
    pub fn new(database_name: impl Into<String>) -> Self {
        Self {
            database_name: database_name.into(),
            security_origin: None,
            storage_key: None,
            storage_bucket: None,
        }
    }
}
impl<T: Into<String>> From<T> for RequestDatabaseParams {
    fn from(url: T) -> Self {
        RequestDatabaseParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RequestDatabaseMethod {
    #[serde(rename = "IndexedDB.requestDatabase")]
    RequestDatabase,
}
impl RequestDatabaseMethod {
    pub const IDENTIFIER: &'static str = "IndexedDB.requestDatabase";
}
#[doc = "Requests database with given name in given frame.\n[requestDatabase](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-requestDatabase)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestDatabase {
    pub method: RequestDatabaseMethod,
    pub params: RequestDatabaseParams,
}
#[doc = "Requests database names for given security origin.\n[requestDatabaseNames](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-requestDatabaseNames)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestDatabaseNamesParams {
    #[doc = "At least and at most one of securityOrigin, storageKey, or storageBucket must be specified.\nSecurity origin."]
    #[serde(rename = "securityOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub security_origin: Option<String>,
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_key: Option<String>,
    #[doc = "Storage bucket. If not specified, it uses the default bucket."]
    #[serde(rename = "storageBucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_bucket: Option<super::super::storage::types::StorageBucket>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RequestDatabaseNamesMethod {
    #[serde(rename = "IndexedDB.requestDatabaseNames")]
    RequestDatabaseNames,
}
impl RequestDatabaseNamesMethod {
    pub const IDENTIFIER: &'static str = "IndexedDB.requestDatabaseNames";
}
#[doc = "Requests database names for given security origin.\n[requestDatabaseNames](https://chromedevtools.github.io/devtools-protocol/tot/IndexedDB/#method-requestDatabaseNames)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestDatabaseNames {
    pub method: RequestDatabaseNamesMethod,
    pub params: RequestDatabaseNamesParams,
}
group_enum ! (Command { ClearObjectStore (ClearObjectStore) , DeleteDatabase (DeleteDatabase) , DeleteObjectStoreEntries (DeleteObjectStoreEntries) , Disable (Disable) , Enable (Enable) , RequestData (RequestData) , GetMetadata (GetMetadata) , RequestDatabase (RequestDatabase) , RequestDatabaseNames (RequestDatabaseNames) });
