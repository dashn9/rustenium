// Generated commands for module

use serde::{Serialize, Deserialize};
use crate::session::types::ProxyConfiguration;
use crate::session::types::UserPromptHandler;
use crate::EmptyParams;
use crate::EmptyResult;
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowserCommand {
    Close(Close),
    CreateUserContext(CreateUserContext),
    GetClientWindows(GetClientWindows),
    GetUserContexts(GetUserContexts),
    RemoveUserContext(RemoveUserContext),
    SetClientWindowState(SetClientWindowState),
    SetDownloadBehavior(SetDownloadBehavior),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserCloseMethod {
    #[serde(rename = "drivers.close")]
    BrowserClose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserCreateUserContextMethod {
    #[serde(rename = "drivers.createUserContext")]
    BrowserCreateUserContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserGetClientWindowsMethod {
    #[serde(rename = "drivers.getClientWindows")]
    BrowserGetClientWindows,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserGetUserContextsMethod {
    #[serde(rename = "drivers.getUserContexts")]
    BrowserGetUserContexts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserRemoveUserContextMethod {
    #[serde(rename = "drivers.removeUserContext")]
    BrowserRemoveUserContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserSetClientWindowStateMethod {
    #[serde(rename = "drivers.setClientWindowState")]
    BrowserSetClientWindowState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowserSetDownloadBehaviorMethod {
    #[serde(rename = "drivers.setDownloadBehavior")]
    BrowserSetDownloadBehavior,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserContextParameters {
    #[serde(rename = "acceptInsecureCerts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_insecure_certs: Option<bool>,
    #[serde(rename = "proxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<ProxyConfiguration>,
    #[serde(rename = "unhandledPromptBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unhandled_prompt_behavior: Option<UserPromptHandler>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveUserContextParameters {
    #[serde(rename = "userContext")]
    pub user_context: UserContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetClientWindowStateParameters {
    #[serde(rename = "clientWindow")]
    pub client_window: ClientWindow,
    #[serde(flatten)]
    pub client_window_named_state_client_window_rect_state_union: ClientWindowNamedStateClientWindowRectStateUnion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDownloadBehaviorParameters {
    #[serde(rename = "downloadBehavior")]
    pub download_behavior: Option<DownloadBehavior>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Close {
    #[serde(rename = "method")]
    pub method: BrowserCloseMethod,
    #[serde(rename = "params")]
    pub params: EmptyParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserContext {
    #[serde(rename = "method")]
    pub method: BrowserCreateUserContextMethod,
    #[serde(rename = "params")]
    pub params: CreateUserContextParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetClientWindows {
    #[serde(rename = "method")]
    pub method: BrowserGetClientWindowsMethod,
    #[serde(rename = "params")]
    pub params: EmptyParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserContexts {
    #[serde(rename = "method")]
    pub method: BrowserGetUserContextsMethod,
    #[serde(rename = "params")]
    pub params: EmptyParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveUserContext {
    #[serde(rename = "method")]
    pub method: BrowserRemoveUserContextMethod,
    #[serde(rename = "params")]
    pub params: RemoveUserContextParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetClientWindowState {
    #[serde(rename = "method")]
    pub method: BrowserSetClientWindowStateMethod,
    #[serde(rename = "params")]
    pub params: SetClientWindowStateParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDownloadBehavior {
    #[serde(rename = "method")]
    pub method: BrowserSetDownloadBehaviorMethod,
    #[serde(rename = "params")]
    pub params: SetDownloadBehaviorParameters,
}

// Generated results

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowserResult {
    CloseResult(CloseResult),
    CreateUserContextResult(CreateUserContextResult),
    GetClientWindowsResult(GetClientWindowsResult),
    GetUserContextsResult(GetUserContextsResult),
    RemoveUserContextResult(RemoveUserContextResult),
    SetClientWindowStateResult(SetClientWindowStateResult),
    SetDownloadBehaviorResult(SetDownloadBehaviorResult),
}


pub type CloseResult = EmptyResult;


pub type CreateUserContextResult = UserContextInfo;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetClientWindowsResult {
    #[serde(rename = "clientWindows")]
    pub client_windows: Vec<ClientWindowInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserContextsResult {
    #[serde(rename = "userContexts")]
    pub user_contexts: Vec<UserContextInfo>,
}

pub type RemoveUserContextResult = EmptyResult;


pub type SetClientWindowStateResult = ClientWindowInfo;


pub type SetDownloadBehaviorResult = EmptyResult;


