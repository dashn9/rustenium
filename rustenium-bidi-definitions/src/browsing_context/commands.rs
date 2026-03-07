use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActivateParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
}
impl ActivateParams {
    pub fn new(context: impl Into<super::types::BrowsingContext>) -> Self {
        Self {
            context: context.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActivateMethod {
    #[serde(rename = "browsingContext.activate")]
    Activate,
}
impl ActivateMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.activate";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Activate {
    pub method: ActivateMethod,
    pub params: ActivateParams,
}
impl crate::CommandResult for Activate {
    type Result = super::results::ActivateResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaptureScreenshotParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub origin: Option<CaptureScreenshotOrigin>,
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub format: Option<super::types::ImageFormat>,
    #[serde(rename = "clip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub clip: Option<super::types::ClipRectangle>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CaptureScreenshotOrigin {
    #[serde(rename = "viewport")]
    Viewport,
    #[serde(rename = "document")]
    Document,
}
impl CaptureScreenshotParams {
    pub fn new(context: impl Into<super::types::BrowsingContext>) -> Self {
        Self {
            context: context.into(),
            origin: None,
            format: None,
            clip: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CaptureScreenshotMethod {
    #[serde(rename = "browsingContext.captureScreenshot")]
    CaptureScreenshot,
}
impl CaptureScreenshotMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.captureScreenshot";
}
#[derive(Debug, Clone, PartialEq)]
pub struct CaptureScreenshot {
    pub method: CaptureScreenshotMethod,
    pub params: CaptureScreenshotParams,
}
impl crate::CommandResult for CaptureScreenshot {
    type Result = super::results::CaptureScreenshotResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloseParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "promptUnload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_close_params_prompt_unload")]
    pub prompt_unload: Option<bool>,
}
fn default_close_params_prompt_unload() -> Option<bool> {
    Some(false)
}
impl CloseParams {
    pub fn new(context: impl Into<super::types::BrowsingContext>) -> Self {
        Self {
            context: context.into(),
            prompt_unload: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CloseMethod {
    #[serde(rename = "browsingContext.close")]
    Close,
}
impl CloseMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.close";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Close {
    pub method: CloseMethod,
    pub params: CloseParams,
}
impl crate::CommandResult for Close {
    type Result = super::results::CloseResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateParams {
    #[serde(rename = "type")]
    pub r#type: super::types::CreateType,
    #[serde(rename = "referenceContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub reference_context: Option<super::types::BrowsingContext>,
    #[serde(rename = "background")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_create_params_background")]
    pub background: Option<bool>,
    #[serde(rename = "userContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_context: Option<crate::browser::types::UserContext>,
}
fn default_create_params_background() -> Option<bool> {
    Some(false)
}
impl CreateParams {
    pub fn new(r#type: impl Into<super::types::CreateType>) -> Self {
        Self {
            r#type: r#type.into(),
            reference_context: None,
            background: None,
            user_context: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CreateMethod {
    #[serde(rename = "browsingContext.create")]
    Create,
}
impl CreateMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.create";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Create {
    pub method: CreateMethod,
    pub params: CreateParams,
}
impl crate::CommandResult for Create {
    type Result = super::results::CreateResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetTreeParams {
    #[serde(rename = "maxDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_depth: Option<u64>,
    #[serde(rename = "root")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub root: Option<super::types::BrowsingContext>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetTreeMethod {
    #[serde(rename = "browsingContext.getTree")]
    GetTree,
}
impl GetTreeMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.getTree";
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetTree {
    pub method: GetTreeMethod,
    pub params: GetTreeParams,
}
impl crate::CommandResult for GetTree {
    type Result = super::results::GetTreeResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HandleUserPromptParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "accept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub accept: Option<bool>,
    #[serde(rename = "userText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_text: Option<String>,
}
impl HandleUserPromptParams {
    pub fn new(context: impl Into<super::types::BrowsingContext>) -> Self {
        Self {
            context: context.into(),
            accept: None,
            user_text: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum HandleUserPromptMethod {
    #[serde(rename = "browsingContext.handleUserPrompt")]
    HandleUserPrompt,
}
impl HandleUserPromptMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.handleUserPrompt";
}
#[derive(Debug, Clone, PartialEq)]
pub struct HandleUserPrompt {
    pub method: HandleUserPromptMethod,
    pub params: HandleUserPromptParams,
}
impl crate::CommandResult for HandleUserPrompt {
    type Result = super::results::HandleUserPromptResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, serde_valid :: Validate)]
pub struct LocateNodesParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "locator")]
    pub locator: super::types::Locator,
    #[serde(rename = "maxNodeCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[validate(minimum = 1u64)]
    pub max_node_count: Option<u64>,
    #[serde(rename = "serializationOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub serialization_options: Option<crate::script::types::SerializationOptions>,
    #[serde(rename = "startNodes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub start_nodes: Option<Vec<crate::script::types::SharedReference>>,
}
impl LocateNodesParams {
    pub fn new(
        context: impl Into<super::types::BrowsingContext>,
        locator: impl Into<super::types::Locator>,
    ) -> Self {
        Self {
            context: context.into(),
            locator: locator.into(),
            max_node_count: None,
            serialization_options: None,
            start_nodes: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LocateNodesMethod {
    #[serde(rename = "browsingContext.locateNodes")]
    LocateNodes,
}
impl LocateNodesMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.locateNodes";
}
#[derive(Debug, Clone, PartialEq)]
pub struct LocateNodes {
    pub method: LocateNodesMethod,
    pub params: LocateNodesParams,
}
impl crate::CommandResult for LocateNodes {
    type Result = super::results::LocateNodesResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NavigateParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "wait")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub wait: Option<super::types::ReadinessState>,
}
impl NavigateParams {
    pub fn new(context: impl Into<super::types::BrowsingContext>, url: impl Into<String>) -> Self {
        Self {
            context: context.into(),
            url: url.into(),
            wait: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NavigateMethod {
    #[serde(rename = "browsingContext.navigate")]
    Navigate,
}
impl NavigateMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.navigate";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Navigate {
    pub method: NavigateMethod,
    pub params: NavigateParams,
}
impl crate::CommandResult for Navigate {
    type Result = super::results::NavigateResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "background")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_params_background")]
    pub background: Option<bool>,
    #[serde(rename = "margin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub margin: Option<super::types::PrintMarginParameters>,
    #[serde(rename = "orientation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub orientation: Option<PrintOrientation>,
    #[serde(rename = "page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub page: Option<super::types::PrintPageParameters>,
    #[serde(rename = "pageRanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub page_ranges: Option<Vec<serde_json::Value>>,
    #[serde(rename = "scale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_params_scale")]
    pub scale: Option<f64>,
    #[serde(rename = "shrinkToFit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_print_params_shrink_to_fit")]
    pub shrink_to_fit: Option<bool>,
}
fn default_print_params_background() -> Option<bool> {
    Some(false)
}
fn default_print_params_scale() -> Option<f64> {
    Some(1f64)
}
fn default_print_params_shrink_to_fit() -> Option<bool> {
    Some(true)
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrintOrientation {
    #[serde(rename = "portrait")]
    Portrait,
    #[serde(rename = "landscape")]
    Landscape,
}
impl PrintParams {
    pub fn new(context: impl Into<super::types::BrowsingContext>) -> Self {
        Self {
            context: context.into(),
            background: None,
            margin: None,
            orientation: None,
            page: None,
            page_ranges: None,
            scale: None,
            shrink_to_fit: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PrintMethod {
    #[serde(rename = "browsingContext.print")]
    Print,
}
impl PrintMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.print";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Print {
    pub method: PrintMethod,
    pub params: PrintParams,
}
impl crate::CommandResult for Print {
    type Result = super::results::PrintResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReloadParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "ignoreCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub ignore_cache: Option<bool>,
    #[serde(rename = "wait")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub wait: Option<super::types::ReadinessState>,
}
impl ReloadParams {
    pub fn new(context: impl Into<super::types::BrowsingContext>) -> Self {
        Self {
            context: context.into(),
            ignore_cache: None,
            wait: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReloadMethod {
    #[serde(rename = "browsingContext.reload")]
    Reload,
}
impl ReloadMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.reload";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Reload {
    pub method: ReloadMethod,
    pub params: ReloadParams,
}
impl crate::CommandResult for Reload {
    type Result = super::results::ReloadResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, serde_valid :: Validate)]
pub struct SetViewportParams {
    #[serde(rename = "context")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub context: Option<super::types::BrowsingContext>,
    #[serde(rename = "viewport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub viewport: Option<super::types::Viewport>,
    #[serde(rename = "devicePixelRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[validate(exclusive_minimum = 0f64)]
    pub device_pixel_ratio: Option<f64>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetViewportMethod {
    #[serde(rename = "browsingContext.setViewport")]
    SetViewport,
}
impl SetViewportMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.setViewport";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetViewport {
    pub method: SetViewportMethod,
    pub params: SetViewportParams,
}
impl crate::CommandResult for SetViewport {
    type Result = super::results::SetViewportResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraverseHistoryParams {
    #[serde(rename = "context")]
    pub context: super::types::BrowsingContext,
    #[serde(rename = "delta")]
    pub delta: i64,
}
impl TraverseHistoryParams {
    pub fn new(context: impl Into<super::types::BrowsingContext>, delta: impl Into<i64>) -> Self {
        Self {
            context: context.into(),
            delta: delta.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TraverseHistoryMethod {
    #[serde(rename = "browsingContext.traverseHistory")]
    TraverseHistory,
}
impl TraverseHistoryMethod {
    pub const IDENTIFIER: &'static str = "browsingContext.traverseHistory";
}
#[derive(Debug, Clone, PartialEq)]
pub struct TraverseHistory {
    pub method: TraverseHistoryMethod,
    pub params: TraverseHistoryParams,
}
impl crate::CommandResult for TraverseHistory {
    type Result = super::results::TraverseHistoryResult;
}
group_enum ! (BrowsingContextCommands { Activate (Activate) , CaptureScreenshot (CaptureScreenshot) , Close (Close) , Create (Create) , GetTree (GetTree) , HandleUserPrompt (HandleUserPrompt) , LocateNodes (LocateNodes) , Navigate (Navigate) , Print (Print) , Reload (Reload) , SetViewport (SetViewport) , TraverseHistory (TraverseHistory) });
