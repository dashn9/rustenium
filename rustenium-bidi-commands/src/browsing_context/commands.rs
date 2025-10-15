// Generated commands for module

use serde::{Serialize, Deserialize};
use crate::browser::types::UserContext;
use crate::script::types::NodeRemoteValue;
use crate::script::types::SerializationOptions;
use crate::script::types::SharedReference;
use crate::EmptyResult;
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
pub enum BrowsingContextActivateMethod {
    #[serde(rename = "browsingContext.activate")]
    BrowsingContextActivate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowsingContextCaptureScreenshotMethod {
    #[serde(rename = "browsingContext.captureScreenshot")]
    BrowsingContextCaptureScreenshot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowsingContextCloseMethod {
    #[serde(rename = "browsingContext.close")]
    BrowsingContextClose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowsingContextCreateMethod {
    #[serde(rename = "browsingContext.create")]
    BrowsingContextCreate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowsingContextGetTreeMethod {
    #[serde(rename = "browsingContext.getTree")]
    BrowsingContextGetTree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowsingContextHandleUserPromptMethod {
    #[serde(rename = "browsingContext.handleUserPrompt")]
    BrowsingContextHandleUserPrompt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowsingContextLocateNodesMethod {
    #[serde(rename = "browsingContext.locateNodes")]
    BrowsingContextLocateNodes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowsingContextNavigateMethod {
    #[serde(rename = "browsingContext.navigate")]
    BrowsingContextNavigate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowsingContextPrintMethod {
    #[serde(rename = "browsingContext.print")]
    BrowsingContextPrint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowsingContextReloadMethod {
    #[serde(rename = "browsingContext.reload")]
    BrowsingContextReload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowsingContextSetViewportMethod {
    #[serde(rename = "browsingContext.setViewport")]
    BrowsingContextSetViewport,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BrowsingContextTraverseHistoryMethod {
    #[serde(rename = "browsingContext.traverseHistory")]
    BrowsingContextTraverseHistory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivateParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureScreenshotParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<OriginUnion>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<ImageFormat>,
    #[serde(rename = "clip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clip: Option<ClipRectangle>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "promptUnload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_unload: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateParameters {
    #[serde(rename = "type")]
    pub r#type: CreateType,
    #[serde(rename = "referenceContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_context: Option<BrowsingContext>,
    #[serde(rename = "background")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(rename = "userContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_context: Option<UserContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTreeParameters {
    #[serde(rename = "maxDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<u64>,
    #[serde(rename = "root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<BrowsingContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandleUserPromptParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "accept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept: Option<bool>,
    #[serde(rename = "userText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_text: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LocateNodesParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "locator")]
    pub locator: Locator,
    #[serde(rename = "maxNodeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 1)]
    pub max_node_count: Option<u64>,
    #[serde(rename = "serializationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serialization_options: Option<SerializationOptions>,
    #[serde(rename = "startNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_nodes: Option<Vec<SharedReference>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigateParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "wait")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<ReadinessState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PrintParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "background")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(rename = "margin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin: Option<PrintMarginParameters>,
    #[serde(rename = "orientation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<OrientationUnion>,
    #[serde(rename = "page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<PrintPageParameters>,
    #[serde(rename = "pageRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_ranges: Option<Vec<PageRangesUnion>>,
    #[serde(rename = "scale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 0.1)]
    #[validate(maximum = 2.0)]
    pub scale: Option<f64>,
    #[serde(rename = "shrinkToFit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shrink_to_fit: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReloadParameters {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "ignoreCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_cache: Option<bool>,
    #[serde(rename = "wait")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait: Option<ReadinessState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct SetViewportParameters {
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<BrowsingContext>,
    #[serde(rename = "viewport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewport: Option<Option<Viewport>>,
    #[serde(rename = "devicePixelRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 0.0000000000000002220446049250313)]
    pub device_pixel_ratio: Option<Option<f64>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    ActivateResult(ActivateResult),
    CaptureScreenshotResult(CaptureScreenshotResult),
    CloseResult(CloseResult),
    CreateResult(CreateResult),
    GetTreeResult(GetTreeResult),
    HandleUserPromptResult(HandleUserPromptResult),
    LocateNodesResult(LocateNodesResult),
    NavigateResult(NavigateResult),
    PrintResult(PrintResult),
    ReloadResult(ReloadResult),
    SetViewportResult(SetViewportResult),
    TraverseHistoryResult(TraverseHistoryResult),
}


pub type ActivateResult = EmptyResult;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureScreenshotResult {
    #[serde(rename = "data")]
    pub data: String,
}

pub type CloseResult = EmptyResult;


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

pub type HandleUserPromptResult = EmptyResult;


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

pub type ReloadResult = NavigateResult;


pub type SetViewportResult = EmptyResult;


pub type TraverseHistoryResult = EmptyResult;


