use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOsAppStateResult {
    #[serde(rename = "badgeCount")]
    pub badge_count: i64,
    #[serde(rename = "fileHandlers")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_handlers: Vec<super::types::FileHandler>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InstallResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UninstallResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaunchResult {
    #[doc = "ID of the tab target created as a result."]
    #[serde(rename = "targetId")]
    pub target_id: crate::browser_protocol::target::types::TargetId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaunchFilesInAppResult {
    #[doc = "IDs of the tab targets created as the result."]
    #[serde(rename = "targetIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_ids: Vec<crate::browser_protocol::target::types::TargetId>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OpenCurrentPageInAppResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeAppUserSettingsResult {}
