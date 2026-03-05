use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HideHighlightResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HighlightNodeResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HighlightQuadResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HighlightRectResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HighlightSourceOrderResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetInspectModeResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowAdHighlightsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPausedInDebuggerMessageResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowDebugBordersResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowFpsCounterResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowGridOverlaysResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowFlexOverlaysResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowScrollSnapOverlaysResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowContainerQueryOverlaysResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowInspectedElementAnchorResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowPaintRectsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowLayoutShiftRegionsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowScrollBottleneckRectsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowViewportSizeOnResizeResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowHingeResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowIsolatedElementsResult {}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowWindowControlsOverlayResult {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHighlightObjectForTestResult {
    #[doc = "Highlight data for the node."]
    #[serde(rename = "highlight")]
    pub highlight: serde_json::Value,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridHighlightObjectsForTestResult {
    #[doc = "Grid Highlight data for the node ids provided."]
    #[serde(rename = "highlights")]
    pub highlights: serde_json::Value,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSourceOrderHighlightObjectForTestResult {
    #[doc = "Source order highlight data for the node id provided."]
    #[serde(rename = "highlight")]
    pub highlight: serde_json::Value,
}
