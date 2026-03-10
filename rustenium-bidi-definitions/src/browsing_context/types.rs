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
    pub const DOMAIN_DIRECTION: &'static str = "all";
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
    pub r#type: AccessibilityLocatorType,
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(rename = "role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub role: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccessibilityLocatorType {
    #[serde(rename = "accessibility")]
    Accessibility,
}
impl AccessibilityLocator {
    pub fn new(r#type: impl Into<AccessibilityLocatorType>) -> Self {
        Self {
            r#type: r#type.into(),
            name: None,
            role: None,
        }
    }
}
impl AccessibilityLocator {
    pub const IDENTIFIER: &'static str = "browsingContext.AccessibilityLocator";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CssLocator {
    #[serde(rename = "type")]
    pub r#type: CssLocatorType,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CssLocatorType {
    #[serde(rename = "css")]
    Css,
}
impl CssLocator {
    pub fn new(r#type: impl Into<CssLocatorType>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl CssLocator {
    pub const IDENTIFIER: &'static str = "browsingContext.CssLocator";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContextLocator {
    #[serde(rename = "type")]
    pub r#type: ContextLocatorType,
    #[serde(rename = "context")]
    pub context: BrowsingContext,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContextLocatorType {
    #[serde(rename = "context")]
    Context,
}
impl ContextLocator {
    pub fn new(r#type: impl Into<ContextLocatorType>, context: impl Into<BrowsingContext>) -> Self {
        Self {
            r#type: r#type.into(),
            context: context.into(),
        }
    }
}
impl ContextLocator {
    pub const IDENTIFIER: &'static str = "browsingContext.ContextLocator";
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerTextLocator {
    #[serde(rename = "type")]
    pub r#type: InnerTextLocatorType,
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
pub enum InnerTextLocatorType {
    #[serde(rename = "innerText")]
    InnerText,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InnerTextLocatorMatchType {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "partial")]
    Partial,
}
impl InnerTextLocator {
    pub fn new(r#type: impl Into<InnerTextLocatorType>, value: impl Into<String>) -> Self {
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
    pub const DOMAIN_DIRECTION: &'static str = "all";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XPathLocator {
    #[serde(rename = "type")]
    pub r#type: XPathLocatorType,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum XPathLocatorType {
    #[serde(rename = "xpath")]
    Xpath,
}
impl XPathLocator {
    pub fn new(r#type: impl Into<XPathLocatorType>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl XPathLocator {
    pub const IDENTIFIER: &'static str = "browsingContext.XPathLocator";
    pub const DOMAIN_DIRECTION: &'static str = "all";
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
    pub const DOMAIN_DIRECTION: &'static str = "all";
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
    pub const DOMAIN_DIRECTION: &'static str = "remote";
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
    pub r#type: ElementClipRectangleType,
    #[serde(rename = "element")]
    pub element: crate::script::types::SharedReference,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementClipRectangleType {
    #[serde(rename = "element")]
    Element,
}
impl ElementClipRectangle {
    pub fn new(
        r#type: impl Into<ElementClipRectangleType>,
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
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BoxClipRectangle {
    #[serde(rename = "type")]
    pub r#type: BoxClipRectangleType,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "y")]
    pub y: f64,
    #[serde(rename = "width")]
    pub width: f64,
    #[serde(rename = "height")]
    pub height: f64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BoxClipRectangleType {
    #[serde(rename = "box")]
    Box,
}
impl BoxClipRectangle {
    pub const IDENTIFIER: &'static str = "browsingContext.BoxClipRectangle";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CreateType {
    #[serde(rename = "tab")]
    Tab,
    #[serde(rename = "window")]
    Window,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, serde_valid :: Validate)]
pub struct PrintMarginParameters {
    #[serde(rename = "bottom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_margin_parameters_bottom")]
    #[validate(minimum = 0f64)]
    pub bottom: Option<f64>,
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_margin_parameters_left")]
    #[validate(minimum = 0f64)]
    pub left: Option<f64>,
    #[serde(rename = "right")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_margin_parameters_right")]
    #[validate(minimum = 0f64)]
    pub right: Option<f64>,
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_margin_parameters_top")]
    #[validate(minimum = 0f64)]
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
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, serde_valid :: Validate)]
pub struct PrintPageParameters {
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_page_parameters_height")]
    #[validate(minimum = 0.0352f64)]
    pub height: Option<f64>,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_page_parameters_width")]
    #[validate(minimum = 0.0352f64)]
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
    pub const DOMAIN_DIRECTION: &'static str = "remote";
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
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    #[serde(rename = "children")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub children: Option<InfoList>,
    #[serde(rename = "clientWindow")]
    pub client_window: crate::browser::types::ClientWindow,
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "originalOpener")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub original_opener: Option<BrowsingContext>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "userContext")]
    pub user_context: crate::browser::types::UserContext,
    #[serde(rename = "parent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent: Option<BrowsingContext>,
}
impl Info {
    pub fn new(
        client_window: impl Into<crate::browser::types::ClientWindow>,
        context: impl Into<BrowsingContext>,
        url: impl Into<String>,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        Self {
            client_window: client_window.into(),
            context: context.into(),
            url: url.into(),
            user_context: user_context.into(),
            children: None,
            original_opener: None,
            parent: None,
        }
    }
}
impl Info {
    pub const IDENTIFIER: &'static str = "browsingContext.Info";
    pub const DOMAIN_DIRECTION: &'static str = "local";
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
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadCanceledParamsDownloadCompleteParamsUnion {
    DownloadCanceledParams(DownloadCanceledParams),
    DownloadCompleteParams(DownloadCompleteParams),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadCanceledParams {
    #[serde(rename = "status")]
    pub status: DownloadCanceledParamsStatus,
    #[serde(flatten)]
    #[serde(default)]
    pub base_navigation_info: BaseNavigationInfo,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DownloadCanceledParamsStatus {
    #[serde(rename = "canceled")]
    Canceled,
}
impl DownloadCanceledParams {
    pub fn new(
        status: impl Into<DownloadCanceledParamsStatus>,
        base_navigation_info: impl Into<BaseNavigationInfo>,
    ) -> Self {
        Self {
            status: status.into(),
            base_navigation_info: base_navigation_info.into(),
        }
    }
}
impl DownloadCanceledParams {
    pub const IDENTIFIER: &'static str = "browsingContext.DownloadCanceledParams";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadCompleteParams {
    #[serde(rename = "status")]
    pub status: DownloadCompleteParamsStatus,
    #[serde(rename = "filepath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub filepath: Option<String>,
    #[serde(flatten)]
    #[serde(default)]
    pub base_navigation_info: BaseNavigationInfo,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DownloadCompleteParamsStatus {
    #[serde(rename = "complete")]
    Complete,
}
impl DownloadCompleteParams {
    pub fn new(
        status: impl Into<DownloadCompleteParamsStatus>,
        base_navigation_info: impl Into<BaseNavigationInfo>,
    ) -> Self {
        Self {
            status: status.into(),
            base_navigation_info: base_navigation_info.into(),
            filepath: None,
        }
    }
}
impl DownloadCompleteParams {
    pub const IDENTIFIER: &'static str = "browsingContext.DownloadCompleteParams";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
group_enum ! (BrowsingContextType { BrowsingContext (BrowsingContext) , Locator (Locator) , Navigation (Navigation) , ReadinessState (ReadinessState) , UserPromptType (UserPromptType) , ImageFormat (ImageFormat) , ClipRectangle (ClipRectangle) , CreateType (CreateType) , PrintMarginParameters (PrintMarginParameters) , PrintPageParameters (PrintPageParameters) , Viewport (Viewport) , InfoList (InfoList) , Info (Info) , BaseNavigationInfo (BaseNavigationInfo) , DownloadCanceledParamsDownloadCompleteParamsUnion (DownloadCanceledParamsDownloadCompleteParamsUnion) });
