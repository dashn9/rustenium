use serde::{Deserialize, Serialize};
#[doc = "Returns storage key for the given frame. If no frame ID is provided,\nthe storage key of the target executing this command is returned.\n[getStorageKey](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getStorageKey)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetStorageKeyParams {
    #[serde(rename = "frameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_id: Option<super::super::page::types::FrameId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetStorageKeyMethod {
    #[serde(rename = "Storage.getStorageKey")]
    GetStorageKey,
}
impl GetStorageKeyMethod {
    pub const IDENTIFIER: &'static str = "Storage.getStorageKey";
}
#[doc = "Returns storage key for the given frame. If no frame ID is provided,\nthe storage key of the target executing this command is returned.\n[getStorageKey](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getStorageKey)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetStorageKey {
    pub method: GetStorageKeyMethod,
    pub params: GetStorageKeyParams,
}
#[doc = "Clears storage for origin.\n[clearDataForOrigin](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-clearDataForOrigin)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearDataForOriginParams {
    #[doc = "Security origin."]
    #[serde(rename = "origin")]
    pub origin: String,
    #[doc = "Comma separated list of StorageType to clear."]
    #[serde(rename = "storageTypes")]
    pub storage_types: String,
}
impl ClearDataForOriginParams {
    pub fn new(origin: impl Into<String>, storage_types: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
            storage_types: storage_types.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearDataForOriginMethod {
    #[serde(rename = "Storage.clearDataForOrigin")]
    ClearDataForOrigin,
}
impl ClearDataForOriginMethod {
    pub const IDENTIFIER: &'static str = "Storage.clearDataForOrigin";
}
#[doc = "Clears storage for origin.\n[clearDataForOrigin](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-clearDataForOrigin)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearDataForOrigin {
    pub method: ClearDataForOriginMethod,
    pub params: ClearDataForOriginParams,
}
#[doc = "Clears storage for storage key.\n[clearDataForStorageKey](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-clearDataForStorageKey)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearDataForStorageKeyParams {
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[doc = "Comma separated list of StorageType to clear."]
    #[serde(rename = "storageTypes")]
    pub storage_types: String,
}
impl ClearDataForStorageKeyParams {
    pub fn new(storage_key: impl Into<String>, storage_types: impl Into<String>) -> Self {
        Self {
            storage_key: storage_key.into(),
            storage_types: storage_types.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearDataForStorageKeyMethod {
    #[serde(rename = "Storage.clearDataForStorageKey")]
    ClearDataForStorageKey,
}
impl ClearDataForStorageKeyMethod {
    pub const IDENTIFIER: &'static str = "Storage.clearDataForStorageKey";
}
#[doc = "Clears storage for storage key.\n[clearDataForStorageKey](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-clearDataForStorageKey)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearDataForStorageKey {
    pub method: ClearDataForStorageKeyMethod,
    pub params: ClearDataForStorageKeyParams,
}
#[doc = "Returns all browser cookies.\n[getCookies](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getCookies)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetCookiesParams {
    #[doc = "Browser context to use when called on the browser endpoint."]
    #[serde(rename = "browserContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub browser_context_id: Option<super::super::browser::types::BrowserContextId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetCookiesMethod {
    #[serde(rename = "Storage.getCookies")]
    GetCookies,
}
impl GetCookiesMethod {
    pub const IDENTIFIER: &'static str = "Storage.getCookies";
}
#[doc = "Returns all browser cookies.\n[getCookies](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getCookies)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetCookies {
    pub method: GetCookiesMethod,
    pub params: GetCookiesParams,
}
#[doc = "Sets given cookies.\n[setCookies](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setCookies)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCookiesParams {
    #[doc = "Cookies to be set."]
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookies: Vec<super::super::network::types::CookieParam>,
    #[doc = "Browser context to use when called on the browser endpoint."]
    #[serde(rename = "browserContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub browser_context_id: Option<super::super::browser::types::BrowserContextId>,
}
impl SetCookiesParams {
    pub fn new(cookies: Vec<super::super::network::types::CookieParam>) -> Self {
        Self {
            cookies,
            browser_context_id: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetCookiesMethod {
    #[serde(rename = "Storage.setCookies")]
    SetCookies,
}
impl SetCookiesMethod {
    pub const IDENTIFIER: &'static str = "Storage.setCookies";
}
#[doc = "Sets given cookies.\n[setCookies](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setCookies)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetCookies {
    pub method: SetCookiesMethod,
    pub params: SetCookiesParams,
}
#[doc = "Clears cookies.\n[clearCookies](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-clearCookies)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearCookiesParams {
    #[doc = "Browser context to use when called on the browser endpoint."]
    #[serde(rename = "browserContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub browser_context_id: Option<super::super::browser::types::BrowserContextId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearCookiesMethod {
    #[serde(rename = "Storage.clearCookies")]
    ClearCookies,
}
impl ClearCookiesMethod {
    pub const IDENTIFIER: &'static str = "Storage.clearCookies";
}
#[doc = "Clears cookies.\n[clearCookies](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-clearCookies)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearCookies {
    pub method: ClearCookiesMethod,
    pub params: ClearCookiesParams,
}
#[doc = "Returns usage and quota in bytes.\n[getUsageAndQuota](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getUsageAndQuota)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUsageAndQuotaParams {
    #[doc = "Security origin."]
    #[serde(rename = "origin")]
    pub origin: String,
}
impl GetUsageAndQuotaParams {
    pub fn new(origin: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
        }
    }
}
impl<T: Into<String>> From<T> for GetUsageAndQuotaParams {
    fn from(url: T) -> Self {
        GetUsageAndQuotaParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetUsageAndQuotaMethod {
    #[serde(rename = "Storage.getUsageAndQuota")]
    GetUsageAndQuota,
}
impl GetUsageAndQuotaMethod {
    pub const IDENTIFIER: &'static str = "Storage.getUsageAndQuota";
}
#[doc = "Returns usage and quota in bytes.\n[getUsageAndQuota](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getUsageAndQuota)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetUsageAndQuota {
    pub method: GetUsageAndQuotaMethod,
    pub params: GetUsageAndQuotaParams,
}
#[doc = "Override quota for the specified origin\n[overrideQuotaForOrigin](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-overrideQuotaForOrigin)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverrideQuotaForOriginParams {
    #[doc = "Security origin."]
    #[serde(rename = "origin")]
    pub origin: String,
    #[doc = "The quota size (in bytes) to override the original quota with.\nIf this is called multiple times, the overridden quota will be equal to\nthe quotaSize provided in the final call. If this is called without\nspecifying a quotaSize, the quota will be reset to the default value for\nthe specified origin. If this is called multiple times with different\norigins, the override will be maintained for each origin until it is\ndisabled (called without a quotaSize)."]
    #[serde(rename = "quotaSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub quota_size: Option<f64>,
}
impl OverrideQuotaForOriginParams {
    pub fn new(origin: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
            quota_size: None,
        }
    }
}
impl<T: Into<String>> From<T> for OverrideQuotaForOriginParams {
    fn from(url: T) -> Self {
        OverrideQuotaForOriginParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OverrideQuotaForOriginMethod {
    #[serde(rename = "Storage.overrideQuotaForOrigin")]
    OverrideQuotaForOrigin,
}
impl OverrideQuotaForOriginMethod {
    pub const IDENTIFIER: &'static str = "Storage.overrideQuotaForOrigin";
}
#[doc = "Override quota for the specified origin\n[overrideQuotaForOrigin](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-overrideQuotaForOrigin)"]
#[derive(Debug, Clone, PartialEq)]
pub struct OverrideQuotaForOrigin {
    pub method: OverrideQuotaForOriginMethod,
    pub params: OverrideQuotaForOriginParams,
}
#[doc = "Registers origin to be notified when an update occurs to its cache storage list.\n[trackCacheStorageForOrigin](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-trackCacheStorageForOrigin)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackCacheStorageForOriginParams {
    #[doc = "Security origin."]
    #[serde(rename = "origin")]
    pub origin: String,
}
impl TrackCacheStorageForOriginParams {
    pub fn new(origin: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
        }
    }
}
impl<T: Into<String>> From<T> for TrackCacheStorageForOriginParams {
    fn from(url: T) -> Self {
        TrackCacheStorageForOriginParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrackCacheStorageForOriginMethod {
    #[serde(rename = "Storage.trackCacheStorageForOrigin")]
    TrackCacheStorageForOrigin,
}
impl TrackCacheStorageForOriginMethod {
    pub const IDENTIFIER: &'static str = "Storage.trackCacheStorageForOrigin";
}
#[doc = "Registers origin to be notified when an update occurs to its cache storage list.\n[trackCacheStorageForOrigin](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-trackCacheStorageForOrigin)"]
#[derive(Debug, Clone, PartialEq)]
pub struct TrackCacheStorageForOrigin {
    pub method: TrackCacheStorageForOriginMethod,
    pub params: TrackCacheStorageForOriginParams,
}
#[doc = "Registers storage key to be notified when an update occurs to its cache storage list.\n[trackCacheStorageForStorageKey](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-trackCacheStorageForStorageKey)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackCacheStorageForStorageKeyParams {
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
}
impl TrackCacheStorageForStorageKeyParams {
    pub fn new(storage_key: impl Into<String>) -> Self {
        Self {
            storage_key: storage_key.into(),
        }
    }
}
impl<T: Into<String>> From<T> for TrackCacheStorageForStorageKeyParams {
    fn from(url: T) -> Self {
        TrackCacheStorageForStorageKeyParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrackCacheStorageForStorageKeyMethod {
    #[serde(rename = "Storage.trackCacheStorageForStorageKey")]
    TrackCacheStorageForStorageKey,
}
impl TrackCacheStorageForStorageKeyMethod {
    pub const IDENTIFIER: &'static str = "Storage.trackCacheStorageForStorageKey";
}
#[doc = "Registers storage key to be notified when an update occurs to its cache storage list.\n[trackCacheStorageForStorageKey](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-trackCacheStorageForStorageKey)"]
#[derive(Debug, Clone, PartialEq)]
pub struct TrackCacheStorageForStorageKey {
    pub method: TrackCacheStorageForStorageKeyMethod,
    pub params: TrackCacheStorageForStorageKeyParams,
}
#[doc = "Registers origin to be notified when an update occurs to its IndexedDB.\n[trackIndexedDBForOrigin](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-trackIndexedDBForOrigin)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackIndexedDbForOriginParams {
    #[doc = "Security origin."]
    #[serde(rename = "origin")]
    pub origin: String,
}
impl TrackIndexedDbForOriginParams {
    pub fn new(origin: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
        }
    }
}
impl<T: Into<String>> From<T> for TrackIndexedDbForOriginParams {
    fn from(url: T) -> Self {
        TrackIndexedDbForOriginParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrackIndexedDbForOriginMethod {
    #[serde(rename = "Storage.trackIndexedDBForOrigin")]
    TrackIndexedDbForOrigin,
}
impl TrackIndexedDbForOriginMethod {
    pub const IDENTIFIER: &'static str = "Storage.trackIndexedDBForOrigin";
}
#[doc = "Registers origin to be notified when an update occurs to its IndexedDB.\n[trackIndexedDBForOrigin](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-trackIndexedDBForOrigin)"]
#[derive(Debug, Clone, PartialEq)]
pub struct TrackIndexedDbForOrigin {
    pub method: TrackIndexedDbForOriginMethod,
    pub params: TrackIndexedDbForOriginParams,
}
#[doc = "Registers storage key to be notified when an update occurs to its IndexedDB.\n[trackIndexedDBForStorageKey](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-trackIndexedDBForStorageKey)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackIndexedDbForStorageKeyParams {
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
}
impl TrackIndexedDbForStorageKeyParams {
    pub fn new(storage_key: impl Into<String>) -> Self {
        Self {
            storage_key: storage_key.into(),
        }
    }
}
impl<T: Into<String>> From<T> for TrackIndexedDbForStorageKeyParams {
    fn from(url: T) -> Self {
        TrackIndexedDbForStorageKeyParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TrackIndexedDbForStorageKeyMethod {
    #[serde(rename = "Storage.trackIndexedDBForStorageKey")]
    TrackIndexedDbForStorageKey,
}
impl TrackIndexedDbForStorageKeyMethod {
    pub const IDENTIFIER: &'static str = "Storage.trackIndexedDBForStorageKey";
}
#[doc = "Registers storage key to be notified when an update occurs to its IndexedDB.\n[trackIndexedDBForStorageKey](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-trackIndexedDBForStorageKey)"]
#[derive(Debug, Clone, PartialEq)]
pub struct TrackIndexedDbForStorageKey {
    pub method: TrackIndexedDbForStorageKeyMethod,
    pub params: TrackIndexedDbForStorageKeyParams,
}
#[doc = "Unregisters origin from receiving notifications for cache storage.\n[untrackCacheStorageForOrigin](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-untrackCacheStorageForOrigin)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UntrackCacheStorageForOriginParams {
    #[doc = "Security origin."]
    #[serde(rename = "origin")]
    pub origin: String,
}
impl UntrackCacheStorageForOriginParams {
    pub fn new(origin: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
        }
    }
}
impl<T: Into<String>> From<T> for UntrackCacheStorageForOriginParams {
    fn from(url: T) -> Self {
        UntrackCacheStorageForOriginParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UntrackCacheStorageForOriginMethod {
    #[serde(rename = "Storage.untrackCacheStorageForOrigin")]
    UntrackCacheStorageForOrigin,
}
impl UntrackCacheStorageForOriginMethod {
    pub const IDENTIFIER: &'static str = "Storage.untrackCacheStorageForOrigin";
}
#[doc = "Unregisters origin from receiving notifications for cache storage.\n[untrackCacheStorageForOrigin](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-untrackCacheStorageForOrigin)"]
#[derive(Debug, Clone, PartialEq)]
pub struct UntrackCacheStorageForOrigin {
    pub method: UntrackCacheStorageForOriginMethod,
    pub params: UntrackCacheStorageForOriginParams,
}
#[doc = "Unregisters storage key from receiving notifications for cache storage.\n[untrackCacheStorageForStorageKey](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-untrackCacheStorageForStorageKey)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UntrackCacheStorageForStorageKeyParams {
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
}
impl UntrackCacheStorageForStorageKeyParams {
    pub fn new(storage_key: impl Into<String>) -> Self {
        Self {
            storage_key: storage_key.into(),
        }
    }
}
impl<T: Into<String>> From<T> for UntrackCacheStorageForStorageKeyParams {
    fn from(url: T) -> Self {
        UntrackCacheStorageForStorageKeyParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UntrackCacheStorageForStorageKeyMethod {
    #[serde(rename = "Storage.untrackCacheStorageForStorageKey")]
    UntrackCacheStorageForStorageKey,
}
impl UntrackCacheStorageForStorageKeyMethod {
    pub const IDENTIFIER: &'static str = "Storage.untrackCacheStorageForStorageKey";
}
#[doc = "Unregisters storage key from receiving notifications for cache storage.\n[untrackCacheStorageForStorageKey](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-untrackCacheStorageForStorageKey)"]
#[derive(Debug, Clone, PartialEq)]
pub struct UntrackCacheStorageForStorageKey {
    pub method: UntrackCacheStorageForStorageKeyMethod,
    pub params: UntrackCacheStorageForStorageKeyParams,
}
#[doc = "Unregisters origin from receiving notifications for IndexedDB.\n[untrackIndexedDBForOrigin](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-untrackIndexedDBForOrigin)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UntrackIndexedDbForOriginParams {
    #[doc = "Security origin."]
    #[serde(rename = "origin")]
    pub origin: String,
}
impl UntrackIndexedDbForOriginParams {
    pub fn new(origin: impl Into<String>) -> Self {
        Self {
            origin: origin.into(),
        }
    }
}
impl<T: Into<String>> From<T> for UntrackIndexedDbForOriginParams {
    fn from(url: T) -> Self {
        UntrackIndexedDbForOriginParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UntrackIndexedDbForOriginMethod {
    #[serde(rename = "Storage.untrackIndexedDBForOrigin")]
    UntrackIndexedDbForOrigin,
}
impl UntrackIndexedDbForOriginMethod {
    pub const IDENTIFIER: &'static str = "Storage.untrackIndexedDBForOrigin";
}
#[doc = "Unregisters origin from receiving notifications for IndexedDB.\n[untrackIndexedDBForOrigin](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-untrackIndexedDBForOrigin)"]
#[derive(Debug, Clone, PartialEq)]
pub struct UntrackIndexedDbForOrigin {
    pub method: UntrackIndexedDbForOriginMethod,
    pub params: UntrackIndexedDbForOriginParams,
}
#[doc = "Unregisters storage key from receiving notifications for IndexedDB.\n[untrackIndexedDBForStorageKey](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-untrackIndexedDBForStorageKey)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UntrackIndexedDbForStorageKeyParams {
    #[doc = "Storage key."]
    #[serde(rename = "storageKey")]
    pub storage_key: String,
}
impl UntrackIndexedDbForStorageKeyParams {
    pub fn new(storage_key: impl Into<String>) -> Self {
        Self {
            storage_key: storage_key.into(),
        }
    }
}
impl<T: Into<String>> From<T> for UntrackIndexedDbForStorageKeyParams {
    fn from(url: T) -> Self {
        UntrackIndexedDbForStorageKeyParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UntrackIndexedDbForStorageKeyMethod {
    #[serde(rename = "Storage.untrackIndexedDBForStorageKey")]
    UntrackIndexedDbForStorageKey,
}
impl UntrackIndexedDbForStorageKeyMethod {
    pub const IDENTIFIER: &'static str = "Storage.untrackIndexedDBForStorageKey";
}
#[doc = "Unregisters storage key from receiving notifications for IndexedDB.\n[untrackIndexedDBForStorageKey](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-untrackIndexedDBForStorageKey)"]
#[derive(Debug, Clone, PartialEq)]
pub struct UntrackIndexedDbForStorageKey {
    pub method: UntrackIndexedDbForStorageKeyMethod,
    pub params: UntrackIndexedDbForStorageKeyParams,
}
#[doc = "Returns the number of stored Trust Tokens per issuer for the\ncurrent browsing context.\n[getTrustTokens](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getTrustTokens)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetTrustTokensParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetTrustTokensMethod {
    #[serde(rename = "Storage.getTrustTokens")]
    GetTrustTokens,
}
impl GetTrustTokensMethod {
    pub const IDENTIFIER: &'static str = "Storage.getTrustTokens";
}
#[doc = "Returns the number of stored Trust Tokens per issuer for the\ncurrent browsing context.\n[getTrustTokens](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getTrustTokens)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetTrustTokens {
    pub method: GetTrustTokensMethod,
    pub params: GetTrustTokensParams,
}
#[doc = "Removes all Trust Tokens issued by the provided issuerOrigin.\nLeaves other stored data, including the issuer's Redemption Records, intact.\n[clearTrustTokens](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-clearTrustTokens)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearTrustTokensParams {
    #[serde(rename = "issuerOrigin")]
    pub issuer_origin: String,
}
impl ClearTrustTokensParams {
    pub fn new(issuer_origin: impl Into<String>) -> Self {
        Self {
            issuer_origin: issuer_origin.into(),
        }
    }
}
impl<T: Into<String>> From<T> for ClearTrustTokensParams {
    fn from(url: T) -> Self {
        ClearTrustTokensParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearTrustTokensMethod {
    #[serde(rename = "Storage.clearTrustTokens")]
    ClearTrustTokens,
}
impl ClearTrustTokensMethod {
    pub const IDENTIFIER: &'static str = "Storage.clearTrustTokens";
}
#[doc = "Removes all Trust Tokens issued by the provided issuerOrigin.\nLeaves other stored data, including the issuer's Redemption Records, intact.\n[clearTrustTokens](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-clearTrustTokens)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearTrustTokens {
    pub method: ClearTrustTokensMethod,
    pub params: ClearTrustTokensParams,
}
#[doc = "Gets details for a named interest group.\n[getInterestGroupDetails](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getInterestGroupDetails)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInterestGroupDetailsParams {
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
    #[serde(rename = "name")]
    pub name: String,
}
impl GetInterestGroupDetailsParams {
    pub fn new(owner_origin: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            owner_origin: owner_origin.into(),
            name: name.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetInterestGroupDetailsMethod {
    #[serde(rename = "Storage.getInterestGroupDetails")]
    GetInterestGroupDetails,
}
impl GetInterestGroupDetailsMethod {
    pub const IDENTIFIER: &'static str = "Storage.getInterestGroupDetails";
}
#[doc = "Gets details for a named interest group.\n[getInterestGroupDetails](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getInterestGroupDetails)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetInterestGroupDetails {
    pub method: GetInterestGroupDetailsMethod,
    pub params: GetInterestGroupDetailsParams,
}
#[doc = "Enables/Disables issuing of interestGroupAccessed events.\n[setInterestGroupTracking](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setInterestGroupTracking)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInterestGroupTrackingParams {
    #[serde(rename = "enable")]
    pub enable: bool,
}
impl SetInterestGroupTrackingParams {
    pub fn new(enable: impl Into<bool>) -> Self {
        Self {
            enable: enable.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetInterestGroupTrackingMethod {
    #[serde(rename = "Storage.setInterestGroupTracking")]
    SetInterestGroupTracking,
}
impl SetInterestGroupTrackingMethod {
    pub const IDENTIFIER: &'static str = "Storage.setInterestGroupTracking";
}
#[doc = "Enables/Disables issuing of interestGroupAccessed events.\n[setInterestGroupTracking](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setInterestGroupTracking)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetInterestGroupTracking {
    pub method: SetInterestGroupTrackingMethod,
    pub params: SetInterestGroupTrackingParams,
}
#[doc = "Enables/Disables issuing of interestGroupAuctionEventOccurred and\ninterestGroupAuctionNetworkRequestCreated.\n[setInterestGroupAuctionTracking](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setInterestGroupAuctionTracking)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInterestGroupAuctionTrackingParams {
    #[serde(rename = "enable")]
    pub enable: bool,
}
impl SetInterestGroupAuctionTrackingParams {
    pub fn new(enable: impl Into<bool>) -> Self {
        Self {
            enable: enable.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetInterestGroupAuctionTrackingMethod {
    #[serde(rename = "Storage.setInterestGroupAuctionTracking")]
    SetInterestGroupAuctionTracking,
}
impl SetInterestGroupAuctionTrackingMethod {
    pub const IDENTIFIER: &'static str = "Storage.setInterestGroupAuctionTracking";
}
#[doc = "Enables/Disables issuing of interestGroupAuctionEventOccurred and\ninterestGroupAuctionNetworkRequestCreated.\n[setInterestGroupAuctionTracking](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setInterestGroupAuctionTracking)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetInterestGroupAuctionTracking {
    pub method: SetInterestGroupAuctionTrackingMethod,
    pub params: SetInterestGroupAuctionTrackingParams,
}
#[doc = "Gets metadata for an origin's shared storage.\n[getSharedStorageMetadata](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getSharedStorageMetadata)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSharedStorageMetadataParams {
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
}
impl GetSharedStorageMetadataParams {
    pub fn new(owner_origin: impl Into<String>) -> Self {
        Self {
            owner_origin: owner_origin.into(),
        }
    }
}
impl<T: Into<String>> From<T> for GetSharedStorageMetadataParams {
    fn from(url: T) -> Self {
        GetSharedStorageMetadataParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetSharedStorageMetadataMethod {
    #[serde(rename = "Storage.getSharedStorageMetadata")]
    GetSharedStorageMetadata,
}
impl GetSharedStorageMetadataMethod {
    pub const IDENTIFIER: &'static str = "Storage.getSharedStorageMetadata";
}
#[doc = "Gets metadata for an origin's shared storage.\n[getSharedStorageMetadata](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getSharedStorageMetadata)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetSharedStorageMetadata {
    pub method: GetSharedStorageMetadataMethod,
    pub params: GetSharedStorageMetadataParams,
}
#[doc = "Gets the entries in an given origin's shared storage.\n[getSharedStorageEntries](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getSharedStorageEntries)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSharedStorageEntriesParams {
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
}
impl GetSharedStorageEntriesParams {
    pub fn new(owner_origin: impl Into<String>) -> Self {
        Self {
            owner_origin: owner_origin.into(),
        }
    }
}
impl<T: Into<String>> From<T> for GetSharedStorageEntriesParams {
    fn from(url: T) -> Self {
        GetSharedStorageEntriesParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetSharedStorageEntriesMethod {
    #[serde(rename = "Storage.getSharedStorageEntries")]
    GetSharedStorageEntries,
}
impl GetSharedStorageEntriesMethod {
    pub const IDENTIFIER: &'static str = "Storage.getSharedStorageEntries";
}
#[doc = "Gets the entries in an given origin's shared storage.\n[getSharedStorageEntries](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getSharedStorageEntries)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetSharedStorageEntries {
    pub method: GetSharedStorageEntriesMethod,
    pub params: GetSharedStorageEntriesParams,
}
#[doc = "Sets entry with `key` and `value` for a given origin's shared storage.\n[setSharedStorageEntry](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setSharedStorageEntry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSharedStorageEntryParams {
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
    #[serde(rename = "key")]
    pub key: String,
    #[serde(rename = "value")]
    pub value: String,
    #[doc = "If `ignoreIfPresent` is included and true, then only sets the entry if\n`key` doesn't already exist."]
    #[serde(rename = "ignoreIfPresent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ignore_if_present: Option<bool>,
}
impl SetSharedStorageEntryParams {
    pub fn new(
        owner_origin: impl Into<String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self {
            owner_origin: owner_origin.into(),
            key: key.into(),
            value: value.into(),
            ignore_if_present: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetSharedStorageEntryMethod {
    #[serde(rename = "Storage.setSharedStorageEntry")]
    SetSharedStorageEntry,
}
impl SetSharedStorageEntryMethod {
    pub const IDENTIFIER: &'static str = "Storage.setSharedStorageEntry";
}
#[doc = "Sets entry with `key` and `value` for a given origin's shared storage.\n[setSharedStorageEntry](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setSharedStorageEntry)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetSharedStorageEntry {
    pub method: SetSharedStorageEntryMethod,
    pub params: SetSharedStorageEntryParams,
}
#[doc = "Deletes entry for `key` (if it exists) for a given origin's shared storage.\n[deleteSharedStorageEntry](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-deleteSharedStorageEntry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteSharedStorageEntryParams {
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
    #[serde(rename = "key")]
    pub key: String,
}
impl DeleteSharedStorageEntryParams {
    pub fn new(owner_origin: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            owner_origin: owner_origin.into(),
            key: key.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteSharedStorageEntryMethod {
    #[serde(rename = "Storage.deleteSharedStorageEntry")]
    DeleteSharedStorageEntry,
}
impl DeleteSharedStorageEntryMethod {
    pub const IDENTIFIER: &'static str = "Storage.deleteSharedStorageEntry";
}
#[doc = "Deletes entry for `key` (if it exists) for a given origin's shared storage.\n[deleteSharedStorageEntry](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-deleteSharedStorageEntry)"]
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteSharedStorageEntry {
    pub method: DeleteSharedStorageEntryMethod,
    pub params: DeleteSharedStorageEntryParams,
}
#[doc = "Clears all entries for a given origin's shared storage.\n[clearSharedStorageEntries](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-clearSharedStorageEntries)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearSharedStorageEntriesParams {
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
}
impl ClearSharedStorageEntriesParams {
    pub fn new(owner_origin: impl Into<String>) -> Self {
        Self {
            owner_origin: owner_origin.into(),
        }
    }
}
impl<T: Into<String>> From<T> for ClearSharedStorageEntriesParams {
    fn from(url: T) -> Self {
        ClearSharedStorageEntriesParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearSharedStorageEntriesMethod {
    #[serde(rename = "Storage.clearSharedStorageEntries")]
    ClearSharedStorageEntries,
}
impl ClearSharedStorageEntriesMethod {
    pub const IDENTIFIER: &'static str = "Storage.clearSharedStorageEntries";
}
#[doc = "Clears all entries for a given origin's shared storage.\n[clearSharedStorageEntries](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-clearSharedStorageEntries)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearSharedStorageEntries {
    pub method: ClearSharedStorageEntriesMethod,
    pub params: ClearSharedStorageEntriesParams,
}
#[doc = "Resets the budget for `ownerOrigin` by clearing all budget withdrawals.\n[resetSharedStorageBudget](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-resetSharedStorageBudget)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetSharedStorageBudgetParams {
    #[serde(rename = "ownerOrigin")]
    pub owner_origin: String,
}
impl ResetSharedStorageBudgetParams {
    pub fn new(owner_origin: impl Into<String>) -> Self {
        Self {
            owner_origin: owner_origin.into(),
        }
    }
}
impl<T: Into<String>> From<T> for ResetSharedStorageBudgetParams {
    fn from(url: T) -> Self {
        ResetSharedStorageBudgetParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResetSharedStorageBudgetMethod {
    #[serde(rename = "Storage.resetSharedStorageBudget")]
    ResetSharedStorageBudget,
}
impl ResetSharedStorageBudgetMethod {
    pub const IDENTIFIER: &'static str = "Storage.resetSharedStorageBudget";
}
#[doc = "Resets the budget for `ownerOrigin` by clearing all budget withdrawals.\n[resetSharedStorageBudget](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-resetSharedStorageBudget)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ResetSharedStorageBudget {
    pub method: ResetSharedStorageBudgetMethod,
    pub params: ResetSharedStorageBudgetParams,
}
#[doc = "Enables/disables issuing of sharedStorageAccessed events.\n[setSharedStorageTracking](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setSharedStorageTracking)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSharedStorageTrackingParams {
    #[serde(rename = "enable")]
    pub enable: bool,
}
impl SetSharedStorageTrackingParams {
    pub fn new(enable: impl Into<bool>) -> Self {
        Self {
            enable: enable.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetSharedStorageTrackingMethod {
    #[serde(rename = "Storage.setSharedStorageTracking")]
    SetSharedStorageTracking,
}
impl SetSharedStorageTrackingMethod {
    pub const IDENTIFIER: &'static str = "Storage.setSharedStorageTracking";
}
#[doc = "Enables/disables issuing of sharedStorageAccessed events.\n[setSharedStorageTracking](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setSharedStorageTracking)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetSharedStorageTracking {
    pub method: SetSharedStorageTrackingMethod,
    pub params: SetSharedStorageTrackingParams,
}
#[doc = "Set tracking for a storage key's buckets.\n[setStorageBucketTracking](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setStorageBucketTracking)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetStorageBucketTrackingParams {
    #[serde(rename = "storageKey")]
    pub storage_key: String,
    #[serde(rename = "enable")]
    pub enable: bool,
}
impl SetStorageBucketTrackingParams {
    pub fn new(storage_key: impl Into<String>, enable: impl Into<bool>) -> Self {
        Self {
            storage_key: storage_key.into(),
            enable: enable.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetStorageBucketTrackingMethod {
    #[serde(rename = "Storage.setStorageBucketTracking")]
    SetStorageBucketTracking,
}
impl SetStorageBucketTrackingMethod {
    pub const IDENTIFIER: &'static str = "Storage.setStorageBucketTracking";
}
#[doc = "Set tracking for a storage key's buckets.\n[setStorageBucketTracking](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setStorageBucketTracking)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetStorageBucketTracking {
    pub method: SetStorageBucketTrackingMethod,
    pub params: SetStorageBucketTrackingParams,
}
#[doc = "Deletes the Storage Bucket with the given storage key and bucket name.\n[deleteStorageBucket](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-deleteStorageBucket)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteStorageBucketParams {
    #[serde(rename = "bucket")]
    pub bucket: super::types::StorageBucket,
}
impl DeleteStorageBucketParams {
    pub fn new(bucket: impl Into<super::types::StorageBucket>) -> Self {
        Self {
            bucket: bucket.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeleteStorageBucketMethod {
    #[serde(rename = "Storage.deleteStorageBucket")]
    DeleteStorageBucket,
}
impl DeleteStorageBucketMethod {
    pub const IDENTIFIER: &'static str = "Storage.deleteStorageBucket";
}
#[doc = "Deletes the Storage Bucket with the given storage key and bucket name.\n[deleteStorageBucket](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-deleteStorageBucket)"]
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteStorageBucket {
    pub method: DeleteStorageBucketMethod,
    pub params: DeleteStorageBucketParams,
}
#[doc = "Deletes state for sites identified as potential bounce trackers, immediately.\n[runBounceTrackingMitigations](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-runBounceTrackingMitigations)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RunBounceTrackingMitigationsParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RunBounceTrackingMitigationsMethod {
    #[serde(rename = "Storage.runBounceTrackingMitigations")]
    RunBounceTrackingMitigations,
}
impl RunBounceTrackingMitigationsMethod {
    pub const IDENTIFIER: &'static str = "Storage.runBounceTrackingMitigations";
}
#[doc = "Deletes state for sites identified as potential bounce trackers, immediately.\n[runBounceTrackingMitigations](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-runBounceTrackingMitigations)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RunBounceTrackingMitigations {
    pub method: RunBounceTrackingMitigationsMethod,
    pub params: RunBounceTrackingMitigationsParams,
}
#[doc = "https://wicg.github.io/attribution-reporting-api/\n[setAttributionReportingLocalTestingMode](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setAttributionReportingLocalTestingMode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAttributionReportingLocalTestingModeParams {
    #[doc = "If enabled, noise is suppressed and reports are sent immediately."]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
impl SetAttributionReportingLocalTestingModeParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self {
            enabled: enabled.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetAttributionReportingLocalTestingModeMethod {
    #[serde(rename = "Storage.setAttributionReportingLocalTestingMode")]
    SetAttributionReportingLocalTestingMode,
}
impl SetAttributionReportingLocalTestingModeMethod {
    pub const IDENTIFIER: &'static str = "Storage.setAttributionReportingLocalTestingMode";
}
#[doc = "https://wicg.github.io/attribution-reporting-api/\n[setAttributionReportingLocalTestingMode](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setAttributionReportingLocalTestingMode)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetAttributionReportingLocalTestingMode {
    pub method: SetAttributionReportingLocalTestingModeMethod,
    pub params: SetAttributionReportingLocalTestingModeParams,
}
#[doc = "Enables/disables issuing of Attribution Reporting events.\n[setAttributionReportingTracking](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setAttributionReportingTracking)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAttributionReportingTrackingParams {
    #[serde(rename = "enable")]
    pub enable: bool,
}
impl SetAttributionReportingTrackingParams {
    pub fn new(enable: impl Into<bool>) -> Self {
        Self {
            enable: enable.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetAttributionReportingTrackingMethod {
    #[serde(rename = "Storage.setAttributionReportingTracking")]
    SetAttributionReportingTracking,
}
impl SetAttributionReportingTrackingMethod {
    pub const IDENTIFIER: &'static str = "Storage.setAttributionReportingTracking";
}
#[doc = "Enables/disables issuing of Attribution Reporting events.\n[setAttributionReportingTracking](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-setAttributionReportingTracking)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetAttributionReportingTracking {
    pub method: SetAttributionReportingTrackingMethod,
    pub params: SetAttributionReportingTrackingParams,
}
#[doc = "Sends all pending Attribution Reports immediately, regardless of their\nscheduled report time.\n[sendPendingAttributionReports](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-sendPendingAttributionReports)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SendPendingAttributionReportsParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SendPendingAttributionReportsMethod {
    #[serde(rename = "Storage.sendPendingAttributionReports")]
    SendPendingAttributionReports,
}
impl SendPendingAttributionReportsMethod {
    pub const IDENTIFIER: &'static str = "Storage.sendPendingAttributionReports";
}
#[doc = "Sends all pending Attribution Reports immediately, regardless of their\nscheduled report time.\n[sendPendingAttributionReports](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-sendPendingAttributionReports)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SendPendingAttributionReports {
    pub method: SendPendingAttributionReportsMethod,
    pub params: SendPendingAttributionReportsParams,
}
#[doc = "Returns the effective Related Website Sets in use by this profile for the browser\nsession. The effective Related Website Sets will not change during a browser session.\n[getRelatedWebsiteSets](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getRelatedWebsiteSets)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetRelatedWebsiteSetsParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetRelatedWebsiteSetsMethod {
    #[serde(rename = "Storage.getRelatedWebsiteSets")]
    GetRelatedWebsiteSets,
}
impl GetRelatedWebsiteSetsMethod {
    pub const IDENTIFIER: &'static str = "Storage.getRelatedWebsiteSets";
}
#[doc = "Returns the effective Related Website Sets in use by this profile for the browser\nsession. The effective Related Website Sets will not change during a browser session.\n[getRelatedWebsiteSets](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getRelatedWebsiteSets)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetRelatedWebsiteSets {
    pub method: GetRelatedWebsiteSetsMethod,
    pub params: GetRelatedWebsiteSetsParams,
}
#[doc = "Returns the list of URLs from a page and its embedded resources that match\nexisting grace period URL pattern rules.\nhttps://developers.google.com/privacy-sandbox/cookies/temporary-exceptions/grace-period\n[getAffectedUrlsForThirdPartyCookieMetadata](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getAffectedUrlsForThirdPartyCookieMetadata)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAffectedUrlsForThirdPartyCookieMetadataParams {
    #[doc = "The URL of the page currently being visited."]
    #[serde(rename = "firstPartyUrl")]
    pub first_party_url: String,
    #[doc = "The list of embedded resource URLs from the page."]
    #[serde(rename = "thirdPartyUrls")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub third_party_urls: Vec<String>,
}
impl GetAffectedUrlsForThirdPartyCookieMetadataParams {
    pub fn new(first_party_url: impl Into<String>, third_party_urls: Vec<String>) -> Self {
        Self {
            first_party_url: first_party_url.into(),
            third_party_urls,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetAffectedUrlsForThirdPartyCookieMetadataMethod {
    #[serde(rename = "Storage.getAffectedUrlsForThirdPartyCookieMetadata")]
    GetAffectedUrlsForThirdPartyCookieMetadata,
}
impl GetAffectedUrlsForThirdPartyCookieMetadataMethod {
    pub const IDENTIFIER: &'static str = "Storage.getAffectedUrlsForThirdPartyCookieMetadata";
}
#[doc = "Returns the list of URLs from a page and its embedded resources that match\nexisting grace period URL pattern rules.\nhttps://developers.google.com/privacy-sandbox/cookies/temporary-exceptions/grace-period\n[getAffectedUrlsForThirdPartyCookieMetadata](https://chromedevtools.github.io/devtools-protocol/tot/Storage/#method-getAffectedUrlsForThirdPartyCookieMetadata)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetAffectedUrlsForThirdPartyCookieMetadata {
    pub method: GetAffectedUrlsForThirdPartyCookieMetadataMethod,
    pub params: GetAffectedUrlsForThirdPartyCookieMetadataParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetProtectedAudienceKAnonymityParams {
    #[serde(rename = "owner")]
    pub owner: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "hashes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub hashes: Vec<super::super::super::Binary>,
}
impl SetProtectedAudienceKAnonymityParams {
    pub fn new(
        owner: impl Into<String>,
        name: impl Into<String>,
        hashes: Vec<super::super::super::Binary>,
    ) -> Self {
        Self {
            owner: owner.into(),
            name: name.into(),
            hashes,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetProtectedAudienceKAnonymityMethod {
    #[serde(rename = "Storage.setProtectedAudienceKAnonymity")]
    SetProtectedAudienceKAnonymity,
}
impl SetProtectedAudienceKAnonymityMethod {
    pub const IDENTIFIER: &'static str = "Storage.setProtectedAudienceKAnonymity";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetProtectedAudienceKAnonymity {
    pub method: SetProtectedAudienceKAnonymityMethod,
    pub params: SetProtectedAudienceKAnonymityParams,
}
group_enum ! (StorageCommands { GetStorageKey (GetStorageKey) , ClearDataForOrigin (ClearDataForOrigin) , ClearDataForStorageKey (ClearDataForStorageKey) , GetCookies (GetCookies) , SetCookies (SetCookies) , ClearCookies (ClearCookies) , GetUsageAndQuota (GetUsageAndQuota) , OverrideQuotaForOrigin (OverrideQuotaForOrigin) , TrackCacheStorageForOrigin (TrackCacheStorageForOrigin) , TrackCacheStorageForStorageKey (TrackCacheStorageForStorageKey) , TrackIndexedDbForOrigin (TrackIndexedDbForOrigin) , TrackIndexedDbForStorageKey (TrackIndexedDbForStorageKey) , UntrackCacheStorageForOrigin (UntrackCacheStorageForOrigin) , UntrackCacheStorageForStorageKey (UntrackCacheStorageForStorageKey) , UntrackIndexedDbForOrigin (UntrackIndexedDbForOrigin) , UntrackIndexedDbForStorageKey (UntrackIndexedDbForStorageKey) , GetTrustTokens (GetTrustTokens) , ClearTrustTokens (ClearTrustTokens) , GetInterestGroupDetails (GetInterestGroupDetails) , SetInterestGroupTracking (SetInterestGroupTracking) , SetInterestGroupAuctionTracking (SetInterestGroupAuctionTracking) , GetSharedStorageMetadata (GetSharedStorageMetadata) , GetSharedStorageEntries (GetSharedStorageEntries) , SetSharedStorageEntry (SetSharedStorageEntry) , DeleteSharedStorageEntry (DeleteSharedStorageEntry) , ClearSharedStorageEntries (ClearSharedStorageEntries) , ResetSharedStorageBudget (ResetSharedStorageBudget) , SetSharedStorageTracking (SetSharedStorageTracking) , SetStorageBucketTracking (SetStorageBucketTracking) , DeleteStorageBucket (DeleteStorageBucket) , RunBounceTrackingMitigations (RunBounceTrackingMitigations) , SetAttributionReportingLocalTestingMode (SetAttributionReportingLocalTestingMode) , SetAttributionReportingTracking (SetAttributionReportingTracking) , SendPendingAttributionReports (SendPendingAttributionReports) , GetRelatedWebsiteSets (GetRelatedWebsiteSets) , GetAffectedUrlsForThirdPartyCookieMetadata (GetAffectedUrlsForThirdPartyCookieMetadata) , SetProtectedAudienceKAnonymity (SetProtectedAudienceKAnonymity) });
