use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CloseMethod {
    #[serde(rename = "browser.close")]
    Close,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Close {
    pub method: CloseMethod,
    pub params: CloseParams,
}
impl Close {
    pub const IDENTIFIER: &'static str = "browser.close";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
impl crate::CommandResult for Close {
    type Result = super::results::CloseResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CreateUserContextParams {
    #[serde(rename = "acceptInsecureCerts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub accept_insecure_certs: Option<bool>,
    #[serde(rename = "proxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub proxy: Option<crate::session::types::ProxyConfiguration>,
    #[serde(rename = "unhandledPromptBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub unhandled_prompt_behavior: Option<crate::session::types::UserPromptHandler>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateUserContextMethod {
    #[serde(rename = "browser.createUserContext")]
    CreateUserContext,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateUserContext {
    pub method: CreateUserContextMethod,
    pub params: CreateUserContextParams,
}
impl CreateUserContext {
    pub const IDENTIFIER: &'static str = "browser.createUserContext";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
impl crate::CommandResult for CreateUserContext {
    type Result = super::results::CreateUserContextResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetClientWindowsParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetClientWindowsMethod {
    #[serde(rename = "browser.getClientWindows")]
    GetClientWindows,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetClientWindows {
    pub method: GetClientWindowsMethod,
    pub params: GetClientWindowsParams,
}
impl GetClientWindows {
    pub const IDENTIFIER: &'static str = "browser.getClientWindows";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
impl crate::CommandResult for GetClientWindows {
    type Result = super::results::GetClientWindowsResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserContextsParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetUserContextsMethod {
    #[serde(rename = "browser.getUserContexts")]
    GetUserContexts,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetUserContexts {
    pub method: GetUserContextsMethod,
    pub params: GetUserContextsParams,
}
impl GetUserContexts {
    pub const IDENTIFIER: &'static str = "browser.getUserContexts";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
impl crate::CommandResult for GetUserContexts {
    type Result = super::results::GetUserContextsResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveUserContextParams {
    #[serde(rename = "userContext")]
    pub user_context: super::types::UserContext,
}
impl RemoveUserContextParams {
    pub fn new(user_context: impl Into<super::types::UserContext>) -> Self {
        Self {
            user_context: user_context.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveUserContextMethod {
    #[serde(rename = "browser.removeUserContext")]
    RemoveUserContext,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveUserContext {
    pub method: RemoveUserContextMethod,
    pub params: RemoveUserContextParams,
}
impl RemoveUserContext {
    pub const IDENTIFIER: &'static str = "browser.removeUserContext";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
impl crate::CommandResult for RemoveUserContext {
    type Result = super::results::RemoveUserContextResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetClientWindowStateParams {
    #[serde(rename = "clientWindow")]
    pub client_window: super::types::ClientWindow,
    #[serde(flatten)]
    #[serde(default)]
    pub client_window_named_state_client_window_rect_state_union:
        super::types::ClientWindowNamedStateClientWindowRectStateUnion,
}
impl SetClientWindowStateParams {
    pub fn new(
        client_window: impl Into<super::types::ClientWindow>,
        client_window_named_state_client_window_rect_state_union: impl Into<
            super::types::ClientWindowNamedStateClientWindowRectStateUnion,
        >,
    ) -> Self {
        Self {
            client_window: client_window.into(),
            client_window_named_state_client_window_rect_state_union:
                client_window_named_state_client_window_rect_state_union.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetClientWindowStateMethod {
    #[serde(rename = "browser.setClientWindowState")]
    SetClientWindowState,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetClientWindowState {
    pub method: SetClientWindowStateMethod,
    pub params: SetClientWindowStateParams,
}
impl SetClientWindowState {
    pub const IDENTIFIER: &'static str = "browser.setClientWindowState";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
impl crate::CommandResult for SetClientWindowState {
    type Result = super::results::SetClientWindowStateResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDownloadBehaviorParams {
    #[serde(rename = "downloadBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub download_behavior: Option<super::types::DownloadBehavior>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_contexts: Option<Vec<super::types::UserContext>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDownloadBehaviorMethod {
    #[serde(rename = "browser.setDownloadBehavior")]
    SetDownloadBehavior,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDownloadBehavior {
    pub method: SetDownloadBehaviorMethod,
    pub params: SetDownloadBehaviorParams,
}
impl SetDownloadBehavior {
    pub const IDENTIFIER: &'static str = "browser.setDownloadBehavior";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
impl crate::CommandResult for SetDownloadBehavior {
    type Result = super::results::SetDownloadBehaviorResult;
}
group_enum ! (BrowserCommand { Close (Close) , CreateUserContext (CreateUserContext) , GetClientWindows (GetClientWindows) , GetUserContexts (GetUserContexts) , RemoveUserContext (RemoveUserContext) , SetClientWindowState (SetClientWindowState) , SetDownloadBehavior (SetDownloadBehavior) });
