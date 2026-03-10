use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStorageKeyForFrameResult {
    #[serde(rename = "storageKey")]
    pub storage_key: super::types::SerializedStorageKey,
}
impl TryFrom<serde_json::Value> for GetStorageKeyForFrameResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStorageKeyResult {
    #[serde(rename = "storageKey")]
    pub storage_key: super::types::SerializedStorageKey,
}
impl TryFrom<serde_json::Value> for GetStorageKeyResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearDataForOriginResult {}
impl TryFrom<serde_json::Value> for ClearDataForOriginResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearDataForStorageKeyResult {}
impl TryFrom<serde_json::Value> for ClearDataForStorageKeyResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCookiesResult {
    #[doc = "Array of cookie objects."]
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookies: Vec<crate::browser_protocol::network::types::Cookie>,
}
impl TryFrom<serde_json::Value> for GetCookiesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetCookiesResult {}
impl TryFrom<serde_json::Value> for SetCookiesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearCookiesResult {}
impl TryFrom<serde_json::Value> for ClearCookiesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
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
impl TryFrom<serde_json::Value> for GetUsageAndQuotaResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OverrideQuotaForOriginResult {}
impl TryFrom<serde_json::Value> for OverrideQuotaForOriginResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TrackCacheStorageForOriginResult {}
impl TryFrom<serde_json::Value> for TrackCacheStorageForOriginResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TrackCacheStorageForStorageKeyResult {}
impl TryFrom<serde_json::Value> for TrackCacheStorageForStorageKeyResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TrackIndexedDbForOriginResult {}
impl TryFrom<serde_json::Value> for TrackIndexedDbForOriginResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TrackIndexedDbForStorageKeyResult {}
impl TryFrom<serde_json::Value> for TrackIndexedDbForStorageKeyResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UntrackCacheStorageForOriginResult {}
impl TryFrom<serde_json::Value> for UntrackCacheStorageForOriginResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UntrackCacheStorageForStorageKeyResult {}
impl TryFrom<serde_json::Value> for UntrackCacheStorageForStorageKeyResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UntrackIndexedDbForOriginResult {}
impl TryFrom<serde_json::Value> for UntrackIndexedDbForOriginResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UntrackIndexedDbForStorageKeyResult {}
impl TryFrom<serde_json::Value> for UntrackIndexedDbForStorageKeyResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTrustTokensResult {
    #[serde(rename = "tokens")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tokens: Vec<super::types::TrustTokens>,
}
impl TryFrom<serde_json::Value> for GetTrustTokensResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearTrustTokensResult {
    #[doc = "True if any tokens were deleted, false otherwise."]
    #[serde(rename = "didDeleteTokens")]
    pub did_delete_tokens: bool,
}
impl TryFrom<serde_json::Value> for ClearTrustTokensResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInterestGroupDetailsResult {
    #[doc = "This largely corresponds to:\nhttps://wicg.github.io/turtledove/#dictdef-generatebidinterestgroup\nbut has absolute expirationTime instead of relative lifetimeMs and\nalso adds joiningOrigin."]
    #[serde(rename = "details")]
    pub details: serde_json::Value,
}
impl TryFrom<serde_json::Value> for GetInterestGroupDetailsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetInterestGroupTrackingResult {}
impl TryFrom<serde_json::Value> for SetInterestGroupTrackingResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetInterestGroupAuctionTrackingResult {}
impl TryFrom<serde_json::Value> for SetInterestGroupAuctionTrackingResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSharedStorageMetadataResult {
    #[serde(rename = "metadata")]
    pub metadata: super::types::SharedStorageMetadata,
}
impl TryFrom<serde_json::Value> for GetSharedStorageMetadataResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSharedStorageEntriesResult {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<super::types::SharedStorageEntry>,
}
impl TryFrom<serde_json::Value> for GetSharedStorageEntriesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSharedStorageEntryResult {}
impl TryFrom<serde_json::Value> for SetSharedStorageEntryResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteSharedStorageEntryResult {}
impl TryFrom<serde_json::Value> for DeleteSharedStorageEntryResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearSharedStorageEntriesResult {}
impl TryFrom<serde_json::Value> for ClearSharedStorageEntriesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResetSharedStorageBudgetResult {}
impl TryFrom<serde_json::Value> for ResetSharedStorageBudgetResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSharedStorageTrackingResult {}
impl TryFrom<serde_json::Value> for SetSharedStorageTrackingResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetStorageBucketTrackingResult {}
impl TryFrom<serde_json::Value> for SetStorageBucketTrackingResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteStorageBucketResult {}
impl TryFrom<serde_json::Value> for DeleteStorageBucketResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunBounceTrackingMitigationsResult {
    #[serde(rename = "deletedSites")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub deleted_sites: Vec<String>,
}
impl TryFrom<serde_json::Value> for RunBounceTrackingMitigationsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAttributionReportingLocalTestingModeResult {}
impl TryFrom<serde_json::Value> for SetAttributionReportingLocalTestingModeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAttributionReportingTrackingResult {}
impl TryFrom<serde_json::Value> for SetAttributionReportingTrackingResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendPendingAttributionReportsResult {
    #[doc = "The number of reports that were sent."]
    #[serde(rename = "numSent")]
    pub num_sent: i64,
}
impl TryFrom<serde_json::Value> for SendPendingAttributionReportsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRelatedWebsiteSetsResult {
    #[serde(rename = "sets")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sets: Vec<super::types::RelatedWebsiteSet>,
}
impl TryFrom<serde_json::Value> for GetRelatedWebsiteSetsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAffectedUrlsForThirdPartyCookieMetadataResult {
    #[doc = "Array of matching URLs. If there is a primary pattern match for the first-\nparty URL, only the first-party URL is returned in the array."]
    #[serde(rename = "matchedUrls")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub matched_urls: Vec<String>,
}
impl TryFrom<serde_json::Value> for GetAffectedUrlsForThirdPartyCookieMetadataResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetProtectedAudienceKAnonymityResult {}
impl TryFrom<serde_json::Value> for SetProtectedAudienceKAnonymityResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
