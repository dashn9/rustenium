use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddScriptToEvaluateOnLoadResult {
    #[doc = "Identifier of the added script."]
    #[serde(rename = "identifier")]
    pub identifier: super::types::ScriptIdentifier,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddScriptToEvaluateOnNewDocumentResult {
    #[doc = "Identifier of the added script."]
    #[serde(rename = "identifier")]
    pub identifier: super::types::ScriptIdentifier,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BringToFrontResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureScreenshotResult {
    #[doc = "Base64-encoded image data."]
    #[serde(rename = "data")]
    pub data: crate::Binary,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureSnapshotResult {
    #[doc = "Serialized page data."]
    #[serde(rename = "data")]
    pub data: String,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearDeviceMetricsOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearDeviceOrientationOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearGeolocationOverrideResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateIsolatedWorldResult {
    #[doc = "Execution context of the isolated world."]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: crate::js_protocol::runtime::types::ExecutionContextId,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteCookieResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAppManifestResult {
    #[doc = "Manifest location."]
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<super::types::AppManifestError>,
    #[doc = "Manifest content."]
    #[serde(rename = "data")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<String>,
    #[serde(rename = "manifest")]
    pub manifest: super::types::WebAppManifest,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInstallabilityErrorsResult {
    #[serde(rename = "installabilityErrors")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub installability_errors: Vec<super::types::InstallabilityError>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetManifestIconsResult {
    #[serde(rename = "primaryIcon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub primary_icon: Option<crate::Binary>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetAppIdResult {
    #[doc = "App id, either from manifest's id attribute or computed from start_url"]
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub app_id: Option<String>,
    #[doc = "Recommendation for manifest's id attribute to match current id computed from start_url"]
    #[serde(rename = "recommendedId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub recommended_id: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetAdScriptAncestryResult {
    #[doc = "The ancestry chain of ad script identifiers leading to this frame's\ncreation, along with the root script's filterlist rule. The ancestry\nchain is ordered from the most immediate script (in the frame creation\nstack) to more distant ancestors (that created the immediately preceding\nscript). Only sent if frame is labelled as an ad and ids are available."]
    #[serde(rename = "adScriptAncestry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ad_script_ancestry: Option<super::types::AdScriptAncestry>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFrameTreeResult {
    #[doc = "Present frame tree structure."]
    #[serde(rename = "frameTree")]
    pub frame_tree: super::types::FrameTree,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLayoutMetricsResult {
    #[doc = "Metrics relating to the layout viewport in CSS pixels."]
    #[serde(rename = "cssLayoutViewport")]
    pub css_layout_viewport: super::types::LayoutViewport,
    #[doc = "Metrics relating to the visual viewport in CSS pixels."]
    #[serde(rename = "cssVisualViewport")]
    pub css_visual_viewport: super::types::VisualViewport,
    #[doc = "Size of scrollable area in CSS pixels."]
    #[serde(rename = "cssContentSize")]
    pub css_content_size: crate::browser_protocol::dom::types::Rect,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNavigationHistoryResult {
    #[doc = "Index of the current navigation history entry."]
    #[serde(rename = "currentIndex")]
    pub current_index: i64,
    #[doc = "Array of navigation history entries."]
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<super::types::NavigationEntry>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResetNavigationHistoryResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResourceContentResult {
    #[doc = "Resource content."]
    #[serde(rename = "content")]
    pub content: String,
    #[doc = "True, if content was served as base64."]
    #[serde(rename = "base64Encoded")]
    pub base64_encoded: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResourceTreeResult {
    #[doc = "Present frame / resource tree structure."]
    #[serde(rename = "frameTree")]
    pub frame_tree: super::types::FrameResourceTree,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HandleJavaScriptDialogResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigateResult {
    #[doc = "Frame id that has navigated (or failed to navigate)"]
    #[serde(rename = "frameId")]
    pub frame_id: Box<super::types::FrameId>,
    #[doc = "Loader identifier. This is omitted in case of same-document navigation,\nas the previously committed loaderId would not change."]
    #[serde(rename = "loaderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub loader_id: Option<crate::browser_protocol::network::types::LoaderId>,
    #[doc = "User friendly error message, present if and only if navigation has failed."]
    #[serde(rename = "errorText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub error_text: Option<String>,
    #[doc = "Whether the navigation resulted in a download."]
    #[serde(rename = "isDownload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_download: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NavigateToHistoryEntryResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintToPdfResult {
    #[doc = "Base64-encoded pdf data. Empty if |returnAsStream| is specified."]
    #[serde(rename = "data")]
    pub data: crate::Binary,
    #[doc = "A handle of the stream that holds resulting PDF data."]
    #[serde(rename = "stream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stream: Option<crate::browser_protocol::io::types::StreamHandle>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ReloadResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveScriptToEvaluateOnLoadResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveScriptToEvaluateOnNewDocumentResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScreencastFrameAckResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchInResourceResult {
    #[doc = "List of search matches."]
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<crate::js_protocol::debugger::types::SearchMatch>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAdBlockingEnabledResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetBypassCspResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPermissionsPolicyStateResult {
    #[serde(rename = "states")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub states: Vec<super::types::PermissionsPolicyFeatureState>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOriginTrialsResult {
    #[serde(rename = "originTrials")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub origin_trials: Vec<super::types::OriginTrial>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDeviceMetricsOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDeviceOrientationOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetFontFamiliesResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetFontSizesResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDocumentContentResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDownloadBehaviorResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetGeolocationOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetLifecycleEventsEnabledResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetTouchEmulationEnabledResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartScreencastResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopLoadingResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CrashResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CloseResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetWebLifecycleStateResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopScreencastResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ProduceCompilationCacheResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddCompilationCacheResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearCompilationCacheResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSpcTransactionModeResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetRphRegistrationModeResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GenerateTestReportResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WaitForDebuggerResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetInterceptFileChooserDialogResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPrerenderingAllowedResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAnnotatedPageContentResult {
    #[doc = "The annotated page content as a base64 encoded protobuf.\nThe format is defined by the `AnnotatedPageContent` message in\ncomponents/optimization_guide/proto/features/common_quality_data.proto"]
    #[serde(rename = "content")]
    pub content: crate::Binary,
}
