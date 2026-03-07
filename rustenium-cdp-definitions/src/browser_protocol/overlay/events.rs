use serde::{Deserialize, Serialize};
#[doc = "Fired when the node should be inspected. This happens after call to `setInspectMode` or when\nuser manually inspects an element.\n[inspectNodeRequested](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-inspectNodeRequested)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectNodeRequestedParams {
    #[doc = "Id of the node to inspect."]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: crate::browser_protocol::dom::types::BackendNodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InspectNodeRequestedMethod {
    #[serde(rename = "Overlay.inspectNodeRequested")]
    InspectNodeRequested,
}
impl InspectNodeRequestedMethod {
    pub const IDENTIFIER: &'static str = "Overlay.inspectNodeRequested";
}
#[doc = "Fired when the node should be inspected. This happens after call to `setInspectMode` or when\nuser manually inspects an element.\n[inspectNodeRequested](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-inspectNodeRequested)"]
#[derive(Debug, Clone, PartialEq)]
pub struct InspectNodeRequested {
    pub method: InspectNodeRequestedMethod,
    pub params: InspectNodeRequestedParams,
}
#[doc = "Fired when the node should be highlighted. This happens after call to `setInspectMode`.\n[nodeHighlightRequested](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-nodeHighlightRequested)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeHighlightRequestedParams {
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeHighlightRequestedMethod {
    #[serde(rename = "Overlay.nodeHighlightRequested")]
    NodeHighlightRequested,
}
impl NodeHighlightRequestedMethod {
    pub const IDENTIFIER: &'static str = "Overlay.nodeHighlightRequested";
}
#[doc = "Fired when the node should be highlighted. This happens after call to `setInspectMode`.\n[nodeHighlightRequested](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-nodeHighlightRequested)"]
#[derive(Debug, Clone, PartialEq)]
pub struct NodeHighlightRequested {
    pub method: NodeHighlightRequestedMethod,
    pub params: NodeHighlightRequestedParams,
}
#[doc = "Fired when user asks to capture screenshot of some area on the page.\n[screenshotRequested](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-screenshotRequested)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreenshotRequestedParams {
    #[doc = "Viewport to capture, in device independent pixels (dip)."]
    #[serde(rename = "viewport")]
    pub viewport: crate::browser_protocol::page::types::Viewport,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ScreenshotRequestedMethod {
    #[serde(rename = "Overlay.screenshotRequested")]
    ScreenshotRequested,
}
impl ScreenshotRequestedMethod {
    pub const IDENTIFIER: &'static str = "Overlay.screenshotRequested";
}
#[doc = "Fired when user asks to capture screenshot of some area on the page.\n[screenshotRequested](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-screenshotRequested)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ScreenshotRequested {
    pub method: ScreenshotRequestedMethod,
    pub params: ScreenshotRequestedParams,
}
#[doc = "Fired when user asks to show the Inspect panel.\n[inspectPanelShowRequested](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-inspectPanelShowRequested)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectPanelShowRequestedParams {
    #[doc = "Id of the node to show in the panel."]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: crate::browser_protocol::dom::types::BackendNodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InspectPanelShowRequestedMethod {
    #[serde(rename = "Overlay.inspectPanelShowRequested")]
    InspectPanelShowRequested,
}
impl InspectPanelShowRequestedMethod {
    pub const IDENTIFIER: &'static str = "Overlay.inspectPanelShowRequested";
}
#[doc = "Fired when user asks to show the Inspect panel.\n[inspectPanelShowRequested](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-inspectPanelShowRequested)"]
#[derive(Debug, Clone, PartialEq)]
pub struct InspectPanelShowRequested {
    pub method: InspectPanelShowRequestedMethod,
    pub params: InspectPanelShowRequestedParams,
}
#[doc = "Fired when user asks to restore the Inspected Element floating window.\n[inspectedElementWindowRestored](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-inspectedElementWindowRestored)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectedElementWindowRestoredParams {
    #[doc = "Id of the node to restore the floating window for."]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: crate::browser_protocol::dom::types::BackendNodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InspectedElementWindowRestoredMethod {
    #[serde(rename = "Overlay.inspectedElementWindowRestored")]
    InspectedElementWindowRestored,
}
impl InspectedElementWindowRestoredMethod {
    pub const IDENTIFIER: &'static str = "Overlay.inspectedElementWindowRestored";
}
#[doc = "Fired when user asks to restore the Inspected Element floating window.\n[inspectedElementWindowRestored](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-inspectedElementWindowRestored)"]
#[derive(Debug, Clone, PartialEq)]
pub struct InspectedElementWindowRestored {
    pub method: InspectedElementWindowRestoredMethod,
    pub params: InspectedElementWindowRestoredParams,
}
#[doc = "Fired when user cancels the inspect mode.\n[inspectModeCanceled](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-inspectModeCanceled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectModeCanceledParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InspectModeCanceledMethod {
    #[serde(rename = "Overlay.inspectModeCanceled")]
    InspectModeCanceled,
}
impl InspectModeCanceledMethod {
    pub const IDENTIFIER: &'static str = "Overlay.inspectModeCanceled";
}
#[doc = "Fired when user cancels the inspect mode.\n[inspectModeCanceled](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-inspectModeCanceled)"]
#[derive(Debug, Clone, PartialEq)]
pub struct InspectModeCanceled {
    pub method: InspectModeCanceledMethod,
    pub params: InspectModeCanceledParams,
}
group_enum ! (OverlayEvents { InspectNodeRequested (InspectNodeRequested) , NodeHighlightRequested (NodeHighlightRequested) , ScreenshotRequested (ScreenshotRequested) , InspectPanelShowRequested (InspectPanelShowRequested) , InspectedElementWindowRestored (InspectedElementWindowRestored) , InspectModeCanceled (InspectModeCanceled) });
