use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPermissionResult {}
impl TryFrom<serde_json::Value> for SetPermissionResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GrantPermissionsResult {}
impl TryFrom<serde_json::Value> for GrantPermissionsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResetPermissionsResult {}
impl TryFrom<serde_json::Value> for ResetPermissionsResult {
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
pub struct CancelDownloadResult {}
impl TryFrom<serde_json::Value> for CancelDownloadResult {
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
pub struct CrashResult {}
impl TryFrom<serde_json::Value> for CrashResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CrashGpuProcessResult {}
impl TryFrom<serde_json::Value> for CrashGpuProcessResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
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
impl TryFrom<serde_json::Value> for GetVersionResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetBrowserCommandLineResult {
    #[doc = "Commandline parameters"]
    #[serde(rename = "arguments")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<String>,
}
impl TryFrom<serde_json::Value> for GetBrowserCommandLineResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHistogramsResult {
    #[doc = "Histograms."]
    #[serde(rename = "histograms")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub histograms: Vec<super::types::Histogram>,
}
impl TryFrom<serde_json::Value> for GetHistogramsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHistogramResult {
    #[doc = "Histogram."]
    #[serde(rename = "histogram")]
    pub histogram: super::types::Histogram,
}
impl TryFrom<serde_json::Value> for GetHistogramResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWindowBoundsResult {
    #[doc = "Bounds information of the window. When window state is 'minimized', the restored window\nposition and size are returned."]
    #[serde(rename = "bounds")]
    pub bounds: super::types::Bounds,
}
impl TryFrom<serde_json::Value> for GetWindowBoundsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
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
impl TryFrom<serde_json::Value> for GetWindowForTargetResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetWindowBoundsResult {}
impl TryFrom<serde_json::Value> for SetWindowBoundsResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetContentsSizeResult {}
impl TryFrom<serde_json::Value> for SetContentsSizeResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDockTileResult {}
impl TryFrom<serde_json::Value> for SetDockTileResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ExecuteBrowserCommandResult {}
impl TryFrom<serde_json::Value> for ExecuteBrowserCommandResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddPrivacySandboxEnrollmentOverrideResult {}
impl TryFrom<serde_json::Value> for AddPrivacySandboxEnrollmentOverrideResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AddPrivacySandboxCoordinatorKeyConfigResult {}
impl TryFrom<serde_json::Value> for AddPrivacySandboxCoordinatorKeyConfigResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
