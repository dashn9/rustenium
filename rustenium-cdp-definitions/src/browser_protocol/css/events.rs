use serde::{Deserialize, Serialize};
#[doc = "Fires whenever a web font is updated.  A non-empty font parameter indicates a successfully loaded\nweb font.\n[fontsUpdated](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-fontsUpdated)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct FontsUpdated {
    #[doc = "The web font that has loaded."]
    #[serde(rename = "font")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub font: Option<super::types::FontFace>,
}
impl FontsUpdated {
    pub const IDENTIFIER: &'static str = "CSS.fontsUpdated";
}
#[doc = "Fires whenever a MediaQuery result changes (for example, after a browser window has been\nresized.) The current implementation considers only viewport-dependent media features.\n[mediaQueryResultChanged](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-mediaQueryResultChanged)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MediaQueryResultChanged {}
impl MediaQueryResultChanged {
    pub const IDENTIFIER: &'static str = "CSS.mediaQueryResultChanged";
}
#[doc = "Fired whenever an active document stylesheet is added.\n[styleSheetAdded](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-styleSheetAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleSheetAdded {
    #[doc = "Added stylesheet metainfo."]
    #[serde(rename = "header")]
    pub header: super::types::CssStyleSheetHeader,
}
impl StyleSheetAdded {
    pub const IDENTIFIER: &'static str = "CSS.styleSheetAdded";
}
#[doc = "Fired whenever a stylesheet is changed as a result of the client operation.\n[styleSheetChanged](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-styleSheetChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleSheetChanged {
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: super::super::dom::types::StyleSheetId,
}
impl StyleSheetChanged {
    pub const IDENTIFIER: &'static str = "CSS.styleSheetChanged";
}
#[doc = "Fired whenever an active document stylesheet is removed.\n[styleSheetRemoved](https://chromedevtools.github.io/devtools-protocol/tot/CSS/#event-styleSheetRemoved)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StyleSheetRemoved {
    #[doc = "Identifier of the removed stylesheet."]
    #[serde(rename = "styleSheetId")]
    pub style_sheet_id: super::super::dom::types::StyleSheetId,
}
impl StyleSheetRemoved {
    pub const IDENTIFIER: &'static str = "CSS.styleSheetRemoved";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComputedStyleUpdated {
    #[doc = "The node id that has updated computed styles."]
    #[serde(rename = "nodeId")]
    pub node_id: super::super::dom::types::NodeId,
}
impl ComputedStyleUpdated {
    pub const IDENTIFIER: &'static str = "CSS.computedStyleUpdated";
}
group_enum ! (CssEvents { FontsUpdated (FontsUpdated) , MediaQueryResultChanged (MediaQueryResultChanged) , StyleSheetAdded (StyleSheetAdded) , StyleSheetChanged (StyleSheetChanged) , StyleSheetRemoved (StyleSheetRemoved) , ComputedStyleUpdated (ComputedStyleUpdated) });
