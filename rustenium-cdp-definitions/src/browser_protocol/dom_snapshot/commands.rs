use serde::{Deserialize, Serialize};
#[doc = "Disables DOM snapshot agent for the given page.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#method-disable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "DOMSnapshot.disable")]
    Disable,
}
impl DisableMethod {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.disable";
}
#[doc = "Disables DOM snapshot agent for the given page.\n[disable](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#method-disable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[doc = "Enables DOM snapshot agent for the given page.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#method-enable)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "DOMSnapshot.enable")]
    Enable,
}
impl EnableMethod {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.enable";
}
#[doc = "Enables DOM snapshot agent for the given page.\n[enable](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#method-enable)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[doc = "Returns a document snapshot, including the full DOM tree of the root node (including iframes,\ntemplate contents, and imported documents) in a flattened array, as well as layout and\nwhite-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is\nflattened.\n[captureSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#method-captureSnapshot)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureSnapshotParams {
    #[doc = "Whitelist of computed styles to return."]
    #[serde(rename = "computedStyles")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub computed_styles: Vec<String>,
    #[doc = "Whether to include layout object paint orders into the snapshot."]
    #[serde(rename = "includePaintOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_paint_order: Option<bool>,
    #[doc = "Whether to include DOM rectangles (offsetRects, clientRects, scrollRects) into the snapshot"]
    #[serde(rename = "includeDOMRects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_dom_rects: Option<bool>,
    #[doc = "Whether to include blended background colors in the snapshot (default: false).\nBlended background color is achieved by blending background colors of all elements\nthat overlap with the current element."]
    #[serde(rename = "includeBlendedBackgroundColors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_blended_background_colors: Option<bool>,
    #[doc = "Whether to include text color opacity in the snapshot (default: false).\nAn element might have the opacity property set that affects the text color of the element.\nThe final text color opacity is computed based on the opacity of all overlapping elements."]
    #[serde(rename = "includeTextColorOpacities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub include_text_color_opacities: Option<bool>,
}
impl CaptureSnapshotParams {
    pub fn new(computed_styles: Vec<String>) -> Self {
        Self {
            computed_styles,
            include_paint_order: None,
            include_dom_rects: None,
            include_blended_background_colors: None,
            include_text_color_opacities: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CaptureSnapshotMethod {
    #[serde(rename = "DOMSnapshot.captureSnapshot")]
    CaptureSnapshot,
}
impl CaptureSnapshotMethod {
    pub const IDENTIFIER: &'static str = "DOMSnapshot.captureSnapshot";
}
#[doc = "Returns a document snapshot, including the full DOM tree of the root node (including iframes,\ntemplate contents, and imported documents) in a flattened array, as well as layout and\nwhite-listed computed style information for the nodes. Shadow DOM in the returned DOM tree is\nflattened.\n[captureSnapshot](https://chromedevtools.github.io/devtools-protocol/tot/DOMSnapshot/#method-captureSnapshot)"]
#[derive(Debug, Clone, PartialEq)]
pub struct CaptureSnapshot {
    pub method: CaptureSnapshotMethod,
    pub params: CaptureSnapshotParams,
}
impl crate::CommandResult for CaptureSnapshot {
    type Result = super::results::CaptureSnapshotResult;
}
group_enum ! (DomSnapshotCommands { Disable (Disable) , Enable (Enable) , CaptureSnapshot (CaptureSnapshot) });
