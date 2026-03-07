use serde::{Deserialize, Serialize};
#[doc = "Fires whenever a web font is updated.  A non-empty font parameter indicates a successfully loaded\nweb font.\n[fontsUpdated](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-fontsUpdated)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FontsUpdatedParams {
    #[doc = "The web font that has loaded."]
    #[serde(rename = "font")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub font: Option<super::types::FontFace>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FontsUpdatedMethod {
    #[serde(rename = "CSS.fontsUpdated")]
    FontsUpdated,
}
impl FontsUpdatedMethod {
    pub const IDENTIFIER: &'static str = "CSS.fontsUpdated";
}
#[doc = "Fires whenever a web font is updated.  A non-empty font parameter indicates a successfully loaded\nweb font.\n[fontsUpdated](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-fontsUpdated)"]
#[derive(Debug, Clone, PartialEq)]
pub struct FontsUpdated {
    pub method: FontsUpdatedMethod,
    pub params: FontsUpdatedParams,
}
#[doc = "Fires whenever a MediaQuery result changes (for example, after a browser window has been\nresized.) The current implementation considers only viewport-dependent media features.\n[mediaQueryResultChanged](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-mediaQueryResultChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaQueryResultChangedParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MediaQueryResultChangedMethod {
    #[serde(rename = "CSS.mediaQueryResultChanged")]
    MediaQueryResultChanged,
}
impl MediaQueryResultChangedMethod {
    pub const IDENTIFIER: &'static str = "CSS.mediaQueryResultChanged";
}
#[doc = "Fires whenever a MediaQuery result changes (for example, after a browser window has been\nresized.) The current implementation considers only viewport-dependent media features.\n[mediaQueryResultChanged](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-mediaQueryResultChanged)"]
#[derive(Debug, Clone, PartialEq)]
pub struct MediaQueryResultChanged {
    pub method: MediaQueryResultChangedMethod,
    pub params: MediaQueryResultChangedParams,
}
#[doc = "Fired whenever an active document stylesheet is added.\n[styleSheetAdded](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-styleSheetAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleSheetAddedParams {
    #[doc = "Added stylesheet metainfo."]
    #[serde(rename = "header")]
    pub header: super::types::CssStyleSheetHeader,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StyleSheetAddedMethod {
    #[serde(rename = "CSS.styleSheetAdded")]
    StyleSheetAdded,
}
impl StyleSheetAddedMethod {
    pub const IDENTIFIER: &'static str = "CSS.styleSheetAdded";
}
#[doc = "Fired whenever an active document stylesheet is added.\n[styleSheetAdded](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-styleSheetAdded)"]
#[derive(Debug, Clone, PartialEq)]
pub struct StyleSheetAdded {
    pub method: StyleSheetAddedMethod,
    pub params: StyleSheetAddedParams,
}
#[doc = "Fired whenever a stylesheet is changed as a result of the client operation.\n[styleSheetChanged](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-styleSheetChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleSheetChangedParams {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StyleSheetChangedMethod {
    #[serde(rename = "CSS.styleSheetChanged")]
    StyleSheetChanged,
}
impl StyleSheetChangedMethod {
    pub const IDENTIFIER: &'static str = "CSS.styleSheetChanged";
}
#[doc = "Fired whenever a stylesheet is changed as a result of the client operation.\n[styleSheetChanged](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-styleSheetChanged)"]
#[derive(Debug, Clone, PartialEq)]
pub struct StyleSheetChanged {
    pub method: StyleSheetChangedMethod,
    pub params: StyleSheetChangedParams,
}
#[doc = "Fired whenever an active document stylesheet is removed.\n[styleSheetRemoved](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-styleSheetRemoved)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleSheetRemovedParams {
    #[doc = "Identifier of the removed stylesheet."]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: crate::browser_protocol::dom::types::StyleSheetId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StyleSheetRemovedMethod {
    #[serde(rename = "CSS.styleSheetRemoved")]
    StyleSheetRemoved,
}
impl StyleSheetRemovedMethod {
    pub const IDENTIFIER: &'static str = "CSS.styleSheetRemoved";
}
#[doc = "Fired whenever an active document stylesheet is removed.\n[styleSheetRemoved](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-styleSheetRemoved)"]
#[derive(Debug, Clone, PartialEq)]
pub struct StyleSheetRemoved {
    pub method: StyleSheetRemovedMethod,
    pub params: StyleSheetRemovedParams,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputedStyleUpdatedParams {
    #[doc = "The node id that has updated computed styles."]
    #[serde(rename = "nodeId")]
    pub node_id: crate::browser_protocol::dom::types::NodeId,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComputedStyleUpdatedMethod {
    #[serde(rename = "CSS.computedStyleUpdated")]
    ComputedStyleUpdated,
}
impl ComputedStyleUpdatedMethod {
    pub const IDENTIFIER: &'static str = "CSS.computedStyleUpdated";
}
#[derive(Debug, Clone, PartialEq)]
pub struct ComputedStyleUpdated {
    pub method: ComputedStyleUpdatedMethod,
    pub params: ComputedStyleUpdatedParams,
}
group_enum ! (CssEvents { FontsUpdated (FontsUpdated) , MediaQueryResultChanged (MediaQueryResultChanged) , StyleSheetAdded (StyleSheetAdded) , StyleSheetChanged (StyleSheetChanged) , StyleSheetRemoved (StyleSheetRemoved) , ComputedStyleUpdated (ComputedStyleUpdated) });
