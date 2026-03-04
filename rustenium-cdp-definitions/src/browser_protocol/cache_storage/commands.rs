use serde::{Deserialize, Serialize};
#[doc = "Deletes a cache.\n[deleteCache](https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#method-deleteCache)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteCacheParams {
    #[doc = "Id of cache for deletion."]
    #[serde(rename = "cacheId")]
    pub cache_id: super::types::CacheId,
}
impl DeleteCacheParams {
    pub fn new(cache_id: impl Into<super::types::CacheId>) -> Self {
        Self {
            cache_id: cache_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteCacheMethod {
    #[serde(rename = "CacheStorage.deleteCache")]
    DeleteCache,
}
impl DeleteCacheMethod {
    pub const IDENTIFIER: &'static str = "CacheStorage.deleteCache";
}
#[doc = "Deletes a cache.\n[deleteCache](https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#method-deleteCache)"]
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteCache {
    pub method: DeleteCacheMethod,
    pub params: DeleteCacheParams,
}
#[doc = "Deletes a cache entry.\n[deleteEntry](https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#method-deleteEntry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteEntryParams {
    #[doc = "Id of cache where the entry will be deleted."]
    #[serde(rename = "cacheId")]
    pub cache_id: super::types::CacheId,
    #[doc = "URL spec of the request."]
    #[serde(rename = "request")]
    pub request: String,
}
impl DeleteEntryParams {
    pub fn new(cache_id: impl Into<super::types::CacheId>, request: impl Into<String>) -> Self {
        Self {
            cache_id: cache_id.into(),
            request: request.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteEntryMethod {
    #[serde(rename = "CacheStorage.deleteEntry")]
    DeleteEntry,
}
impl DeleteEntryMethod {
    pub const IDENTIFIER: &'static str = "CacheStorage.deleteEntry";
}
#[doc = "Deletes a cache entry.\n[deleteEntry](https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#method-deleteEntry)"]
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteEntry {
    pub method: DeleteEntryMethod,
    pub params: DeleteEntryParams,
}
#[doc = "Requests cache names.\n[requestCacheNames](https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#method-requestCacheNames)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestCacheNamesParams {
    #[doc = "At least and at most one of securityOrigin, storageKey, storageBucket must be specified.\nSecurity origin."]
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
pub enum RequestCacheNamesMethod {
    #[serde(rename = "CacheStorage.requestCacheNames")]
    RequestCacheNames,
}
impl RequestCacheNamesMethod {
    pub const IDENTIFIER: &'static str = "CacheStorage.requestCacheNames";
}
#[doc = "Requests cache names.\n[requestCacheNames](https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#method-requestCacheNames)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestCacheNames {
    pub method: RequestCacheNamesMethod,
    pub params: RequestCacheNamesParams,
}
#[doc = "Fetches cache entry.\n[requestCachedResponse](https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#method-requestCachedResponse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestCachedResponseParams {
    #[doc = "Id of cache that contains the entry."]
    #[serde(rename = "cacheId")]
    pub cache_id: super::types::CacheId,
    #[doc = "URL spec of the request."]
    #[serde(rename = "requestURL")]
    pub request_url: String,
    #[doc = "headers of the request."]
    #[serde(rename = "requestHeaders")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub request_headers: Vec<super::types::Header>,
}
impl RequestCachedResponseParams {
    pub fn new(
        cache_id: impl Into<super::types::CacheId>,
        request_url: impl Into<String>,
        request_headers: Vec<super::types::Header>,
    ) -> Self {
        Self {
            cache_id: cache_id.into(),
            request_url: request_url.into(),
            request_headers,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RequestCachedResponseMethod {
    #[serde(rename = "CacheStorage.requestCachedResponse")]
    RequestCachedResponse,
}
impl RequestCachedResponseMethod {
    pub const IDENTIFIER: &'static str = "CacheStorage.requestCachedResponse";
}
#[doc = "Fetches cache entry.\n[requestCachedResponse](https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#method-requestCachedResponse)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestCachedResponse {
    pub method: RequestCachedResponseMethod,
    pub params: RequestCachedResponseParams,
}
#[doc = "Requests data from cache.\n[requestEntries](https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#method-requestEntries)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestEntriesParams {
    #[doc = "ID of cache to get entries from."]
    #[serde(rename = "cacheId")]
    pub cache_id: super::types::CacheId,
    #[doc = "Number of records to skip."]
    #[serde(rename = "skipCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub skip_count: Option<i64>,
    #[doc = "Number of records to fetch."]
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub page_size: Option<i64>,
    #[doc = "If present, only return the entries containing this substring in the path"]
    #[serde(rename = "pathFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub path_filter: Option<String>,
}
impl RequestEntriesParams {
    pub fn new(cache_id: impl Into<super::types::CacheId>) -> Self {
        Self {
            cache_id: cache_id.into(),
            skip_count: None,
            page_size: None,
            path_filter: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RequestEntriesMethod {
    #[serde(rename = "CacheStorage.requestEntries")]
    RequestEntries,
}
impl RequestEntriesMethod {
    pub const IDENTIFIER: &'static str = "CacheStorage.requestEntries";
}
#[doc = "Requests data from cache.\n[requestEntries](https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#method-requestEntries)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestEntries {
    pub method: RequestEntriesMethod,
    pub params: RequestEntriesParams,
}
group_enum ! (Command { DeleteCache (DeleteCache) , DeleteEntry (DeleteEntry) , RequestCacheNames (RequestCacheNames) , RequestCachedResponse (RequestCachedResponse) , RequestEntries (RequestEntries) });
