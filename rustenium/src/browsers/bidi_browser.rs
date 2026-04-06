use crate::conduit::bidi::drivers::BidiDriver;
use crate::domain::context::BrowsingContext as DomainBrowsingContext;
use crate::error::bidi::{
    BrowserCloseError, ContextCloseError, ContextCreationError, ContextIndexError, EmulationError,
    EvaluateResultError, FindNodesError, InterceptNetworkError, NavigateError, ScreenshotError,
};
use crate::input::{BidiKeyboard, BidiMouse, HumanMouse, HumanTouchscreen, Touchscreen};
use crate::nodes::Node;
use rustenium_bidi_definitions::base::CommandResponse;
use rustenium_bidi_definitions::browser::types::UserContext;
use rustenium_bidi_definitions::browsing_context::command_builders::{
    CaptureScreenshotBuilder, CreateBuilder, LocateNodesBuilder, NavigateBuilder,
};
use rustenium_bidi_definitions::browsing_context::results::NavigateResult;
use rustenium_bidi_definitions::browsing_context::types::{BrowsingContext, CreateType, Locator};
use rustenium_bidi_definitions::emulation::command_builders::SetTimezoneOverrideBuilder;
use rustenium_bidi_definitions::script::command_builders::{
    AddPreloadScriptBuilder, EvaluateBuilder, RemovePreloadScriptBuilder,
};
use rustenium_bidi_definitions::script::types::{
    ContextTarget, EvaluateResultSuccess, NodeRemoteValue, SerializationOptions,
    SerializationOptionsIncludeShadowTree, Target,
};
use rustenium_bidi_definitions::session::results::SubscribeResult;
use rustenium_bidi_definitions::{Command, Event};
use rustenium_core::error::{CommandResultError, SessionSendError};
use rustenium_core::events::BidiEventManagement;
use rustenium_core::transport::ConnectionTransport;
use rustenium_core::{BidiSession, NetworkRequest};
use std::collections::HashSet;
use std::future::Future;

pub mod options {
    use rustenium_bidi_definitions::browsing_context::commands::CaptureScreenshotOrigin;
    use rustenium_bidi_definitions::browsing_context::types::{
        BrowsingContext, ClipRectangle, CreateType, ImageFormat, ReadinessState,
    };
    use rustenium_bidi_definitions::network::types::UrlPattern;
    use rustenium_bidi_definitions::script::types::{
        ChannelValue, ResultOwnership, SerializationOptions, SharedReference, Target,
    };

    #[derive(Debug, Clone, Default)]
    pub struct BrowserScreenshotOptions {
        pub context_id: Option<BrowsingContext>,
        pub origin: Option<CaptureScreenshotOrigin>,
        pub format: Option<ImageFormat>,
        pub clip: Option<ClipRectangle>,
        pub save_path: Option<String>,
    }

    #[derive(Default, Clone)]
    pub struct BrowserScreenshotOptionsBuilder {
        context_id: Option<BrowsingContext>,
        origin: Option<CaptureScreenshotOrigin>,
        format: Option<ImageFormat>,
        clip: Option<ClipRectangle>,
        save_path: Option<String>,
    }

    impl BrowserScreenshotOptionsBuilder {
        pub fn context_id(mut self, v: impl Into<BrowsingContext>) -> Self { self.context_id = Some(v.into()); self }
        pub fn origin(mut self, v: impl Into<CaptureScreenshotOrigin>) -> Self { self.origin = Some(v.into()); self }
        pub fn format(mut self, v: impl Into<ImageFormat>) -> Self { self.format = Some(v.into()); self }
        pub fn clip(mut self, v: impl Into<ClipRectangle>) -> Self { self.clip = Some(v.into()); self }
        pub fn save_path(mut self, v: impl Into<String>) -> Self { self.save_path = Some(v.into()); self }
        pub fn build(self) -> BrowserScreenshotOptions {
            BrowserScreenshotOptions {
                context_id: self.context_id, origin: self.origin, format: self.format,
                clip: self.clip, save_path: self.save_path,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct NavigateOptions {
        pub wait: Option<ReadinessState>,
        pub context_id: Option<BrowsingContext>,
    }

    #[derive(Default, Clone)]
    pub struct NavigateOptionsBuilder {
        wait: Option<ReadinessState>,
        context_id: Option<BrowsingContext>,
    }

    impl NavigateOptionsBuilder {
        pub fn wait(mut self, v: impl Into<ReadinessState>) -> Self { self.wait = Some(v.into()); self }
        pub fn context_id(mut self, v: impl Into<BrowsingContext>) -> Self { self.context_id = Some(v.into()); self }
        pub fn build(self) -> NavigateOptions {
            NavigateOptions { wait: self.wait, context_id: self.context_id }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct CreateContextOptions {
        pub context_type: Option<CreateType>,
        pub reference_context: Option<BrowsingContext>,
    }

    #[derive(Default, Clone)]
    pub struct CreateContextOptionsBuilder {
        context_type: Option<CreateType>,
        reference_context: Option<BrowsingContext>,
    }

    impl CreateContextOptionsBuilder {
        pub fn context_type(mut self, v: impl Into<CreateType>) -> Self { self.context_type = Some(v.into()); self }
        pub fn reference_context(mut self, v: impl Into<BrowsingContext>) -> Self { self.reference_context = Some(v.into()); self }
        pub fn build(self) -> CreateContextOptions {
            CreateContextOptions { context_type: self.context_type, reference_context: self.reference_context }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct FindNodesOptions {
        pub context_id: Option<BrowsingContext>,
        pub max_node_count: Option<u64>,
        pub serialization_options: Option<SerializationOptions>,
        pub start_nodes: Option<Vec<SharedReference>>,
    }

    #[derive(Default, Clone)]
    pub struct FindNodesOptionsBuilder {
        context_id: Option<BrowsingContext>,
        max_node_count: Option<u64>,
        serialization_options: Option<SerializationOptions>,
        start_nodes: Option<Vec<SharedReference>>,
    }

    impl FindNodesOptionsBuilder {
        pub fn context_id(mut self, v: impl Into<BrowsingContext>) -> Self { self.context_id = Some(v.into()); self }
        pub fn max_node_count(mut self, v: impl Into<u64>) -> Self { self.max_node_count = Some(v.into()); self }
        pub fn serialization_options(mut self, v: impl Into<SerializationOptions>) -> Self { self.serialization_options = Some(v.into()); self }
        pub fn start_nodes(mut self, v: Vec<SharedReference>) -> Self { self.start_nodes = Some(v); self }
        pub fn build(self) -> FindNodesOptions {
            FindNodesOptions {
                context_id: self.context_id, max_node_count: self.max_node_count,
                serialization_options: self.serialization_options, start_nodes: self.start_nodes,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct WaitForNodesOptions {
        pub context_id: Option<BrowsingContext>,
        pub timeout_ms: Option<u64>,
        pub poll_interval_ms: Option<u64>,
    }

    #[derive(Default, Clone)]
    pub struct WaitForNodesOptionsBuilder {
        context_id: Option<BrowsingContext>,
        timeout_ms: Option<u64>,
        poll_interval_ms: Option<u64>,
    }

    impl WaitForNodesOptionsBuilder {
        pub fn context_id(mut self, v: impl Into<BrowsingContext>) -> Self { self.context_id = Some(v.into()); self }
        pub fn timeout_ms(mut self, v: impl Into<u64>) -> Self { self.timeout_ms = Some(v.into()); self }
        pub fn poll_interval_ms(mut self, v: impl Into<u64>) -> Self { self.poll_interval_ms = Some(v.into()); self }
        pub fn build(self) -> WaitForNodesOptions {
            WaitForNodesOptions {
                context_id: self.context_id, timeout_ms: self.timeout_ms,
                poll_interval_ms: self.poll_interval_ms,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct OnRequestOptions {
        pub url_patterns: Option<Vec<UrlPattern>>,
        pub contexts: Option<Vec<String>>,
    }

    #[derive(Default, Clone)]
    pub struct OnRequestOptionsBuilder {
        url_patterns: Option<Vec<UrlPattern>>,
        contexts: Option<Vec<String>>,
    }

    impl OnRequestOptionsBuilder {
        pub fn url_patterns(mut self, v: Vec<UrlPattern>) -> Self { self.url_patterns = Some(v); self }
        pub fn contexts(mut self, v: Vec<String>) -> Self { self.contexts = Some(v); self }
        pub fn build(self) -> OnRequestOptions {
            OnRequestOptions { url_patterns: self.url_patterns, contexts: self.contexts }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct SubscribeEventsOptions {
        pub browsing_contexts: Option<Vec<String>>,
        pub user_contexts: Option<Vec<String>>,
    }

    #[derive(Default, Clone)]
    pub struct SubscribeEventsOptionsBuilder {
        browsing_contexts: Option<Vec<String>>,
        user_contexts: Option<Vec<String>>,
    }

    impl SubscribeEventsOptionsBuilder {
        pub fn browsing_contexts(mut self, v: Vec<String>) -> Self { self.browsing_contexts = Some(v); self }
        pub fn user_contexts(mut self, v: Vec<String>) -> Self { self.user_contexts = Some(v); self }
        pub fn build(self) -> SubscribeEventsOptions {
            SubscribeEventsOptions { browsing_contexts: self.browsing_contexts, user_contexts: self.user_contexts }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct EvaluateScriptOptions {
        pub target: Option<Target>,
        pub result_ownership: Option<ResultOwnership>,
        pub serialization_options: Option<SerializationOptions>,
        pub user_activation: Option<bool>,
    }

    #[derive(Default, Clone)]
    pub struct EvaluateScriptOptionsBuilder {
        target: Option<Target>,
        result_ownership: Option<ResultOwnership>,
        serialization_options: Option<SerializationOptions>,
        user_activation: Option<bool>,
    }

    impl EvaluateScriptOptionsBuilder {
        pub fn target(mut self, v: impl Into<Target>) -> Self { self.target = Some(v.into()); self }
        pub fn result_ownership(mut self, v: impl Into<ResultOwnership>) -> Self { self.result_ownership = Some(v.into()); self }
        pub fn serialization_options(mut self, v: impl Into<SerializationOptions>) -> Self { self.serialization_options = Some(v.into()); self }
        pub fn user_activation(mut self, v: impl Into<bool>) -> Self { self.user_activation = Some(v.into()); self }
        pub fn build(self) -> EvaluateScriptOptions {
            EvaluateScriptOptions {
                target: self.target, result_ownership: self.result_ownership,
                serialization_options: self.serialization_options, user_activation: self.user_activation,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct AddPreloadScriptOptions {
        pub arguments: Option<Vec<ChannelValue>>,
        pub contexts: Option<Vec<BrowsingContext>>,
        pub user_contexts: Option<Vec<String>>,
        pub sandbox: Option<String>,
    }

    #[derive(Default, Clone)]
    pub struct AddPreloadScriptOptionsBuilder {
        arguments: Option<Vec<ChannelValue>>,
        contexts: Option<Vec<BrowsingContext>>,
        user_contexts: Option<Vec<String>>,
        sandbox: Option<String>,
    }

    impl AddPreloadScriptOptionsBuilder {
        pub fn arguments(mut self, v: Vec<ChannelValue>) -> Self { self.arguments = Some(v); self }
        pub fn contexts(mut self, v: Vec<BrowsingContext>) -> Self { self.contexts = Some(v); self }
        pub fn user_contexts(mut self, v: Vec<String>) -> Self { self.user_contexts = Some(v); self }
        pub fn sandbox(mut self, v: impl Into<String>) -> Self { self.sandbox = Some(v.into()); self }
        pub fn build(self) -> AddPreloadScriptOptions {
            AddPreloadScriptOptions {
                arguments: self.arguments, contexts: self.contexts,
                user_contexts: self.user_contexts, sandbox: self.sandbox,
            }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct EmulateTimezoneOptions {
        pub contexts: Option<Vec<BrowsingContext>>,
        pub user_contexts: Option<Vec<String>>,
    }

    #[derive(Default, Clone)]
    pub struct EmulateTimezoneOptionsBuilder {
        contexts: Option<Vec<BrowsingContext>>,
        user_contexts: Option<Vec<String>>,
    }

    impl EmulateTimezoneOptionsBuilder {
        pub fn contexts(mut self, v: Vec<BrowsingContext>) -> Self { self.contexts = Some(v); self }
        pub fn user_contexts(mut self, v: Vec<String>) -> Self { self.user_contexts = Some(v); self }
        pub fn build(self) -> EmulateTimezoneOptions {
            EmulateTimezoneOptions { contexts: self.contexts, user_contexts: self.user_contexts }
        }
    }

    #[derive(Debug, Clone, Default)]
    pub struct AuthenticateOptions {
        pub url_patterns: Option<Vec<UrlPattern>>,
        pub contexts: Option<Vec<String>>,
    }

    #[derive(Default, Clone)]
    pub struct AuthenticateOptionsBuilder {
        url_patterns: Option<Vec<UrlPattern>>,
        contexts: Option<Vec<String>>,
    }

    impl AuthenticateOptionsBuilder {
        pub fn url_patterns(mut self, v: Vec<UrlPattern>) -> Self { self.url_patterns = Some(v); self }
        pub fn contexts(mut self, v: Vec<String>) -> Self { self.contexts = Some(v); self }
        pub fn build(self) -> AuthenticateOptions {
            AuthenticateOptions { url_patterns: self.url_patterns, contexts: self.contexts }
        }
    }
}


pub use options::{
    AddPreloadScriptOptions, AddPreloadScriptOptionsBuilder,
    AuthenticateOptions, AuthenticateOptionsBuilder,
    BrowserScreenshotOptions, BrowserScreenshotOptionsBuilder,
    CreateContextOptions, CreateContextOptionsBuilder,
    EmulateTimezoneOptions, EmulateTimezoneOptionsBuilder,
    EvaluateScriptOptions, EvaluateScriptOptionsBuilder,
    FindNodesOptions, FindNodesOptionsBuilder,
    NavigateOptions, NavigateOptionsBuilder,
    OnRequestOptions, OnRequestOptionsBuilder,
    SubscribeEventsOptions, SubscribeEventsOptionsBuilder,
    WaitForNodesOptions, WaitForNodesOptionsBuilder,
};

pub trait BidiBrowser: Send + Sync {
    type Transport: ConnectionTransport + Send + Sync + 'static;
    type BrowserNode: Node + Send;

    // ── Required methods ─────────────────────────────────────────────────────
    // These are the only methods a browser implementation must provide.

    /// Access the underlying BiDi driver (immutable).
    fn driver(&self) -> &BidiDriver<Self::Transport>;

    /// Access the underlying BiDi driver (mutable).
    fn driver_mut(&mut self) -> &mut BidiDriver<Self::Transport>;

    /// Construct a browser-specific node from raw BiDi data.
    fn build_node(
        &self,
        raw_node: NodeRemoteValue,
        locator: Locator,
        context: BrowsingContext,
    ) -> Self::BrowserNode;

    /// Closes the browser: ends the BiDi session, closes the WebSocket connection,
    /// and performs any browser-specific cleanup (e.g. kills the browser process).
    fn close(self) -> impl Future<Output = Result<(), BrowserCloseError>> + Send;

    // ── Navigation ───────────────────────────────────────────────────────────

    /// Navigates to the specified URL in the active context.
    fn navigate(
        &mut self,
        url: &str,
    ) -> impl Future<Output = Result<NavigateResult, NavigateError>> + Send {
        async move {
            self.navigate_with_options(url, NavigateOptions::default())
                .await
        }
    }

    /// Navigates to the specified URL with custom options (wait state, context).
    fn navigate_with_options(
        &mut self,
        url: &str,
        options: NavigateOptions,
    ) -> impl Future<Output = Result<NavigateResult, NavigateError>> + Send {
        async move {
            let context = options
                .context_id
                .unwrap_or_else(|| self.driver().get_active_context_id().unwrap());
            let mut builder = NavigateBuilder::default().url(url).context(context);
            if let Some(wait) = options.wait {
                builder = builder.wait(wait);
            }
            self.driver_mut().navigate(builder.build().unwrap()).await
        }
    }

    // ── Context ──────────────────────────────────────────────────────────────

    /// Closes a browsing context (tab/window).
    fn close_context(
        &mut self,
        context: DomainBrowsingContext,
    ) -> impl Future<Output = Result<(), ContextCloseError>> + Send
    {
        async move {
            self.driver_mut().close_context(context.id().clone()).await
        }
    }

    /// Creates a new browsing context (tab) with default options.
    fn create_context(
        &mut self,
        background: bool,
    ) -> impl Future<Output = Result<DomainBrowsingContext, ContextCreationError>> + Send
    {
        async move {
            self.create_context_with_options(background, CreateContextOptions::default())
                .await
        }
    }

    /// Creates a new browsing context with custom options (type, reference context, background).
    fn create_context_with_options(
        &mut self,
        background: bool,
        options: CreateContextOptions,
    ) -> impl Future<Output = Result<DomainBrowsingContext, ContextCreationError>> + Send
    {
        async move {
            let context_type = options.context_type.unwrap_or(CreateType::Tab);
            let mut builder = CreateBuilder::default().r#type(context_type);
            if let Some(ref_ctx) = options.reference_context {
                builder = builder.reference_context(ref_ctx);
            };
            builder = builder.background(background);
            self.driver_mut()
                .create_context(builder.build().unwrap())
                .await
        }
    }

    // ── Node finding ─────────────────────────────────────────────────────────

    /// Finds all elements matching the given locator in the active context.
    fn find_nodes(
        &mut self,
        locator: Locator,
    ) -> impl Future<Output = Result<Vec<Self::BrowserNode>, FindNodesError>> + Send {
        async move {
            self.find_nodes_with_options(locator, FindNodesOptions::default())
                .await
        }
    }

    /// Finds all elements matching the given locator with custom options (context, count, serialization, start nodes).
    fn find_nodes_with_options(
        &mut self,
        locator: Locator,
        options: FindNodesOptions,
    ) -> impl Future<Output = Result<Vec<Self::BrowserNode>, FindNodesError>> + Send {
        async move {
            let context = options
                .context_id
                .clone()
                .unwrap_or_else(|| self.driver().get_active_context_id().unwrap());
            let serialization_options =
                options
                    .serialization_options
                    .unwrap_or(SerializationOptions {
                        max_dom_depth: Some(40),
                        max_object_depth: Some(0),
                        include_shadow_tree: Some(SerializationOptionsIncludeShadowTree::None),
                    });
            let mut builder = LocateNodesBuilder::default()
                .context(context.clone())
                .locator(locator.clone())
                .serialization_options(serialization_options);
            if let Some(max_count) = options.max_node_count {
                builder = builder.max_node_count(max_count);
            }
            if let Some(start_nodes) = options.start_nodes {
                builder = builder.start_nodes(start_nodes);
            }
            let node_result = self
                .driver_mut()
                .find_nodes(builder.build().unwrap())
                .await?;
            let mut nodes = Vec::new();
            for node in node_result.nodes.iter() {
                nodes.push(self.build_node(node.clone(), locator.clone(), context.clone()));
            }
            Ok(nodes)
        }
    }

    /// Finds the first element matching the given locator in the active context.
    ///
    /// Returns `None` if no matching element is found.
    fn find_node(
        &mut self,
        locator: Locator,
    ) -> impl Future<Output = Result<Option<Self::BrowserNode>, FindNodesError>> + Send {
        async move {
            self.find_node_with_options(locator, FindNodesOptions::default())
                .await
        }
    }

    /// Finds the first element matching the given locator with custom options.
    ///
    /// Returns `None` if no matching element is found.
    fn find_node_with_options(
        &mut self,
        locator: Locator,
        options: FindNodesOptions,
    ) -> impl Future<Output = Result<Option<Self::BrowserNode>, FindNodesError>> + Send {
        async move {
            let nodes = self.find_nodes_with_options(locator, options).await?;
            Ok(nodes.into_iter().next())
        }
    }

    // ── Waiting ──────────────────────────────────────────────────────────────

    /// Waits for elements matching the locator to appear (default 4s timeout).
    fn wait_for_nodes(
        &mut self,
        locator: Locator,
    ) -> impl Future<Output = Result<Vec<Self::BrowserNode>, FindNodesError>> + Send {
        async move {
            self.wait_for_nodes_with_options(locator, WaitForNodesOptions::default())
                .await
        }
    }

    /// Waits for elements matching the locator with custom options (context, timeout, poll interval).
    fn wait_for_nodes_with_options(
        &mut self,
        locator: Locator,
        options: WaitForNodesOptions,
    ) -> impl Future<Output = Result<Vec<Self::BrowserNode>, FindNodesError>> + Send {
        async move {
            let timeout = options.timeout_ms.unwrap_or(4000);
            let poll_interval = options.poll_interval_ms.unwrap_or(timeout / 6);
            let start = std::time::Instant::now();

            loop {
                let nodes = self
                    .find_nodes_with_options(
                        locator.clone(),
                        FindNodesOptions {
                            context_id: options.context_id.clone(),
                            ..Default::default()
                        },
                    )
                    .await?;

                if !nodes.is_empty() {
                    return Ok(nodes);
                }

                if start.elapsed().as_millis() as u64 >= timeout {
                    return Ok(Vec::new());
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(poll_interval)).await;
            }
        }
    }

    /// Waits for a single element matching the locator to appear (default 4s timeout).
    ///
    /// Returns `None` if no matching element appears within the timeout.
    fn wait_for_node(
        &mut self,
        locator: Locator,
    ) -> impl Future<Output = Result<Option<Self::BrowserNode>, FindNodesError>> + Send {
        async move {
            self.wait_for_node_with_options(locator, WaitForNodesOptions::default())
                .await
        }
    }

    /// Waits for a single element matching the locator with custom options.
    ///
    /// Returns `None` if no matching element appears within the timeout.
    fn wait_for_node_with_options(
        &mut self,
        locator: Locator,
        options: WaitForNodesOptions,
    ) -> impl Future<Output = Result<Option<Self::BrowserNode>, FindNodesError>> + Send {
        async move {
            let nodes = self.wait_for_nodes_with_options(locator, options).await?;
            Ok(nodes.into_iter().next())
        }
    }

    // ── Network interception ─────────────────────────────────────────────────

    /// Registers a handler called for each network request (all URLs, all contexts).
    fn on_request<F, Fut>(
        &mut self,
        handler: F,
    ) -> impl Future<Output = Result<(), InterceptNetworkError>> + Send
    where
        F: Fn(NetworkRequest<Self::Transport>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        async move {
            self.on_request_with_options(handler, OnRequestOptions::default())
                .await
        }
    }

    /// Registers a handler called for each network request with custom URL pattern and context filters.
    fn on_request_with_options<F, Fut>(
        &mut self,
        handler: F,
        options: OnRequestOptions,
    ) -> impl Future<Output = Result<(), InterceptNetworkError>> + Send
    where
        F: Fn(NetworkRequest<Self::Transport>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        async move {
            let mut builder = self.driver_mut().on_request(handler);
            if let Some(patterns) = options.url_patterns {
                builder = builder.url_patterns(patterns);
            }
            if let Some(contexts) = options.contexts {
                builder = builder.contexts(contexts);
            }
            builder.execute().await
        }
    }

    // ── Events ───────────────────────────────────────────────────────────────

    /// Subscribes to browser events and registers a handler.
    fn subscribe_events<F, R>(
        &mut self,
        events: HashSet<&str>,
        handler: F,
    ) -> impl Future<Output = Result<Option<SubscribeResult>, CommandResultError>> + Send
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        self.subscribe_events_with_options(events, handler, SubscribeEventsOptions::default())
    }

    /// Subscribes to browser events with context and user context filters.
    fn subscribe_events_with_options<F, R>(
        &mut self,
        events: HashSet<&str>,
        handler: F,
        options: SubscribeEventsOptions,
    ) -> impl Future<Output = Result<Option<SubscribeResult>, CommandResultError>> + Send
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        async move {
            let mut bidi_event = {
                let mut session = self.driver().session.lock().await;
                session.create_event::<_, _, BidiSession<Self::Transport>>(events, handler)
            };
            if let Some(contexts) = options.browsing_contexts {
                for ctx in contexts {
                    bidi_event.add_browsing_context(ctx);
                }
            }
            if let Some(user_ctxs) = options.user_contexts {
                for uctx in user_ctxs {
                    bidi_event.add_user_context(uctx);
                }
            }
            self.driver()
                .session
                .lock()
                .await
                .subscribe_events(bidi_event)
                .await
        }
    }

    /// Adds an event handler without sending a subscription command.
    ///
    /// Returns the auto-generated handler ID.
    fn add_event_handler<F, R>(
        &mut self,
        events: HashSet<&str>,
        handler: F,
    ) -> impl Future<Output = String> + Send
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        async move { self.driver_mut().add_event_handler(events, handler).await }
    }

    // ── Script evaluation ────────────────────────────────────────────────────

    /// Evaluates a JavaScript expression in the active browsing context.
    fn evaluate_script(
        &mut self,
        expression: String,
        await_promise: bool,
    ) -> impl Future<Output = Result<EvaluateResultSuccess, EvaluateResultError>> + Send {
        async move {
            self.evaluate_script_with_options(
                expression,
                await_promise,
                EvaluateScriptOptions::default(),
            )
            .await
        }
    }

    /// Evaluates a JavaScript expression with custom options (target, result ownership, serialization).
    fn evaluate_script_with_options(
        &mut self,
        expression: String,
        await_promise: bool,
        options: EvaluateScriptOptions,
    ) -> impl Future<Output = Result<EvaluateResultSuccess, EvaluateResultError>> + Send {
        async move {
            let target = options.target.unwrap_or_else(|| {
                let context = self.driver().get_active_context_id().unwrap();
                Target::ContextTarget(ContextTarget::new(context))
            });
            let mut builder = EvaluateBuilder::default()
                .expression(expression)
                .await_promise(await_promise)
                .target(target);
            if let Some(ro) = options.result_ownership {
                builder = builder.result_ownership(ro);
            }
            if let Some(so) = options.serialization_options {
                builder = builder.serialization_options(so);
            }
            if let Some(ua) = options.user_activation {
                builder = builder.user_activation(ua);
            }
            self.driver_mut()
                .evaluate_script(builder.build().unwrap())
                .await
        }
    }

    // ── Preload scripts ──────────────────────────────────────────────────────

    /// Adds a preload script that runs in every new browsing context.
    fn add_preload_script(
        &mut self,
        function_declaration: String,
    ) -> impl Future<Output = Result<String, EvaluateResultError>> + Send {
        async move {
            self.add_preload_script_with_options(
                function_declaration,
                AddPreloadScriptOptions::default(),
            )
            .await
        }
    }

    /// Adds a preload script with custom options (arguments, contexts, sandbox).
    fn add_preload_script_with_options(
        &mut self,
        function_declaration: String,
        options: AddPreloadScriptOptions,
    ) -> impl Future<Output = Result<String, EvaluateResultError>> + Send {
        async move {
            let mut builder =
                AddPreloadScriptBuilder::default().function_declaration(function_declaration);
            if let Some(args) = options.arguments {
                builder = builder.arguments(args);
            }
            if let Some(contexts) = options.contexts {
                builder = builder.contexts(contexts);
            }
            if let Some(user_contexts) = options.user_contexts {
                builder = builder.user_contexts(user_contexts.into_iter().map(UserContext::new));
            }
            if let Some(sandbox) = options.sandbox {
                builder = builder.sandbox(sandbox);
            }
            self.driver_mut()
                .add_preload_script(builder.build().unwrap())
                .await
                .map(|ps| ps.into())
        }
    }

    /// Removes a preload script by its ID.
    fn remove_preload_script(
        &mut self,
        script: String,
    ) -> impl Future<Output = Result<(), EvaluateResultError>> + Send {
        async move {
            let remove_cmd = RemovePreloadScriptBuilder::default()
                .script(script)
                .build()
                .unwrap();
            self.driver_mut().remove_preload_script(remove_cmd).await
        }
    }

    // ── Screenshot ───────────────────────────────────────────────────────────

    /// Captures a screenshot of the active browsing context and returns base64-encoded image data.
    fn screenshot(&mut self) -> impl Future<Output = Result<String, ScreenshotError>> + Send {
        async move {
            self.screenshot_with_options(BrowserScreenshotOptions::default())
                .await
        }
    }

    /// Captures a screenshot with custom options (context, origin, format, clip, save path).
    ///
    /// If `save_path` is a directory, saves with auto-generated filename (screenshot_TIMESTAMP.png).
    /// If `save_path` is a file path, saves to that exact location and returns the path.
    /// If `save_path` is `None`, returns base64-encoded image data.
    fn screenshot_with_options(
        &mut self,
        options: BrowserScreenshotOptions,
    ) -> impl Future<Output = Result<String, ScreenshotError>> + Send {
        async move {
            let context = options
                .context_id
                .unwrap_or_else(|| self.driver().get_active_context_id().unwrap());
            let mut builder = CaptureScreenshotBuilder::default().context(context);
            if let Some(origin) = options.origin {
                builder = builder.origin(origin);
            }
            if let Some(format) = options.format {
                builder = builder.format(format);
            }
            if let Some(clip) = options.clip {
                builder = builder.clip(clip);
            }
            let command = builder.build().unwrap();
            self.driver_mut()
                .screenshot(command, options.save_path.as_deref())
                .await
        }
    }

    // ── Emulation ────────────────────────────────────────────────────────────

    /// Emulates a timezone for the active browsing context.
    ///
    /// Pass `None` to clear the override.
    fn emulate_timezone(
        &mut self,
        timezone: Option<String>,
    ) -> impl Future<Output = Result<(), EmulationError>> + Send {
        async move {
            self.emulate_timezone_with_options(timezone, EmulateTimezoneOptions::default())
                .await
        }
    }

    /// Emulates a timezone with custom context and user context filters.
    ///
    /// Pass `None` for `timezone` to clear the override.
    fn emulate_timezone_with_options(
        &mut self,
        timezone: Option<String>,
        mut options: EmulateTimezoneOptions,
    ) -> impl Future<Output = Result<(), EmulationError>> + Send {
        async move {
            if options.contexts.is_none() && options.user_contexts.is_none() {
                options.contexts = Some(vec![self.driver().get_active_context_id().unwrap()]);
            }
            let mut builder = SetTimezoneOverrideBuilder::default();
            if let Some(tz) = timezone {
                builder = builder.timezone(tz);
            }
            if let Some(contexts) = options.contexts {
                builder = builder.contexts(contexts);
            }
            if let Some(user_contexts) = options.user_contexts {
                builder = builder.user_contexts(user_contexts.into_iter().map(UserContext::new));
            }
            self.driver_mut()
                .set_timezone_override(builder.build())
                .await
        }
    }

    // ── Authentication ───────────────────────────────────────────────────────

    /// Sets HTTP authentication credentials for all requests.
    fn authenticate(
        &mut self,
        username: impl Into<String> + Send + 'static,
        password: impl Into<String> + Send + 'static,
    ) -> impl Future<Output = Result<(), InterceptNetworkError>> + Send {
        async move {
            self.authenticate_with_options(username, password, AuthenticateOptions::default())
                .await
        }
    }

    /// Sets HTTP authentication credentials with URL pattern and context filters.
    fn authenticate_with_options(
        &mut self,
        username: impl Into<String> + Send + 'static,
        password: impl Into<String> + Send + 'static,
        options: AuthenticateOptions,
    ) -> impl Future<Output = Result<(), InterceptNetworkError>> + Send {
        async move {
            let mut builder = self.driver_mut().authenticate(username, password);
            if let Some(patterns) = options.url_patterns {
                builder = builder.url_patterns(patterns);
            }
            if let Some(contexts) = options.contexts {
                builder = builder.contexts(contexts);
            }
            builder.execute().await
        }
    }

    // ── Accessors ────────────────────────────────────────────────────────────

    /// Returns a reference to the direct BiDi mouse for precise, instant movements.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustenium::browsers::ChromeBrowser;
    /// # use rustenium::input::Point;
    /// # use rustenium_bidi_commands::browsing_context::types::BrowsingContext;
    /// # async fn example(browser: ChromeBrowser, context: BrowsingContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let mouse = browser.mouse();
    /// mouse.move_to(Point { x: 100.0, y: 200.0 }, &context, None).await?;
    /// mouse.click(None, &context, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn mouse(&self) -> &BidiMouse<Self::Transport> {
        self.driver().mouse.as_ref()
    }

    /// Returns a reference to the human mouse for realistic, human-like movements with Bezier curves and jitter.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustenium::browsers::ChromeBrowser;
    /// # use rustenium::input::Point;
    /// # use rustenium_bidi_commands::browsing_context::types::BrowsingContext;
    /// # async fn example(browser: ChromeBrowser, context: BrowsingContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let human_mouse = browser.human_mouse();
    /// // Moves with natural curve and realistic delays
    /// human_mouse.move_to(Point { x: 300.0, y: 400.0 }, &context, None).await?;
    /// human_mouse.click(None, &context, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn human_mouse(&self) -> &HumanMouse<BidiMouse<Self::Transport>> {
        self.driver().human_mouse.as_ref()
    }

    /// Returns a reference to the keyboard for text input and key presses.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use rustenium::browsers::ChromeBrowser;
    /// # use rustenium_bidi_commands::browsing_context::types::BrowsingContext;
    /// # async fn example(browser: ChromeBrowser, context: BrowsingContext) -> Result<(), Box<dyn std::error::Error>> {
    /// let keyboard = browser.keyboard();
    /// keyboard.type_text("Hello, World!", &context).await?;
    /// keyboard.press("Enter", &context).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn keyboard(&self) -> &BidiKeyboard<Self::Transport> {
        self.driver().keyboard.as_ref()
    }

    fn touchscreen(&self) -> &Touchscreen<Self::Transport> {
        self.driver().touchscreen.as_ref()
    }

    fn human_touchscreen(&self) -> &HumanTouchscreen<Self::Transport> {
        self.driver().human_touchscreen.as_ref()
    }

    fn get_active_context_id(&self) -> Result<BrowsingContext, ContextIndexError> {
        self.driver().get_active_context_id()
    }

    // ── Session ──────────────────────────────────────────────────────────────

    /// Sends a raw WebDriver BiDi command.
    fn send_command(
        &mut self,
        command: Command,
    ) -> impl Future<Output = Result<CommandResponse, SessionSendError>> + Send {
        async move { self.driver_mut().send_command(command).await }
    }

    /// Ends the BiDi session and cleans up resources.
    fn end_session(&mut self) -> impl Future<Output = Result<(), SessionSendError>> + Send {
        async move { self.driver_mut().end_session().await }
    }
}
