use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextCreatedParams {
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub children: Option<super::types::InfoList>,
    #[serde(rename = "clientWindow")]
    pub client_window: crate::browser::types::ClientWindow,
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "originalOpener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub original_opener: Option<super::types::BrowsingContext>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "userContext")]
    pub user_context: crate::browser::types::UserContext,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent: Option<super::types::BrowsingContext>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContextCreatedMethod {
    #[serde(rename = "browsingContext.contextCreated")]
    ContextCreated,
}
impl ContextCreatedMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.contextCreated";
}
#[derive(Debug, Clone, PartialEq)]
pub struct ContextCreated {
    pub method: ContextCreatedMethod,
    pub params: ContextCreatedParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextDestroyedParams {
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub children: Option<super::types::InfoList>,
    #[serde(rename = "clientWindow")]
    pub client_window: crate::browser::types::ClientWindow,
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "originalOpener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub original_opener: Option<super::types::BrowsingContext>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "userContext")]
    pub user_context: crate::browser::types::UserContext,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent: Option<super::types::BrowsingContext>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContextDestroyedMethod {
    #[serde(rename = "browsingContext.contextDestroyed")]
    ContextDestroyed,
}
impl ContextDestroyedMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.contextDestroyed";
}
#[derive(Debug, Clone, PartialEq)]
pub struct ContextDestroyed {
    pub method: ContextDestroyedMethod,
    pub params: ContextDestroyedParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationStartedParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<super::types::Navigation>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigationStartedMethod {
    #[serde(rename = "browsingContext.navigationStarted")]
    NavigationStarted,
}
impl NavigationStartedMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.navigationStarted";
}
#[derive(Debug, Clone, PartialEq)]
pub struct NavigationStarted {
    pub method: NavigationStartedMethod,
    pub params: NavigationStartedParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FragmentNavigatedParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<super::types::Navigation>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FragmentNavigatedMethod {
    #[serde(rename = "browsingContext.fragmentNavigated")]
    FragmentNavigated,
}
impl FragmentNavigatedMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.fragmentNavigated";
}
#[derive(Debug, Clone, PartialEq)]
pub struct FragmentNavigated {
    pub method: FragmentNavigatedMethod,
    pub params: FragmentNavigatedParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryUpdatedParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HistoryUpdatedMethod {
    #[serde(rename = "browsingContext.historyUpdated")]
    HistoryUpdated,
}
impl HistoryUpdatedMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.historyUpdated";
}
#[derive(Debug, Clone, PartialEq)]
pub struct HistoryUpdated {
    pub method: HistoryUpdatedMethod,
    pub params: HistoryUpdatedParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomContentLoadedParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<super::types::Navigation>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomContentLoadedMethod {
    #[serde(rename = "browsingContext.domContentLoaded")]
    DomContentLoaded,
}
impl DomContentLoadedMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.domContentLoaded";
}
#[derive(Debug, Clone, PartialEq)]
pub struct DomContentLoaded {
    pub method: DomContentLoadedMethod,
    pub params: DomContentLoadedParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<super::types::Navigation>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoadMethod {
    #[serde(rename = "browsingContext.load")]
    Load,
}
impl LoadMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.load";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Load {
    pub method: LoadMethod,
    pub params: LoadParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadWillBeginParams {
    #[serde(rename = "suggestedFilename")]
    pub suggested_filename: String,
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<super::types::Navigation>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DownloadWillBeginMethod {
    #[serde(rename = "browsingContext.downloadWillBegin")]
    DownloadWillBegin,
}
impl DownloadWillBeginMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.downloadWillBegin";
}
#[derive(Debug, Clone, PartialEq)]
pub struct DownloadWillBegin {
    pub method: DownloadWillBeginMethod,
    pub params: DownloadWillBeginParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadEndParams {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "filepath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub filepath: Option<String>,
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<super::types::Navigation>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DownloadEndMethod {
    #[serde(rename = "browsingContext.downloadEnd")]
    DownloadEnd,
}
impl DownloadEndMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.downloadEnd";
}
#[derive(Debug, Clone, PartialEq)]
pub struct DownloadEnd {
    pub method: DownloadEndMethod,
    pub params: DownloadEndParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationAbortedParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<super::types::Navigation>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigationAbortedMethod {
    #[serde(rename = "browsingContext.navigationAborted")]
    NavigationAborted,
}
impl NavigationAbortedMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.navigationAborted";
}
#[derive(Debug, Clone, PartialEq)]
pub struct NavigationAborted {
    pub method: NavigationAbortedMethod,
    pub params: NavigationAbortedParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationCommittedParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<super::types::Navigation>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigationCommittedMethod {
    #[serde(rename = "browsingContext.navigationCommitted")]
    NavigationCommitted,
}
impl NavigationCommittedMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.navigationCommitted";
}
#[derive(Debug, Clone, PartialEq)]
pub struct NavigationCommitted {
    pub method: NavigationCommittedMethod,
    pub params: NavigationCommittedParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationFailedParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<super::types::Navigation>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigationFailedMethod {
    #[serde(rename = "browsingContext.navigationFailed")]
    NavigationFailed,
}
impl NavigationFailedMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.navigationFailed";
}
#[derive(Debug, Clone, PartialEq)]
pub struct NavigationFailed {
    pub method: NavigationFailedMethod,
    pub params: NavigationFailedParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPromptClosedParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "accepted")]
    pub accepted: bool,
    #[serde(rename = "type")]
    pub r#type: super::types::UserPromptType,
    #[serde(rename = "userText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_text: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserPromptClosedMethod {
    #[serde(rename = "browsingContext.userPromptClosed")]
    UserPromptClosed,
}
impl UserPromptClosedMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.userPromptClosed";
}
#[derive(Debug, Clone, PartialEq)]
pub struct UserPromptClosed {
    pub method: UserPromptClosedMethod,
    pub params: UserPromptClosedParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPromptOpenedParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "handler")]
    pub handler: crate::session::types::UserPromptHandlerType,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "type")]
    pub r#type: super::types::UserPromptType,
    #[serde(rename = "defaultValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub default_value: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UserPromptOpenedMethod {
    #[serde(rename = "browsingContext.userPromptOpened")]
    UserPromptOpened,
}
impl UserPromptOpenedMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.userPromptOpened";
}
#[derive(Debug, Clone, PartialEq)]
pub struct UserPromptOpened {
    pub method: UserPromptOpenedMethod,
    pub params: UserPromptOpenedParams,
}
group_enum ! (BrowsingContextEvents { ContextCreated (ContextCreated) , ContextDestroyed (ContextDestroyed) , NavigationStarted (NavigationStarted) , FragmentNavigated (FragmentNavigated) , HistoryUpdated (HistoryUpdated) , DomContentLoaded (DomContentLoaded) , Load (Load) , DownloadWillBegin (DownloadWillBegin) , DownloadEnd (DownloadEnd) , NavigationAborted (NavigationAborted) , NavigationCommitted (NavigationCommitted) , NavigationFailed (NavigationFailed) , UserPromptClosed (UserPromptClosed) , UserPromptOpened (UserPromptOpened) });
