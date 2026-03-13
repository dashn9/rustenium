use rustenium_core::{process::Process, transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport}, NetworkRequest, BidiSession};
use rustenium_core::BrowsingContext as Context;

use crate::error::{ContextCreationListenError, ContextIndexError, EmulationError, EvaluateResultError, FindNodesError, InterceptNetworkError, OpenUrlError};
use rustenium_bidi_definitions::browsing_context::types::{
    BrowsingContext as BidiBrowsingContext, ClipRectangle, CreateType, ImageFormat, Locator, ReadinessState,
};
use rustenium_bidi_definitions::browsing_context::commands::CaptureScreenshotOrigin;
use rustenium_bidi_definitions::browsing_context::command_builders::{
    NavigateBuilder, LocateNodesBuilder, CaptureScreenshotBuilder,
};
use rustenium_bidi_definitions::browsing_context::results::{
    NavigateResult, LocateNodesResult, CaptureScreenshotResult,
};
use rustenium_bidi_definitions::script::command_builders::{
    EvaluateBuilder, CallFunctionBuilder, AddPreloadScriptBuilder, RemovePreloadScriptBuilder,
};
use rustenium_bidi_definitions::script::results::AddPreloadScriptResult;
use rustenium_bidi_definitions::script::types::{
    ChannelValue, ContextTarget, EvaluateResultSuccess, EvaluateResultException, LocalValue, PreloadScript,
    ResultOwnership, SerializationOptions, SharedReference, Target,
};
use rustenium_bidi_definitions::session::results::SubscribeResult;
use rustenium_bidi_definitions::session::types::CapabilitiesRequest;
use rustenium_bidi_definitions::network::command_builders::AddInterceptBuilder;
use rustenium_bidi_definitions::network::results::AddInterceptResult;
use rustenium_bidi_definitions::network::types::{InterceptPhase, UrlPattern};
use rustenium_bidi_definitions::network::events::NetworkEvent;
use rustenium_bidi_definitions::browsing_context::events::BrowsingContextEvent;
use rustenium_bidi_definitions::Event;
use rustenium_bidi_definitions::Command;
use rustenium_core::events::BidiEventManagement;
use rustenium_core::session::SessionConnectionType;
use rustenium_core::error::{CommandResultError, SessionSendError};
use rustenium_core::contexts::CreateBrowsingContext;
use std::collections::HashSet;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::sleep;

use crate::input::{BidiMouse, HumanMouse, Keyboard, Point};

pub trait DriverConfiguration {
    fn exe_path(&self) -> &str;
    fn flags(&self) -> Vec<String>;
}

pub trait BidiDrive<T: ConnectionTransport> {
    fn start(
        driver_config: &impl DriverConfiguration,
        connection_transport_config: &ConnectionTransportConfig,
        session_connection_type: SessionConnectionType,
        capabilities: CapabilitiesRequest,
    ) -> impl Future<Output = (Arc<TokioMutex<BidiSession<WebsocketConnectionTransport>>>, Process)> { async {
        let driver_process = Process::create(driver_config.exe_path(), driver_config.flags());
        let mut session = BidiSession::<T>::ws_new(connection_transport_config).await;
        session
            .create_new_bidi_session(session_connection_type, capabilities)
            .await;
        (Arc::new(TokioMutex::new(session)), driver_process)
    }
    }
}

pub struct BidiDriver<T: ConnectionTransport + Send + Sync> {
    pub exe_path: String,
    pub flags: Vec<String>,
    pub session: Arc<TokioMutex<BidiSession<T>>>,
    pub active_bc_index: usize,
    pub browsing_contexts: Arc<Mutex<Vec<Context>>>,
    pub driver_process: Process,
    pub mouse: BidiMouse<T>,
    pub human_mouse: HumanMouse<BidiMouse<T>>,
    pub keyboard: Keyboard<T>,
}

impl<T: ConnectionTransport + Send + Sync + 'static> BidiDriver<T> {
    pub fn new(
        exe_path: String,
        flags: Vec<String>,
        session: Arc<TokioMutex<BidiSession<T>>>,
        active_bc_index: usize,
        browsing_contexts: Arc<Mutex<Vec<Context>>>,
        driver_process: Process,
    ) -> Self {
        let mouse = BidiMouse::new(session.clone());
        let human_mouse = HumanMouse::new(BidiMouse::new(session.clone()));
        let keyboard = Keyboard::new(session.clone());

        Self {
            exe_path,
            flags,
            session,
            active_bc_index,
            browsing_contexts,
            driver_process,
            mouse,
            human_mouse,
            keyboard,
        }
    }

    pub async fn send_command(&mut self, command: impl Into<Command>) -> Result<serde_json::Value, SessionSendError> {
        let rx = {
            let mut session = self.session.lock().await;
            session.send_and_get_receiver(command).await
        };

        match tokio::time::timeout(Duration::from_secs(100), rx).await {
            Ok(Ok(command_result)) => match command_result {
                rustenium_core::CommandResponseState::Success(response) => Ok(response.result),
                rustenium_core::CommandResponseState::Error(err) => Err(SessionSendError::ErrorResponse(err))
            }
            Ok(Err(err)) => panic!("A recv error occurred: {}", err),
            Err(_) => Err(SessionSendError::ResponseReceiveTimeoutError(rustenium_core::error::ResponseReceiveTimeoutError))
        }
    }

    pub async fn listen_to_context_creation(
        &mut self,
    ) -> Result<Option<SubscribeResult>, ContextCreationListenError> {
        let browsing_contexts = self.browsing_contexts.clone();
        let events = HashSet::from(["browsingContext.contextCreated"]);
        let handler = move |event: Event| {
            let bc = browsing_contexts.clone();
            async move {
                if let Event::BrowsingContext(BrowsingContextEvent::ContextCreated(context)) = event {
                    bc.lock().unwrap().push(Context::from_id(
                        context.params.context,
                        CreateType::Window,
                    ));
                }
            }
        };

        let bidi_event = self.session.lock().await.create_event::<_, _, BidiSession<T>>(events, handler);
        let result = self
            .session
            .lock()
            .await
            .subscribe_events(bidi_event)
            .await;

        // Wait for 2s, to allow current BrowsingContext be updated via the event.
        sleep(Duration::from_millis(2000)).await;
        if self
            .browsing_contexts
            .lock()
            .expect("Unable to acquire lock")
            .len()
            > 0
        {
            if let Ok(Some(result)) = &result {
                match self
                    .session
                    .lock()
                    .await
                    .unsubscribe_events_by_ids(vec![result.subscription.clone()])
                    .await
                {
                    Err(error) => {
                        return Err(ContextCreationListenError::CommandResultError(error))
                    }
                    Ok(_) => {}
                }
            };
        }
        match result {
            Err(error) => Err(ContextCreationListenError::CommandResultError(error)),
            Ok(result) => Ok(result),
        }
    }

    pub async fn open_url(
        &mut self,
        url: String,
        wait: Option<ReadinessState>,
        context_id: Option<BidiBrowsingContext>,
    ) -> Result<NavigateResult, OpenUrlError> {
        let context_id = context_id.unwrap_or(self.get_active_context_id()?);
        let mut builder = NavigateBuilder::default();
        builder = builder.context(context_id).url(url);
        if let Some(w) = wait {
            builder = builder.wait(w);
        }
        let command = builder.build().unwrap();

        let result_value = self.send_command(command).await
            .map_err(|err| OpenUrlError::CommandResultError(
                CommandResultError::SessionSendError(err),
            ))?;

        NavigateResult::try_from(result_value.clone()).map_err(|_| {
            OpenUrlError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result_value),
            )
        })
    }

    pub async fn find_nodes(
        &mut self,
        locator: Locator,
        context_id: Option<BidiBrowsingContext>,
        max_node_count: Option<u64>,
        serialization_options: Option<SerializationOptions>,
        start_nodes: Option<Vec<SharedReference>>,
    ) -> Result<LocateNodesResult, FindNodesError> {
        let context_id = context_id.unwrap_or(self.get_active_context_id()?);
        let mut builder = LocateNodesBuilder::default();
        builder = builder.context(context_id).locator(locator);
        if let Some(count) = max_node_count {
            builder = builder.max_node_count(count);
        }
        if let Some(opts) = serialization_options {
            builder = builder.serialization_options(opts);
        }
        if let Some(nodes) = start_nodes {
            builder = builder.start_nodes(nodes);
        }
        let command = builder.build().unwrap();

        let result_value = self.send_command(command).await
            .map_err(|err| FindNodesError::CommandResultError(
                CommandResultError::SessionSendError(err),
            ))?;

        LocateNodesResult::try_from(result_value.clone()).map_err(|_| {
            FindNodesError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result_value),
            )
        })
    }

    pub async fn evaluate_script(
        &mut self,
        expression: String,
        await_promise: bool,
        target: Option<Target>,
        result_ownership: Option<ResultOwnership>,
        serialization_options: Option<SerializationOptions>,
        user_activation: Option<bool>,
    ) -> Result<EvaluateResultSuccess, EvaluateResultError> {
        let target = target.unwrap_or(Target::ContextTarget(ContextTarget {
            context: self.get_active_context_id()?,
            sandbox: None,
        }));
        let mut builder = EvaluateBuilder::default();
        builder = builder.expression(expression).target(target).await_promise(await_promise);
        if let Some(ro) = result_ownership {
            builder = builder.result_ownership(ro);
        }
        if let Some(so) = serialization_options {
            builder = builder.serialization_options(so);
        }
        if let Some(ua) = user_activation {
            builder = builder.user_activation(ua);
        }
        let command = builder.build().unwrap();

        let result_value = self.send_command(command).await
            .map_err(|err| EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(err),
            ))?;

        if let Ok(success) = serde_json::from_value::<EvaluateResultSuccess>(result_value.clone()) {
            return Ok(success);
        }
        if let Ok(exception) = serde_json::from_value::<EvaluateResultException>(result_value.clone()) {
            return Err(EvaluateResultError::ExceptionError(exception));
        }

        Err(EvaluateResultError::CommandResultError(
            CommandResultError::InvalidResultTypeError(result_value),
        ))
    }

    pub async fn call_function(
        &mut self,
        function_declaration: String,
        await_promise: bool,
        target: Option<Target>,
        arguments: Option<Vec<LocalValue>>,
        result_ownership: Option<ResultOwnership>,
        serialization_options: Option<SerializationOptions>,
        this: Option<LocalValue>,
        user_activation: Option<bool>,
    ) -> Result<EvaluateResultSuccess, EvaluateResultError> {
        let target = target.unwrap_or(Target::ContextTarget(ContextTarget {
            context: self.get_active_context_id()?,
            sandbox: None,
        }));

        let mut builder = CallFunctionBuilder::default();
        builder = builder.function_declaration(function_declaration).await_promise(await_promise).target(target);
        if let Some(args) = arguments {
            builder = builder.arguments(args);
        }
        if let Some(ro) = result_ownership {
            builder = builder.result_ownership(ro);
        }
        if let Some(so) = serialization_options {
            builder = builder.serialization_options(so);
        }
        if let Some(t) = this {
            builder = builder.this(t);
        }
        if let Some(ua) = user_activation {
            builder = builder.user_activation(ua);
        }
        let command = builder.build().unwrap();

        let result_value = self.send_command(command).await
            .map_err(|err| EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(err),
            ))?;

        if let Ok(success) = serde_json::from_value::<EvaluateResultSuccess>(result_value.clone()) {
            return Ok(success);
        }
        if let Ok(exception) = serde_json::from_value::<EvaluateResultException>(result_value.clone()) {
            return Err(EvaluateResultError::ExceptionError(exception));
        }

        Err(EvaluateResultError::CommandResultError(
            CommandResultError::InvalidResultTypeError(result_value),
        ))
    }

    pub async fn add_preload_script(
        &mut self,
        function_declaration: String,
        arguments: Option<Vec<ChannelValue>>,
        contexts: Option<Vec<String>>,
        user_contexts: Option<Vec<String>>,
        sandbox: Option<String>,
    ) -> Result<PreloadScript, EvaluateResultError> {
        let mut builder = AddPreloadScriptBuilder::default();
        builder = builder.function_declaration(function_declaration);
        if let Some(args) = arguments {
            builder = builder.arguments(args);
        }
        if let Some(ctx) = contexts {
            builder = builder.contexts(ctx);
        }
        if let Some(uc) = user_contexts {
            builder = builder.user_contexts(uc);
        }
        if let Some(sb) = sandbox {
            builder = builder.sandbox(sb);
        }
        let command = builder.build().unwrap();

        let result_value = self.send_command(command).await
            .map_err(|err| EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(err),
            ))?;

        let add_result = AddPreloadScriptResult::try_from(result_value.clone()).map_err(|_| {
            EvaluateResultError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result_value),
            )
        })?;

        Ok(add_result.script)
    }

    pub async fn remove_preload_script(
        &mut self,
        script: PreloadScript,
    ) -> Result<(), EvaluateResultError> {
        let command = RemovePreloadScriptBuilder::default()
            .script(script)
            .build()
            .unwrap();

        self.send_command(command).await
            .map_err(|err| EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(err),
            ))?;

        Ok(())
    }

    pub fn get_active_context_id(&self) -> Result<BidiBrowsingContext, ContextIndexError> {
        let contexts = self.browsing_contexts.lock().unwrap();
        match contexts.get(self.active_bc_index) {
            Some(context) => Ok(context.id().to_string()),
            None => Err(ContextIndexError {}),
        }
    }

    pub async fn add_event_handler<F, R>(
        &mut self,
        events: HashSet<&str>,
        handler: F,
    ) -> String
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        self.session.lock().await.add_event_handler(events, handler)
    }

    pub async fn create_context(
        &mut self,
        context_type: Option<CreateType>,
        reference_context: Option<&Context>,
        background: bool,
    ) -> Result<Context, CommandResultError> {
        let mut session = self.session.lock().await;
        let mut creator = CreateBrowsingContext::new(&mut *session);
        if let Some(ct) = context_type {
            creator = creator.r#type(ct);
        }
        creator = creator.background(background);
        if let Some(rc) = reference_context {
            creator = creator.reference_context(rc);
        }
        creator.create().await
    }

    pub async fn move_mouse_to_node<N: crate::nodes::Node>(
        &mut self,
        node: &mut N,
        context: Option<&BidiBrowsingContext>,
        scroll_into_view: bool,
    ) -> Result<(), crate::error::MouseInputError> {
        let context = match context {
            Some(ctx) => ctx,
            None => &self.get_active_context_id()?,
        };

        if scroll_into_view {
            node.scroll_into_view().await?;
        }

        let position = node.get_position().await
            .ok_or(crate::error::InvalidPositionError)?;

        let center_point = Point {
            x: position.x + (position.width / 2.0),
            y: position.y + (position.height / 2.0),
        };

        self.mouse.move_to(center_point, context, None).await?;
        Ok(())
    }

    pub async fn click_on_node<N: crate::nodes::Node>(
        &mut self,
        node: &mut N,
        context: Option<&BidiBrowsingContext>,
        options: Option<crate::input::MouseClickOptions>,
    ) -> Result<(), crate::error::MouseInputError> {
        let context = match context {
            Some(ctx) => ctx,
            None => &self.get_active_context_id()?,
        };

        self.move_mouse_to_node(node, Some(context), true).await?;
        self.mouse.click(None, context, options).await?;
        Ok(())
    }

    async fn on_network<F, R>(
        &mut self,
        phases: Vec<InterceptPhase>,
        event_names: Vec<&'static str>,
        handler: F,
        url_patterns: Option<Vec<UrlPattern>>,
        _contexts: Option<Vec<String>>,
    ) -> Result<(), InterceptNetworkError>
    where
        F: Fn(Event) -> R + Send + Sync + 'static,
        R: Future<Output=()> + Send + 'static,
    {
        let mut builder = AddInterceptBuilder::default();
        builder = builder.phases(phases);
        if let Some(patterns) = url_patterns {
            builder = builder.url_patterns(patterns);
        }
        let add_intercept_command = builder.build().unwrap();

        let result_value = self.send_command(add_intercept_command).await
            .map_err(|e| InterceptNetworkError::CommandResultError(CommandResultError::SessionSendError(e)))?;

        let _ = AddInterceptResult::try_from(result_value.clone()).map_err(|_| {
            InterceptNetworkError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result_value)
            )
        })?;

        let events = HashSet::from_iter(event_names);
        let bidi_event = self.session.lock().await.create_event::<_, _, BidiSession<T>>(events, handler);
        self.session.lock().await.subscribe_events(bidi_event).await
            .map_err(|e| InterceptNetworkError::CommandResultError(e))?;

        Ok(())
    }

    pub async fn on_request<F, Fut>(
        &mut self,
        handler: F,
        url_patterns: Option<Vec<UrlPattern>>,
        contexts: Option<Vec<String>>,
    ) -> Result<(), InterceptNetworkError>
    where
        F: Fn(NetworkRequest<T>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let session = Arc::clone(&self.session);
        let handler = Arc::new(handler);

        self.on_network(
            vec![InterceptPhase::BeforeRequestSent],
            vec!["network.beforeRequestSent"],
            move |event: Event| {
                let handler = Arc::clone(&handler);
                let session = Arc::clone(&session);
                async move {
                    if let Event::Network(NetworkEvent::BeforeRequestSent(before_request)) = event {
                        let request = NetworkRequest::new(before_request.params, session);
                        handler(request).await;
                    }
                }
            },
            url_patterns,
            contexts,
        ).await
    }

    pub async fn authenticate(
        &mut self,
        username: impl Into<String> + Send + 'static,
        password: impl Into<String> + Send + 'static,
        url_patterns: Option<Vec<UrlPattern>>,
        contexts: Option<Vec<String>>,
    ) -> Result<(), InterceptNetworkError> {
        use rustenium_bidi_definitions::network::types::{AuthCredentials, AuthCredentialsType};

        let username = username.into();
        let password = password.into();
        let session = Arc::clone(&self.session);

        tracing::debug!("Activated authentication handler");
        self.on_network(
            vec![InterceptPhase::AuthRequired, InterceptPhase::ResponseStarted],
            vec!["network.authRequired"],
            move |event: Event| {
                let session = Arc::clone(&session);
                let username = username.clone();
                let password = password.clone();
                tracing::trace!("Authentication handler invoked");
                async move {
                    tracing::trace!("Processing authentication request");
                    if let Event::Network(NetworkEvent::AuthRequired(auth_required)) = event {
                        let request = NetworkRequest::from_auth_required(auth_required.params, session);

                        let credentials = AuthCredentials {
                            r#type: AuthCredentialsType::Password,
                            username,
                            password,
                        };

                        let _ = request.continue_with_auth(Some(credentials)).await;
                    }
                }
            },
            url_patterns,
            contexts,
        ).await
    }

    pub async fn screenshot(
        &mut self,
        context_id: Option<BidiBrowsingContext>,
        origin: Option<CaptureScreenshotOrigin>,
        format: Option<ImageFormat>,
        clip: Option<ClipRectangle>,
        save_path: Option<&str>,
    ) -> Result<String, crate::error::ScreenshotError> {
        let context_id = context_id.unwrap_or(self.get_active_context_id()?);

        let mut builder = CaptureScreenshotBuilder::default();
        builder = builder.context(context_id);
        if let Some(o) = origin {
            builder = builder.origin(o);
        }
        if let Some(f) = format {
            builder = builder.format(f);
        }
        if let Some(c) = clip {
            builder = builder.clip(c);
        }
        let command = builder.build().unwrap();

        let result_value = self.send_command(command).await
            .map_err(|err| crate::error::ScreenshotError::CommandResultError(
                CommandResultError::SessionSendError(err),
            ))?;

        let screenshot_result = CaptureScreenshotResult::try_from(result_value.clone()).map_err(|_| {
            crate::error::ScreenshotError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result_value),
            )
        })?;

        let base64_data = screenshot_result.data;

        if let Some(path) = save_path {
            use std::path::Path;

            let path_obj = Path::new(path);

            let final_path = if path_obj.is_dir() {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis())
                    .unwrap_or(0);
                let filename = format!("screenshot_{}.png", timestamp);
                path_obj.join(filename)
            } else {
                if let Some(parent) = path_obj.parent() {
                    if !parent.as_os_str().is_empty() && !parent.exists() {
                        return Err(crate::error::ScreenshotError::InvalidPath(
                            format!("Parent directory does not exist: {}", parent.display())
                        ));
                    }
                }
                path_obj.to_path_buf()
            };

            use base64::{Engine as _, engine::general_purpose};
            let decoded = general_purpose::STANDARD.decode(&base64_data)
                .map_err(|e| crate::error::ScreenshotError::Base64DecodeError(e.to_string()))?;

            std::fs::write(&final_path, decoded)
                .map_err(|e| crate::error::ScreenshotError::FileWriteError(e.to_string()))?;

            Ok(final_path.to_string_lossy().to_string())
        } else {
            Ok(base64_data)
        }
    }

    pub async fn set_timezone_override(
        &mut self,
        timezone: Option<String>,
        contexts: Option<Vec<BidiBrowsingContext>>,
        user_contexts: Option<Vec<String>>,
    ) -> Result<(), EmulationError> {
        use rustenium_bidi_definitions::emulation::command_builders::SetTimezoneOverrideBuilder;

        let contexts = match contexts {
            Some(contexts) => Some(contexts),
            None => Some(vec![self.get_active_context_id()?]),
        };

        let mut builder = SetTimezoneOverrideBuilder::default();
        if let Some(tz) = timezone {
            builder = builder.timezone(tz);
        }
        if let Some(ctx) = contexts {
            builder = builder.contexts(ctx);
        }
        if let Some(uc) = user_contexts {
            builder = builder.user_contexts(uc);
        }
        let command = builder.build().unwrap();

        self.send_command(command).await
            .map_err(|err| EmulationError::CommandResultError(
                CommandResultError::SessionSendError(err),
            ))?;

        Ok(())
    }

    pub async fn end_session(&mut self) -> Result<(), SessionSendError> {
        self.session.lock().await.end_session().await?;
        Ok(())
    }
}
