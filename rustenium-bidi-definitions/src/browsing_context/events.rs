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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextCreated {
    pub method: ContextCreatedMethod,
    pub params: ContextCreatedParams,
}
impl ContextCreated {
    pub const IDENTIFIER: &'static str = "browsingContext.contextCreated";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextDestroyed {
    pub method: ContextDestroyedMethod,
    pub params: ContextDestroyedParams,
}
impl ContextDestroyed {
    pub const IDENTIFIER: &'static str = "browsingContext.contextDestroyed";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationStartedParams {
    #[serde(flatten)]
    #[serde(default)]
    pub base_navigation_info: super::types::BaseNavigationInfo,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigationStartedMethod {
    #[serde(rename = "browsingContext.navigationStarted")]
    NavigationStarted,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationStarted {
    pub method: NavigationStartedMethod,
    pub params: NavigationStartedParams,
}
impl NavigationStarted {
    pub const IDENTIFIER: &'static str = "browsingContext.navigationStarted";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FragmentNavigatedParams {
    #[serde(flatten)]
    #[serde(default)]
    pub base_navigation_info: super::types::BaseNavigationInfo,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FragmentNavigatedMethod {
    #[serde(rename = "browsingContext.fragmentNavigated")]
    FragmentNavigated,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FragmentNavigated {
    pub method: FragmentNavigatedMethod,
    pub params: FragmentNavigatedParams,
}
impl FragmentNavigated {
    pub const IDENTIFIER: &'static str = "browsingContext.fragmentNavigated";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HistoryUpdated {
    pub method: HistoryUpdatedMethod,
    pub params: HistoryUpdatedParams,
}
impl HistoryUpdated {
    pub const IDENTIFIER: &'static str = "browsingContext.historyUpdated";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomContentLoadedParams {
    #[serde(flatten)]
    #[serde(default)]
    pub base_navigation_info: super::types::BaseNavigationInfo,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomContentLoadedMethod {
    #[serde(rename = "browsingContext.domContentLoaded")]
    DomContentLoaded,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomContentLoaded {
    pub method: DomContentLoadedMethod,
    pub params: DomContentLoadedParams,
}
impl DomContentLoaded {
    pub const IDENTIFIER: &'static str = "browsingContext.domContentLoaded";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadParams {
    #[serde(flatten)]
    #[serde(default)]
    pub base_navigation_info: super::types::BaseNavigationInfo,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoadMethod {
    #[serde(rename = "browsingContext.load")]
    Load,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Load {
    pub method: LoadMethod,
    pub params: LoadParams,
}
impl Load {
    pub const IDENTIFIER: &'static str = "browsingContext.load";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadWillBeginParams {
    #[serde(rename = "suggestedFilename")]
    pub suggested_filename: String,
    #[serde(flatten)]
    #[serde(default)]
    pub base_navigation_info: super::types::BaseNavigationInfo,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DownloadWillBeginMethod {
    #[serde(rename = "browsingContext.downloadWillBegin")]
    DownloadWillBegin,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadWillBegin {
    pub method: DownloadWillBeginMethod,
    pub params: DownloadWillBeginParams,
}
impl DownloadWillBegin {
    pub const IDENTIFIER: &'static str = "browsingContext.downloadWillBegin";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadEndParams {
    #[serde(flatten)]
    #[serde(default)]
    pub download_canceled_params_download_complete_params_union:
        super::types::DownloadCanceledParamsDownloadCompleteParamsUnion,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DownloadEndMethod {
    #[serde(rename = "browsingContext.downloadEnd")]
    DownloadEnd,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadEnd {
    pub method: DownloadEndMethod,
    pub params: DownloadEndParams,
}
impl DownloadEnd {
    pub const IDENTIFIER: &'static str = "browsingContext.downloadEnd";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationAbortedParams {
    #[serde(flatten)]
    #[serde(default)]
    pub base_navigation_info: super::types::BaseNavigationInfo,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigationAbortedMethod {
    #[serde(rename = "browsingContext.navigationAborted")]
    NavigationAborted,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationAborted {
    pub method: NavigationAbortedMethod,
    pub params: NavigationAbortedParams,
}
impl NavigationAborted {
    pub const IDENTIFIER: &'static str = "browsingContext.navigationAborted";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationCommittedParams {
    #[serde(flatten)]
    #[serde(default)]
    pub base_navigation_info: super::types::BaseNavigationInfo,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigationCommittedMethod {
    #[serde(rename = "browsingContext.navigationCommitted")]
    NavigationCommitted,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationCommitted {
    pub method: NavigationCommittedMethod,
    pub params: NavigationCommittedParams,
}
impl NavigationCommitted {
    pub const IDENTIFIER: &'static str = "browsingContext.navigationCommitted";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationFailedParams {
    #[serde(flatten)]
    #[serde(default)]
    pub base_navigation_info: super::types::BaseNavigationInfo,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigationFailedMethod {
    #[serde(rename = "browsingContext.navigationFailed")]
    NavigationFailed,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigationFailed {
    pub method: NavigationFailedMethod,
    pub params: NavigationFailedParams,
}
impl NavigationFailed {
    pub const IDENTIFIER: &'static str = "browsingContext.navigationFailed";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPromptClosed {
    pub method: UserPromptClosedMethod,
    pub params: UserPromptClosedParams,
}
impl UserPromptClosed {
    pub const IDENTIFIER: &'static str = "browsingContext.userPromptClosed";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserPromptOpened {
    pub method: UserPromptOpenedMethod,
    pub params: UserPromptOpenedParams,
}
impl UserPromptOpened {
    pub const IDENTIFIER: &'static str = "browsingContext.userPromptOpened";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (BrowsingContextEvent { ContextCreated (ContextCreated) , ContextDestroyed (ContextDestroyed) , NavigationStarted (NavigationStarted) , FragmentNavigated (FragmentNavigated) , HistoryUpdated (HistoryUpdated) , DomContentLoaded (DomContentLoaded) , Load (Load) , DownloadWillBegin (DownloadWillBegin) , DownloadEnd (DownloadEnd) , NavigationAborted (NavigationAborted) , NavigationCommitted (NavigationCommitted) , NavigationFailed (NavigationFailed) , UserPromptClosed (UserPromptClosed) , UserPromptOpened (UserPromptOpened) });
