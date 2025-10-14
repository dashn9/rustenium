// Generated commands for module

use serde::{Serialize, Deserialize};
use crate::browser::types::UserContext;
use crate::script::types::NodeRemoteValue;
use crate::script::types::SerializationOptions;
use crate::script::types::SharedReference;
use serde_valid::Validate;
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextCommand {
    Activate(Activate),
    CaptureScreenshot(CaptureScreenshot),
    Close(Close),
    Create(Create),
    GetTree(GetTree),
    HandleUserPrompt(HandleUserPrompt),
    LocateNodes(LocateNodes),
    Navigate(Navigate),
    Print(Print),
    Reload(Reload),
    SetViewport(SetViewport),
    TraverseHistory(TraverseHistory),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextActivateMethod {
    #[serde(rename = "browsingContext.Activate")]
    BrowsingContextActivate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextCaptureScreenshotMethod {
    #[serde(rename = "browsingContext.CaptureScreenshot")]
    BrowsingContextCaptureScreenshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextCloseMethod {
    #[serde(rename = "browsingContext.Close")]
    BrowsingContextClose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextCreateMethod {
    #[serde(rename = "browsingContext.Create")]
    BrowsingContextCreate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextGetTreeMethod {
    #[serde(rename = "browsingContext.GetTree")]
    BrowsingContextGetTree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextHandleUserPromptMethod {
    #[serde(rename = "browsingContext.HandleUserPrompt")]
    BrowsingContextHandleUserPrompt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextLocateNodesMethod {
    #[serde(rename = "browsingContext.LocateNodes")]
    BrowsingContextLocateNodes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextNavigateMethod {
    #[serde(rename = "browsingContext.Navigate")]
    BrowsingContextNavigate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextPrintMethod {
    #[serde(rename = "browsingContext.Print")]
    BrowsingContextPrint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextReloadMethod {
    #[serde(rename = "browsingContext.Reload")]
    BrowsingContextReload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextSetViewportMethod {
    #[serde(rename = "browsingContext.SetViewport")]
    BrowsingContextSetViewport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextTraverseHistoryMethod {
    #[serde(rename = "browsingContext.TraverseHistory")]
    BrowsingContextTraverseHistory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
}

fn capture_screenshot_parameters_default_origin() -> OriginUnion {
    OriginUnion::Viewport
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureScreenshotParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "origin")]
    #[serde(default = "capture_screenshot_parameters_default_origin")]
    pub origin: OriginUnion,
    #[serde(rename = "format")]
    pub format: Option<ImageFormat>,
    #[serde(rename = "clip")]
    pub clip: Option<ClipRectangle>,
}

fn close_parameters_default_prompt_unload() -> bool {
    false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "promptUnload")]
    #[serde(default = "close_parameters_default_prompt_unload")]
    pub prompt_unload: bool,
}

fn create_parameters_default_background() -> bool {
    false
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateParameters {
    #[serde(rename = "type")]
    pub r#type: CreateType,
    #[serde(rename = "referenceContext")]
    pub reference_context: Option<BrowsingContext>,
    #[serde(rename = "background")]
    #[serde(default = "create_parameters_default_background")]
    pub background: bool,
    #[serde(rename = "userContext")]
    pub user_context: Option<UserContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTreeParameters {
    #[serde(rename = "maxDepth")]
    pub max_depth: Option<u64>,
    #[serde(rename = "root")]
    pub root: Option<BrowsingContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandleUserPromptParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "accept")]
    pub accept: Option<bool>,
    #[serde(rename = "userText")]
    pub user_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LocateNodesParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "locator")]
    pub locator: Locator,
    #[serde(rename = "maxNodeCount")]
    #[validate(minimum = 1)]
    pub max_node_count: Option<u64>,
    #[serde(rename = "serializationOptions")]
    pub serialization_options: Option<SerializationOptions>,
    #[serde(rename = "startNodes")]
    pub start_nodes: Option<Vec<SharedReference>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigateParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "wait")]
    pub wait: Option<ReadinessState>,
}

fn print_parameters_default_background() -> bool {
    false
}

fn print_parameters_default_orientation() -> OrientationUnion {
    OrientationUnion::Portrait
}

fn print_parameters_default_scale() -> f64 {
    1.0
}

fn print_parameters_default_shrink_to_fit() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PrintParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "background")]
    #[serde(default = "print_parameters_default_background")]
    pub background: bool,
    #[serde(rename = "margin")]
    pub margin: Option<PrintMarginParameters>,
    #[serde(rename = "orientation")]
    #[serde(default = "print_parameters_default_orientation")]
    pub orientation: OrientationUnion,
    #[serde(rename = "page")]
    pub page: Option<PrintPageParameters>,
    #[serde(rename = "pageRanges")]
    pub page_ranges: Option<Vec<PageRangesUnion>>,
    #[serde(rename = "scale")]
    #[validate(minimum = 0.1)]
    #[validate(maximum = 2.0)]
    #[serde(default = "print_parameters_default_scale")]
    pub scale: f64,
    #[serde(rename = "shrinkToFit")]
    #[serde(default = "print_parameters_default_shrink_to_fit")]
    pub shrink_to_fit: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReloadParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "ignoreCache")]
    pub ignore_cache: Option<bool>,
    #[serde(rename = "wait")]
    pub wait: Option<ReadinessState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SetViewportParameters {
    #[serde(rename = "context")]
    pub context: Option<BrowsingContext>,
    #[serde(rename = "viewport")]
    pub viewport: Option<Option<Viewport>>,
    #[serde(rename = "devicePixelRatio")]
    #[validate(minimum = 0.0000000000000002220446049250313)]
    pub device_pixel_ratio: Option<Option<f64>>,
    #[serde(rename = "userContexts")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraverseHistoryParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "delta")]
    pub delta: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activate {
    #[serde(rename = "method")]
    pub method: BrowsingContextActivateMethod,
    #[serde(rename = "params")]
    pub params: ActivateParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureScreenshot {
    #[serde(rename = "method")]
    pub method: BrowsingContextCaptureScreenshotMethod,
    #[serde(rename = "params")]
    pub params: CaptureScreenshotParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Close {
    #[serde(rename = "method")]
    pub method: BrowsingContextCloseMethod,
    #[serde(rename = "params")]
    pub params: CloseParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Create {
    #[serde(rename = "method")]
    pub method: BrowsingContextCreateMethod,
    #[serde(rename = "params")]
    pub params: CreateParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTree {
    #[serde(rename = "method")]
    pub method: BrowsingContextGetTreeMethod,
    #[serde(rename = "params")]
    pub params: GetTreeParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandleUserPrompt {
    #[serde(rename = "method")]
    pub method: BrowsingContextHandleUserPromptMethod,
    #[serde(rename = "params")]
    pub params: HandleUserPromptParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocateNodes {
    #[serde(rename = "method")]
    pub method: BrowsingContextLocateNodesMethod,
    #[serde(rename = "params")]
    pub params: LocateNodesParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Navigate {
    #[serde(rename = "method")]
    pub method: BrowsingContextNavigateMethod,
    #[serde(rename = "params")]
    pub params: NavigateParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Print {
    #[serde(rename = "method")]
    pub method: BrowsingContextPrintMethod,
    #[serde(rename = "params")]
    pub params: PrintParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reload {
    #[serde(rename = "method")]
    pub method: BrowsingContextReloadMethod,
    #[serde(rename = "params")]
    pub params: ReloadParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetViewport {
    #[serde(rename = "method")]
    pub method: BrowsingContextSetViewportMethod,
    #[serde(rename = "params")]
    pub params: SetViewportParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraverseHistory {
    #[serde(rename = "method")]
    pub method: BrowsingContextTraverseHistoryMethod,
    #[serde(rename = "params")]
    pub params: TraverseHistoryParameters,
}

// Generated results

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowsingContextResult {
    CaptureScreenshotResult(CaptureScreenshotResult),
    CreateResult(CreateResult),
    GetTreeResult(GetTreeResult),
    LocateNodesResult(LocateNodesResult),
    NavigateResult(NavigateResult),
    PrintResult(PrintResult),
    TraverseHistoryResult(TraverseHistoryResult),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureScreenshotResult {
    #[serde(rename = "data")]
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResult {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTreeResult {
    #[serde(rename = "contexts")]
    pub contexts: InfoList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocateNodesResult {
    #[serde(rename = "nodes")]
    pub nodes: Vec<NodeRemoteValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigateResult {
    #[serde(rename = "navigation")]
    pub navigation: Option<Navigation>,
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintResult {
    #[serde(rename = "data")]
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraverseHistoryResult {
}

