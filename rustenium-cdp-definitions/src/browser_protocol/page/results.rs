use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddScriptToEvaluateOnLoadReturns {
    #[doc = "Identifier of the added script."]
    #[serde(rename = "identifier")]
    pub identifier: super::types::ScriptIdentifier,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddScriptToEvaluateOnNewDocumentReturns {
    #[doc = "Identifier of the added script."]
    #[serde(rename = "identifier")]
    pub identifier: super::types::ScriptIdentifier,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureScreenshotReturns {
    #[doc = "Base64-encoded image data."]
    #[serde(rename = "data")]
    pub data: super::super::super::Binary,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureSnapshotReturns {
    #[doc = "Serialized page data."]
    #[serde(rename = "data")]
    pub data: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateIsolatedWorldReturns {
    #[doc = "Execution context of the isolated world."]
    #[serde(rename = "executionContextId")]
    pub execution_context_id: super::super::super::js_protocol::runtime::types::ExecutionContextId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAppManifestReturns {
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
pub struct GetInstallabilityErrorsReturns {
    #[serde(rename = "installabilityErrors")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub installability_errors: Vec<super::types::InstallabilityError>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetManifestIconsReturns {
    #[serde(rename = "primaryIcon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub primary_icon: Option<super::super::super::Binary>,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetAppIdReturns {
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
pub struct GetAdScriptAncestryReturns {
    #[doc = "The ancestry chain of ad script identifiers leading to this frame's\ncreation, along with the root script's filterlist rule. The ancestry\nchain is ordered from the most immediate script (in the frame creation\nstack) to more distant ancestors (that created the immediately preceding\nscript). Only sent if frame is labelled as an ad and ids are available."]
    #[serde(rename = "adScriptAncestry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ad_script_ancestry: Option<super::types::AdScriptAncestry>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetFrameTreeReturns {
    #[doc = "Present frame tree structure."]
    #[serde(rename = "frameTree")]
    pub frame_tree: super::types::FrameTree,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetLayoutMetricsReturns {
    #[doc = "Metrics relating to the layout viewport in CSS pixels."]
    #[serde(rename = "cssLayoutViewport")]
    pub css_layout_viewport: super::types::LayoutViewport,
    #[doc = "Metrics relating to the visual viewport in CSS pixels."]
    #[serde(rename = "cssVisualViewport")]
    pub css_visual_viewport: super::types::VisualViewport,
    #[doc = "Size of scrollable area in CSS pixels."]
    #[serde(rename = "cssContentSize")]
    pub css_content_size: super::super::dom::types::Rect,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetNavigationHistoryReturns {
    #[doc = "Index of the current navigation history entry."]
    #[serde(rename = "currentIndex")]
    pub current_index: i64,
    #[doc = "Array of navigation history entries."]
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<super::types::NavigationEntry>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResourceContentReturns {
    #[doc = "Resource content."]
    #[serde(rename = "content")]
    pub content: String,
    #[doc = "True, if content was served as base64."]
    #[serde(rename = "base64Encoded")]
    pub base64_encoded: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetResourceTreeReturns {
    #[doc = "Present frame / resource tree structure."]
    #[serde(rename = "frameTree")]
    pub frame_tree: super::types::FrameResourceTree,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigateReturns {
    #[doc = "Frame id that has navigated (or failed to navigate)"]
    #[serde(rename = "frameId")]
    pub frame_id: super::types::FrameId,
    #[doc = "Loader identifier. This is omitted in case of same-document navigation,\nas the previously committed loaderId would not change."]
    #[serde(rename = "loaderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub loader_id: Option<super::super::network::types::LoaderId>,
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintToPdfReturns {
    #[doc = "Base64-encoded pdf data. Empty if |returnAsStream| is specified."]
    #[serde(rename = "data")]
    pub data: super::super::super::Binary,
    #[doc = "A handle of the stream that holds resulting PDF data."]
    #[serde(rename = "stream")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stream: Option<super::super::io::types::StreamHandle>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchInResourceReturns {
    #[doc = "List of search matches."]
    #[serde(rename = "result")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub result: Vec<super::super::super::js_protocol::debugger::types::SearchMatch>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetPermissionsPolicyStateReturns {
    #[serde(rename = "states")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub states: Vec<super::types::PermissionsPolicyFeatureState>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOriginTrialsReturns {
    #[serde(rename = "originTrials")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub origin_trials: Vec<super::types::OriginTrial>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetAnnotatedPageContentReturns {
    #[doc = "The annotated page content as a base64 encoded protobuf.\nThe format is defined by the `AnnotatedPageContent` message in\ncomponents/optimization_guide/proto/features/common_quality_data.proto"]
    #[serde(rename = "content")]
    pub content: super::super::super::Binary,
}
