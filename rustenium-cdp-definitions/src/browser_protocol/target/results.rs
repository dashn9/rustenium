use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ActivateTargetResult {}
impl TryFrom<serde_json::Value> for ActivateTargetResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachToTargetResult {
    #[doc = "Id assigned to the session."]
    #[serde(rename = "sessionId")]
    pub session_id: super::types::SessionId,
}
impl TryFrom<serde_json::Value> for AttachToTargetResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachToBrowserTargetResult {
    #[doc = "Id assigned to the session."]
    #[serde(rename = "sessionId")]
    pub session_id: super::types::SessionId,
}
impl TryFrom<serde_json::Value> for AttachToBrowserTargetResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CloseTargetResult {}
impl TryFrom<serde_json::Value> for CloseTargetResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExposeDevToolsProtocolResult {}
impl TryFrom<serde_json::Value> for ExposeDevToolsProtocolResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateBrowserContextResult {
    #[doc = "The id of the context created."]
    #[serde(rename = "browserContextId")]
    pub browser_context_id: crate::browser_protocol::browser::types::BrowserContextId,
}
impl TryFrom<serde_json::Value> for CreateBrowserContextResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBrowserContextsResult {
    #[doc = "An array of browser context ids."]
    #[serde(rename = "browserContextIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub browser_context_ids: Vec<crate::browser_protocol::browser::types::BrowserContextId>,
    #[doc = "The id of the default browser context if available."]
    #[serde(rename = "defaultBrowserContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub default_browser_context_id:
        Option<crate::browser_protocol::browser::types::BrowserContextId>,
}
impl TryFrom<serde_json::Value> for GetBrowserContextsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTargetResult {
    #[doc = "The id of the page opened."]
    #[serde(rename = "targetId")]
    pub target_id: super::types::TargetId,
}
impl TryFrom<serde_json::Value> for CreateTargetResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DetachFromTargetResult {}
impl TryFrom<serde_json::Value> for DetachFromTargetResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisposeBrowserContextResult {}
impl TryFrom<serde_json::Value> for DisposeBrowserContextResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTargetInfoResult {
    #[serde(rename = "targetInfo")]
    pub target_info: super::types::TargetInfo,
}
impl TryFrom<serde_json::Value> for GetTargetInfoResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTargetsResult {
    #[doc = "The list of targets."]
    #[serde(rename = "targetInfos")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_infos: Vec<super::types::TargetInfo>,
}
impl TryFrom<serde_json::Value> for GetTargetsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SendMessageToTargetResult {}
impl TryFrom<serde_json::Value> for SendMessageToTargetResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAutoAttachResult {}
impl TryFrom<serde_json::Value> for SetAutoAttachResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AutoAttachRelatedResult {}
impl TryFrom<serde_json::Value> for AutoAttachRelatedResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDiscoverTargetsResult {}
impl TryFrom<serde_json::Value> for SetDiscoverTargetsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetRemoteLocationsResult {}
impl TryFrom<serde_json::Value> for SetRemoteLocationsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetDevToolsTargetResult {
    #[doc = "The targetId of DevTools page target if exists."]
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub target_id: Option<super::types::TargetId>,
}
impl TryFrom<serde_json::Value> for GetDevToolsTargetResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenDevToolsResult {
    #[doc = "The targetId of DevTools page target."]
    #[serde(rename = "targetId")]
    pub target_id: super::types::TargetId,
}
impl TryFrom<serde_json::Value> for OpenDevToolsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
