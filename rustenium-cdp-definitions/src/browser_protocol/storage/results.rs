use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStorageKeyForFrameResult {
    #[serde(rename = "storageKey")]
    pub storage_key: super::types::SerializedStorageKey,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStorageKeyResult {
    #[serde(rename = "storageKey")]
    pub storage_key: super::types::SerializedStorageKey,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearDataForOriginResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearDataForStorageKeyResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCookiesResult {
    #[doc = "Array of cookie objects."]
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookies: Vec<crate::browser_protocol::network::types::Cookie>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetCookiesResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearCookiesResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUsageAndQuotaResult {
    #[doc = "Storage usage (bytes)."]
    #[serde(rename = "usage")]
    pub usage: f64,
    #[doc = "Storage quota (bytes)."]
    #[serde(rename = "quota")]
    pub quota: f64,
    #[doc = "Whether or not the origin has an active storage quota override"]
    #[serde(rename = "overrideActive")]
    pub override_active: bool,
    #[doc = "Storage usage per type (bytes)."]
    #[serde(rename = "usageBreakdown")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub usage_breakdown: Vec<super::types::UsageForType>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OverrideQuotaForOriginResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TrackCacheStorageForOriginResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TrackCacheStorageForStorageKeyResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TrackIndexedDbForOriginResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TrackIndexedDbForStorageKeyResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UntrackCacheStorageForOriginResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UntrackCacheStorageForStorageKeyResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UntrackIndexedDbForOriginResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UntrackIndexedDbForStorageKeyResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTrustTokensResult {
    #[serde(rename = "tokens")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tokens: Vec<super::types::TrustTokens>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearTrustTokensResult {
    #[doc = "True if any tokens were deleted, false otherwise."]
    #[serde(rename = "didDeleteTokens")]
    pub did_delete_tokens: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInterestGroupDetailsResult {
    #[doc = "This largely corresponds to:\nhttps://wicg.github.io/turtledove/#dictdef-generatebidinterestgroup\nbut has absolute expirationTime instead of relative lifetimeMs and\nalso adds joiningOrigin."]
    #[serde(rename = "details")]
    pub details: serde_json::Value,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetInterestGroupTrackingResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetInterestGroupAuctionTrackingResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSharedStorageMetadataResult {
    #[serde(rename = "metadata")]
    pub metadata: super::types::SharedStorageMetadata,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSharedStorageEntriesResult {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<super::types::SharedStorageEntry>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSharedStorageEntryResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteSharedStorageEntryResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearSharedStorageEntriesResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResetSharedStorageBudgetResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSharedStorageTrackingResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetStorageBucketTrackingResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteStorageBucketResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunBounceTrackingMitigationsResult {
    #[serde(rename = "deletedSites")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub deleted_sites: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAttributionReportingLocalTestingModeResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAttributionReportingTrackingResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendPendingAttributionReportsResult {
    #[doc = "The number of reports that were sent."]
    #[serde(rename = "numSent")]
    pub num_sent: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRelatedWebsiteSetsResult {
    #[serde(rename = "sets")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sets: Vec<super::types::RelatedWebsiteSet>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAffectedUrlsForThirdPartyCookieMetadataResult {
    #[doc = "Array of matching URLs. If there is a primary pattern match for the first-\nparty URL, only the first-party URL is returned in the array."]
    #[serde(rename = "matchedUrls")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub matched_urls: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetProtectedAudienceKAnonymityResult {}
