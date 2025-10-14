// Generated commands for module

use serde::{Serialize, Deserialize};
use crate::session::types::ProxyConfiguration;
use crate::session::types::UserPromptHandler;
use crate::Extensible;
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
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowserCloseMethod {
    #[serde(rename = "browser.Close")]
    BrowserClose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowserCreateUserContextMethod {
    #[serde(rename = "browser.CreateUserContext")]
    BrowserCreateUserContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowserGetClientWindowsMethod {
    #[serde(rename = "browser.GetClientWindows")]
    BrowserGetClientWindows,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowserGetUserContextsMethod {
    #[serde(rename = "browser.GetUserContexts")]
    BrowserGetUserContexts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowserRemoveUserContextMethod {
    #[serde(rename = "browser.RemoveUserContext")]
    BrowserRemoveUserContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowserSetClientWindowStateMethod {
    #[serde(rename = "browser.SetClientWindowState")]
    BrowserSetClientWindowState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmptyParams {
    Extensible(Extensible),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserContextParameters {
    #[serde(rename = "acceptInsecureCerts")]
    pub accept_insecure_certs: Option<bool>,
    #[serde(rename = "proxy")]
    pub proxy: Option<ProxyConfiguration>,
    #[serde(rename = "unhandledPromptBehavior")]
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

// Generated results

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowserResult {
    CreateUserContextResult(CreateUserContextResult),
    GetUserContextsResult(GetUserContextsResult),
}


pub type CreateUserContextResult = UserContextInfo;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserContextsResult {
    #[serde(rename = "userContexts")]
    pub user_contexts: Vec<UserContextInfo>,
}

