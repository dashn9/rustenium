use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetVersionReturns {
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
pub struct GetBrowserCommandLineReturns {
    #[doc = "Commandline parameters"]
    #[serde(rename = "arguments")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHistogramsReturns {
    #[doc = "Histograms."]
    #[serde(rename = "histograms")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub histograms: Vec<super::types::Histogram>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHistogramReturns {
    #[doc = "Histogram."]
    #[serde(rename = "histogram")]
    pub histogram: super::types::Histogram,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWindowBoundsReturns {
    #[doc = "Bounds information of the window. When window state is 'minimized', the restored window\nposition and size are returned."]
    #[serde(rename = "bounds")]
    pub bounds: super::types::Bounds,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWindowForTargetReturns {
    #[doc = "Browser window id."]
    #[serde(rename = "windowId")]
    pub window_id: super::types::WindowId,
    #[doc = "Bounds information of the window. When window state is 'minimized', the restored window\nposition and size are returned."]
    #[serde(rename = "bounds")]
    pub bounds: super::types::Bounds,
}
