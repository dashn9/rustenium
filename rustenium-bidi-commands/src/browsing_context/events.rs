// Generated events for module

use serde::{Serialize, Deserialize};
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextEvent {
    ContextCreated(ContextCreated),
    ContextDestroyed(ContextDestroyed),
    DomContentLoaded(DomContentLoaded),
    DownloadEnd(DownloadEnd),
    DownloadWillBegin(DownloadWillBegin),
    FragmentNavigated(FragmentNavigated),
    HistoryUpdated(HistoryUpdated),
    Load(Load),
    NavigationAborted(NavigationAborted),
    NavigationCommitted(NavigationCommitted),
    NavigationFailed(NavigationFailed),
    NavigationStarted(NavigationStarted),
    UserPromptClosed(UserPromptClosed),
    UserPromptOpened(UserPromptOpened),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextContextCreatedMethod {
    #[serde(rename = "browsingContext.ContextCreated")]
    BrowsingContextContextCreated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextContextDestroyedMethod {
    #[serde(rename = "browsingContext.ContextDestroyed")]
    BrowsingContextContextDestroyed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextDomContentLoadedMethod {
    #[serde(rename = "browsingContext.DomContentLoaded")]
    BrowsingContextDomContentLoaded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextDownloadEndMethod {
    #[serde(rename = "browsingContext.DownloadEnd")]
    BrowsingContextDownloadEnd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextDownloadWillBeginMethod {
    #[serde(rename = "browsingContext.DownloadWillBegin")]
    BrowsingContextDownloadWillBegin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextFragmentNavigatedMethod {
    #[serde(rename = "browsingContext.FragmentNavigated")]
    BrowsingContextFragmentNavigated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextHistoryUpdatedMethod {
    #[serde(rename = "browsingContext.HistoryUpdated")]
    BrowsingContextHistoryUpdated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextLoadMethod {
    #[serde(rename = "browsingContext.Load")]
    BrowsingContextLoad,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextNavigationAbortedMethod {
    #[serde(rename = "browsingContext.NavigationAborted")]
    BrowsingContextNavigationAborted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextNavigationCommittedMethod {
    #[serde(rename = "browsingContext.NavigationCommitted")]
    BrowsingContextNavigationCommitted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextNavigationFailedMethod {
    #[serde(rename = "browsingContext.NavigationFailed")]
    BrowsingContextNavigationFailed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextNavigationStartedMethod {
    #[serde(rename = "browsingContext.NavigationStarted")]
    BrowsingContextNavigationStarted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextUserPromptClosedMethod {
    #[serde(rename = "browsingContext.UserPromptClosed")]
    BrowsingContextUserPromptClosed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextUserPromptOpenedMethod {
    #[serde(rename = "browsingContext.UserPromptOpened")]
    BrowsingContextUserPromptOpened,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadEndParams {
    #[serde(flatten)]
    pub download_canceled_params_download_complete_params_union: DownloadCanceledParamsDownloadCompleteParamsUnion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadWillBeginParams {
    #[serde(rename = "suggestedFilename")]
    pub suggested_filename: String,
    #[serde(flatten)]
    pub base_navigation_info: BaseNavigationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextCreated {
    #[serde(rename = "method")]
    pub method: BrowsingContextContextCreatedMethod,
    #[serde(rename = "params")]
    pub params: Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextDestroyed {
    #[serde(rename = "method")]
    pub method: BrowsingContextContextDestroyedMethod,
    #[serde(rename = "params")]
    pub params: Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomContentLoaded {
    #[serde(rename = "method")]
    pub method: BrowsingContextDomContentLoadedMethod,
    #[serde(rename = "params")]
    pub params: NavigationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadEnd {
    #[serde(rename = "method")]
    pub method: BrowsingContextDownloadEndMethod,
    #[serde(rename = "params")]
    pub params: DownloadEndParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadWillBegin {
    #[serde(rename = "method")]
    pub method: BrowsingContextDownloadWillBeginMethod,
    #[serde(rename = "params")]
    pub params: DownloadWillBeginParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FragmentNavigated {
    #[serde(rename = "method")]
    pub method: BrowsingContextFragmentNavigatedMethod,
    #[serde(rename = "params")]
    pub params: NavigationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryUpdated {
    #[serde(rename = "method")]
    pub method: BrowsingContextHistoryUpdatedMethod,
    #[serde(rename = "params")]
    pub params: HistoryUpdatedParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Load {
    #[serde(rename = "method")]
    pub method: BrowsingContextLoadMethod,
    #[serde(rename = "params")]
    pub params: NavigationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationAborted {
    #[serde(rename = "method")]
    pub method: BrowsingContextNavigationAbortedMethod,
    #[serde(rename = "params")]
    pub params: NavigationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationCommitted {
    #[serde(rename = "method")]
    pub method: BrowsingContextNavigationCommittedMethod,
    #[serde(rename = "params")]
    pub params: NavigationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationFailed {
    #[serde(rename = "method")]
    pub method: BrowsingContextNavigationFailedMethod,
    #[serde(rename = "params")]
    pub params: NavigationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationStarted {
    #[serde(rename = "method")]
    pub method: BrowsingContextNavigationStartedMethod,
    #[serde(rename = "params")]
    pub params: NavigationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPromptClosed {
    #[serde(rename = "method")]
    pub method: BrowsingContextUserPromptClosedMethod,
    #[serde(rename = "params")]
    pub params: UserPromptClosedParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPromptOpened {
    #[serde(rename = "method")]
    pub method: BrowsingContextUserPromptOpenedMethod,
    #[serde(rename = "params")]
    pub params: UserPromptOpenedParameters,
}

