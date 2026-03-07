use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomContentEventFiredParams {
    #[serde(rename = "timestamp")]
    pub timestamp: crate::browser_protocol::network::types::MonotonicTime,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DomContentEventFiredMethod {
    #[serde(rename = "Page.domContentEventFired")]
    DomContentEventFired,
}
impl DomContentEventFiredMethod {
    pub const IDENTIFIER: &'static str = "Page.domContentEventFired";
}
#[derive(Debug, Clone, PartialEq)]
pub struct DomContentEventFired {
    pub method: DomContentEventFiredMethod,
    pub params: DomContentEventFiredParams,
}
#[doc = "Emitted only when `page.interceptFileChooser` is enabled.\n[fileChooserOpened](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-fileChooserOpened)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileChooserOpenedParams {
    #[doc = "Id of the frame containing input node."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[doc = "Input mode."]
    #[serde(rename = "mode")]
    pub mode: FileChooserOpenedMode,
    #[doc = "Input node id. Only present for file choosers opened via an `<input type=\"file\">` element."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
}
#[doc = "Input mode."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileChooserOpenedMode {
    #[serde(rename = "selectSingle")]
    SelectSingle,
    #[serde(rename = "selectMultiple")]
    SelectMultiple,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FileChooserOpenedMethod {
    #[serde(rename = "Page.fileChooserOpened")]
    FileChooserOpened,
}
impl FileChooserOpenedMethod {
    pub const IDENTIFIER: &'static str = "Page.fileChooserOpened";
}
#[doc = "Emitted only when `page.interceptFileChooser` is enabled.\n[fileChooserOpened](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-fileChooserOpened)"]
#[derive(Debug, Clone, PartialEq)]
pub struct FileChooserOpened {
    pub method: FileChooserOpenedMethod,
    pub params: FileChooserOpenedParams,
}
#[doc = "Fired when frame has been attached to its parent.\n[frameAttached](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameAttached)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameAttachedParams {
    #[doc = "Id of the frame that has been attached."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[doc = "Parent frame identifier."]
    #[serde(rename = "parentFrameId")]
    pub parent_frame_id: super::types::FrameId,
    #[doc = "JavaScript stack trace of when frame was attached, only set if frame initiated from script."]
    #[serde(rename = "stack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack: Option<crate::js_protocol::runtime::types::StackTrace>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FrameAttachedMethod {
    #[serde(rename = "Page.frameAttached")]
    FrameAttached,
}
impl FrameAttachedMethod {
    pub const IDENTIFIER: &'static str = "Page.frameAttached";
}
#[doc = "Fired when frame has been attached to its parent.\n[frameAttached](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameAttached)"]
#[derive(Debug, Clone, PartialEq)]
pub struct FrameAttached {
    pub method: FrameAttachedMethod,
    pub params: FrameAttachedParams,
}
#[doc = "Fired when frame has been detached from its parent.\n[frameDetached](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameDetached)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameDetachedParams {
    #[doc = "Id of the frame that has been detached."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[serde(rename = "reason")]
    pub reason: FrameDetachedReason,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FrameDetachedReason {
    #[doc = "The frame is removed from the DOM."]
    #[serde(rename = "remove")]
    Remove,
    #[doc = "The frame is being swapped out in favor of an out-of-process iframe.\nA new frame target will be created (see Target.attachedToTarget)."]
    #[serde(rename = "swap")]
    Swap,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FrameDetachedMethod {
    #[serde(rename = "Page.frameDetached")]
    FrameDetached,
}
impl FrameDetachedMethod {
    pub const IDENTIFIER: &'static str = "Page.frameDetached";
}
#[doc = "Fired when frame has been detached from its parent.\n[frameDetached](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameDetached)"]
#[derive(Debug, Clone, PartialEq)]
pub struct FrameDetached {
    pub method: FrameDetachedMethod,
    pub params: FrameDetachedParams,
}
#[doc = "Fired before frame subtree is detached. Emitted before any frame of the\nsubtree is actually detached.\n[frameSubtreeWillBeDetached](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameSubtreeWillBeDetached)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameSubtreeWillBeDetachedParams {
    #[doc = "Id of the frame that is the root of the subtree that will be detached."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FrameSubtreeWillBeDetachedMethod {
    #[serde(rename = "Page.frameSubtreeWillBeDetached")]
    FrameSubtreeWillBeDetached,
}
impl FrameSubtreeWillBeDetachedMethod {
    pub const IDENTIFIER: &'static str = "Page.frameSubtreeWillBeDetached";
}
#[doc = "Fired before frame subtree is detached. Emitted before any frame of the\nsubtree is actually detached.\n[frameSubtreeWillBeDetached](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameSubtreeWillBeDetached)"]
#[derive(Debug, Clone, PartialEq)]
pub struct FrameSubtreeWillBeDetached {
    pub method: FrameSubtreeWillBeDetachedMethod,
    pub params: FrameSubtreeWillBeDetachedParams,
}
#[doc = "Fired once navigation of the frame has completed. Frame is now associated with the new loader.\n[frameNavigated](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameNavigated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameNavigatedParams {
    #[doc = "Frame object."]
    #[serde(rename = "frame")]
    pub frame: Box<super::types::Frame>,
    #[serde(rename = "type")]
    pub r#type: super::types::NavigationType,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FrameNavigatedMethod {
    #[serde(rename = "Page.frameNavigated")]
    FrameNavigated,
}
impl FrameNavigatedMethod {
    pub const IDENTIFIER: &'static str = "Page.frameNavigated";
}
#[doc = "Fired once navigation of the frame has completed. Frame is now associated with the new loader.\n[frameNavigated](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameNavigated)"]
#[derive(Debug, Clone, PartialEq)]
pub struct FrameNavigated {
    pub method: FrameNavigatedMethod,
    pub params: FrameNavigatedParams,
}
#[doc = "Fired when opening document to write to.\n[documentOpened](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-documentOpened)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentOpenedParams {
    #[doc = "Frame object."]
    #[serde(rename = "frame")]
    pub frame: Box<super::types::Frame>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DocumentOpenedMethod {
    #[serde(rename = "Page.documentOpened")]
    DocumentOpened,
}
impl DocumentOpenedMethod {
    pub const IDENTIFIER: &'static str = "Page.documentOpened";
}
#[doc = "Fired when opening document to write to.\n[documentOpened](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-documentOpened)"]
#[derive(Debug, Clone, PartialEq)]
pub struct DocumentOpened {
    pub method: DocumentOpenedMethod,
    pub params: DocumentOpenedParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameResizedParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FrameResizedMethod {
    #[serde(rename = "Page.frameResized")]
    FrameResized,
}
impl FrameResizedMethod {
    pub const IDENTIFIER: &'static str = "Page.frameResized";
}
#[derive(Debug, Clone, PartialEq)]
pub struct FrameResized {
    pub method: FrameResizedMethod,
    pub params: FrameResizedParams,
}
#[doc = "Fired when a navigation starts. This event is fired for both\nrenderer-initiated and browser-initiated navigations. For renderer-initiated\nnavigations, the event is fired after `frameRequestedNavigation`.\nNavigation may still be cancelled after the event is issued. Multiple events\ncan be fired for a single navigation, for example, when a same-document\nnavigation becomes a cross-document navigation (such as in the case of a\nframeset).\n[frameStartedNavigating](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameStartedNavigating)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameStartedNavigatingParams {
    #[doc = "ID of the frame that is being navigated."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[doc = "The URL the navigation started with. The final URL can be different."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Loader identifier. Even though it is present in case of same-document\nnavigation, the previously committed loaderId would not change unless\nthe navigation changes from a same-document to a cross-document\nnavigation."]
    #[serde(rename = "loaderId")]
    pub loader_id: crate::browser_protocol::network::types::LoaderId,
    #[serde(rename = "navigationType")]
    pub navigation_type: FrameStartedNavigatingNavigationType,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FrameStartedNavigatingNavigationType {
    #[serde(rename = "reload")]
    Reload,
    #[serde(rename = "reloadBypassingCache")]
    ReloadBypassingCache,
    #[serde(rename = "restore")]
    Restore,
    #[serde(rename = "restoreWithPost")]
    RestoreWithPost,
    #[serde(rename = "historySameDocument")]
    HistorySameDocument,
    #[serde(rename = "historyDifferentDocument")]
    HistoryDifferentDocument,
    #[serde(rename = "sameDocument")]
    SameDocument,
    #[serde(rename = "differentDocument")]
    DifferentDocument,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FrameStartedNavigatingMethod {
    #[serde(rename = "Page.frameStartedNavigating")]
    FrameStartedNavigating,
}
impl FrameStartedNavigatingMethod {
    pub const IDENTIFIER: &'static str = "Page.frameStartedNavigating";
}
#[doc = "Fired when a navigation starts. This event is fired for both\nrenderer-initiated and browser-initiated navigations. For renderer-initiated\nnavigations, the event is fired after `frameRequestedNavigation`.\nNavigation may still be cancelled after the event is issued. Multiple events\ncan be fired for a single navigation, for example, when a same-document\nnavigation becomes a cross-document navigation (such as in the case of a\nframeset).\n[frameStartedNavigating](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameStartedNavigating)"]
#[derive(Debug, Clone, PartialEq)]
pub struct FrameStartedNavigating {
    pub method: FrameStartedNavigatingMethod,
    pub params: FrameStartedNavigatingParams,
}
#[doc = "Fired when a renderer-initiated navigation is requested.\nNavigation may still be cancelled after the event is issued.\n[frameRequestedNavigation](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameRequestedNavigation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameRequestedNavigationParams {
    #[doc = "Id of the frame that is being navigated."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[doc = "The reason for the navigation."]
    #[serde(rename = "reason")]
    pub reason: super::types::ClientNavigationReason,
    #[doc = "The destination URL for the requested navigation."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "The disposition for the navigation."]
    #[serde(rename = "disposition")]
    pub disposition: super::types::ClientNavigationDisposition,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FrameRequestedNavigationMethod {
    #[serde(rename = "Page.frameRequestedNavigation")]
    FrameRequestedNavigation,
}
impl FrameRequestedNavigationMethod {
    pub const IDENTIFIER: &'static str = "Page.frameRequestedNavigation";
}
#[doc = "Fired when a renderer-initiated navigation is requested.\nNavigation may still be cancelled after the event is issued.\n[frameRequestedNavigation](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameRequestedNavigation)"]
#[derive(Debug, Clone, PartialEq)]
pub struct FrameRequestedNavigation {
    pub method: FrameRequestedNavigationMethod,
    pub params: FrameRequestedNavigationParams,
}
#[doc = "Fired when frame has started loading.\n[frameStartedLoading](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameStartedLoading)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameStartedLoadingParams {
    #[doc = "Id of the frame that has started loading."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FrameStartedLoadingMethod {
    #[serde(rename = "Page.frameStartedLoading")]
    FrameStartedLoading,
}
impl FrameStartedLoadingMethod {
    pub const IDENTIFIER: &'static str = "Page.frameStartedLoading";
}
#[doc = "Fired when frame has started loading.\n[frameStartedLoading](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameStartedLoading)"]
#[derive(Debug, Clone, PartialEq)]
pub struct FrameStartedLoading {
    pub method: FrameStartedLoadingMethod,
    pub params: FrameStartedLoadingParams,
}
#[doc = "Fired when frame has stopped loading.\n[frameStoppedLoading](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameStoppedLoading)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameStoppedLoadingParams {
    #[doc = "Id of the frame that has stopped loading."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FrameStoppedLoadingMethod {
    #[serde(rename = "Page.frameStoppedLoading")]
    FrameStoppedLoading,
}
impl FrameStoppedLoadingMethod {
    pub const IDENTIFIER: &'static str = "Page.frameStoppedLoading";
}
#[doc = "Fired when frame has stopped loading.\n[frameStoppedLoading](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameStoppedLoading)"]
#[derive(Debug, Clone, PartialEq)]
pub struct FrameStoppedLoading {
    pub method: FrameStoppedLoadingMethod,
    pub params: FrameStoppedLoadingParams,
}
#[doc = "Fired when interstitial page was hidden\n[interstitialHidden](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-interstitialHidden)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterstitialHiddenParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterstitialHiddenMethod {
    #[serde(rename = "Page.interstitialHidden")]
    InterstitialHidden,
}
impl InterstitialHiddenMethod {
    pub const IDENTIFIER: &'static str = "Page.interstitialHidden";
}
#[doc = "Fired when interstitial page was hidden\n[interstitialHidden](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-interstitialHidden)"]
#[derive(Debug, Clone, PartialEq)]
pub struct InterstitialHidden {
    pub method: InterstitialHiddenMethod,
    pub params: InterstitialHiddenParams,
}
#[doc = "Fired when interstitial page was shown\n[interstitialShown](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-interstitialShown)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterstitialShownParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterstitialShownMethod {
    #[serde(rename = "Page.interstitialShown")]
    InterstitialShown,
}
impl InterstitialShownMethod {
    pub const IDENTIFIER: &'static str = "Page.interstitialShown";
}
#[doc = "Fired when interstitial page was shown\n[interstitialShown](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-interstitialShown)"]
#[derive(Debug, Clone, PartialEq)]
pub struct InterstitialShown {
    pub method: InterstitialShownMethod,
    pub params: InterstitialShownParams,
}
#[doc = "Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) has been\nclosed.\n[javascriptDialogClosed](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-javascriptDialogClosed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JavascriptDialogClosedParams {
    #[doc = "Frame id."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[doc = "Whether dialog was confirmed."]
    #[serde(rename = "result")]
    pub result: bool,
    #[doc = "User input in case of prompt."]
    #[serde(rename = "userInput")]
    pub user_input: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JavascriptDialogClosedMethod {
    #[serde(rename = "Page.javascriptDialogClosed")]
    JavascriptDialogClosed,
}
impl JavascriptDialogClosedMethod {
    pub const IDENTIFIER: &'static str = "Page.javascriptDialogClosed";
}
#[doc = "Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) has been\nclosed.\n[javascriptDialogClosed](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-javascriptDialogClosed)"]
#[derive(Debug, Clone, PartialEq)]
pub struct JavascriptDialogClosed {
    pub method: JavascriptDialogClosedMethod,
    pub params: JavascriptDialogClosedParams,
}
#[doc = "Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) is about to\nopen.\n[javascriptDialogOpening](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-javascriptDialogOpening)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JavascriptDialogOpeningParams {
    #[doc = "Frame url."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Frame id."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[doc = "Message that will be displayed by the dialog."]
    #[serde(rename = "message")]
    pub message: String,
    #[doc = "Dialog type."]
    #[serde(rename = "type")]
    pub r#type: super::types::DialogType,
    #[doc = "True iff browser is capable showing or acting on the given dialog. When browser has no\ndialog handler for given target, calling alert while Page domain is engaged will stall\nthe page execution. Execution can be resumed via calling Page.handleJavaScriptDialog."]
    #[serde(rename = "hasBrowserHandler")]
    pub has_browser_handler: bool,
    #[doc = "Default dialog prompt."]
    #[serde(rename = "defaultPrompt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub default_prompt: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum JavascriptDialogOpeningMethod {
    #[serde(rename = "Page.javascriptDialogOpening")]
    JavascriptDialogOpening,
}
impl JavascriptDialogOpeningMethod {
    pub const IDENTIFIER: &'static str = "Page.javascriptDialogOpening";
}
#[doc = "Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) is about to\nopen.\n[javascriptDialogOpening](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-javascriptDialogOpening)"]
#[derive(Debug, Clone, PartialEq)]
pub struct JavascriptDialogOpening {
    pub method: JavascriptDialogOpeningMethod,
    pub params: JavascriptDialogOpeningParams,
}
#[doc = "Fired for lifecycle events (navigation, load, paint, etc) in the current\ntarget (including local frames).\n[lifecycleEvent](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-lifecycleEvent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LifecycleEventParams {
    #[doc = "Id of the frame."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[doc = "Loader identifier. Empty string if the request is fetched from worker."]
    #[serde(rename = "loaderId")]
    pub loader_id: crate::browser_protocol::network::types::LoaderId,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "timestamp")]
    pub timestamp: crate::browser_protocol::network::types::MonotonicTime,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LifecycleEventMethod {
    #[serde(rename = "Page.lifecycleEvent")]
    LifecycleEvent,
}
impl LifecycleEventMethod {
    pub const IDENTIFIER: &'static str = "Page.lifecycleEvent";
}
#[doc = "Fired for lifecycle events (navigation, load, paint, etc) in the current\ntarget (including local frames).\n[lifecycleEvent](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-lifecycleEvent)"]
#[derive(Debug, Clone, PartialEq)]
pub struct LifecycleEvent {
    pub method: LifecycleEventMethod,
    pub params: LifecycleEventParams,
}
#[doc = "Fired for failed bfcache history navigations if BackForwardCache feature is enabled. Do\nnot assume any ordering with the Page.frameNavigated event. This event is fired only for\nmain-frame history navigation where the document changes (non-same-document navigations),\nwhen bfcache navigation fails.\n[backForwardCacheNotUsed](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-backForwardCacheNotUsed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackForwardCacheNotUsedParams {
    #[doc = "The loader id for the associated navigation."]
    #[serde(rename = "loaderId")]
    pub loader_id: crate::browser_protocol::network::types::LoaderId,
    #[doc = "The frame id of the associated frame."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[doc = "Array of reasons why the page could not be cached. This must not be empty."]
    #[serde(rename = "notRestoredExplanations")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub not_restored_explanations: Vec<super::types::BackForwardCacheNotRestoredExplanation>,
    #[doc = "Tree structure of reasons why the page could not be cached for each frame."]
    #[serde(rename = "notRestoredExplanationsTree")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub not_restored_explanations_tree:
        Option<super::types::BackForwardCacheNotRestoredExplanationTree>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BackForwardCacheNotUsedMethod {
    #[serde(rename = "Page.backForwardCacheNotUsed")]
    BackForwardCacheNotUsed,
}
impl BackForwardCacheNotUsedMethod {
    pub const IDENTIFIER: &'static str = "Page.backForwardCacheNotUsed";
}
#[doc = "Fired for failed bfcache history navigations if BackForwardCache feature is enabled. Do\nnot assume any ordering with the Page.frameNavigated event. This event is fired only for\nmain-frame history navigation where the document changes (non-same-document navigations),\nwhen bfcache navigation fails.\n[backForwardCacheNotUsed](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-backForwardCacheNotUsed)"]
#[derive(Debug, Clone, PartialEq)]
pub struct BackForwardCacheNotUsed {
    pub method: BackForwardCacheNotUsedMethod,
    pub params: BackForwardCacheNotUsedParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadEventFiredParams {
    #[serde(rename = "timestamp")]
    pub timestamp: crate::browser_protocol::network::types::MonotonicTime,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LoadEventFiredMethod {
    #[serde(rename = "Page.loadEventFired")]
    LoadEventFired,
}
impl LoadEventFiredMethod {
    pub const IDENTIFIER: &'static str = "Page.loadEventFired";
}
#[derive(Debug, Clone, PartialEq)]
pub struct LoadEventFired {
    pub method: LoadEventFiredMethod,
    pub params: LoadEventFiredParams,
}
#[doc = "Fired when same-document navigation happens, e.g. due to history API usage or anchor navigation.\n[navigatedWithinDocument](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-navigatedWithinDocument)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigatedWithinDocumentParams {
    #[doc = "Id of the frame."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[doc = "Frame's new url."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Navigation type"]
    #[serde(rename = "navigationType")]
    pub navigation_type: NavigatedWithinDocumentNavigationType,
}
#[doc = "Navigation type"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NavigatedWithinDocumentNavigationType {
    #[doc = "Navigation due to fragment navigation."]
    #[serde(rename = "fragment")]
    Fragment,
    #[doc = "Navigation due to history API usage."]
    #[serde(rename = "historyApi")]
    HistoryApi,
    #[doc = "Navigation due to other reasons."]
    #[serde(rename = "other")]
    Other,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigatedWithinDocumentMethod {
    #[serde(rename = "Page.navigatedWithinDocument")]
    NavigatedWithinDocument,
}
impl NavigatedWithinDocumentMethod {
    pub const IDENTIFIER: &'static str = "Page.navigatedWithinDocument";
}
#[doc = "Fired when same-document navigation happens, e.g. due to history API usage or anchor navigation.\n[navigatedWithinDocument](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-navigatedWithinDocument)"]
#[derive(Debug, Clone, PartialEq)]
pub struct NavigatedWithinDocument {
    pub method: NavigatedWithinDocumentMethod,
    pub params: NavigatedWithinDocumentParams,
}
#[doc = "Compressed image data requested by the `startScreencast`.\n[screencastFrame](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-screencastFrame)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreencastFrameParams {
    #[doc = "Base64-encoded compressed image."]
    #[serde(rename = "data")]
    pub data: crate::Binary,
    #[doc = "Screencast frame metadata."]
    #[serde(rename = "metadata")]
    pub metadata: super::types::ScreencastFrameMetadata,
    #[doc = "Frame number."]
    #[serde(rename = "sessionId")]
    pub session_id: i64,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScreencastFrameMethod {
    #[serde(rename = "Page.screencastFrame")]
    ScreencastFrame,
}
impl ScreencastFrameMethod {
    pub const IDENTIFIER: &'static str = "Page.screencastFrame";
}
#[doc = "Compressed image data requested by the `startScreencast`.\n[screencastFrame](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-screencastFrame)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ScreencastFrame {
    pub method: ScreencastFrameMethod,
    pub params: ScreencastFrameParams,
}
#[doc = "Fired when the page with currently enabled screencast was shown or hidden `.\n[screencastVisibilityChanged](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-screencastVisibilityChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreencastVisibilityChangedParams {
    #[doc = "True if the page is visible."]
    #[serde(rename = "visible")]
    pub visible: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScreencastVisibilityChangedMethod {
    #[serde(rename = "Page.screencastVisibilityChanged")]
    ScreencastVisibilityChanged,
}
impl ScreencastVisibilityChangedMethod {
    pub const IDENTIFIER: &'static str = "Page.screencastVisibilityChanged";
}
#[doc = "Fired when the page with currently enabled screencast was shown or hidden `.\n[screencastVisibilityChanged](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-screencastVisibilityChanged)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ScreencastVisibilityChanged {
    pub method: ScreencastVisibilityChangedMethod,
    pub params: ScreencastVisibilityChangedParams,
}
#[doc = "Fired when a new window is going to be opened, via window.open(), link click, form submission,\netc.\n[windowOpen](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-windowOpen)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowOpenParams {
    #[doc = "The URL for the new window."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Window name."]
    #[serde(rename = "windowName")]
    pub window_name: String,
    #[doc = "An array of enabled window features."]
    #[serde(rename = "windowFeatures")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub window_features: Vec<String>,
    #[doc = "Whether or not it was triggered by user gesture."]
    #[serde(rename = "userGesture")]
    pub user_gesture: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WindowOpenMethod {
    #[serde(rename = "Page.windowOpen")]
    WindowOpen,
}
impl WindowOpenMethod {
    pub const IDENTIFIER: &'static str = "Page.windowOpen";
}
#[doc = "Fired when a new window is going to be opened, via window.open(), link click, form submission,\netc.\n[windowOpen](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-windowOpen)"]
#[derive(Debug, Clone, PartialEq)]
pub struct WindowOpen {
    pub method: WindowOpenMethod,
    pub params: WindowOpenParams,
}
#[doc = "Issued for every compilation cache generated.\n[compilationCacheProduced](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-compilationCacheProduced)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompilationCacheProducedParams {
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Base64-encoded data"]
    #[serde(rename = "data")]
    pub data: crate::Binary,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CompilationCacheProducedMethod {
    #[serde(rename = "Page.compilationCacheProduced")]
    CompilationCacheProduced,
}
impl CompilationCacheProducedMethod {
    pub const IDENTIFIER: &'static str = "Page.compilationCacheProduced";
}
#[doc = "Issued for every compilation cache generated.\n[compilationCacheProduced](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-compilationCacheProduced)"]
#[derive(Debug, Clone, PartialEq)]
pub struct CompilationCacheProduced {
    pub method: CompilationCacheProducedMethod,
    pub params: CompilationCacheProducedParams,
}
group_enum ! (PageEvents { DomContentEventFired (DomContentEventFired) , FileChooserOpened (FileChooserOpened) , FrameAttached (FrameAttached) , FrameDetached (FrameDetached) , FrameSubtreeWillBeDetached (FrameSubtreeWillBeDetached) , FrameNavigated (FrameNavigated) , DocumentOpened (DocumentOpened) , FrameResized (FrameResized) , FrameStartedNavigating (FrameStartedNavigating) , FrameRequestedNavigation (FrameRequestedNavigation) , FrameStartedLoading (FrameStartedLoading) , FrameStoppedLoading (FrameStoppedLoading) , InterstitialHidden (InterstitialHidden) , InterstitialShown (InterstitialShown) , JavascriptDialogClosed (JavascriptDialogClosed) , JavascriptDialogOpening (JavascriptDialogOpening) , LifecycleEvent (LifecycleEvent) , BackForwardCacheNotUsed (BackForwardCacheNotUsed) , LoadEventFired (LoadEventFired) , NavigatedWithinDocument (NavigatedWithinDocument) , ScreencastFrame (ScreencastFrame) , ScreencastVisibilityChanged (ScreencastVisibilityChanged) , WindowOpen (WindowOpen) , CompilationCacheProduced (CompilationCacheProduced) });
