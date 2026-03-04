use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOsAppStateReturns {
    #[serde(rename = "badgeCount")]
    pub badge_count: i64,
    #[serde(rename = "fileHandlers")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_handlers: Vec<super::types::FileHandler>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaunchReturns {
    #[doc = "ID of the tab target created as a result."]
    #[serde(rename = "targetId")]
    pub target_id: super::super::target::types::TargetId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaunchFilesInAppReturns {
    #[doc = "IDs of the tab targets created as the result."]
    #[serde(rename = "targetIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_ids: Vec<super::super::target::types::TargetId>,
}
