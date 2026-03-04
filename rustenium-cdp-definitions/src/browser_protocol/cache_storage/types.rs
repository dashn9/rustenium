use serde::{Deserialize, Serialize};
#[doc = "Unique identifier of the Cache object.\n[CacheId](https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#type-CacheId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct CacheId(String);
impl CacheId {
    pub fn new(val: impl Into<String>) -> Self {
        CacheId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for CacheId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<CacheId> for String {
    fn from(el: CacheId) -> String {
        el.0
    }
}
impl From<String> for CacheId {
    fn from(expr: String) -> Self {
        CacheId(expr)
    }
}
impl std::borrow::Borrow<str> for CacheId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl CacheId {
    pub const IDENTIFIER: &'static str = "CacheStorage.CacheId";
}
#[doc = "type of HTTP response cached"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CachedResponseType {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "cors")]
    Cors,
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "opaqueResponse")]
    OpaqueResponse,
    #[serde(rename = "opaqueRedirect")]
    OpaqueRedirect,
}
#[doc = "Data entry.\n[DataEntry](https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#type-DataEntry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataEntry {
    #[doc = "Request URL."]
    #[serde(rename = "requestURL")]
    pub request_url: String,
    #[doc = "Request method."]
    #[serde(rename = "requestMethod")]
    pub request_method: String,
    #[doc = "Request headers"]
    #[serde(rename = "requestHeaders")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub request_headers: Vec<Header>,
    #[doc = "Number of seconds since epoch."]
    #[serde(rename = "responseTime")]
    pub response_time: f64,
    #[doc = "HTTP response status code."]
    #[serde(rename = "responseStatus")]
    pub response_status: i64,
    #[doc = "HTTP response status text."]
    #[serde(rename = "responseStatusText")]
    pub response_status_text: String,
    #[doc = "HTTP response type"]
    #[serde(rename = "responseType")]
    pub response_type: CachedResponseType,
    #[doc = "Response headers"]
    #[serde(rename = "responseHeaders")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub response_headers: Vec<Header>,
}
impl DataEntry {
    pub const IDENTIFIER: &'static str = "CacheStorage.DataEntry";
}
#[doc = "Cache identifier.\n[Cache](https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#type-Cache)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cache {
    #[doc = "An opaque unique id of the cache."]
    #[serde(rename = "cacheId")]
    pub cache_id: CacheId,
    #[doc = "Security origin of the cache."]
    #[serde(rename = "securityOrigin")]
    pub security_origin: String,
    #[doc = "Storage key of the cache."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[doc = "Storage bucket of the cache."]
    #[serde(rename = "storageBucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub storage_bucket: Option<super::super::storage::types::StorageBucket>,
    #[doc = "The name of the cache."]
    #[serde(rename = "cacheName")]
    pub cache_name: String,
}
impl Cache {
    pub fn new(
        cache_id: impl Into<CacheId>,
        security_origin: impl Into<String>,
        storage_key: impl Into<String>,
        cache_name: impl Into<String>,
    ) -> Self {
        Self {
            cache_id: cache_id.into(),
            security_origin: security_origin.into(),
            storage_key: storage_key.into(),
            cache_name: cache_name.into(),
            storage_bucket: None,
        }
    }
}
impl Cache {
    pub const IDENTIFIER: &'static str = "CacheStorage.Cache";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Header {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl Header {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
impl Header {
    pub const IDENTIFIER: &'static str = "CacheStorage.Header";
}
#[doc = "Cached response\n[CachedResponse](https://chromedevtools.github.io/devtools-protocol/tot/CacheStorage/#type-CachedResponse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CachedResponse {
    #[doc = "Entry content, base64-encoded."]
    #[serde(rename = "body")]
    pub body: super::super::super::Binary,
}
impl CachedResponse {
    pub fn new(body: impl Into<super::super::super::Binary>) -> Self {
        Self { body: body.into() }
    }
}
impl CachedResponse {
    pub const IDENTIFIER: &'static str = "CacheStorage.CachedResponse";
}
group_enum ! (Type { CacheId (CacheId) , CachedResponseType (CachedResponseType) , DataEntry (DataEntry) , Cache (Cache) , Header (Header) , CachedResponse (CachedResponse) });
