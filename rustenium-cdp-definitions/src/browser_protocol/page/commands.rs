use serde::{Deserialize, Serialize};
#[doc = "Evaluates given script in every frame upon creation (before loading frame's scripts).\n[addScriptToEvaluateOnNewDocument](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-addScriptToEvaluateOnNewDocument)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddScriptToEvaluateOnNewDocumentParams {
    #[serde(rename = "source")]
    pub source: String,
    #[doc = "If specified, creates an isolated world with the given name and evaluates given script in it.\nThis world name will be used as the ExecutionContextDescription::name when the corresponding\nevent is emitted."]
    #[serde(rename = "worldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub world_name: Option<String>,
    #[doc = "Specifies whether command line API should be available to the script, defaults\nto false."]
    #[serde(rename = "includeCommandLineAPI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_command_line_api: Option<bool>,
    #[doc = "If true, runs the script immediately on existing execution contexts or worlds.\nDefault: false."]
    #[serde(rename = "runImmediately")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub run_immediately: Option<bool>,
}
impl AddScriptToEvaluateOnNewDocumentParams {
    pub fn new(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            world_name: None,
            include_command_line_api: None,
            run_immediately: None,
        }
    }
}
impl<T: Into<String>> From<T> for AddScriptToEvaluateOnNewDocumentParams {
    fn from(url: T) -> Self {
        AddScriptToEvaluateOnNewDocumentParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddScriptToEvaluateOnNewDocumentMethod {
    #[serde(rename = "Page.addScriptToEvaluateOnNewDocument")]
    AddScriptToEvaluateOnNewDocument,
}
#[doc = "Evaluates given script in every frame upon creation (before loading frame's scripts).\n[addScriptToEvaluateOnNewDocument](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-addScriptToEvaluateOnNewDocument)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddScriptToEvaluateOnNewDocument {
    pub method: AddScriptToEvaluateOnNewDocumentMethod,
    pub params: AddScriptToEvaluateOnNewDocumentParams,
}
impl AddScriptToEvaluateOnNewDocument {
    pub const IDENTIFIER: &'static str = "Page.addScriptToEvaluateOnNewDocument";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for AddScriptToEvaluateOnNewDocument {
    type Result = super::results::AddScriptToEvaluateOnNewDocumentResult;
}
#[doc = "Brings page to front (activates tab).\n[bringToFront](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-bringToFront)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BringToFrontParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BringToFrontMethod {
    #[serde(rename = "Page.bringToFront")]
    BringToFront,
}
#[doc = "Brings page to front (activates tab).\n[bringToFront](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-bringToFront)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BringToFront {
    pub method: BringToFrontMethod,
    pub params: BringToFrontParams,
}
impl BringToFront {
    pub const IDENTIFIER: &'static str = "Page.bringToFront";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for BringToFront {
    type Result = super::results::BringToFrontResult;
}
#[doc = "Capture page screenshot.\n[captureScreenshot](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-captureScreenshot)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CaptureScreenshotParams {
    #[doc = "Image compression format (defaults to png)."]
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub format: Option<CaptureScreenshotFormat>,
    #[doc = "Compression quality from range [0..100] (jpeg only)."]
    #[serde(rename = "quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub quality: Option<i64>,
    #[doc = "Capture the screenshot of a given region only."]
    #[serde(rename = "clip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub clip: Option<super::types::Viewport>,
    #[doc = "Capture the screenshot from the surface, rather than the view. Defaults to true."]
    #[serde(rename = "fromSurface")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub from_surface: Option<bool>,
    #[doc = "Capture the screenshot beyond the viewport. Defaults to false."]
    #[serde(rename = "captureBeyondViewport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub capture_beyond_viewport: Option<bool>,
    #[doc = "Optimize image encoding for speed, not for resulting size (defaults to false)"]
    #[serde(rename = "optimizeForSpeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub optimize_for_speed: Option<bool>,
}
#[doc = "Image compression format (defaults to png)."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CaptureScreenshotFormat {
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CaptureScreenshotMethod {
    #[serde(rename = "Page.captureScreenshot")]
    CaptureScreenshot,
}
#[doc = "Capture page screenshot.\n[captureScreenshot](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-captureScreenshot)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureScreenshot {
    pub method: CaptureScreenshotMethod,
    pub params: CaptureScreenshotParams,
}
impl CaptureScreenshot {
    pub const IDENTIFIER: &'static str = "Page.captureScreenshot";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for CaptureScreenshot {
    type Result = super::results::CaptureScreenshotResult;
}
#[doc = "Returns a snapshot of the page as a string. For MHTML format, the serialization includes\niframes, shadow DOM, external resources, and element-inline styles.\n[captureSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-captureSnapshot)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CaptureSnapshotParams {
    #[doc = "Format (defaults to mhtml)."]
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub format: Option<CaptureSnapshotFormat>,
}
#[doc = "Format (defaults to mhtml)."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CaptureSnapshotFormat {
    #[serde(rename = "mhtml")]
    Mhtml,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CaptureSnapshotMethod {
    #[serde(rename = "Page.captureSnapshot")]
    CaptureSnapshot,
}
#[doc = "Returns a snapshot of the page as a string. For MHTML format, the serialization includes\niframes, shadow DOM, external resources, and element-inline styles.\n[captureSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-captureSnapshot)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureSnapshot {
    pub method: CaptureSnapshotMethod,
    pub params: CaptureSnapshotParams,
}
impl CaptureSnapshot {
    pub const IDENTIFIER: &'static str = "Page.captureSnapshot";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for CaptureSnapshot {
    type Result = super::results::CaptureSnapshotResult;
}
#[doc = "Creates an isolated world for the given frame.\n[createIsolatedWorld](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-createIsolatedWorld)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateIsolatedWorldParams {
    #[doc = "Id of the frame in which the isolated world should be created."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[doc = "An optional name which is reported in the Execution Context."]
    #[serde(rename = "worldName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub world_name: Option<String>,
    #[doc = "Whether or not universal access should be granted to the isolated world. This is a powerful\noption, use with caution."]
    #[serde(rename = "grantUniveralAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub grant_univeral_access: Option<bool>,
}
impl CreateIsolatedWorldParams {
    pub fn new(frame_id: impl Into<super::types::FrameId>) -> Self {
        Self {
            frame_id: Box::new(frame_id.into()),
            world_name: None,
            grant_univeral_access: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateIsolatedWorldMethod {
    #[serde(rename = "Page.createIsolatedWorld")]
    CreateIsolatedWorld,
}
#[doc = "Creates an isolated world for the given frame.\n[createIsolatedWorld](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-createIsolatedWorld)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateIsolatedWorld {
    pub method: CreateIsolatedWorldMethod,
    pub params: CreateIsolatedWorldParams,
}
impl CreateIsolatedWorld {
    pub const IDENTIFIER: &'static str = "Page.createIsolatedWorld";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for CreateIsolatedWorld {
    type Result = super::results::CreateIsolatedWorldResult;
}
#[doc = "Disables page domain notifications.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Page.disable")]
    Disable,
}
#[doc = "Disables page domain notifications.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "Page.disable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables page domain notifications.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {
    #[doc = "If true, the `Page.fileChooserOpened` event will be emitted regardless of the state set by\n`Page.setInterceptFileChooserDialog` command (default: false)."]
    #[serde(rename = "enableFileChooserOpenedEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enable_file_chooser_opened_event: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Page.enable")]
    Enable,
}
#[doc = "Enables page domain notifications.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "Page.enable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Gets the processed manifest for this current document.\nThis API always waits for the manifest to be loaded.\nIf manifestId is provided, and it does not match the manifest of the\ncurrent document, this API errors out.\nIf there is not a loaded page, this API errors out immediately.\n[getAppManifest](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getAppManifest)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetAppManifestParams {
    #[serde(rename = "manifestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub manifest_id: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetAppManifestMethod {
    #[serde(rename = "Page.getAppManifest")]
    GetAppManifest,
}
#[doc = "Gets the processed manifest for this current document.\nThis API always waits for the manifest to be loaded.\nIf manifestId is provided, and it does not match the manifest of the\ncurrent document, this API errors out.\nIf there is not a loaded page, this API errors out immediately.\n[getAppManifest](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getAppManifest)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAppManifest {
    pub method: GetAppManifestMethod,
    pub params: GetAppManifestParams,
}
impl GetAppManifest {
    pub const IDENTIFIER: &'static str = "Page.getAppManifest";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetAppManifest {
    type Result = super::results::GetAppManifestResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInstallabilityErrorsParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetInstallabilityErrorsMethod {
    #[serde(rename = "Page.getInstallabilityErrors")]
    GetInstallabilityErrors,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInstallabilityErrors {
    pub method: GetInstallabilityErrorsMethod,
    pub params: GetInstallabilityErrorsParams,
}
impl GetInstallabilityErrors {
    pub const IDENTIFIER: &'static str = "Page.getInstallabilityErrors";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetInstallabilityErrors {
    type Result = super::results::GetInstallabilityErrorsResult;
}
#[doc = "Returns the unique (PWA) app id.\nOnly returns values if the feature flag 'WebAppEnableManifestId' is enabled\n[getAppId](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getAppId)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAppIdParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetAppIdMethod {
    #[serde(rename = "Page.getAppId")]
    GetAppId,
}
#[doc = "Returns the unique (PWA) app id.\nOnly returns values if the feature flag 'WebAppEnableManifestId' is enabled\n[getAppId](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getAppId)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAppId {
    pub method: GetAppIdMethod,
    pub params: GetAppIdParams,
}
impl GetAppId {
    pub const IDENTIFIER: &'static str = "Page.getAppId";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetAppId {
    type Result = super::results::GetAppIdResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAdScriptAncestryParams {
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
}
impl GetAdScriptAncestryParams {
    pub fn new(frame_id: impl Into<super::types::FrameId>) -> Self {
        Self {
            frame_id: Box::new(frame_id.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetAdScriptAncestryMethod {
    #[serde(rename = "Page.getAdScriptAncestry")]
    GetAdScriptAncestry,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAdScriptAncestry {
    pub method: GetAdScriptAncestryMethod,
    pub params: GetAdScriptAncestryParams,
}
impl GetAdScriptAncestry {
    pub const IDENTIFIER: &'static str = "Page.getAdScriptAncestry";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetAdScriptAncestry {
    type Result = super::results::GetAdScriptAncestryResult;
}
#[doc = "Returns present frame tree structure.\n[getFrameTree](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getFrameTree)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFrameTreeParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetFrameTreeMethod {
    #[serde(rename = "Page.getFrameTree")]
    GetFrameTree,
}
#[doc = "Returns present frame tree structure.\n[getFrameTree](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getFrameTree)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFrameTree {
    pub method: GetFrameTreeMethod,
    pub params: GetFrameTreeParams,
}
impl GetFrameTree {
    pub const IDENTIFIER: &'static str = "Page.getFrameTree";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetFrameTree {
    type Result = super::results::GetFrameTreeResult;
}
#[doc = "Returns metrics relating to the layouting of the page, such as viewport bounds/scale.\n[getLayoutMetrics](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getLayoutMetrics)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLayoutMetricsParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetLayoutMetricsMethod {
    #[serde(rename = "Page.getLayoutMetrics")]
    GetLayoutMetrics,
}
#[doc = "Returns metrics relating to the layouting of the page, such as viewport bounds/scale.\n[getLayoutMetrics](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getLayoutMetrics)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLayoutMetrics {
    pub method: GetLayoutMetricsMethod,
    pub params: GetLayoutMetricsParams,
}
impl GetLayoutMetrics {
    pub const IDENTIFIER: &'static str = "Page.getLayoutMetrics";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetLayoutMetrics {
    type Result = super::results::GetLayoutMetricsResult;
}
#[doc = "Returns navigation history for the current page.\n[getNavigationHistory](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getNavigationHistory)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNavigationHistoryParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetNavigationHistoryMethod {
    #[serde(rename = "Page.getNavigationHistory")]
    GetNavigationHistory,
}
#[doc = "Returns navigation history for the current page.\n[getNavigationHistory](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getNavigationHistory)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNavigationHistory {
    pub method: GetNavigationHistoryMethod,
    pub params: GetNavigationHistoryParams,
}
impl GetNavigationHistory {
    pub const IDENTIFIER: &'static str = "Page.getNavigationHistory";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetNavigationHistory {
    type Result = super::results::GetNavigationHistoryResult;
}
#[doc = "Resets navigation history for the current page.\n[resetNavigationHistory](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-resetNavigationHistory)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetNavigationHistoryParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResetNavigationHistoryMethod {
    #[serde(rename = "Page.resetNavigationHistory")]
    ResetNavigationHistory,
}
#[doc = "Resets navigation history for the current page.\n[resetNavigationHistory](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-resetNavigationHistory)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetNavigationHistory {
    pub method: ResetNavigationHistoryMethod,
    pub params: ResetNavigationHistoryParams,
}
impl ResetNavigationHistory {
    pub const IDENTIFIER: &'static str = "Page.resetNavigationHistory";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ResetNavigationHistory {
    type Result = super::results::ResetNavigationHistoryResult;
}
#[doc = "Returns content of the given resource.\n[getResourceContent](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getResourceContent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResourceContentParams {
    #[doc = "Frame id to get resource for."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[doc = "URL of the resource to get content for."]
    #[serde(rename = "url")]
    pub url: String,
}
impl GetResourceContentParams {
    pub fn new(frame_id: impl Into<super::types::FrameId>, url: impl Into<String>) -> Self {
        Self {
            frame_id: Box::new(frame_id.into()),
            url: url.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetResourceContentMethod {
    #[serde(rename = "Page.getResourceContent")]
    GetResourceContent,
}
#[doc = "Returns content of the given resource.\n[getResourceContent](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getResourceContent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResourceContent {
    pub method: GetResourceContentMethod,
    pub params: GetResourceContentParams,
}
impl GetResourceContent {
    pub const IDENTIFIER: &'static str = "Page.getResourceContent";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetResourceContent {
    type Result = super::results::GetResourceContentResult;
}
#[doc = "Returns present frame / resource tree structure.\n[getResourceTree](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getResourceTree)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResourceTreeParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetResourceTreeMethod {
    #[serde(rename = "Page.getResourceTree")]
    GetResourceTree,
}
#[doc = "Returns present frame / resource tree structure.\n[getResourceTree](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getResourceTree)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResourceTree {
    pub method: GetResourceTreeMethod,
    pub params: GetResourceTreeParams,
}
impl GetResourceTree {
    pub const IDENTIFIER: &'static str = "Page.getResourceTree";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetResourceTree {
    type Result = super::results::GetResourceTreeResult;
}
#[doc = "Accepts or dismisses a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload).\n[handleJavaScriptDialog](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-handleJavaScriptDialog)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandleJavaScriptDialogParams {
    #[doc = "Whether to accept or dismiss the dialog."]
    #[serde(rename = "accept")]
    pub accept: bool,
    #[doc = "The text to enter into the dialog prompt before accepting. Used only if this is a prompt\ndialog."]
    #[serde(rename = "promptText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub prompt_text: Option<String>,
}
impl HandleJavaScriptDialogParams {
    pub fn new(accept: impl Into<bool>) -> Self {
        Self {
            accept: accept.into(),
            prompt_text: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HandleJavaScriptDialogMethod {
    #[serde(rename = "Page.handleJavaScriptDialog")]
    HandleJavaScriptDialog,
}
#[doc = "Accepts or dismisses a JavaScript initiated dialog (alert, confirm, prompt, or onbeforeunload).\n[handleJavaScriptDialog](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-handleJavaScriptDialog)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandleJavaScriptDialog {
    pub method: HandleJavaScriptDialogMethod,
    pub params: HandleJavaScriptDialogParams,
}
impl HandleJavaScriptDialog {
    pub const IDENTIFIER: &'static str = "Page.handleJavaScriptDialog";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for HandleJavaScriptDialog {
    type Result = super::results::HandleJavaScriptDialogResult;
}
#[doc = "Navigates current page to the given URL.\n[navigate](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-navigate)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigateParams {
    #[doc = "URL to navigate the page to."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Referrer URL."]
    #[serde(rename = "referrer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub referrer: Option<String>,
    #[doc = "Intended transition type."]
    #[serde(rename = "transitionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transition_type: Option<super::types::TransitionType>,
    #[doc = "Frame id to navigate, if not specified navigates the top frame."]
    #[serde(rename = "frameId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub frame_id: Option<Box<super::types::FrameId>>,
    #[doc = "Referrer-policy used for the navigation."]
    #[serde(rename = "referrerPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub referrer_policy: Option<super::types::ReferrerPolicy>,
}
impl NavigateParams {
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            referrer: None,
            transition_type: None,
            frame_id: None,
            referrer_policy: None,
        }
    }
}
impl<T: Into<String>> From<T> for NavigateParams {
    fn from(url: T) -> Self {
        NavigateParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigateMethod {
    #[serde(rename = "Page.navigate")]
    Navigate,
}
#[doc = "Navigates current page to the given URL.\n[navigate](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-navigate)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Navigate {
    pub method: NavigateMethod,
    pub params: NavigateParams,
}
impl Navigate {
    pub const IDENTIFIER: &'static str = "Page.navigate";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Navigate {
    type Result = super::results::NavigateResult;
}
#[doc = "Navigates current page to the given history entry.\n[navigateToHistoryEntry](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-navigateToHistoryEntry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigateToHistoryEntryParams {
    #[doc = "Unique id of the entry to navigate to."]
    #[serde(rename = "entryId")]
    pub entry_id: i64,
}
impl NavigateToHistoryEntryParams {
    pub fn new(entry_id: impl Into<i64>) -> Self {
        Self {
            entry_id: entry_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigateToHistoryEntryMethod {
    #[serde(rename = "Page.navigateToHistoryEntry")]
    NavigateToHistoryEntry,
}
#[doc = "Navigates current page to the given history entry.\n[navigateToHistoryEntry](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-navigateToHistoryEntry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigateToHistoryEntry {
    pub method: NavigateToHistoryEntryMethod,
    pub params: NavigateToHistoryEntryParams,
}
impl NavigateToHistoryEntry {
    pub const IDENTIFIER: &'static str = "Page.navigateToHistoryEntry";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for NavigateToHistoryEntry {
    type Result = super::results::NavigateToHistoryEntryResult;
}
#[doc = "Print page as PDF.\n[printToPDF](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-printToPDF)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PrintToPdfParams {
    #[doc = "Paper orientation. Defaults to false."]
    #[serde(rename = "landscape")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub landscape: Option<bool>,
    #[doc = "Display header and footer. Defaults to false."]
    #[serde(rename = "displayHeaderFooter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_header_footer: Option<bool>,
    #[doc = "Print background graphics. Defaults to false."]
    #[serde(rename = "printBackground")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub print_background: Option<bool>,
    #[doc = "Scale of the webpage rendering. Defaults to 1."]
    #[serde(rename = "scale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scale: Option<f64>,
    #[doc = "Paper width in inches. Defaults to 8.5 inches."]
    #[serde(rename = "paperWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub paper_width: Option<f64>,
    #[doc = "Paper height in inches. Defaults to 11 inches."]
    #[serde(rename = "paperHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub paper_height: Option<f64>,
    #[doc = "Top margin in inches. Defaults to 1cm (~0.4 inches)."]
    #[serde(rename = "marginTop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub margin_top: Option<f64>,
    #[doc = "Bottom margin in inches. Defaults to 1cm (~0.4 inches)."]
    #[serde(rename = "marginBottom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub margin_bottom: Option<f64>,
    #[doc = "Left margin in inches. Defaults to 1cm (~0.4 inches)."]
    #[serde(rename = "marginLeft")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub margin_left: Option<f64>,
    #[doc = "Right margin in inches. Defaults to 1cm (~0.4 inches)."]
    #[serde(rename = "marginRight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub margin_right: Option<f64>,
    #[doc = "Paper ranges to print, one based, e.g., '1-5, 8, 11-13'. Pages are\nprinted in the document order, not in the order specified, and no\nmore than once.\nDefaults to empty string, which implies the entire document is printed.\nThe page numbers are quietly capped to actual page count of the\ndocument, and ranges beyond the end of the document are ignored.\nIf this results in no pages to print, an error is reported.\nIt is an error to specify a range with start greater than end."]
    #[serde(rename = "pageRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub page_ranges: Option<String>,
    #[doc = "HTML template for the print header. Should be valid HTML markup with following\nclasses used to inject printing values into them:\n- `date`: formatted print date\n- `title`: document title\n- `url`: document location\n- `pageNumber`: current page number\n- `totalPages`: total pages in the document\n\nFor example, `<span class=title></span>` would generate span containing the title."]
    #[serde(rename = "headerTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub header_template: Option<String>,
    #[doc = "HTML template for the print footer. Should use the same format as the `headerTemplate`."]
    #[serde(rename = "footerTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub footer_template: Option<String>,
    #[doc = "Whether or not to prefer page size as defined by css. Defaults to false,\nin which case the content will be scaled to fit the paper size."]
    #[serde(rename = "preferCSSPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub prefer_css_page_size: Option<bool>,
    #[doc = "return as stream"]
    #[serde(rename = "transferMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transfer_mode: Option<PrintToPdfTransferMode>,
    #[doc = "Whether or not to generate tagged (accessible) PDF. Defaults to embedder choice."]
    #[serde(rename = "generateTaggedPDF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub generate_tagged_pdf: Option<bool>,
    #[doc = "Whether or not to embed the document outline into the PDF."]
    #[serde(rename = "generateDocumentOutline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub generate_document_outline: Option<bool>,
}
#[doc = "return as stream"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrintToPdfTransferMode {
    #[serde(rename = "ReturnAsBase64")]
    ReturnAsBase64,
    #[serde(rename = "ReturnAsStream")]
    ReturnAsStream,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PrintToPdfMethod {
    #[serde(rename = "Page.printToPDF")]
    PrintToPdf,
}
#[doc = "Print page as PDF.\n[printToPDF](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-printToPDF)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintToPdf {
    pub method: PrintToPdfMethod,
    pub params: PrintToPdfParams,
}
impl PrintToPdf {
    pub const IDENTIFIER: &'static str = "Page.printToPDF";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for PrintToPdf {
    type Result = super::results::PrintToPdfResult;
}
#[doc = "Reloads given page optionally ignoring the cache.\n[reload](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-reload)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ReloadParams {
    #[doc = "If true, browser cache is ignored (as if the user pressed Shift+refresh)."]
    #[serde(rename = "ignoreCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ignore_cache: Option<bool>,
    #[doc = "If set, the script will be injected into all frames of the inspected page after reload.\nArgument will be ignored if reloading dataURL origin."]
    #[serde(rename = "scriptToEvaluateOnLoad")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub script_to_evaluate_on_load: Option<String>,
    #[doc = "If set, an error will be thrown if the target page's main frame's\nloader id does not match the provided id. This prevents accidentally\nreloading an unintended target in case there's a racing navigation."]
    #[serde(rename = "loaderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub loader_id: Option<crate::browser_protocol::network::types::LoaderId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReloadMethod {
    #[serde(rename = "Page.reload")]
    Reload,
}
#[doc = "Reloads given page optionally ignoring the cache.\n[reload](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-reload)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reload {
    pub method: ReloadMethod,
    pub params: ReloadParams,
}
impl Reload {
    pub const IDENTIFIER: &'static str = "Page.reload";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Reload {
    type Result = super::results::ReloadResult;
}
#[doc = "Removes given script from the list.\n[removeScriptToEvaluateOnNewDocument](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-removeScriptToEvaluateOnNewDocument)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveScriptToEvaluateOnNewDocumentParams {
    #[serde(rename = "identifier")]
    pub identifier: super::types::ScriptIdentifier,
}
impl RemoveScriptToEvaluateOnNewDocumentParams {
    pub fn new(identifier: impl Into<super::types::ScriptIdentifier>) -> Self {
        Self {
            identifier: identifier.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveScriptToEvaluateOnNewDocumentMethod {
    #[serde(rename = "Page.removeScriptToEvaluateOnNewDocument")]
    RemoveScriptToEvaluateOnNewDocument,
}
#[doc = "Removes given script from the list.\n[removeScriptToEvaluateOnNewDocument](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-removeScriptToEvaluateOnNewDocument)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveScriptToEvaluateOnNewDocument {
    pub method: RemoveScriptToEvaluateOnNewDocumentMethod,
    pub params: RemoveScriptToEvaluateOnNewDocumentParams,
}
impl RemoveScriptToEvaluateOnNewDocument {
    pub const IDENTIFIER: &'static str = "Page.removeScriptToEvaluateOnNewDocument";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for RemoveScriptToEvaluateOnNewDocument {
    type Result = super::results::RemoveScriptToEvaluateOnNewDocumentResult;
}
#[doc = "Acknowledges that a screencast frame has been received by the frontend.\n[screencastFrameAck](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-screencastFrameAck)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreencastFrameAckParams {
    #[doc = "Frame number."]
    #[serde(rename = "sessionId")]
    pub session_id: i64,
}
impl ScreencastFrameAckParams {
    pub fn new(session_id: impl Into<i64>) -> Self {
        Self {
            session_id: session_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScreencastFrameAckMethod {
    #[serde(rename = "Page.screencastFrameAck")]
    ScreencastFrameAck,
}
#[doc = "Acknowledges that a screencast frame has been received by the frontend.\n[screencastFrameAck](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-screencastFrameAck)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreencastFrameAck {
    pub method: ScreencastFrameAckMethod,
    pub params: ScreencastFrameAckParams,
}
impl ScreencastFrameAck {
    pub const IDENTIFIER: &'static str = "Page.screencastFrameAck";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ScreencastFrameAck {
    type Result = super::results::ScreencastFrameAckResult;
}
#[doc = "Searches for given string in resource content.\n[searchInResource](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-searchInResource)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchInResourceParams {
    #[doc = "Frame id for resource to search in."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[doc = "URL of the resource to search in."]
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "String to search for."]
    #[serde(rename = "query")]
    pub query: String,
    #[doc = "If true, search is case sensitive."]
    #[serde(rename = "caseSensitive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub case_sensitive: Option<bool>,
    #[doc = "If true, treats string parameter as regex."]
    #[serde(rename = "isRegex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_regex: Option<bool>,
}
impl SearchInResourceParams {
    pub fn new(
        frame_id: impl Into<super::types::FrameId>,
        url: impl Into<String>,
        query: impl Into<String>,
    ) -> Self {
        Self {
            frame_id: Box::new(frame_id.into()),
            url: url.into(),
            query: query.into(),
            case_sensitive: None,
            is_regex: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SearchInResourceMethod {
    #[serde(rename = "Page.searchInResource")]
    SearchInResource,
}
#[doc = "Searches for given string in resource content.\n[searchInResource](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-searchInResource)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchInResource {
    pub method: SearchInResourceMethod,
    pub params: SearchInResourceParams,
}
impl SearchInResource {
    pub const IDENTIFIER: &'static str = "Page.searchInResource";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SearchInResource {
    type Result = super::results::SearchInResourceResult;
}
#[doc = "Enable Chrome's experimental ad filter on all sites.\n[setAdBlockingEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setAdBlockingEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAdBlockingEnabledParams {
    #[doc = "Whether to block ads."]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
impl SetAdBlockingEnabledParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self {
            enabled: enabled.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetAdBlockingEnabledMethod {
    #[serde(rename = "Page.setAdBlockingEnabled")]
    SetAdBlockingEnabled,
}
#[doc = "Enable Chrome's experimental ad filter on all sites.\n[setAdBlockingEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setAdBlockingEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAdBlockingEnabled {
    pub method: SetAdBlockingEnabledMethod,
    pub params: SetAdBlockingEnabledParams,
}
impl SetAdBlockingEnabled {
    pub const IDENTIFIER: &'static str = "Page.setAdBlockingEnabled";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetAdBlockingEnabled {
    type Result = super::results::SetAdBlockingEnabledResult;
}
#[doc = "Enable page Content Security Policy by-passing.\n[setBypassCSP](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setBypassCSP)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBypassCspParams {
    #[doc = "Whether to bypass page CSP."]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
impl SetBypassCspParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self {
            enabled: enabled.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetBypassCspMethod {
    #[serde(rename = "Page.setBypassCSP")]
    SetBypassCsp,
}
#[doc = "Enable page Content Security Policy by-passing.\n[setBypassCSP](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setBypassCSP)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetBypassCsp {
    pub method: SetBypassCspMethod,
    pub params: SetBypassCspParams,
}
impl SetBypassCsp {
    pub const IDENTIFIER: &'static str = "Page.setBypassCSP";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetBypassCsp {
    type Result = super::results::SetBypassCspResult;
}
#[doc = "Get Permissions Policy state on given frame.\n[getPermissionsPolicyState](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getPermissionsPolicyState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPermissionsPolicyStateParams {
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
}
impl GetPermissionsPolicyStateParams {
    pub fn new(frame_id: impl Into<super::types::FrameId>) -> Self {
        Self {
            frame_id: Box::new(frame_id.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetPermissionsPolicyStateMethod {
    #[serde(rename = "Page.getPermissionsPolicyState")]
    GetPermissionsPolicyState,
}
#[doc = "Get Permissions Policy state on given frame.\n[getPermissionsPolicyState](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getPermissionsPolicyState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPermissionsPolicyState {
    pub method: GetPermissionsPolicyStateMethod,
    pub params: GetPermissionsPolicyStateParams,
}
impl GetPermissionsPolicyState {
    pub const IDENTIFIER: &'static str = "Page.getPermissionsPolicyState";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetPermissionsPolicyState {
    type Result = super::results::GetPermissionsPolicyStateResult;
}
#[doc = "Get Origin Trials on given frame.\n[getOriginTrials](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getOriginTrials)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOriginTrialsParams {
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
}
impl GetOriginTrialsParams {
    pub fn new(frame_id: impl Into<super::types::FrameId>) -> Self {
        Self {
            frame_id: Box::new(frame_id.into()),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetOriginTrialsMethod {
    #[serde(rename = "Page.getOriginTrials")]
    GetOriginTrials,
}
#[doc = "Get Origin Trials on given frame.\n[getOriginTrials](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getOriginTrials)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOriginTrials {
    pub method: GetOriginTrialsMethod,
    pub params: GetOriginTrialsParams,
}
impl GetOriginTrials {
    pub const IDENTIFIER: &'static str = "Page.getOriginTrials";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetOriginTrials {
    type Result = super::results::GetOriginTrialsResult;
}
#[doc = "Set generic font families.\n[setFontFamilies](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setFontFamilies)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetFontFamiliesParams {
    #[doc = "Specifies font families to set. If a font family is not specified, it won't be changed."]
    #[serde(rename = "fontFamilies")]
    pub font_families: super::types::FontFamilies,
    #[doc = "Specifies font families to set for individual scripts."]
    #[serde(rename = "forScripts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub for_scripts: Option<Vec<super::types::ScriptFontFamilies>>,
}
impl SetFontFamiliesParams {
    pub fn new(font_families: impl Into<super::types::FontFamilies>) -> Self {
        Self {
            font_families: font_families.into(),
            for_scripts: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetFontFamiliesMethod {
    #[serde(rename = "Page.setFontFamilies")]
    SetFontFamilies,
}
#[doc = "Set generic font families.\n[setFontFamilies](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setFontFamilies)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetFontFamilies {
    pub method: SetFontFamiliesMethod,
    pub params: SetFontFamiliesParams,
}
impl SetFontFamilies {
    pub const IDENTIFIER: &'static str = "Page.setFontFamilies";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetFontFamilies {
    type Result = super::results::SetFontFamiliesResult;
}
#[doc = "Set default font sizes.\n[setFontSizes](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setFontSizes)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetFontSizesParams {
    #[doc = "Specifies font sizes to set. If a font size is not specified, it won't be changed."]
    #[serde(rename = "fontSizes")]
    pub font_sizes: super::types::FontSizes,
}
impl SetFontSizesParams {
    pub fn new(font_sizes: impl Into<super::types::FontSizes>) -> Self {
        Self {
            font_sizes: font_sizes.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetFontSizesMethod {
    #[serde(rename = "Page.setFontSizes")]
    SetFontSizes,
}
#[doc = "Set default font sizes.\n[setFontSizes](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setFontSizes)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetFontSizes {
    pub method: SetFontSizesMethod,
    pub params: SetFontSizesParams,
}
impl SetFontSizes {
    pub const IDENTIFIER: &'static str = "Page.setFontSizes";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetFontSizes {
    type Result = super::results::SetFontSizesResult;
}
#[doc = "Sets given markup as the document's HTML.\n[setDocumentContent](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setDocumentContent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDocumentContentParams {
    #[doc = "Frame id to set HTML for."]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[doc = "HTML content to set."]
    #[serde(rename = "html")]
    pub html: String,
}
impl SetDocumentContentParams {
    pub fn new(frame_id: impl Into<super::types::FrameId>, html: impl Into<String>) -> Self {
        Self {
            frame_id: Box::new(frame_id.into()),
            html: html.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDocumentContentMethod {
    #[serde(rename = "Page.setDocumentContent")]
    SetDocumentContent,
}
#[doc = "Sets given markup as the document's HTML.\n[setDocumentContent](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setDocumentContent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDocumentContent {
    pub method: SetDocumentContentMethod,
    pub params: SetDocumentContentParams,
}
impl SetDocumentContent {
    pub const IDENTIFIER: &'static str = "Page.setDocumentContent";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetDocumentContent {
    type Result = super::results::SetDocumentContentResult;
}
#[doc = "Controls whether page will emit lifecycle events.\n[setLifecycleEventsEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setLifecycleEventsEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLifecycleEventsEnabledParams {
    #[doc = "If true, starts emitting lifecycle events."]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
impl SetLifecycleEventsEnabledParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self {
            enabled: enabled.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetLifecycleEventsEnabledMethod {
    #[serde(rename = "Page.setLifecycleEventsEnabled")]
    SetLifecycleEventsEnabled,
}
#[doc = "Controls whether page will emit lifecycle events.\n[setLifecycleEventsEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setLifecycleEventsEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLifecycleEventsEnabled {
    pub method: SetLifecycleEventsEnabledMethod,
    pub params: SetLifecycleEventsEnabledParams,
}
impl SetLifecycleEventsEnabled {
    pub const IDENTIFIER: &'static str = "Page.setLifecycleEventsEnabled";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetLifecycleEventsEnabled {
    type Result = super::results::SetLifecycleEventsEnabledResult;
}
#[doc = "Starts sending each frame using the `screencastFrame` event.\n[startScreencast](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-startScreencast)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartScreencastParams {
    #[doc = "Image compression format."]
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub format: Option<StartScreencastFormat>,
    #[doc = "Compression quality from range [0..100]."]
    #[serde(rename = "quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub quality: Option<i64>,
    #[doc = "Maximum screenshot width."]
    #[serde(rename = "maxWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_width: Option<i64>,
    #[doc = "Maximum screenshot height."]
    #[serde(rename = "maxHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_height: Option<i64>,
    #[doc = "Send every n-th frame."]
    #[serde(rename = "everyNthFrame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub every_nth_frame: Option<i64>,
}
#[doc = "Image compression format."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StartScreencastFormat {
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StartScreencastMethod {
    #[serde(rename = "Page.startScreencast")]
    StartScreencast,
}
#[doc = "Starts sending each frame using the `screencastFrame` event.\n[startScreencast](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-startScreencast)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StartScreencast {
    pub method: StartScreencastMethod,
    pub params: StartScreencastParams,
}
impl StartScreencast {
    pub const IDENTIFIER: &'static str = "Page.startScreencast";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StartScreencast {
    type Result = super::results::StartScreencastResult;
}
#[doc = "Force the page stop all navigations and pending resource fetches.\n[stopLoading](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-stopLoading)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopLoadingParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopLoadingMethod {
    #[serde(rename = "Page.stopLoading")]
    StopLoading,
}
#[doc = "Force the page stop all navigations and pending resource fetches.\n[stopLoading](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-stopLoading)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopLoading {
    pub method: StopLoadingMethod,
    pub params: StopLoadingParams,
}
impl StopLoading {
    pub const IDENTIFIER: &'static str = "Page.stopLoading";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StopLoading {
    type Result = super::results::StopLoadingResult;
}
#[doc = "Crashes renderer on the IO thread, generates minidumps.\n[crash](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-crash)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CrashParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CrashMethod {
    #[serde(rename = "Page.crash")]
    Crash,
}
#[doc = "Crashes renderer on the IO thread, generates minidumps.\n[crash](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-crash)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Crash {
    pub method: CrashMethod,
    pub params: CrashParams,
}
impl Crash {
    pub const IDENTIFIER: &'static str = "Page.crash";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Crash {
    type Result = super::results::CrashResult;
}
#[doc = "Tries to close page, running its beforeunload hooks, if any.\n[close](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-close)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CloseMethod {
    #[serde(rename = "Page.close")]
    Close,
}
#[doc = "Tries to close page, running its beforeunload hooks, if any.\n[close](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-close)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Close {
    pub method: CloseMethod,
    pub params: CloseParams,
}
impl Close {
    pub const IDENTIFIER: &'static str = "Page.close";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Close {
    type Result = super::results::CloseResult;
}
#[doc = "Tries to update the web lifecycle state of the page.\nIt will transition the page to the given state according to:\nhttps://github.com/WICG/web-lifecycle/\n[setWebLifecycleState](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setWebLifecycleState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetWebLifecycleStateParams {
    #[doc = "Target lifecycle state"]
    #[serde(rename = "state")]
    pub state: SetWebLifecycleStateState,
}
#[doc = "Target lifecycle state"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetWebLifecycleStateState {
    #[serde(rename = "frozen")]
    Frozen,
    #[serde(rename = "active")]
    Active,
}
impl SetWebLifecycleStateParams {
    pub fn new(state: impl Into<SetWebLifecycleStateState>) -> Self {
        Self {
            state: state.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetWebLifecycleStateMethod {
    #[serde(rename = "Page.setWebLifecycleState")]
    SetWebLifecycleState,
}
#[doc = "Tries to update the web lifecycle state of the page.\nIt will transition the page to the given state according to:\nhttps://github.com/WICG/web-lifecycle/\n[setWebLifecycleState](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setWebLifecycleState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetWebLifecycleState {
    pub method: SetWebLifecycleStateMethod,
    pub params: SetWebLifecycleStateParams,
}
impl SetWebLifecycleState {
    pub const IDENTIFIER: &'static str = "Page.setWebLifecycleState";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetWebLifecycleState {
    type Result = super::results::SetWebLifecycleStateResult;
}
#[doc = "Stops sending each frame in the `screencastFrame`.\n[stopScreencast](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-stopScreencast)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopScreencastParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StopScreencastMethod {
    #[serde(rename = "Page.stopScreencast")]
    StopScreencast,
}
#[doc = "Stops sending each frame in the `screencastFrame`.\n[stopScreencast](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-stopScreencast)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StopScreencast {
    pub method: StopScreencastMethod,
    pub params: StopScreencastParams,
}
impl StopScreencast {
    pub const IDENTIFIER: &'static str = "Page.stopScreencast";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for StopScreencast {
    type Result = super::results::StopScreencastResult;
}
#[doc = "Requests backend to produce compilation cache for the specified scripts.\n`scripts` are appended to the list of scripts for which the cache\nwould be produced. The list may be reset during page navigation.\nWhen script with a matching URL is encountered, the cache is optionally\nproduced upon backend discretion, based on internal heuristics.\nSee also: `Page.compilationCacheProduced`.\n[produceCompilationCache](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-produceCompilationCache)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProduceCompilationCacheParams {
    #[serde(rename = "scripts")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scripts: Vec<super::types::CompilationCacheParams>,
}
impl ProduceCompilationCacheParams {
    pub fn new(scripts: Vec<super::types::CompilationCacheParams>) -> Self {
        Self { scripts }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProduceCompilationCacheMethod {
    #[serde(rename = "Page.produceCompilationCache")]
    ProduceCompilationCache,
}
#[doc = "Requests backend to produce compilation cache for the specified scripts.\n`scripts` are appended to the list of scripts for which the cache\nwould be produced. The list may be reset during page navigation.\nWhen script with a matching URL is encountered, the cache is optionally\nproduced upon backend discretion, based on internal heuristics.\nSee also: `Page.compilationCacheProduced`.\n[produceCompilationCache](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-produceCompilationCache)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProduceCompilationCache {
    pub method: ProduceCompilationCacheMethod,
    pub params: ProduceCompilationCacheParams,
}
impl ProduceCompilationCache {
    pub const IDENTIFIER: &'static str = "Page.produceCompilationCache";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ProduceCompilationCache {
    type Result = super::results::ProduceCompilationCacheResult;
}
#[doc = "Seeds compilation cache for given url. Compilation cache does not survive\ncross-process navigation.\n[addCompilationCache](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-addCompilationCache)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddCompilationCacheParams {
    #[serde(rename = "url")]
    pub url: String,
    #[doc = "Base64-encoded data"]
    #[serde(rename = "data")]
    pub data: crate::Binary,
}
impl AddCompilationCacheParams {
    pub fn new(url: impl Into<String>, data: impl Into<crate::Binary>) -> Self {
        Self {
            url: url.into(),
            data: data.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddCompilationCacheMethod {
    #[serde(rename = "Page.addCompilationCache")]
    AddCompilationCache,
}
#[doc = "Seeds compilation cache for given url. Compilation cache does not survive\ncross-process navigation.\n[addCompilationCache](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-addCompilationCache)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddCompilationCache {
    pub method: AddCompilationCacheMethod,
    pub params: AddCompilationCacheParams,
}
impl AddCompilationCache {
    pub const IDENTIFIER: &'static str = "Page.addCompilationCache";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for AddCompilationCache {
    type Result = super::results::AddCompilationCacheResult;
}
#[doc = "Clears seeded compilation cache.\n[clearCompilationCache](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-clearCompilationCache)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearCompilationCacheParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearCompilationCacheMethod {
    #[serde(rename = "Page.clearCompilationCache")]
    ClearCompilationCache,
}
#[doc = "Clears seeded compilation cache.\n[clearCompilationCache](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-clearCompilationCache)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearCompilationCache {
    pub method: ClearCompilationCacheMethod,
    pub params: ClearCompilationCacheParams,
}
impl ClearCompilationCache {
    pub const IDENTIFIER: &'static str = "Page.clearCompilationCache";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ClearCompilationCache {
    type Result = super::results::ClearCompilationCacheResult;
}
#[doc = "Sets the Secure Payment Confirmation transaction mode.\nhttps://w3c.github.io/secure-payment-confirmation/#sctn-automation-set-spc-transaction-mode\n[setSPCTransactionMode](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setSPCTransactionMode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSpcTransactionModeParams {
    #[serde(rename = "mode")]
    pub mode: SetSpcTransactionModeMode,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetSpcTransactionModeMode {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "autoAccept")]
    AutoAccept,
    #[serde(rename = "autoChooseToAuthAnotherWay")]
    AutoChooseToAuthAnotherWay,
    #[serde(rename = "autoReject")]
    AutoReject,
    #[serde(rename = "autoOptOut")]
    AutoOptOut,
}
impl SetSpcTransactionModeParams {
    pub fn new(mode: impl Into<SetSpcTransactionModeMode>) -> Self {
        Self { mode: mode.into() }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetSpcTransactionModeMethod {
    #[serde(rename = "Page.setSPCTransactionMode")]
    SetSpcTransactionMode,
}
#[doc = "Sets the Secure Payment Confirmation transaction mode.\nhttps://w3c.github.io/secure-payment-confirmation/#sctn-automation-set-spc-transaction-mode\n[setSPCTransactionMode](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setSPCTransactionMode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSpcTransactionMode {
    pub method: SetSpcTransactionModeMethod,
    pub params: SetSpcTransactionModeParams,
}
impl SetSpcTransactionMode {
    pub const IDENTIFIER: &'static str = "Page.setSPCTransactionMode";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetSpcTransactionMode {
    type Result = super::results::SetSpcTransactionModeResult;
}
#[doc = "Extensions for Custom Handlers API:\nhttps://html.spec.whatwg.org/multipage/system-state.html#rph-automation\n[setRPHRegistrationMode](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setRPHRegistrationMode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRphRegistrationModeParams {
    #[serde(rename = "mode")]
    pub mode: SetRphRegistrationModeMode,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetRphRegistrationModeMode {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "autoAccept")]
    AutoAccept,
    #[serde(rename = "autoReject")]
    AutoReject,
}
impl SetRphRegistrationModeParams {
    pub fn new(mode: impl Into<SetRphRegistrationModeMode>) -> Self {
        Self { mode: mode.into() }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetRphRegistrationModeMethod {
    #[serde(rename = "Page.setRPHRegistrationMode")]
    SetRphRegistrationMode,
}
#[doc = "Extensions for Custom Handlers API:\nhttps://html.spec.whatwg.org/multipage/system-state.html#rph-automation\n[setRPHRegistrationMode](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setRPHRegistrationMode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetRphRegistrationMode {
    pub method: SetRphRegistrationModeMethod,
    pub params: SetRphRegistrationModeParams,
}
impl SetRphRegistrationMode {
    pub const IDENTIFIER: &'static str = "Page.setRPHRegistrationMode";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetRphRegistrationMode {
    type Result = super::results::SetRphRegistrationModeResult;
}
#[doc = "Generates a report for testing.\n[generateTestReport](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-generateTestReport)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateTestReportParams {
    #[doc = "Message to be displayed in the report."]
    #[serde(rename = "message")]
    pub message: String,
    #[doc = "Specifies the endpoint group to deliver the report to."]
    #[serde(rename = "group")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub group: Option<String>,
}
impl GenerateTestReportParams {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            group: None,
        }
    }
}
impl<T: Into<String>> From<T> for GenerateTestReportParams {
    fn from(url: T) -> Self {
        GenerateTestReportParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GenerateTestReportMethod {
    #[serde(rename = "Page.generateTestReport")]
    GenerateTestReport,
}
#[doc = "Generates a report for testing.\n[generateTestReport](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-generateTestReport)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenerateTestReport {
    pub method: GenerateTestReportMethod,
    pub params: GenerateTestReportParams,
}
impl GenerateTestReport {
    pub const IDENTIFIER: &'static str = "Page.generateTestReport";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GenerateTestReport {
    type Result = super::results::GenerateTestReportResult;
}
#[doc = "Pauses page execution. Can be resumed using generic Runtime.runIfWaitingForDebugger.\n[waitForDebugger](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-waitForDebugger)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaitForDebuggerParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WaitForDebuggerMethod {
    #[serde(rename = "Page.waitForDebugger")]
    WaitForDebugger,
}
#[doc = "Pauses page execution. Can be resumed using generic Runtime.runIfWaitingForDebugger.\n[waitForDebugger](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-waitForDebugger)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WaitForDebugger {
    pub method: WaitForDebuggerMethod,
    pub params: WaitForDebuggerParams,
}
impl WaitForDebugger {
    pub const IDENTIFIER: &'static str = "Page.waitForDebugger";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for WaitForDebugger {
    type Result = super::results::WaitForDebuggerResult;
}
#[doc = "Intercept file chooser requests and transfer control to protocol clients.\nWhen file chooser interception is enabled, native file chooser dialog is not shown.\nInstead, a protocol event `Page.fileChooserOpened` is emitted.\n[setInterceptFileChooserDialog](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setInterceptFileChooserDialog)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInterceptFileChooserDialogParams {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[doc = "If true, cancels the dialog by emitting relevant events (if any)\nin addition to not showing it if the interception is enabled\n(default: false)."]
    #[serde(rename = "cancel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub cancel: Option<bool>,
}
impl SetInterceptFileChooserDialogParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self {
            enabled: enabled.into(),
            cancel: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetInterceptFileChooserDialogMethod {
    #[serde(rename = "Page.setInterceptFileChooserDialog")]
    SetInterceptFileChooserDialog,
}
#[doc = "Intercept file chooser requests and transfer control to protocol clients.\nWhen file chooser interception is enabled, native file chooser dialog is not shown.\nInstead, a protocol event `Page.fileChooserOpened` is emitted.\n[setInterceptFileChooserDialog](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setInterceptFileChooserDialog)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInterceptFileChooserDialog {
    pub method: SetInterceptFileChooserDialogMethod,
    pub params: SetInterceptFileChooserDialogParams,
}
impl SetInterceptFileChooserDialog {
    pub const IDENTIFIER: &'static str = "Page.setInterceptFileChooserDialog";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetInterceptFileChooserDialog {
    type Result = super::results::SetInterceptFileChooserDialogResult;
}
#[doc = "Enable/disable prerendering manually.\n\nThis command is a short-term solution for https://crbug.com/1440085.\nSee https://docs.google.com/document/d/12HVmFxYj5Jc-eJr5OmWsa2bqTJsbgGLKI6ZIyx0_wpA\nfor more details.\n\nTODO(https://crbug.com/1440085): Remove this once Puppeteer supports tab targets.\n[setPrerenderingAllowed](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setPrerenderingAllowed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPrerenderingAllowedParams {
    #[serde(rename = "isAllowed")]
    pub is_allowed: bool,
}
impl SetPrerenderingAllowedParams {
    pub fn new(is_allowed: impl Into<bool>) -> Self {
        Self {
            is_allowed: is_allowed.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetPrerenderingAllowedMethod {
    #[serde(rename = "Page.setPrerenderingAllowed")]
    SetPrerenderingAllowed,
}
#[doc = "Enable/disable prerendering manually.\n\nThis command is a short-term solution for https://crbug.com/1440085.\nSee https://docs.google.com/document/d/12HVmFxYj5Jc-eJr5OmWsa2bqTJsbgGLKI6ZIyx0_wpA\nfor more details.\n\nTODO(https://crbug.com/1440085): Remove this once Puppeteer supports tab targets.\n[setPrerenderingAllowed](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-setPrerenderingAllowed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPrerenderingAllowed {
    pub method: SetPrerenderingAllowedMethod,
    pub params: SetPrerenderingAllowedParams,
}
impl SetPrerenderingAllowed {
    pub const IDENTIFIER: &'static str = "Page.setPrerenderingAllowed";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetPrerenderingAllowed {
    type Result = super::results::SetPrerenderingAllowedResult;
}
#[doc = "Get the annotated page content for the main frame.\nThis is an experimental command that is subject to change.\n[getAnnotatedPageContent](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getAnnotatedPageContent)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetAnnotatedPageContentParams {
    #[doc = "Whether to include actionable information. Defaults to true."]
    #[serde(rename = "includeActionableInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_actionable_information: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetAnnotatedPageContentMethod {
    #[serde(rename = "Page.getAnnotatedPageContent")]
    GetAnnotatedPageContent,
}
#[doc = "Get the annotated page content for the main frame.\nThis is an experimental command that is subject to change.\n[getAnnotatedPageContent](https://chromedevtools.github.io/devtools-protocol/tot/Page/#method-getAnnotatedPageContent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAnnotatedPageContent {
    pub method: GetAnnotatedPageContentMethod,
    pub params: GetAnnotatedPageContentParams,
}
impl GetAnnotatedPageContent {
    pub const IDENTIFIER: &'static str = "Page.getAnnotatedPageContent";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetAnnotatedPageContent {
    type Result = super::results::GetAnnotatedPageContentResult;
}
group_enum ! (PageCommands { AddScriptToEvaluateOnNewDocument (AddScriptToEvaluateOnNewDocument) , BringToFront (BringToFront) , CaptureScreenshot (CaptureScreenshot) , CaptureSnapshot (CaptureSnapshot) , CreateIsolatedWorld (CreateIsolatedWorld) , Disable (Disable) , Enable (Enable) , GetAppManifest (GetAppManifest) , GetInstallabilityErrors (GetInstallabilityErrors) , GetAppId (GetAppId) , GetAdScriptAncestry (GetAdScriptAncestry) , GetFrameTree (GetFrameTree) , GetLayoutMetrics (GetLayoutMetrics) , GetNavigationHistory (GetNavigationHistory) , ResetNavigationHistory (ResetNavigationHistory) , GetResourceContent (GetResourceContent) , GetResourceTree (GetResourceTree) , HandleJavaScriptDialog (HandleJavaScriptDialog) , Navigate (Navigate) , NavigateToHistoryEntry (NavigateToHistoryEntry) , PrintToPdf (PrintToPdf) , Reload (Reload) , RemoveScriptToEvaluateOnNewDocument (RemoveScriptToEvaluateOnNewDocument) , ScreencastFrameAck (ScreencastFrameAck) , SearchInResource (SearchInResource) , SetAdBlockingEnabled (SetAdBlockingEnabled) , SetBypassCsp (SetBypassCsp) , GetPermissionsPolicyState (GetPermissionsPolicyState) , GetOriginTrials (GetOriginTrials) , SetFontFamilies (SetFontFamilies) , SetFontSizes (SetFontSizes) , SetDocumentContent (SetDocumentContent) , SetLifecycleEventsEnabled (SetLifecycleEventsEnabled) , StartScreencast (StartScreencast) , StopLoading (StopLoading) , Crash (Crash) , Close (Close) , SetWebLifecycleState (SetWebLifecycleState) , StopScreencast (StopScreencast) , ProduceCompilationCache (ProduceCompilationCache) , AddCompilationCache (AddCompilationCache) , ClearCompilationCache (ClearCompilationCache) , SetSpcTransactionMode (SetSpcTransactionMode) , SetRphRegistrationMode (SetRphRegistrationMode) , GenerateTestReport (GenerateTestReport) , WaitForDebugger (WaitForDebugger) , SetInterceptFileChooserDialog (SetInterceptFileChooserDialog) , SetPrerenderingAllowed (SetPrerenderingAllowed) , GetAnnotatedPageContent (GetAnnotatedPageContent) });
