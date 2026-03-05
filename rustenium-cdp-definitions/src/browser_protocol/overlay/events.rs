use serde::{Deserialize, Serialize};
#[doc = "Fired when the node should be inspected. This happens after call to `setInspectMode` or when\nuser manually inspects an element.\n[inspectNodeRequested](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-inspectNodeRequested)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectNodeRequested {
    #[doc = "Id of the node to inspect."]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: super::super::dom::types::BackendNodeId,
}
impl InspectNodeRequested {
    pub const IDENTIFIER: &'static str = "Overlay.inspectNodeRequested";
}
#[doc = "Fired when the node should be highlighted. This happens after call to `setInspectMode`.\n[nodeHighlightRequested](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-nodeHighlightRequested)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeHighlightRequested {
    #[serde(rename = "nodeId")]
    pub node_id: super::super::dom::types::NodeId,
}
impl NodeHighlightRequested {
    pub const IDENTIFIER: &'static str = "Overlay.nodeHighlightRequested";
}
#[doc = "Fired when user asks to capture screenshot of some area on the page.\n[screenshotRequested](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-screenshotRequested)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreenshotRequested {
    #[doc = "Viewport to capture, in device independent pixels (dip)."]
    #[serde(rename = "viewport")]
    pub viewport: super::super::page::types::Viewport,
}
impl ScreenshotRequested {
    pub const IDENTIFIER: &'static str = "Overlay.screenshotRequested";
}
#[doc = "Fired when user asks to show the Inspect panel.\n[inspectPanelShowRequested](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-inspectPanelShowRequested)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectPanelShowRequested {
    #[doc = "Id of the node to show in the panel."]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: super::super::dom::types::BackendNodeId,
}
impl InspectPanelShowRequested {
    pub const IDENTIFIER: &'static str = "Overlay.inspectPanelShowRequested";
}
#[doc = "Fired when user asks to restore the Inspected Element floating window.\n[inspectedElementWindowRestored](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-inspectedElementWindowRestored)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspectedElementWindowRestored {
    #[doc = "Id of the node to restore the floating window for."]
    #[serde(rename = "backendNodeId")]
    pub backend_node_id: super::super::dom::types::BackendNodeId,
}
impl InspectedElementWindowRestored {
    pub const IDENTIFIER: &'static str = "Overlay.inspectedElementWindowRestored";
}
#[doc = "Fired when user cancels the inspect mode.\n[inspectModeCanceled](https://chromedevtools.github.io/devtools-protocol/tot/Overlay/#event-inspectModeCanceled)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InspectModeCanceled {}
impl InspectModeCanceled {
    pub const IDENTIFIER: &'static str = "Overlay.inspectModeCanceled";
}
group_enum ! (OverlayEvents { InspectNodeRequested (InspectNodeRequested) , NodeHighlightRequested (NodeHighlightRequested) , ScreenshotRequested (ScreenshotRequested) , InspectPanelShowRequested (InspectPanelShowRequested) , InspectedElementWindowRestored (InspectedElementWindowRestored) , InspectModeCanceled (InspectModeCanceled) });
