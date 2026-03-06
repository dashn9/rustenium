use super::commands::*;
impl Activate {
    pub fn builder() -> ActivateBuilder {
        ActivateBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ActivateBuilder {
    context: Option<super::types::BrowsingContext>,
}
impl ActivateBuilder {
    pub fn context(mut self, context: impl Into<super::types::BrowsingContext>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn build(self) -> Result<Activate, String> {
        Ok(Activate {
            method: ActivateMethod::Activate,
            params: ActivateParams {
                context: self
                    .context
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
            },
        })
    }
}
impl CaptureScreenshot {
    pub fn builder() -> CaptureScreenshotBuilder {
        CaptureScreenshotBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CaptureScreenshotBuilder {
    context: Option<super::types::BrowsingContext>,
    origin: Option<CaptureScreenshotOrigin>,
    format: Option<super::types::ImageFormat>,
    clip: Option<super::types::ClipRectangle>,
}
impl CaptureScreenshotBuilder {
    pub fn context(mut self, context: impl Into<super::types::BrowsingContext>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<CaptureScreenshotOrigin>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn format(mut self, format: impl Into<super::types::ImageFormat>) -> Self {
        self.format = Some(format.into());
        self
    }
    pub fn clip(mut self, clip: impl Into<super::types::ClipRectangle>) -> Self {
        self.clip = Some(clip.into());
        self
    }
    pub fn build(self) -> Result<CaptureScreenshot, String> {
        Ok(CaptureScreenshot {
            method: CaptureScreenshotMethod::CaptureScreenshot,
            params: CaptureScreenshotParams {
                context: self
                    .context
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
                origin: self.origin,
                format: self.format,
                clip: self.clip,
            },
        })
    }
}
impl Close {
    pub fn builder() -> CloseBuilder {
        CloseBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CloseBuilder {
    context: Option<super::types::BrowsingContext>,
    prompt_unload: Option<bool>,
}
impl CloseBuilder {
    pub fn context(mut self, context: impl Into<super::types::BrowsingContext>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn prompt_unload(mut self, prompt_unload: impl Into<bool>) -> Self {
        self.prompt_unload = Some(prompt_unload.into());
        self
    }
    pub fn build(self) -> Result<Close, String> {
        Ok(Close {
            method: CloseMethod::Close,
            params: CloseParams {
                context: self
                    .context
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
                prompt_unload: self.prompt_unload,
            },
        })
    }
}
impl Create {
    pub fn builder() -> CreateBuilder {
        CreateBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CreateBuilder {
    r#type: Option<super::types::CreateType>,
    reference_context: Option<super::types::BrowsingContext>,
    background: Option<bool>,
    user_context: Option<crate::browser::types::UserContext>,
}
impl CreateBuilder {
    pub fn r#type(mut self, r#type: impl Into<super::types::CreateType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn reference_context(
        mut self,
        reference_context: impl Into<super::types::BrowsingContext>,
    ) -> Self {
        self.reference_context = Some(reference_context.into());
        self
    }
    pub fn background(mut self, background: impl Into<bool>) -> Self {
        self.background = Some(background.into());
        self
    }
    pub fn user_context(
        mut self,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        self.user_context = Some(user_context.into());
        self
    }
    pub fn build(self) -> Result<Create, String> {
        Ok(Create {
            method: CreateMethod::Create,
            params: CreateParams {
                r#type: self
                    .r#type
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
                reference_context: self.reference_context,
                background: self.background,
                user_context: self.user_context,
            },
        })
    }
}
impl GetTree {
    pub fn builder() -> GetTreeBuilder {
        GetTreeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetTreeBuilder {
    max_depth: Option<u64>,
    root: Option<super::types::BrowsingContext>,
}
impl GetTreeBuilder {
    pub fn max_depth(mut self, max_depth: impl Into<u64>) -> Self {
        self.max_depth = Some(max_depth.into());
        self
    }
    pub fn root(mut self, root: impl Into<super::types::BrowsingContext>) -> Self {
        self.root = Some(root.into());
        self
    }
    pub fn build(self) -> GetTree {
        GetTree {
            method: GetTreeMethod::GetTree,
            params: GetTreeParams {
                max_depth: self.max_depth,
                root: self.root,
            },
        }
    }
}
impl HandleUserPrompt {
    pub fn builder() -> HandleUserPromptBuilder {
        HandleUserPromptBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct HandleUserPromptBuilder {
    context: Option<super::types::BrowsingContext>,
    accept: Option<bool>,
    user_text: Option<String>,
}
impl HandleUserPromptBuilder {
    pub fn context(mut self, context: impl Into<super::types::BrowsingContext>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn accept(mut self, accept: impl Into<bool>) -> Self {
        self.accept = Some(accept.into());
        self
    }
    pub fn user_text(mut self, user_text: impl Into<String>) -> Self {
        self.user_text = Some(user_text.into());
        self
    }
    pub fn build(self) -> Result<HandleUserPrompt, String> {
        Ok(HandleUserPrompt {
            method: HandleUserPromptMethod::HandleUserPrompt,
            params: HandleUserPromptParams {
                context: self
                    .context
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
                accept: self.accept,
                user_text: self.user_text,
            },
        })
    }
}
impl LocateNodes {
    pub fn builder() -> LocateNodesBuilder {
        LocateNodesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct LocateNodesBuilder {
    context: Option<super::types::BrowsingContext>,
    locator: Option<super::types::Locator>,
    max_node_count: Option<u64>,
    serialization_options: Option<crate::script::types::SerializationOptions>,
    start_nodes: Option<Vec<crate::script::types::SharedReference>>,
}
impl LocateNodesBuilder {
    pub fn context(mut self, context: impl Into<super::types::BrowsingContext>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn locator(mut self, locator: impl Into<super::types::Locator>) -> Self {
        self.locator = Some(locator.into());
        self
    }
    pub fn max_node_count(mut self, max_node_count: impl Into<u64>) -> Self {
        self.max_node_count = Some(max_node_count.into());
        self
    }
    pub fn serialization_options(
        mut self,
        serialization_options: impl Into<crate::script::types::SerializationOptions>,
    ) -> Self {
        self.serialization_options = Some(serialization_options.into());
        self
    }
    pub fn start_node(
        mut self,
        start_node: impl Into<crate::script::types::SharedReference>,
    ) -> Self {
        let v = self.start_nodes.get_or_insert(Vec::new());
        v.push(start_node.into());
        self
    }
    pub fn start_nodes<I, S>(mut self, start_nodes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::script::types::SharedReference>,
    {
        let v = self.start_nodes.get_or_insert(Vec::new());
        for val in start_nodes {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<LocateNodes, String> {
        Ok(LocateNodes {
            method: LocateNodesMethod::LocateNodes,
            params: LocateNodesParams {
                context: self
                    .context
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
                locator: self
                    .locator
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(locator)))?,
                max_node_count: self.max_node_count,
                serialization_options: self.serialization_options,
                start_nodes: self.start_nodes,
            },
        })
    }
}
impl Navigate {
    pub fn builder() -> NavigateBuilder {
        NavigateBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct NavigateBuilder {
    context: Option<super::types::BrowsingContext>,
    url: Option<String>,
    wait: Option<super::types::ReadinessState>,
}
impl NavigateBuilder {
    pub fn context(mut self, context: impl Into<super::types::BrowsingContext>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn wait(mut self, wait: impl Into<super::types::ReadinessState>) -> Self {
        self.wait = Some(wait.into());
        self
    }
    pub fn build(self) -> Result<Navigate, String> {
        Ok(Navigate {
            method: NavigateMethod::Navigate,
            params: NavigateParams {
                context: self
                    .context
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
                url: self
                    .url
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
                wait: self.wait,
            },
        })
    }
}
impl Print {
    pub fn builder() -> PrintBuilder {
        PrintBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PrintBuilder {
    context: Option<super::types::BrowsingContext>,
    background: Option<bool>,
    margin: Option<super::types::PrintMarginParameters>,
    orientation: Option<PrintOrientation>,
    page: Option<super::types::PrintPageParameters>,
    page_ranges: Option<Vec<serde_json::Value>>,
    scale: Option<f64>,
    shrink_to_fit: Option<bool>,
}
impl PrintBuilder {
    pub fn context(mut self, context: impl Into<super::types::BrowsingContext>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn background(mut self, background: impl Into<bool>) -> Self {
        self.background = Some(background.into());
        self
    }
    pub fn margin(mut self, margin: impl Into<super::types::PrintMarginParameters>) -> Self {
        self.margin = Some(margin.into());
        self
    }
    pub fn orientation(mut self, orientation: impl Into<PrintOrientation>) -> Self {
        self.orientation = Some(orientation.into());
        self
    }
    pub fn page(mut self, page: impl Into<super::types::PrintPageParameters>) -> Self {
        self.page = Some(page.into());
        self
    }
    pub fn page_range(mut self, page_range: impl Into<serde_json::Value>) -> Self {
        let v = self.page_ranges.get_or_insert(Vec::new());
        v.push(page_range.into());
        self
    }
    pub fn page_ranges<I, S>(mut self, page_ranges: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<serde_json::Value>,
    {
        let v = self.page_ranges.get_or_insert(Vec::new());
        for val in page_ranges {
            v.push(val.into());
        }
        self
    }
    pub fn scale(mut self, scale: impl Into<f64>) -> Self {
        self.scale = Some(scale.into());
        self
    }
    pub fn shrink_to_fit(mut self, shrink_to_fit: impl Into<bool>) -> Self {
        self.shrink_to_fit = Some(shrink_to_fit.into());
        self
    }
    pub fn build(self) -> Result<Print, String> {
        Ok(Print {
            method: PrintMethod::Print,
            params: PrintParams {
                context: self
                    .context
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
                background: self.background,
                margin: self.margin,
                orientation: self.orientation,
                page: self.page,
                page_ranges: self.page_ranges,
                scale: self.scale,
                shrink_to_fit: self.shrink_to_fit,
            },
        })
    }
}
impl Reload {
    pub fn builder() -> ReloadBuilder {
        ReloadBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ReloadBuilder {
    context: Option<super::types::BrowsingContext>,
    ignore_cache: Option<bool>,
    wait: Option<super::types::ReadinessState>,
}
impl ReloadBuilder {
    pub fn context(mut self, context: impl Into<super::types::BrowsingContext>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn ignore_cache(mut self, ignore_cache: impl Into<bool>) -> Self {
        self.ignore_cache = Some(ignore_cache.into());
        self
    }
    pub fn wait(mut self, wait: impl Into<super::types::ReadinessState>) -> Self {
        self.wait = Some(wait.into());
        self
    }
    pub fn build(self) -> Result<Reload, String> {
        Ok(Reload {
            method: ReloadMethod::Reload,
            params: ReloadParams {
                context: self
                    .context
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
                ignore_cache: self.ignore_cache,
                wait: self.wait,
            },
        })
    }
}
impl SetViewport {
    pub fn builder() -> SetViewportBuilder {
        SetViewportBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetViewportBuilder {
    context: Option<super::types::BrowsingContext>,
    viewport: Option<super::types::Viewport>,
    device_pixel_ratio: Option<f64>,
    user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl SetViewportBuilder {
    pub fn context(mut self, context: impl Into<super::types::BrowsingContext>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn viewport(mut self, viewport: impl Into<super::types::Viewport>) -> Self {
        self.viewport = Some(viewport.into());
        self
    }
    pub fn device_pixel_ratio(mut self, device_pixel_ratio: impl Into<f64>) -> Self {
        self.device_pixel_ratio = Some(device_pixel_ratio.into());
        self
    }
    pub fn user_context(
        mut self,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        let v = self.user_contexts.get_or_insert(Vec::new());
        v.push(user_context.into());
        self
    }
    pub fn user_contexts<I, S>(mut self, user_contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser::types::UserContext>,
    {
        let v = self.user_contexts.get_or_insert(Vec::new());
        for val in user_contexts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> SetViewport {
        SetViewport {
            method: SetViewportMethod::SetViewport,
            params: SetViewportParams {
                context: self.context,
                viewport: self.viewport,
                device_pixel_ratio: self.device_pixel_ratio,
                user_contexts: self.user_contexts,
            },
        }
    }
}
impl TraverseHistory {
    pub fn builder() -> TraverseHistoryBuilder {
        TraverseHistoryBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct TraverseHistoryBuilder {
    context: Option<super::types::BrowsingContext>,
    delta: Option<i64>,
}
impl TraverseHistoryBuilder {
    pub fn context(mut self, context: impl Into<super::types::BrowsingContext>) -> Self {
        self.context = Some(context.into());
        self
    }
    pub fn delta(mut self, delta: impl Into<i64>) -> Self {
        self.delta = Some(delta.into());
        self
    }
    pub fn build(self) -> Result<TraverseHistory, String> {
        Ok(TraverseHistory {
            method: TraverseHistoryMethod::TraverseHistory,
            params: TraverseHistoryParams {
                context: self
                    .context
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(context)))?,
                delta: self
                    .delta
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(delta)))?,
            },
        })
    }
}
