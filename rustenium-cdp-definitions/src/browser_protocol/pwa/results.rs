use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOsAppStateResult {
    #[serde(rename = "badgeCount")]
    pub badge_count: i64,
    #[serde(rename = "fileHandlers")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_handlers: Vec<super::types::FileHandler>,
}
impl TryFrom<serde_json::Value> for GetOsAppStateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InstallResult {}
impl TryFrom<serde_json::Value> for InstallResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct UninstallResult {}
impl TryFrom<serde_json::Value> for UninstallResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaunchResult {
    #[doc = "ID of the tab target created as a result."]
    #[serde(rename = "targetId")]
    pub target_id: crate::browser_protocol::target::types::TargetId,
}
impl TryFrom<serde_json::Value> for LaunchResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaunchFilesInAppResult {
    #[doc = "IDs of the tab targets created as the result."]
    #[serde(rename = "targetIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_ids: Vec<crate::browser_protocol::target::types::TargetId>,
}
impl TryFrom<serde_json::Value> for LaunchFilesInAppResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct OpenCurrentPageInAppResult {}
impl TryFrom<serde_json::Value> for OpenCurrentPageInAppResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ChangeAppUserSettingsResult {}
impl TryFrom<serde_json::Value> for ChangeAppUserSettingsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
