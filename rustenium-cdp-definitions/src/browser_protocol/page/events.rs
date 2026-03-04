use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DomContentEventFired {
    #[serde(rename = "timestamp")]
    pub timestamp: super::super::network::types::MonotonicTime,
}
impl DomContentEventFired {
    pub const IDENTIFIER: &'static str = "Page.domContentEventFired";
}
#[doc = "Emitted only when `page.interceptFileChooser` is enabled.\n[fileChooserOpened](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-fileChooserOpened)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileChooserOpened {
    #[doc = "Id of the frame containing input node."]
    #[serde(rename = "frameId")]
    pub frame_id: super::types::FrameId,
    #[doc = "Input mode."]
    #[serde(rename = "mode")]
    pub mode: FileChooserOpenedMode,
    #[doc = "Input node id. Only present for file choosers opened via an `<input type=\"file\">` element."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<super::super::dom::types::BackendNodeId>,
}
#[doc = "Input mode."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FileChooserOpenedMode {
    #[serde(rename = "selectSingle")]
    SelectSingle,
    #[serde(rename = "selectMultiple")]
    SelectMultiple,
}
impl FileChooserOpened {
    pub const IDENTIFIER: &'static str = "Page.fileChooserOpened";
}
#[doc = "Fired when frame has been attached to its parent.\n[frameAttached](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameAttached)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameAttached {
    #[doc = "Id of the frame that has been attached."]
    #[serde(rename = "frameId")]
    pub frame_id: super::types::FrameId,
    #[doc = "Parent frame identifier."]
    #[serde(rename = "parentFrameId")]
    pub parent_frame_id: super::types::FrameId,
    #[doc = "JavaScript stack trace of when frame was attached, only set if frame initiated from script."]
    #[serde(rename = "stack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack: Option<super::super::super::js_protocol::runtime::types::StackTrace>,
}
impl FrameAttached {
    pub const IDENTIFIER: &'static str = "Page.frameAttached";
}
#[doc = "Fired when frame has been detached from its parent.\n[frameDetached](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameDetached)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameDetached {
    #[doc = "Id of the frame that has been detached."]
    #[serde(rename = "frameId")]
    pub frame_id: super::types::FrameId,
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
impl FrameDetached {
    pub const IDENTIFIER: &'static str = "Page.frameDetached";
}
#[doc = "Fired before frame subtree is detached. Emitted before any frame of the\nsubtree is actually detached.\n[frameSubtreeWillBeDetached](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameSubtreeWillBeDetached)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameSubtreeWillBeDetached {
    #[doc = "Id of the frame that is the root of the subtree that will be detached."]
    #[serde(rename = "frameId")]
    pub frame_id: super::types::FrameId,
}
impl FrameSubtreeWillBeDetached {
    pub const IDENTIFIER: &'static str = "Page.frameSubtreeWillBeDetached";
}
#[doc = "Fired once navigation of the frame has completed. Frame is now associated with the new loader.\n[frameNavigated](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameNavigated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameNavigated {
    #[doc = "Frame object."]
    #[serde(rename = "frame")]
    pub frame: super::types::Frame,
    #[serde(rename = "type")]
    pub r#type: super::types::NavigationType,
}
impl FrameNavigated {
    pub const IDENTIFIER: &'static str = "Page.frameNavigated";
}
#[doc = "Fired when opening document to write to.\n[documentOpened](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-documentOpened)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentOpened {
    #[doc = "Frame object."]
    #[serde(rename = "frame")]
    pub frame: super::types::Frame,
}
impl DocumentOpened {
    pub const IDENTIFIER: &'static str = "Page.documentOpened";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FrameResized {}
impl FrameResized {
    pub const IDENTIFIER: &'static str = "Page.frameResized";
}
#[doc = "Fired when a navigation starts. This event is fired for both\nrenderer-initiated and browser-initiated navigations. For renderer-initiated\nnavigations, the event is fired after `frameRequestedNavigation`.\nNavigation may still be cancelled after the event is issued. Multiple events\ncan be fired for a single navigation, for example, when a same-document\nnavigation becomes a cross-document navigation (such as in the case of a\nframeset).\n[frameStartedNavigating](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameStartedNavigating)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameStartedNavigating {
    #[doc = "ID of the frame that is being navigated."]
    #[serde(rename = "frameId")]
    pub frame_id: super::types::FrameId,
    #[doc = "The URL the navigation started with. The final URL can be different."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Loader identifier. Even though it is present in case of same-document\nnavigation, the previously committed loaderId would not change unless\nthe navigation changes from a same-document to a cross-document\nnavigation."]
    #[serde(rename = "loaderId")]
    pub loader_id: super::super::network::types::LoaderId,
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
impl FrameStartedNavigating {
    pub const IDENTIFIER: &'static str = "Page.frameStartedNavigating";
}
#[doc = "Fired when a renderer-initiated navigation is requested.\nNavigation may still be cancelled after the event is issued.\n[frameRequestedNavigation](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameRequestedNavigation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameRequestedNavigation {
    #[doc = "Id of the frame that is being navigated."]
    #[serde(rename = "frameId")]
    pub frame_id: super::types::FrameId,
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
impl FrameRequestedNavigation {
    pub const IDENTIFIER: &'static str = "Page.frameRequestedNavigation";
}
#[doc = "Fired when frame has started loading.\n[frameStartedLoading](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameStartedLoading)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameStartedLoading {
    #[doc = "Id of the frame that has started loading."]
    #[serde(rename = "frameId")]
    pub frame_id: super::types::FrameId,
}
impl FrameStartedLoading {
    pub const IDENTIFIER: &'static str = "Page.frameStartedLoading";
}
#[doc = "Fired when frame has stopped loading.\n[frameStoppedLoading](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-frameStoppedLoading)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameStoppedLoading {
    #[doc = "Id of the frame that has stopped loading."]
    #[serde(rename = "frameId")]
    pub frame_id: super::types::FrameId,
}
impl FrameStoppedLoading {
    pub const IDENTIFIER: &'static str = "Page.frameStoppedLoading";
}
#[doc = "Fired when interstitial page was hidden\n[interstitialHidden](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-interstitialHidden)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InterstitialHidden {}
impl InterstitialHidden {
    pub const IDENTIFIER: &'static str = "Page.interstitialHidden";
}
#[doc = "Fired when interstitial page was shown\n[interstitialShown](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-interstitialShown)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InterstitialShown {}
impl InterstitialShown {
    pub const IDENTIFIER: &'static str = "Page.interstitialShown";
}
#[doc = "Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) has been\nclosed.\n[javascriptDialogClosed](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-javascriptDialogClosed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JavascriptDialogClosed {
    #[doc = "Frame id."]
    #[serde(rename = "frameId")]
    pub frame_id: super::types::FrameId,
    #[doc = "Whether dialog was confirmed."]
    #[serde(rename = "result")]
    pub result: bool,
    #[doc = "User input in case of prompt."]
    #[serde(rename = "userInput")]
    pub user_input: String,
}
impl JavascriptDialogClosed {
    pub const IDENTIFIER: &'static str = "Page.javascriptDialogClosed";
}
#[doc = "Fired when a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload) is about to\nopen.\n[javascriptDialogOpening](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-javascriptDialogOpening)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JavascriptDialogOpening {
    #[doc = "Frame url."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Frame id."]
    #[serde(rename = "frameId")]
    pub frame_id: super::types::FrameId,
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
impl JavascriptDialogOpening {
    pub const IDENTIFIER: &'static str = "Page.javascriptDialogOpening";
}
#[doc = "Fired for lifecycle events (navigation, load, paint, etc) in the current\ntarget (including local frames).\n[lifecycleEvent](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-lifecycleEvent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LifecycleEvent {
    #[doc = "Id of the frame."]
    #[serde(rename = "frameId")]
    pub frame_id: super::types::FrameId,
    #[doc = "Loader identifier. Empty string if the request is fetched from worker."]
    #[serde(rename = "loaderId")]
    pub loader_id: super::super::network::types::LoaderId,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "timestamp")]
    pub timestamp: super::super::network::types::MonotonicTime,
}
impl LifecycleEvent {
    pub const IDENTIFIER: &'static str = "Page.lifecycleEvent";
}
#[doc = "Fired for failed bfcache history navigations if BackForwardCache feature is enabled. Do\nnot assume any ordering with the Page.frameNavigated event. This event is fired only for\nmain-frame history navigation where the document changes (non-same-document navigations),\nwhen bfcache navigation fails.\n[backForwardCacheNotUsed](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-backForwardCacheNotUsed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BackForwardCacheNotUsed {
    #[doc = "The loader id for the associated navigation."]
    #[serde(rename = "loaderId")]
    pub loader_id: super::super::network::types::LoaderId,
    #[doc = "The frame id of the associated frame."]
    #[serde(rename = "frameId")]
    pub frame_id: super::types::FrameId,
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
impl BackForwardCacheNotUsed {
    pub const IDENTIFIER: &'static str = "Page.backForwardCacheNotUsed";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoadEventFired {
    #[serde(rename = "timestamp")]
    pub timestamp: super::super::network::types::MonotonicTime,
}
impl LoadEventFired {
    pub const IDENTIFIER: &'static str = "Page.loadEventFired";
}
#[doc = "Fired when same-document navigation happens, e.g. due to history API usage or anchor navigation.\n[navigatedWithinDocument](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-navigatedWithinDocument)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigatedWithinDocument {
    #[doc = "Id of the frame."]
    #[serde(rename = "frameId")]
    pub frame_id: super::types::FrameId,
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
impl NavigatedWithinDocument {
    pub const IDENTIFIER: &'static str = "Page.navigatedWithinDocument";
}
#[doc = "Compressed image data requested by the `startScreencast`.\n[screencastFrame](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-screencastFrame)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreencastFrame {
    #[doc = "Base64-encoded compressed image."]
    #[serde(rename = "data")]
    pub data: super::super::super::Binary,
    #[doc = "Screencast frame metadata."]
    #[serde(rename = "metadata")]
    pub metadata: super::types::ScreencastFrameMetadata,
    #[doc = "Frame number."]
    #[serde(rename = "sessionId")]
    pub session_id: i64,
}
impl ScreencastFrame {
    pub const IDENTIFIER: &'static str = "Page.screencastFrame";
}
#[doc = "Fired when the page with currently enabled screencast was shown or hidden `.\n[screencastVisibilityChanged](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-screencastVisibilityChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreencastVisibilityChanged {
    #[doc = "True if the page is visible."]
    #[serde(rename = "visible")]
    pub visible: bool,
}
impl ScreencastVisibilityChanged {
    pub const IDENTIFIER: &'static str = "Page.screencastVisibilityChanged";
}
#[doc = "Fired when a new window is going to be opened, via window.open(), link click, form submission,\netc.\n[windowOpen](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-windowOpen)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WindowOpen {
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
impl WindowOpen {
    pub const IDENTIFIER: &'static str = "Page.windowOpen";
}
#[doc = "Issued for every compilation cache generated.\n[compilationCacheProduced](https://chromedevtools.github.io/devtools-protocol/tot/Page/#event-compilationCacheProduced)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompilationCacheProduced {
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Base64-encoded data"]
    #[serde(rename = "data")]
    pub data: super::super::super::Binary,
}
impl CompilationCacheProduced {
    pub const IDENTIFIER: &'static str = "Page.compilationCacheProduced";
}
group_enum ! (Event { DomContentEventFired (DomContentEventFired) , FileChooserOpened (FileChooserOpened) , FrameAttached (FrameAttached) , FrameDetached (FrameDetached) , FrameSubtreeWillBeDetached (FrameSubtreeWillBeDetached) , FrameNavigated (FrameNavigated) , DocumentOpened (DocumentOpened) , FrameResized (FrameResized) , FrameStartedNavigating (FrameStartedNavigating) , FrameRequestedNavigation (FrameRequestedNavigation) , FrameStartedLoading (FrameStartedLoading) , FrameStoppedLoading (FrameStoppedLoading) , InterstitialHidden (InterstitialHidden) , InterstitialShown (InterstitialShown) , JavascriptDialogClosed (JavascriptDialogClosed) , JavascriptDialogOpening (JavascriptDialogOpening) , LifecycleEvent (LifecycleEvent) , BackForwardCacheNotUsed (BackForwardCacheNotUsed) , LoadEventFired (LoadEventFired) , NavigatedWithinDocument (NavigatedWithinDocument) , ScreencastFrame (ScreencastFrame) , ScreencastVisibilityChanged (ScreencastVisibilityChanged) , WindowOpen (WindowOpen) , CompilationCacheProduced (CompilationCacheProduced) });
