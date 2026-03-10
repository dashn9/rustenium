use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddScriptToEvaluateOnLoadResult {
    #[doc = "Identifier of the added script."]
    #[serde(rename = "identifier")]
    pub identifier: super::types::ScriptIdentifier,
}
impl TryFrom<serde_json::Value> for AddScriptToEvaluateOnLoadResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddScriptToEvaluateOnNewDocumentResult {
    #[doc = "Identifier of the added script."]
    #[serde(rename = "identifier")]
    pub identifier: super::types::ScriptIdentifier,
}
impl TryFrom<serde_json::Value> for AddScriptToEvaluateOnNewDocumentResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BringToFrontResult {}
impl TryFrom<serde_json::Value> for BringToFrontResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureScreenshotResult {
    #[doc = "Base64-encoded image data."]
    #[serde(rename = "data")]
    pub data: crate::Binary,
}
impl TryFrom<serde_json::Value> for CaptureScreenshotResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureSnapshotResult {
    #[doc = "Serialized page data."]
    #[serde(rename = "data")]
    pub data: String,
}
impl TryFrom<serde_json::Value> for CaptureSnapshotResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearDeviceMetricsOverrideResult {}
impl TryFrom<serde_json::Value> for ClearDeviceMetricsOverrideResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearDeviceOrientationOverrideResult {}
impl TryFrom<serde_json::Value> for ClearDeviceOrientationOverrideResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearGeolocationOverrideResult {}
impl TryFrom<serde_json::Value> for ClearGeolocationOverrideResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateIsolatedWorldResult {
    #[doc = "Execution context of the isolated world."]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: crate::js_protocol::runtime::types::ExecutionContextId,
}
impl TryFrom<serde_json::Value> for CreateIsolatedWorldResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DeleteCookieResult {}
impl TryFrom<serde_json::Value> for DeleteCookieResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
impl TryFrom<serde_json::Value> for DisableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
impl TryFrom<serde_json::Value> for EnableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
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
impl TryFrom<serde_json::Value> for GetAppManifestResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetInstallabilityErrorsResult {
    #[serde(rename = "installabilityErrors")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub installability_errors: Vec<super::types::InstallabilityError>,
}
impl TryFrom<serde_json::Value> for GetInstallabilityErrorsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetManifestIconsResult {
    #[serde(rename = "primaryIcon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub primary_icon: Option<crate::Binary>,
}
impl TryFrom<serde_json::Value> for GetManifestIconsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
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
impl TryFrom<serde_json::Value> for GetAppIdResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetAdScriptAncestryResult {
    #[doc = "The ancestry chain of ad script identifiers leading to this frame's\ncreation, along with the root script's filterlist rule. The ancestry\nchain is ordered from the most immediate script (in the frame creation\nstack) to more distant ancestors (that created the immediately preceding\nscript). Only sent if frame is labelled as an ad and ids are available."]
    #[serde(rename = "adScriptAncestry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ad_script_ancestry: Option<super::types::AdScriptAncestry>,
}
impl TryFrom<serde_json::Value> for GetAdScriptAncestryResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFrameTreeResult {
    #[doc = "Present frame tree structure."]
    #[serde(rename = "frameTree")]
    pub frame_tree: super::types::FrameTree,
}
impl TryFrom<serde_json::Value> for GetFrameTreeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
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
impl TryFrom<serde_json::Value> for GetLayoutMetricsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
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
impl TryFrom<serde_json::Value> for GetNavigationHistoryResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResetNavigationHistoryResult {}
impl TryFrom<serde_json::Value> for ResetNavigationHistoryResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResourceContentResult {
    #[doc = "Resource content."]
    #[serde(rename = "content")]
    pub content: String,
    #[doc = "True, if content was served as base64."]
    #[serde(rename = "base64Encoded")]
    pub base64_encoded: bool,
}
impl TryFrom<serde_json::Value> for GetResourceContentResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResourceTreeResult {
    #[doc = "Present frame / resource tree structure."]
    #[serde(rename = "frameTree")]
    pub frame_tree: super::types::FrameResourceTree,
}
impl TryFrom<serde_json::Value> for GetResourceTreeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HandleJavaScriptDialogResult {}
impl TryFrom<serde_json::Value> for HandleJavaScriptDialogResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
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
impl TryFrom<serde_json::Value> for NavigateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NavigateToHistoryEntryResult {}
impl TryFrom<serde_json::Value> for NavigateToHistoryEntryResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
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
impl TryFrom<serde_json::Value> for PrintToPdfResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ReloadResult {}
impl TryFrom<serde_json::Value> for ReloadResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveScriptToEvaluateOnLoadResult {}
impl TryFrom<serde_json::Value> for RemoveScriptToEvaluateOnLoadResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoveScriptToEvaluateOnNewDocumentResult {}
impl TryFrom<serde_json::Value> for RemoveScriptToEvaluateOnNewDocumentResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScreencastFrameAckResult {}
impl TryFrom<serde_json::Value> for ScreencastFrameAckResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchInResourceResult {
    #[doc = "List of search matches."]
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<crate::js_protocol::debugger::types::SearchMatch>,
}
impl TryFrom<serde_json::Value> for SearchInResourceResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAdBlockingEnabledResult {}
impl TryFrom<serde_json::Value> for SetAdBlockingEnabledResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetBypassCspResult {}
impl TryFrom<serde_json::Value> for SetBypassCspResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPermissionsPolicyStateResult {
    #[serde(rename = "states")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub states: Vec<super::types::PermissionsPolicyFeatureState>,
}
impl TryFrom<serde_json::Value> for GetPermissionsPolicyStateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOriginTrialsResult {
    #[serde(rename = "originTrials")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub origin_trials: Vec<super::types::OriginTrial>,
}
impl TryFrom<serde_json::Value> for GetOriginTrialsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDeviceMetricsOverrideResult {}
impl TryFrom<serde_json::Value> for SetDeviceMetricsOverrideResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDeviceOrientationOverrideResult {}
impl TryFrom<serde_json::Value> for SetDeviceOrientationOverrideResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetFontFamiliesResult {}
impl TryFrom<serde_json::Value> for SetFontFamiliesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetFontSizesResult {}
impl TryFrom<serde_json::Value> for SetFontSizesResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDocumentContentResult {}
impl TryFrom<serde_json::Value> for SetDocumentContentResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDownloadBehaviorResult {}
impl TryFrom<serde_json::Value> for SetDownloadBehaviorResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetGeolocationOverrideResult {}
impl TryFrom<serde_json::Value> for SetGeolocationOverrideResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetLifecycleEventsEnabledResult {}
impl TryFrom<serde_json::Value> for SetLifecycleEventsEnabledResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetTouchEmulationEnabledResult {}
impl TryFrom<serde_json::Value> for SetTouchEmulationEnabledResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StartScreencastResult {}
impl TryFrom<serde_json::Value> for StartScreencastResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopLoadingResult {}
impl TryFrom<serde_json::Value> for StopLoadingResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CrashResult {}
impl TryFrom<serde_json::Value> for CrashResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CloseResult {}
impl TryFrom<serde_json::Value> for CloseResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetWebLifecycleStateResult {}
impl TryFrom<serde_json::Value> for SetWebLifecycleStateResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StopScreencastResult {}
impl TryFrom<serde_json::Value> for StopScreencastResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ProduceCompilationCacheResult {}
impl TryFrom<serde_json::Value> for ProduceCompilationCacheResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddCompilationCacheResult {}
impl TryFrom<serde_json::Value> for AddCompilationCacheResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearCompilationCacheResult {}
impl TryFrom<serde_json::Value> for ClearCompilationCacheResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetSpcTransactionModeResult {}
impl TryFrom<serde_json::Value> for SetSpcTransactionModeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetRphRegistrationModeResult {}
impl TryFrom<serde_json::Value> for SetRphRegistrationModeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GenerateTestReportResult {}
impl TryFrom<serde_json::Value> for GenerateTestReportResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WaitForDebuggerResult {}
impl TryFrom<serde_json::Value> for WaitForDebuggerResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetInterceptFileChooserDialogResult {}
impl TryFrom<serde_json::Value> for SetInterceptFileChooserDialogResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPrerenderingAllowedResult {}
impl TryFrom<serde_json::Value> for SetPrerenderingAllowedResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAnnotatedPageContentResult {
    #[doc = "The annotated page content as a base64 encoded protobuf.\nThe format is defined by the `AnnotatedPageContent` message in\ncomponents/optimization_guide/proto/features/common_quality_data.proto"]
    #[serde(rename = "content")]
    pub content: crate::Binary,
}
impl TryFrom<serde_json::Value> for GetAnnotatedPageContentResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
