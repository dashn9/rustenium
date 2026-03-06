use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct BrowsingContext(String);
impl BrowsingContext {
    pub fn new(val: impl Into<String>) -> Self {
        BrowsingContext(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for BrowsingContext {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<BrowsingContext> for String {
    fn from(el: BrowsingContext) -> String {
        el.0
    }
}
impl From<String> for BrowsingContext {
    fn from(expr: String) -> Self {
        BrowsingContext(expr)
    }
}
impl BrowsingContext {
    pub const IDENTIFIER: &'static str = "browsingContext.BrowsingContext";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Locator {
    AccessibilityLocator(AccessibilityLocator),
    CssLocator(CssLocator),
    ContextLocator(ContextLocator),
    InnerTextLocator(InnerTextLocator),
    XPathLocator(XPathLocator),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessibilityLocator {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub role: Option<String>,
}
impl AccessibilityLocator {
    pub fn new(r#type: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            name: None,
            role: None,
        }
    }
}
impl<T: Into<String>> From<T> for AccessibilityLocator {
    fn from(url: T) -> Self {
        AccessibilityLocator::new(url)
    }
}
impl AccessibilityLocator {
    pub const IDENTIFIER: &'static str = "browsingContext.AccessibilityLocator";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssLocator {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl CssLocator {
    pub fn new(r#type: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl CssLocator {
    pub const IDENTIFIER: &'static str = "browsingContext.CssLocator";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextLocator {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "context")]
    pub context: BrowsingContext,
}
impl ContextLocator {
    pub fn new(r#type: impl Into<String>, context: impl Into<BrowsingContext>) -> Self {
        Self {
            r#type: r#type.into(),
            context: context.into(),
        }
    }
}
impl ContextLocator {
    pub const IDENTIFIER: &'static str = "browsingContext.ContextLocator";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerTextLocator {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "ignoreCase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ignore_case: Option<bool>,
    #[serde(rename = "matchType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub match_type: Option<InnerTextLocatorMatchType>,
    #[serde(rename = "maxDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_depth: Option<u64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InnerTextLocatorMatchType {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "partial")]
    Partial,
}
impl InnerTextLocator {
    pub fn new(r#type: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
            ignore_case: None,
            match_type: None,
            max_depth: None,
        }
    }
}
impl InnerTextLocator {
    pub const IDENTIFIER: &'static str = "browsingContext.InnerTextLocator";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XPathLocator {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl XPathLocator {
    pub fn new(r#type: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl XPathLocator {
    pub const IDENTIFIER: &'static str = "browsingContext.XPathLocator";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct Navigation(String);
impl Navigation {
    pub fn new(val: impl Into<String>) -> Self {
        Navigation(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for Navigation {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<Navigation> for String {
    fn from(el: Navigation) -> String {
        el.0
    }
}
impl From<String> for Navigation {
    fn from(expr: String) -> Self {
        Navigation(expr)
    }
}
impl Navigation {
    pub const IDENTIFIER: &'static str = "browsingContext.Navigation";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReadinessState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "interactive")]
    Interactive,
    #[serde(rename = "complete")]
    Complete,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UserPromptType {
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "beforeunload")]
    Beforeunload,
    #[serde(rename = "confirm")]
    Confirm,
    #[serde(rename = "prompt")]
    Prompt,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageFormat {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub quality: Option<f64>,
}
impl ImageFormat {
    pub fn new(r#type: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            quality: None,
        }
    }
}
impl<T: Into<String>> From<T> for ImageFormat {
    fn from(url: T) -> Self {
        ImageFormat::new(url)
    }
}
impl ImageFormat {
    pub const IDENTIFIER: &'static str = "browsingContext.ImageFormat";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClipRectangle {
    BoxClipRectangle(BoxClipRectangle),
    ElementClipRectangle(ElementClipRectangle),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElementClipRectangle {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "element")]
    pub element: crate::script::types::SharedReference,
}
impl ElementClipRectangle {
    pub fn new(
        r#type: impl Into<String>,
        element: impl Into<crate::script::types::SharedReference>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            element: element.into(),
        }
    }
}
impl ElementClipRectangle {
    pub const IDENTIFIER: &'static str = "browsingContext.ElementClipRectangle";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoxClipRectangle {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "y")]
    pub y: f64,
    #[serde(rename = "width")]
    pub width: f64,
    #[serde(rename = "height")]
    pub height: f64,
}
impl BoxClipRectangle {
    pub const IDENTIFIER: &'static str = "browsingContext.BoxClipRectangle";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreateType {
    #[serde(rename = "tab")]
    Tab,
    #[serde(rename = "window")]
    Window,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PrintMarginParameters {
    #[serde(rename = "bottom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_margin_parameters_bottom")]
    #[serde_valid::validate(minimum = 0f64)]
    pub bottom: Option<f64>,
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_margin_parameters_left")]
    #[serde_valid::validate(minimum = 0f64)]
    pub left: Option<f64>,
    #[serde(rename = "right")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_margin_parameters_right")]
    #[serde_valid::validate(minimum = 0f64)]
    pub right: Option<f64>,
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_margin_parameters_top")]
    #[serde_valid::validate(minimum = 0f64)]
    pub top: Option<f64>,
}
fn default_print_margin_parameters_bottom() -> Option<f64> {
    Some(1f64)
}
fn default_print_margin_parameters_left() -> Option<f64> {
    Some(1f64)
}
fn default_print_margin_parameters_right() -> Option<f64> {
    Some(1f64)
}
fn default_print_margin_parameters_top() -> Option<f64> {
    Some(1f64)
}
impl PrintMarginParameters {
    pub const IDENTIFIER: &'static str = "browsingContext.PrintMarginParameters";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PrintPageParameters {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_page_parameters_height")]
    #[serde_valid::validate(minimum = 0.0352f64)]
    pub height: Option<f64>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_page_parameters_width")]
    #[serde_valid::validate(minimum = 0.0352f64)]
    pub width: Option<f64>,
}
fn default_print_page_parameters_height() -> Option<f64> {
    Some(27.94f64)
}
fn default_print_page_parameters_width() -> Option<f64> {
    Some(21.59f64)
}
impl PrintPageParameters {
    pub const IDENTIFIER: &'static str = "browsingContext.PrintPageParameters";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Viewport {
    #[serde(rename = "width")]
    pub width: u64,
    #[serde(rename = "height")]
    pub height: u64,
}
impl Viewport {
    pub fn new(width: impl Into<u64>, height: impl Into<u64>) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }
}
impl Viewport {
    pub const IDENTIFIER: &'static str = "browsingContext.Viewport";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct InfoList(Vec<Info>);
impl InfoList {
    pub fn new(val: impl Into<Vec<Info>>) -> Self {
        InfoList(val.into())
    }
    pub fn inner(&self) -> &Vec<Info> {
        &self.0
    }
}
impl InfoList {
    pub const IDENTIFIER: &'static str = "browsingContext.InfoList";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseNavigationInfo {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<Navigation>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}
impl BaseNavigationInfo {
    pub fn new(
        context: impl Into<BrowsingContext>,
        timestamp: impl Into<u64>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            context: context.into(),
            timestamp: timestamp.into(),
            url: url.into(),
            navigation: None,
        }
    }
}
impl BaseNavigationInfo {
    pub const IDENTIFIER: &'static str = "browsingContext.BaseNavigationInfo";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadCanceledParams {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<Navigation>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}
impl DownloadCanceledParams {
    pub fn new(
        status: impl Into<String>,
        context: impl Into<BrowsingContext>,
        timestamp: impl Into<u64>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            status: status.into(),
            context: context.into(),
            timestamp: timestamp.into(),
            url: url.into(),
            navigation: None,
        }
    }
}
impl DownloadCanceledParams {
    pub const IDENTIFIER: &'static str = "browsingContext.DownloadCanceledParams";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadCompleteParams {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "filepath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub filepath: Option<String>,
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "navigation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub navigation: Option<Navigation>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}
impl DownloadCompleteParams {
    pub fn new(
        status: impl Into<String>,
        context: impl Into<BrowsingContext>,
        timestamp: impl Into<u64>,
        url: impl Into<String>,
    ) -> Self {
        Self {
            status: status.into(),
            context: context.into(),
            timestamp: timestamp.into(),
            url: url.into(),
            filepath: None,
            navigation: None,
        }
    }
}
impl DownloadCompleteParams {
    pub const IDENTIFIER: &'static str = "browsingContext.DownloadCompleteParams";
}
group_enum ! (BrowsingContextTypes { BrowsingContext (BrowsingContext) , Locator (Locator) , AccessibilityLocator (AccessibilityLocator) , CssLocator (CssLocator) , ContextLocator (ContextLocator) , InnerTextLocator (InnerTextLocator) , XPathLocator (XPathLocator) , Navigation (Navigation) , ReadinessState (ReadinessState) , UserPromptType (UserPromptType) , ImageFormat (ImageFormat) , ClipRectangle (ClipRectangle) , ElementClipRectangle (ElementClipRectangle) , BoxClipRectangle (BoxClipRectangle) , CreateType (CreateType) , PrintMarginParameters (PrintMarginParameters) , PrintPageParameters (PrintPageParameters) , Viewport (Viewport) , InfoList (InfoList) , BaseNavigationInfo (BaseNavigationInfo) , DownloadCanceledParams (DownloadCanceledParams) , DownloadCompleteParams (DownloadCompleteParams) });
