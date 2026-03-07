use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPermissionResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GrantPermissionsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResetPermissionsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDownloadBehaviorResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CancelDownloadResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CloseResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CrashResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CrashGpuProcessResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVersionResult {
    #[doc = "Protocol version."]
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    #[doc = "Product name."]
    #[serde(rename = "product")]
    pub product: String,
    #[doc = "Product revision."]
    #[serde(rename = "revision")]
    pub revision: String,
    #[doc = "User-Agent."]
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    #[doc = "V8 version."]
    #[serde(rename = "jsVersion")]
    pub js_version: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBrowserCommandLineResult {
    #[doc = "Commandline parameters"]
    #[serde(rename = "arguments")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHistogramsResult {
    #[doc = "Histograms."]
    #[serde(rename = "histograms")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub histograms: Vec<super::types::Histogram>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHistogramResult {
    #[doc = "Histogram."]
    #[serde(rename = "histogram")]
    pub histogram: super::types::Histogram,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWindowBoundsResult {
    #[doc = "Bounds information of the window. When window state is 'minimized', the restored window\nposition and size are returned."]
    #[serde(rename = "bounds")]
    pub bounds: super::types::Bounds,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWindowForTargetResult {
    #[doc = "Browser window id."]
    #[serde(rename = "windowId")]
    pub window_id: super::types::WindowId,
    #[doc = "Bounds information of the window. When window state is 'minimized', the restored window\nposition and size are returned."]
    #[serde(rename = "bounds")]
    pub bounds: super::types::Bounds,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetWindowBoundsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetContentsSizeResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDockTileResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExecuteBrowserCommandResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddPrivacySandboxEnrollmentOverrideResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddPrivacySandboxCoordinatorKeyConfigResult {}
