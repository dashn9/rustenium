use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ActivateTargetResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachToTargetResult {
    #[doc = "Id assigned to the session."]
    #[serde(rename = "sessionId")]
    pub session_id: super::types::SessionId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachToBrowserTargetResult {
    #[doc = "Id assigned to the session."]
    #[serde(rename = "sessionId")]
    pub session_id: super::types::SessionId,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CloseTargetResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExposeDevToolsProtocolResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateBrowserContextResult {
    #[doc = "The id of the context created."]
    #[serde(rename = "browserContextId")]
    pub browser_context_id: crate::browser_protocol::browser::types::BrowserContextId,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateTargetResult {
    #[doc = "The id of the page opened."]
    #[serde(rename = "targetId")]
    pub target_id: super::types::TargetId,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DetachFromTargetResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisposeBrowserContextResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTargetInfoResult {
    #[serde(rename = "targetInfo")]
    pub target_info: super::types::TargetInfo,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetTargetsResult {
    #[doc = "The list of targets."]
    #[serde(rename = "targetInfos")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_infos: Vec<super::types::TargetInfo>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SendMessageToTargetResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAutoAttachResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AutoAttachRelatedResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDiscoverTargetsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetRemoteLocationsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetDevToolsTargetResult {
    #[doc = "The targetId of DevTools page target if exists."]
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub target_id: Option<super::types::TargetId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenDevToolsResult {
    #[doc = "The targetId of DevTools page target."]
    #[serde(rename = "targetId")]
    pub target_id: super::types::TargetId,
}
