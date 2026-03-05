use serde::{Deserialize, Serialize};
#[doc = "Disables domain notifications.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "Overlay.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "Overlay.disable";
}
#[doc = "Disables domain notifications.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl super::super::super::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables domain notifications.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "Overlay.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "Overlay.enable";
}
#[doc = "Enables domain notifications.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl super::super::super::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "For testing.\n[getHighlightObjectForTest](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-getHighlightObjectForTest)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHighlightObjectForTestParams {
    #[doc = "Id of the node to get highlight object for."]
    #[serde(rename = "nodeId")]
    pub node_id: super::super::dom::types::NodeId,
    #[doc = "Whether to include distance info."]
    #[serde(rename = "includeDistance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_distance: Option<bool>,
    #[doc = "Whether to include style info."]
    #[serde(rename = "includeStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_style: Option<bool>,
    #[doc = "The color format to get config with (default: hex)."]
    #[serde(rename = "colorFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub color_format: Option<super::types::ColorFormat>,
    #[doc = "Whether to show accessibility info (default: true)."]
    #[serde(rename = "showAccessibilityInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub show_accessibility_info: Option<bool>,
}
impl GetHighlightObjectForTestParams {
    pub fn new(node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
            include_distance: None,
            include_style: None,
            color_format: None,
            show_accessibility_info: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetHighlightObjectForTestMethod {
    #[serde(rename = "Overlay.getHighlightObjectForTest")]
    GetHighlightObjectForTest,
}
impl GetHighlightObjectForTestMethod {
    pub const IDENTIFIER: &'static str = "Overlay.getHighlightObjectForTest";
}
#[doc = "For testing.\n[getHighlightObjectForTest](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-getHighlightObjectForTest)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetHighlightObjectForTest {
    pub method: GetHighlightObjectForTestMethod,
    pub params: GetHighlightObjectForTestParams,
}
impl super::super::super::CommandResult for GetHighlightObjectForTest {
    type Result = super::results::GetHighlightObjectForTestResult;
}
#[doc = "For Persistent Grid testing.\n[getGridHighlightObjectsForTest](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-getGridHighlightObjectsForTest)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetGridHighlightObjectsForTestParams {
    #[doc = "Ids of the node to get highlight object for."]
    #[serde(rename = "nodeIds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<super::super::dom::types::NodeId>,
}
impl GetGridHighlightObjectsForTestParams {
    pub fn new(node_ids: Vec<super::super::dom::types::NodeId>) -> Self {
        Self { node_ids }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetGridHighlightObjectsForTestMethod {
    #[serde(rename = "Overlay.getGridHighlightObjectsForTest")]
    GetGridHighlightObjectsForTest,
}
impl GetGridHighlightObjectsForTestMethod {
    pub const IDENTIFIER: &'static str = "Overlay.getGridHighlightObjectsForTest";
}
#[doc = "For Persistent Grid testing.\n[getGridHighlightObjectsForTest](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-getGridHighlightObjectsForTest)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetGridHighlightObjectsForTest {
    pub method: GetGridHighlightObjectsForTestMethod,
    pub params: GetGridHighlightObjectsForTestParams,
}
impl super::super::super::CommandResult for GetGridHighlightObjectsForTest {
    type Result = super::results::GetGridHighlightObjectsForTestResult;
}
#[doc = "For Source Order Viewer testing.\n[getSourceOrderHighlightObjectForTest](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-getSourceOrderHighlightObjectForTest)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetSourceOrderHighlightObjectForTestParams {
    #[doc = "Id of the node to highlight."]
    #[serde(rename = "nodeId")]
    pub node_id: super::super::dom::types::NodeId,
}
impl GetSourceOrderHighlightObjectForTestParams {
    pub fn new(node_id: impl Into<super::super::dom::types::NodeId>) -> Self {
        Self {
            node_id: node_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetSourceOrderHighlightObjectForTestMethod {
    #[serde(rename = "Overlay.getSourceOrderHighlightObjectForTest")]
    GetSourceOrderHighlightObjectForTest,
}
impl GetSourceOrderHighlightObjectForTestMethod {
    pub const IDENTIFIER: &'static str = "Overlay.getSourceOrderHighlightObjectForTest";
}
#[doc = "For Source Order Viewer testing.\n[getSourceOrderHighlightObjectForTest](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-getSourceOrderHighlightObjectForTest)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetSourceOrderHighlightObjectForTest {
    pub method: GetSourceOrderHighlightObjectForTestMethod,
    pub params: GetSourceOrderHighlightObjectForTestParams,
}
impl super::super::super::CommandResult for GetSourceOrderHighlightObjectForTest {
    type Result = super::results::GetSourceOrderHighlightObjectForTestResult;
}
#[doc = "Hides any highlight.\n[hideHighlight](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-hideHighlight)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct HideHighlightParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HideHighlightMethod {
    #[serde(rename = "Overlay.hideHighlight")]
    HideHighlight,
}
impl HideHighlightMethod {
    pub const IDENTIFIER: &'static str = "Overlay.hideHighlight";
}
#[doc = "Hides any highlight.\n[hideHighlight](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-hideHighlight)"]
#[derive(Debug, Clone, PartialEq)]
pub struct HideHighlight {
    pub method: HideHighlightMethod,
    pub params: HideHighlightParams,
}
impl super::super::super::CommandResult for HideHighlight {
    type Result = super::results::HideHighlightResult;
}
#[doc = "Highlights DOM node with given id or with the given JavaScript object wrapper. Either nodeId or\nobjectId must be specified.\n[highlightNode](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-highlightNode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HighlightNodeParams {
    #[doc = "A descriptor for the highlight appearance."]
    #[serde(rename = "highlightConfig")]
    pub highlight_config: super::types::HighlightConfig,
    #[doc = "Identifier of the node to highlight."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<super::super::dom::types::NodeId>,
    #[doc = "Identifier of the backend node to highlight."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<super::super::dom::types::BackendNodeId>,
    #[doc = "JavaScript object id of the node to be highlighted."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
    #[doc = "Selectors to highlight relevant nodes."]
    #[serde(rename = "selector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub selector: Option<String>,
}
impl HighlightNodeParams {
    pub fn new(highlight_config: impl Into<super::types::HighlightConfig>) -> Self {
        Self {
            highlight_config: highlight_config.into(),
            node_id: None,
            backend_node_id: None,
            object_id: None,
            selector: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HighlightNodeMethod {
    #[serde(rename = "Overlay.highlightNode")]
    HighlightNode,
}
impl HighlightNodeMethod {
    pub const IDENTIFIER: &'static str = "Overlay.highlightNode";
}
#[doc = "Highlights DOM node with given id or with the given JavaScript object wrapper. Either nodeId or\nobjectId must be specified.\n[highlightNode](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-highlightNode)"]
#[derive(Debug, Clone, PartialEq)]
pub struct HighlightNode {
    pub method: HighlightNodeMethod,
    pub params: HighlightNodeParams,
}
impl super::super::super::CommandResult for HighlightNode {
    type Result = super::results::HighlightNodeResult;
}
#[doc = "Highlights given quad. Coordinates are absolute with respect to the main frame viewport.\n[highlightQuad](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-highlightQuad)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HighlightQuadParams {
    #[doc = "Quad to highlight"]
    #[serde(rename = "quad")]
    pub quad: super::super::dom::types::Quad,
    #[doc = "The highlight fill color (default: transparent)."]
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub color: Option<super::super::dom::types::Rgba>,
    #[doc = "The highlight outline color (default: transparent)."]
    #[serde(rename = "outlineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub outline_color: Option<super::super::dom::types::Rgba>,
}
impl HighlightQuadParams {
    pub fn new(quad: impl Into<super::super::dom::types::Quad>) -> Self {
        Self {
            quad: quad.into(),
            color: None,
            outline_color: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HighlightQuadMethod {
    #[serde(rename = "Overlay.highlightQuad")]
    HighlightQuad,
}
impl HighlightQuadMethod {
    pub const IDENTIFIER: &'static str = "Overlay.highlightQuad";
}
#[doc = "Highlights given quad. Coordinates are absolute with respect to the main frame viewport.\n[highlightQuad](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-highlightQuad)"]
#[derive(Debug, Clone, PartialEq)]
pub struct HighlightQuad {
    pub method: HighlightQuadMethod,
    pub params: HighlightQuadParams,
}
impl super::super::super::CommandResult for HighlightQuad {
    type Result = super::results::HighlightQuadResult;
}
#[doc = "Highlights given rectangle. Coordinates are absolute with respect to the main frame viewport.\nIssue: the method does not handle device pixel ratio (DPR) correctly.\nThe coordinates currently have to be adjusted by the client\nif DPR is not 1 (see crbug.com/437807128).\n[highlightRect](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-highlightRect)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HighlightRectParams {
    #[doc = "X coordinate"]
    #[serde(rename = "x")]
    pub x: i64,
    #[doc = "Y coordinate"]
    #[serde(rename = "y")]
    pub y: i64,
    #[doc = "Rectangle width"]
    #[serde(rename = "width")]
    pub width: i64,
    #[doc = "Rectangle height"]
    #[serde(rename = "height")]
    pub height: i64,
    #[doc = "The highlight fill color (default: transparent)."]
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub color: Option<super::super::dom::types::Rgba>,
    #[doc = "The highlight outline color (default: transparent)."]
    #[serde(rename = "outlineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub outline_color: Option<super::super::dom::types::Rgba>,
}
impl HighlightRectParams {
    pub fn new(
        x: impl Into<i64>,
        y: impl Into<i64>,
        width: impl Into<i64>,
        height: impl Into<i64>,
    ) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            width: width.into(),
            height: height.into(),
            color: None,
            outline_color: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HighlightRectMethod {
    #[serde(rename = "Overlay.highlightRect")]
    HighlightRect,
}
impl HighlightRectMethod {
    pub const IDENTIFIER: &'static str = "Overlay.highlightRect";
}
#[doc = "Highlights given rectangle. Coordinates are absolute with respect to the main frame viewport.\nIssue: the method does not handle device pixel ratio (DPR) correctly.\nThe coordinates currently have to be adjusted by the client\nif DPR is not 1 (see crbug.com/437807128).\n[highlightRect](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-highlightRect)"]
#[derive(Debug, Clone, PartialEq)]
pub struct HighlightRect {
    pub method: HighlightRectMethod,
    pub params: HighlightRectParams,
}
impl super::super::super::CommandResult for HighlightRect {
    type Result = super::results::HighlightRectResult;
}
#[doc = "Highlights the source order of the children of the DOM node with given id or with the given\nJavaScript object wrapper. Either nodeId or objectId must be specified.\n[highlightSourceOrder](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-highlightSourceOrder)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HighlightSourceOrderParams {
    #[doc = "A descriptor for the appearance of the overlay drawing."]
    #[serde(rename = "sourceOrderConfig")]
    pub source_order_config: super::types::SourceOrderConfig,
    #[doc = "Identifier of the node to highlight."]
    #[serde(rename = "nodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub node_id: Option<super::super::dom::types::NodeId>,
    #[doc = "Identifier of the backend node to highlight."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<super::super::dom::types::BackendNodeId>,
    #[doc = "JavaScript object id of the node to be highlighted."]
    #[serde(rename = "objectId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub object_id: Option<super::super::super::js_protocol::runtime::types::RemoteObjectId>,
}
impl HighlightSourceOrderParams {
    pub fn new(source_order_config: impl Into<super::types::SourceOrderConfig>) -> Self {
        Self {
            source_order_config: source_order_config.into(),
            node_id: None,
            backend_node_id: None,
            object_id: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HighlightSourceOrderMethod {
    #[serde(rename = "Overlay.highlightSourceOrder")]
    HighlightSourceOrder,
}
impl HighlightSourceOrderMethod {
    pub const IDENTIFIER: &'static str = "Overlay.highlightSourceOrder";
}
#[doc = "Highlights the source order of the children of the DOM node with given id or with the given\nJavaScript object wrapper. Either nodeId or objectId must be specified.\n[highlightSourceOrder](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-highlightSourceOrder)"]
#[derive(Debug, Clone, PartialEq)]
pub struct HighlightSourceOrder {
    pub method: HighlightSourceOrderMethod,
    pub params: HighlightSourceOrderParams,
}
impl super::super::super::CommandResult for HighlightSourceOrder {
    type Result = super::results::HighlightSourceOrderResult;
}
#[doc = "Enters the 'inspect' mode. In this mode, elements that user is hovering over are highlighted.\nBackend then generates 'inspectNodeRequested' event upon element selection.\n[setInspectMode](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setInspectMode)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetInspectModeParams {
    #[doc = "Set an inspection mode."]
    #[serde(rename = "mode")]
    pub mode: super::types::InspectMode,
    #[doc = "A descriptor for the highlight appearance of hovered-over nodes. May be omitted if `enabled\n== false`."]
    #[serde(rename = "highlightConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub highlight_config: Option<super::types::HighlightConfig>,
}
impl SetInspectModeParams {
    pub fn new(mode: impl Into<super::types::InspectMode>) -> Self {
        Self {
            mode: mode.into(),
            highlight_config: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetInspectModeMethod {
    #[serde(rename = "Overlay.setInspectMode")]
    SetInspectMode,
}
impl SetInspectModeMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setInspectMode";
}
#[doc = "Enters the 'inspect' mode. In this mode, elements that user is hovering over are highlighted.\nBackend then generates 'inspectNodeRequested' event upon element selection.\n[setInspectMode](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setInspectMode)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetInspectMode {
    pub method: SetInspectModeMethod,
    pub params: SetInspectModeParams,
}
impl super::super::super::CommandResult for SetInspectMode {
    type Result = super::results::SetInspectModeResult;
}
#[doc = "Highlights owner element of all frames detected to be ads.\n[setShowAdHighlights](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowAdHighlights)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetShowAdHighlightsParams {
    #[doc = "True for showing ad highlights"]
    #[serde(rename = "show")]
    pub show: bool,
}
impl SetShowAdHighlightsParams {
    pub fn new(show: impl Into<bool>) -> Self {
        Self { show: show.into() }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowAdHighlightsMethod {
    #[serde(rename = "Overlay.setShowAdHighlights")]
    SetShowAdHighlights,
}
impl SetShowAdHighlightsMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowAdHighlights";
}
#[doc = "Highlights owner element of all frames detected to be ads.\n[setShowAdHighlights](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowAdHighlights)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowAdHighlights {
    pub method: SetShowAdHighlightsMethod,
    pub params: SetShowAdHighlightsParams,
}
impl super::super::super::CommandResult for SetShowAdHighlights {
    type Result = super::results::SetShowAdHighlightsResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetPausedInDebuggerMessageParams {
    #[doc = "The message to display, also triggers resume and step over controls."]
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub message: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetPausedInDebuggerMessageMethod {
    #[serde(rename = "Overlay.setPausedInDebuggerMessage")]
    SetPausedInDebuggerMessage,
}
impl SetPausedInDebuggerMessageMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setPausedInDebuggerMessage";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetPausedInDebuggerMessage {
    pub method: SetPausedInDebuggerMessageMethod,
    pub params: SetPausedInDebuggerMessageParams,
}
impl super::super::super::CommandResult for SetPausedInDebuggerMessage {
    type Result = super::results::SetPausedInDebuggerMessageResult;
}
#[doc = "Requests that backend shows debug borders on layers\n[setShowDebugBorders](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowDebugBorders)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetShowDebugBordersParams {
    #[doc = "True for showing debug borders"]
    #[serde(rename = "show")]
    pub show: bool,
}
impl SetShowDebugBordersParams {
    pub fn new(show: impl Into<bool>) -> Self {
        Self { show: show.into() }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowDebugBordersMethod {
    #[serde(rename = "Overlay.setShowDebugBorders")]
    SetShowDebugBorders,
}
impl SetShowDebugBordersMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowDebugBorders";
}
#[doc = "Requests that backend shows debug borders on layers\n[setShowDebugBorders](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowDebugBorders)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowDebugBorders {
    pub method: SetShowDebugBordersMethod,
    pub params: SetShowDebugBordersParams,
}
impl super::super::super::CommandResult for SetShowDebugBorders {
    type Result = super::results::SetShowDebugBordersResult;
}
#[doc = "Requests that backend shows the FPS counter\n[setShowFPSCounter](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowFPSCounter)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetShowFpsCounterParams {
    #[doc = "True for showing the FPS counter"]
    #[serde(rename = "show")]
    pub show: bool,
}
impl SetShowFpsCounterParams {
    pub fn new(show: impl Into<bool>) -> Self {
        Self { show: show.into() }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowFpsCounterMethod {
    #[serde(rename = "Overlay.setShowFPSCounter")]
    SetShowFpsCounter,
}
impl SetShowFpsCounterMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowFPSCounter";
}
#[doc = "Requests that backend shows the FPS counter\n[setShowFPSCounter](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowFPSCounter)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowFpsCounter {
    pub method: SetShowFpsCounterMethod,
    pub params: SetShowFpsCounterParams,
}
impl super::super::super::CommandResult for SetShowFpsCounter {
    type Result = super::results::SetShowFpsCounterResult;
}
#[doc = "Highlight multiple elements with the CSS Grid overlay.\n[setShowGridOverlays](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowGridOverlays)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetShowGridOverlaysParams {
    #[doc = "An array of node identifiers and descriptors for the highlight appearance."]
    #[serde(rename = "gridNodeHighlightConfigs")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub grid_node_highlight_configs: Vec<super::types::GridNodeHighlightConfig>,
}
impl SetShowGridOverlaysParams {
    pub fn new(grid_node_highlight_configs: Vec<super::types::GridNodeHighlightConfig>) -> Self {
        Self {
            grid_node_highlight_configs,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowGridOverlaysMethod {
    #[serde(rename = "Overlay.setShowGridOverlays")]
    SetShowGridOverlays,
}
impl SetShowGridOverlaysMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowGridOverlays";
}
#[doc = "Highlight multiple elements with the CSS Grid overlay.\n[setShowGridOverlays](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowGridOverlays)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowGridOverlays {
    pub method: SetShowGridOverlaysMethod,
    pub params: SetShowGridOverlaysParams,
}
impl super::super::super::CommandResult for SetShowGridOverlays {
    type Result = super::results::SetShowGridOverlaysResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetShowFlexOverlaysParams {
    #[doc = "An array of node identifiers and descriptors for the highlight appearance."]
    #[serde(rename = "flexNodeHighlightConfigs")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub flex_node_highlight_configs: Vec<super::types::FlexNodeHighlightConfig>,
}
impl SetShowFlexOverlaysParams {
    pub fn new(flex_node_highlight_configs: Vec<super::types::FlexNodeHighlightConfig>) -> Self {
        Self {
            flex_node_highlight_configs,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowFlexOverlaysMethod {
    #[serde(rename = "Overlay.setShowFlexOverlays")]
    SetShowFlexOverlays,
}
impl SetShowFlexOverlaysMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowFlexOverlays";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowFlexOverlays {
    pub method: SetShowFlexOverlaysMethod,
    pub params: SetShowFlexOverlaysParams,
}
impl super::super::super::CommandResult for SetShowFlexOverlays {
    type Result = super::results::SetShowFlexOverlaysResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetShowScrollSnapOverlaysParams {
    #[doc = "An array of node identifiers and descriptors for the highlight appearance."]
    #[serde(rename = "scrollSnapHighlightConfigs")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub scroll_snap_highlight_configs: Vec<super::types::ScrollSnapHighlightConfig>,
}
impl SetShowScrollSnapOverlaysParams {
    pub fn new(
        scroll_snap_highlight_configs: Vec<super::types::ScrollSnapHighlightConfig>,
    ) -> Self {
        Self {
            scroll_snap_highlight_configs,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowScrollSnapOverlaysMethod {
    #[serde(rename = "Overlay.setShowScrollSnapOverlays")]
    SetShowScrollSnapOverlays,
}
impl SetShowScrollSnapOverlaysMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowScrollSnapOverlays";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowScrollSnapOverlays {
    pub method: SetShowScrollSnapOverlaysMethod,
    pub params: SetShowScrollSnapOverlaysParams,
}
impl super::super::super::CommandResult for SetShowScrollSnapOverlays {
    type Result = super::results::SetShowScrollSnapOverlaysResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetShowContainerQueryOverlaysParams {
    #[doc = "An array of node identifiers and descriptors for the highlight appearance."]
    #[serde(rename = "containerQueryHighlightConfigs")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub container_query_highlight_configs: Vec<super::types::ContainerQueryHighlightConfig>,
}
impl SetShowContainerQueryOverlaysParams {
    pub fn new(
        container_query_highlight_configs: Vec<super::types::ContainerQueryHighlightConfig>,
    ) -> Self {
        Self {
            container_query_highlight_configs,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowContainerQueryOverlaysMethod {
    #[serde(rename = "Overlay.setShowContainerQueryOverlays")]
    SetShowContainerQueryOverlays,
}
impl SetShowContainerQueryOverlaysMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowContainerQueryOverlays";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowContainerQueryOverlays {
    pub method: SetShowContainerQueryOverlaysMethod,
    pub params: SetShowContainerQueryOverlaysParams,
}
impl super::super::super::CommandResult for SetShowContainerQueryOverlays {
    type Result = super::results::SetShowContainerQueryOverlaysResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetShowInspectedElementAnchorParams {
    #[doc = "Node identifier for which to show an anchor for."]
    #[serde(rename = "inspectedElementAnchorConfig")]
    pub inspected_element_anchor_config: super::types::InspectedElementAnchorConfig,
}
impl SetShowInspectedElementAnchorParams {
    pub fn new(
        inspected_element_anchor_config: impl Into<super::types::InspectedElementAnchorConfig>,
    ) -> Self {
        Self {
            inspected_element_anchor_config: inspected_element_anchor_config.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowInspectedElementAnchorMethod {
    #[serde(rename = "Overlay.setShowInspectedElementAnchor")]
    SetShowInspectedElementAnchor,
}
impl SetShowInspectedElementAnchorMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowInspectedElementAnchor";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowInspectedElementAnchor {
    pub method: SetShowInspectedElementAnchorMethod,
    pub params: SetShowInspectedElementAnchorParams,
}
impl super::super::super::CommandResult for SetShowInspectedElementAnchor {
    type Result = super::results::SetShowInspectedElementAnchorResult;
}
#[doc = "Requests that backend shows paint rectangles\n[setShowPaintRects](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowPaintRects)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetShowPaintRectsParams {
    #[doc = "True for showing paint rectangles"]
    #[serde(rename = "result")]
    pub result: bool,
}
impl SetShowPaintRectsParams {
    pub fn new(result: impl Into<bool>) -> Self {
        Self {
            result: result.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowPaintRectsMethod {
    #[serde(rename = "Overlay.setShowPaintRects")]
    SetShowPaintRects,
}
impl SetShowPaintRectsMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowPaintRects";
}
#[doc = "Requests that backend shows paint rectangles\n[setShowPaintRects](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowPaintRects)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowPaintRects {
    pub method: SetShowPaintRectsMethod,
    pub params: SetShowPaintRectsParams,
}
impl super::super::super::CommandResult for SetShowPaintRects {
    type Result = super::results::SetShowPaintRectsResult;
}
#[doc = "Requests that backend shows layout shift regions\n[setShowLayoutShiftRegions](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowLayoutShiftRegions)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetShowLayoutShiftRegionsParams {
    #[doc = "True for showing layout shift regions"]
    #[serde(rename = "result")]
    pub result: bool,
}
impl SetShowLayoutShiftRegionsParams {
    pub fn new(result: impl Into<bool>) -> Self {
        Self {
            result: result.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowLayoutShiftRegionsMethod {
    #[serde(rename = "Overlay.setShowLayoutShiftRegions")]
    SetShowLayoutShiftRegions,
}
impl SetShowLayoutShiftRegionsMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowLayoutShiftRegions";
}
#[doc = "Requests that backend shows layout shift regions\n[setShowLayoutShiftRegions](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowLayoutShiftRegions)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowLayoutShiftRegions {
    pub method: SetShowLayoutShiftRegionsMethod,
    pub params: SetShowLayoutShiftRegionsParams,
}
impl super::super::super::CommandResult for SetShowLayoutShiftRegions {
    type Result = super::results::SetShowLayoutShiftRegionsResult;
}
#[doc = "Requests that backend shows scroll bottleneck rects\n[setShowScrollBottleneckRects](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowScrollBottleneckRects)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetShowScrollBottleneckRectsParams {
    #[doc = "True for showing scroll bottleneck rects"]
    #[serde(rename = "show")]
    pub show: bool,
}
impl SetShowScrollBottleneckRectsParams {
    pub fn new(show: impl Into<bool>) -> Self {
        Self { show: show.into() }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowScrollBottleneckRectsMethod {
    #[serde(rename = "Overlay.setShowScrollBottleneckRects")]
    SetShowScrollBottleneckRects,
}
impl SetShowScrollBottleneckRectsMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowScrollBottleneckRects";
}
#[doc = "Requests that backend shows scroll bottleneck rects\n[setShowScrollBottleneckRects](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowScrollBottleneckRects)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowScrollBottleneckRects {
    pub method: SetShowScrollBottleneckRectsMethod,
    pub params: SetShowScrollBottleneckRectsParams,
}
impl super::super::super::CommandResult for SetShowScrollBottleneckRects {
    type Result = super::results::SetShowScrollBottleneckRectsResult;
}
#[doc = "Paints viewport size upon main frame resize.\n[setShowViewportSizeOnResize](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowViewportSizeOnResize)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetShowViewportSizeOnResizeParams {
    #[doc = "Whether to paint size or not."]
    #[serde(rename = "show")]
    pub show: bool,
}
impl SetShowViewportSizeOnResizeParams {
    pub fn new(show: impl Into<bool>) -> Self {
        Self { show: show.into() }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowViewportSizeOnResizeMethod {
    #[serde(rename = "Overlay.setShowViewportSizeOnResize")]
    SetShowViewportSizeOnResize,
}
impl SetShowViewportSizeOnResizeMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowViewportSizeOnResize";
}
#[doc = "Paints viewport size upon main frame resize.\n[setShowViewportSizeOnResize](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowViewportSizeOnResize)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowViewportSizeOnResize {
    pub method: SetShowViewportSizeOnResizeMethod,
    pub params: SetShowViewportSizeOnResizeParams,
}
impl super::super::super::CommandResult for SetShowViewportSizeOnResize {
    type Result = super::results::SetShowViewportSizeOnResizeResult;
}
#[doc = "Add a dual screen device hinge\n[setShowHinge](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowHinge)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowHingeParams {
    #[doc = "hinge data, null means hideHinge"]
    #[serde(rename = "hingeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub hinge_config: Option<super::types::HingeConfig>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowHingeMethod {
    #[serde(rename = "Overlay.setShowHinge")]
    SetShowHinge,
}
impl SetShowHingeMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowHinge";
}
#[doc = "Add a dual screen device hinge\n[setShowHinge](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowHinge)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowHinge {
    pub method: SetShowHingeMethod,
    pub params: SetShowHingeParams,
}
impl super::super::super::CommandResult for SetShowHinge {
    type Result = super::results::SetShowHingeResult;
}
#[doc = "Show elements in isolation mode with overlays.\n[setShowIsolatedElements](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowIsolatedElements)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetShowIsolatedElementsParams {
    #[doc = "An array of node identifiers and descriptors for the highlight appearance."]
    #[serde(rename = "isolatedElementHighlightConfigs")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub isolated_element_highlight_configs: Vec<super::types::IsolatedElementHighlightConfig>,
}
impl SetShowIsolatedElementsParams {
    pub fn new(
        isolated_element_highlight_configs: Vec<super::types::IsolatedElementHighlightConfig>,
    ) -> Self {
        Self {
            isolated_element_highlight_configs,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowIsolatedElementsMethod {
    #[serde(rename = "Overlay.setShowIsolatedElements")]
    SetShowIsolatedElements,
}
impl SetShowIsolatedElementsMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowIsolatedElements";
}
#[doc = "Show elements in isolation mode with overlays.\n[setShowIsolatedElements](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowIsolatedElements)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowIsolatedElements {
    pub method: SetShowIsolatedElementsMethod,
    pub params: SetShowIsolatedElementsParams,
}
impl super::super::super::CommandResult for SetShowIsolatedElements {
    type Result = super::results::SetShowIsolatedElementsResult;
}
#[doc = "Show Window Controls Overlay for PWA\n[setShowWindowControlsOverlay](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowWindowControlsOverlay)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetShowWindowControlsOverlayParams {
    #[doc = "Window Controls Overlay data, null means hide Window Controls Overlay"]
    #[serde(rename = "windowControlsOverlayConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub window_controls_overlay_config: Option<super::types::WindowControlsOverlayConfig>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetShowWindowControlsOverlayMethod {
    #[serde(rename = "Overlay.setShowWindowControlsOverlay")]
    SetShowWindowControlsOverlay,
}
impl SetShowWindowControlsOverlayMethod {
    pub const IDENTIFIER: &'static str = "Overlay.setShowWindowControlsOverlay";
}
#[doc = "Show Window Controls Overlay for PWA\n[setShowWindowControlsOverlay](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#method-setShowWindowControlsOverlay)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetShowWindowControlsOverlay {
    pub method: SetShowWindowControlsOverlayMethod,
    pub params: SetShowWindowControlsOverlayParams,
}
impl super::super::super::CommandResult for SetShowWindowControlsOverlay {
    type Result = super::results::SetShowWindowControlsOverlayResult;
}
group_enum ! (OverlayCommands { Disable (Disable) , Enable (Enable) , GetHighlightObjectForTest (GetHighlightObjectForTest) , GetGridHighlightObjectsForTest (GetGridHighlightObjectsForTest) , GetSourceOrderHighlightObjectForTest (GetSourceOrderHighlightObjectForTest) , HideHighlight (HideHighlight) , HighlightNode (HighlightNode) , HighlightQuad (HighlightQuad) , HighlightRect (HighlightRect) , HighlightSourceOrder (HighlightSourceOrder) , SetInspectMode (SetInspectMode) , SetShowAdHighlights (SetShowAdHighlights) , SetPausedInDebuggerMessage (SetPausedInDebuggerMessage) , SetShowDebugBorders (SetShowDebugBorders) , SetShowFpsCounter (SetShowFpsCounter) , SetShowGridOverlays (SetShowGridOverlays) , SetShowFlexOverlays (SetShowFlexOverlays) , SetShowScrollSnapOverlays (SetShowScrollSnapOverlays) , SetShowContainerQueryOverlays (SetShowContainerQueryOverlays) , SetShowInspectedElementAnchor (SetShowInspectedElementAnchor) , SetShowPaintRects (SetShowPaintRects) , SetShowLayoutShiftRegions (SetShowLayoutShiftRegions) , SetShowScrollBottleneckRects (SetShowScrollBottleneckRects) , SetShowViewportSizeOnResize (SetShowViewportSizeOnResize) , SetShowHinge (SetShowHinge) , SetShowIsolatedElements (SetShowIsolatedElements) , SetShowWindowControlsOverlay (SetShowWindowControlsOverlay) });
