use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStorageKeyForFrameReturns {
    #[serde(rename = "storageKey")]
    pub storage_key: super::types::SerializedStorageKey,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetStorageKeyReturns {
    #[serde(rename = "storageKey")]
    pub storage_key: super::types::SerializedStorageKey,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetCookiesReturns {
    #[doc = "Array of cookie objects."]
    #[serde(rename = "cookies")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cookies: Vec<super::super::network::types::Cookie>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUsageAndQuotaReturns {
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTrustTokensReturns {
    #[serde(rename = "tokens")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tokens: Vec<super::types::TrustTokens>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearTrustTokensReturns {
    #[doc = "True if any tokens were deleted, false otherwise."]
    #[serde(rename = "didDeleteTokens")]
    pub did_delete_tokens: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInterestGroupDetailsReturns {
    #[doc = "This largely corresponds to:\nhttps://wicg.github.io/turtledove/#dictdef-generatebidinterestgroup\nbut has absolute expirationTime instead of relative lifetimeMs and\nalso adds joiningOrigin."]
    #[serde(rename = "details")]
    pub details: serde_json::Value,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSharedStorageMetadataReturns {
    #[serde(rename = "metadata")]
    pub metadata: super::types::SharedStorageMetadata,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSharedStorageEntriesReturns {
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<super::types::SharedStorageEntry>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunBounceTrackingMitigationsReturns {
    #[serde(rename = "deletedSites")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub deleted_sites: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SendPendingAttributionReportsReturns {
    #[doc = "The number of reports that were sent."]
    #[serde(rename = "numSent")]
    pub num_sent: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetRelatedWebsiteSetsReturns {
    #[serde(rename = "sets")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sets: Vec<super::types::RelatedWebsiteSet>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAffectedUrlsForThirdPartyCookieMetadataReturns {
    #[doc = "Array of matching URLs. If there is a primary pattern match for the first-\nparty URL, only the first-party URL is returned in the array."]
    #[serde(rename = "matchedUrls")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub matched_urls: Vec<String>,
}
