// Generated types for module

use serde::{Serialize, Deserialize};
use crate::browser::types::ClientWindow;
use crate::browser::types::UserContext;
use crate::script::types::SharedReference;
use crate::session::types::UserPromptHandlerType;
use serde_valid::Validate;

pub type BrowsingContext = String;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OriginUnion {
    #[serde(rename = "viewport")]
    Viewport,
    #[serde(rename = "document")]
    Document,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ImageFormat {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "quality")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.0)]
    pub quality: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BoxEnum {
    #[serde(rename = "box")]
    Box,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxClipRectangle {
    #[serde(rename = "type")]
    pub r#type: BoxEnum,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "y")]
    pub y: f64,
    #[serde(rename = "width")]
    pub width: f64,
    #[serde(rename = "height")]
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ElementEnum {
    #[serde(rename = "element")]
    Element,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementClipRectangle {
    #[serde(rename = "type")]
    pub r#type: ElementEnum,
    #[serde(rename = "element")]
    pub element: SharedReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClipRectangle {
    BoxClipRectangle(BoxClipRectangle),
    ElementClipRectangle(ElementClipRectangle),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateType {
    #[serde(rename = "tab")]
    Tab,
    #[serde(rename = "window")]
    Window,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AccessibilityEnum {
    #[serde(rename = "accessibility")]
    Accessibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityLocatorValue {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "role")]
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityLocator {
    #[serde(rename = "type")]
    pub r#type: AccessibilityEnum,
    #[serde(rename = "value")]
    pub value: AccessibilityLocatorValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CssEnum {
    #[serde(rename = "css")]
    Css,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CssLocator {
    #[serde(rename = "type")]
    pub r#type: CssEnum,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContextEnum {
    #[serde(rename = "context")]
    Context,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextLocatorValue {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextLocator {
    #[serde(rename = "type")]
    pub r#type: ContextEnum,
    #[serde(rename = "value")]
    pub value: ContextLocatorValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InnerTextEnum {
    #[serde(rename = "innerText")]
    InnerText,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MatchTypeUnion {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "partial")]
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerTextLocator {
    #[serde(rename = "type")]
    pub r#type: InnerTextEnum,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "ignoreCase")]
    pub ignore_case: Option<bool>,
    #[serde(rename = "matchType")]
    pub match_type: Option<MatchTypeUnion>,
    #[serde(rename = "maxDepth")]
    pub max_depth: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum XpathEnum {
    #[serde(rename = "xpath")]
    Xpath,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XPathLocator {
    #[serde(rename = "type")]
    pub r#type: XpathEnum,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Locator {
    AccessibilityLocator(AccessibilityLocator),
    CssLocator(CssLocator),
    ContextLocator(ContextLocator),
    InnerTextLocator(InnerTextLocator),
    XPathLocator(XPathLocator),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReadinessState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "interactive")]
    Interactive,
    #[serde(rename = "complete")]
    Complete,
}

fn print_margin_parameters_default_bottom() -> f64 {
    1.0
}

fn print_margin_parameters_default_left() -> f64 {
    1.0
}

fn print_margin_parameters_default_right() -> f64 {
    1.0
}

fn print_margin_parameters_default_top() -> f64 {
    1.0
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PrintMarginParameters {
    #[serde(rename = "bottom")]
    #[validate(minimum = 0.0)]
    #[serde(default = "print_margin_parameters_default_bottom")]
    pub bottom: f64,
    #[serde(rename = "left")]
    #[validate(minimum = 0.0)]
    #[serde(default = "print_margin_parameters_default_left")]
    pub left: f64,
    #[serde(rename = "right")]
    #[validate(minimum = 0.0)]
    #[serde(default = "print_margin_parameters_default_right")]
    pub right: f64,
    #[serde(rename = "top")]
    #[validate(minimum = 0.0)]
    #[serde(default = "print_margin_parameters_default_top")]
    pub top: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrientationUnion {
    #[serde(rename = "portrait")]
    Portrait,
    #[serde(rename = "landscape")]
    Landscape,
}

fn print_page_parameters_default_height() -> f64 {
    27.94
}

fn print_page_parameters_default_width() -> f64 {
    21.59
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PrintPageParameters {
    #[serde(rename = "height")]
    #[validate(minimum = 0.0352)]
    #[serde(default = "print_page_parameters_default_height")]
    pub height: f64,
    #[serde(rename = "width")]
    #[validate(minimum = 0.0352)]
    #[serde(default = "print_page_parameters_default_width")]
    pub width: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PageRangesUnion {
    U64(u64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewport {
    #[serde(rename = "width")]
    pub width: u64,
    #[serde(rename = "height")]
    pub height: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    #[serde(rename = "children")]
    pub children: Option<InfoList>,
    #[serde(rename = "clientWindow")]
    pub client_window: ClientWindow,
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "originalOpener")]
    pub original_opener: Option<BrowsingContext>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "userContext")]
    pub user_context: UserContext,
    #[serde(rename = "parent")]
    pub parent: Option<Option<BrowsingContext>>,
}

pub type InfoList = Vec<Info>;


pub type Navigation = String;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseNavigationInfo {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "navigation")]
    pub navigation: Option<Navigation>,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationInfo {
    #[serde(flatten)]
    pub base_navigation_info: BaseNavigationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CanceledEnum {
    #[serde(rename = "canceled")]
    Canceled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadCanceledParams {
    #[serde(rename = "status")]
    pub status: CanceledEnum,
    #[serde(flatten)]
    pub base_navigation_info: BaseNavigationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompleteEnum {
    #[serde(rename = "complete")]
    Complete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadCompleteParams {
    #[serde(rename = "status")]
    pub status: CompleteEnum,
    #[serde(rename = "filepath")]
    pub filepath: Option<String>,
    #[serde(flatten)]
    pub base_navigation_info: BaseNavigationInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadCanceledParamsDownloadCompleteParamsUnion {
    DownloadCanceledParams(DownloadCanceledParams),
    DownloadCompleteParams(DownloadCompleteParams),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryUpdatedParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPromptClosedParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "accepted")]
    pub accepted: bool,
    #[serde(rename = "type")]
    pub r#type: UserPromptType,
    #[serde(rename = "userText")]
    pub user_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPromptOpenedParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "handler")]
    pub handler: UserPromptHandlerType,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "type")]
    pub r#type: UserPromptType,
    #[serde(rename = "defaultValue")]
    pub default_value: Option<String>,
}

