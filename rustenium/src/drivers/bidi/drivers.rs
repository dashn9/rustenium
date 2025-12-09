use rustenium_core::{process::Process, transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport}, NetworkRequest, Session};
use std::error::Error;

use crate::error::{ContextCreationListenError, ContextIndexError, EvaluateResultError, FindNodesError, InterceptNetworkError, OpenUrlError};
use rustenium_bidi_commands::browsing_context::types::{
    BrowsingContext as BidiBrowsingContext, ClipRectangle, CreateType, ImageFormat, Locator, OriginUnion, ReadinessState,
};
use rustenium_bidi_commands::script::commands::{
    AddPreloadScript, AddPreloadScriptParameters, CallFunction,
    CallFunctionParameters, EvaluateResult, RemovePreloadScript, RemovePreloadScriptParameters,
    ScriptAddPreloadScriptMethod, ScriptCallFunctionMethod, ScriptRemovePreloadScriptMethod,
};
use rustenium_bidi_commands::session::commands::SubscribeResult;
use rustenium_bidi_commands::session::types::CapabilitiesRequest;
use rustenium_bidi_commands::{BrowsingContextCommand, BrowsingContextEvent, BrowsingContextResult, CommandData, EmulationCommand, EmulationResult, Event, EventData, NetworkCommand, NetworkEvent, NetworkResult, ResultData, ScriptCommand, ScriptResult};
use rustenium_core::Context;
use rustenium_core::events::EventManagement;
use rustenium_core::session::SessionConnectionType;
use std::collections::HashSet;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::sleep;

use crate::nodes::{Node, NodePosition};
use rustenium_bidi_commands::browsing_context::commands::{
    BrowsingContextCaptureScreenshotMethod, BrowsingContextLocateNodesMethod, BrowsingContextNavigateMethod,
    CaptureScreenshot, CaptureScreenshotParameters, CaptureScreenshotResult, LocateNodes,
    LocateNodesParameters, LocateNodesResult, Navigate, NavigateParameters, NavigateResult,
};
use rustenium_bidi_commands::script::commands::{
    Evaluate, EvaluateParameters, ScriptEvaluateMethod,
};
use rustenium_bidi_commands::script::types::{
    ChannelValue, ContextTarget, EvaluateResultSuccess, LocalValue, PreloadScript,
    PrimitiveProtocolValue, RemoteReference, RemoteValue, ResultOwnership, SerializationOptions,
    SharedReference, Target
};
use rustenium_core::error::{CommandResultError, SessionSendError};
use tokio::io;
use rustenium_bidi_commands::network::commands::{AddIntercept, AddInterceptParameters, NetworkAddInterceptMethod};
use rustenium_bidi_commands::network::types::{InterceptPhase, UrlPattern};
use crate::input::{BidiMouse, HumanMouse, Keyboard, Mouse, Point};

fn is_connection_refused(e: &reqwest::Error) -> bool {
    if let Some(io_err) = e.source().and_then(|s| s.downcast_ref::<io::Error>()) {
        return io_err.kind() == io::ErrorKind::ConnectionRefused;
    }
    false
}

pub trait DriverConfiguration {
    fn exe_path(&self) -> &str;
    fn flags(&self) -> Vec<String>;
}

pub trait BidiDrive<T: ConnectionTransport> {
    async fn start(
        driver_config: &impl DriverConfiguration,
        connection_transport_config: &ConnectionTransportConfig,
        session_connection_type: SessionConnectionType,
        capabilities: Option<CapabilitiesRequest>,
    ) -> (Arc<TokioMutex<Session<WebsocketConnectionTransport>>>, Process) {
        let driver_process = Process::create(driver_config.exe_path(), driver_config.flags());
        let mut session = Session::<T>::ws_new(connection_transport_config).await;
        session
            .create_new_bidi_session(session_connection_type, capabilities)
            .await;
        (Arc::new(TokioMutex::new(session)), driver_process)
    }
}

pub struct BidiDriver<T: ConnectionTransport + Send + Sync> {
    pub exe_path: String,
    pub flags: Vec<String>,
    pub session: Arc<TokioMutex<Session<T>>>,
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
        session: Arc<TokioMutex<Session<T>>>,
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

    pub async fn new_session(
        &mut self,
        connection_type: SessionConnectionType,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub async fn send_command(&mut self, command: CommandData) -> Result<ResultData, SessionSendError> {
        let rx = {
            let mut session = self.session.lock().await;
            session.send_and_get_receiver(command).await
        };

        match tokio::time::timeout(std::time::Duration::from_secs(100), rx).await {
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
        let result = self
            .session
            .lock()
            .await
            .subscribe_events(
                HashSet::from(["browsingContext.contextCreated"]),
                move |event| {
                    let bc = browsing_contexts.clone();
                    async move {
                        if let EventData::BrowsingContextEvent(
                            BrowsingContextEvent::ContextCreated(context),
                        ) = event.event_data
                        {
                            bc.lock().unwrap().push(Context::from_id(
                                context.params.context,
                                CreateType::Window,
                            ));
                        }
                    }
                },
                None,
                None,
            )
            .await;
        // Wait for 2s, to allow current BrowsingContext be updated via the event.
        sleep(Duration::from_millis(2000)).await;
        if (self
            .browsing_contexts
            .lock()
            .expect("Unable to acquire lock")
            .len()
            > 0)
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
                    Ok(None) => return Ok(None),
                    _ => {}
                }
            };
        } else {
        }
        match result {
            Err(error) => Err(ContextCreationListenError::CommandResultError(error)),
            Ok(result) => Ok(result),
        }
    }

    async fn new_browsing_context(&mut self) -> bool {
        let browsing_context = Context::new(&mut *self.session.lock().await, None, None, false)
            .await
            .unwrap();
        self.browsing_contexts
            .lock()
            .unwrap()
            .push(browsing_context);
        true
    }

    //TODO: ReloadResult points to NavigateResult, update Generator to prevent Results from being generated into types again due to being pointed by
    pub async fn open_url(
        &mut self,
        url: String,
        wait: Option<ReadinessState>,
        context_id: Option<BidiBrowsingContext>,
    ) -> Result<NavigateResult, OpenUrlError> {
        let context_id = context_id.unwrap_or(self.get_active_context_id()?);
        let result = self
            .send_command(CommandData::BrowsingContextCommand(
                BrowsingContextCommand::Navigate(Navigate {
                    method: BrowsingContextNavigateMethod::BrowsingContextNavigate,
                    params: NavigateParameters {
                        url,
                        context: context_id,
                        wait,
                    },
                }),
            ))
            .await;
        match result {
            Ok(ResultData::BrowsingContextResult(browsing_context_result)) => {
                match browsing_context_result {
                    BrowsingContextResult::NavigateResult(navigate_result) => Ok(navigate_result),
                    _ => Err(OpenUrlError::CommandResultError(
                        CommandResultError::InvalidResultTypeError(
                            ResultData::BrowsingContextResult(browsing_context_result),
                        ),
                    )),
                }
            }
            Ok(result) => Err(OpenUrlError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result),
            )),
            Err(err) => Err(OpenUrlError::CommandResultError(
                CommandResultError::SessionSendError(err),
            )),
        }
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
        let result = self
            .send_command(CommandData::BrowsingContextCommand(
                BrowsingContextCommand::LocateNodes(LocateNodes {
                    method: BrowsingContextLocateNodesMethod::BrowsingContextLocateNodes,
                    params: LocateNodesParameters {
                        locator,
                        context: context_id,
                        max_node_count,
                        serialization_options,
                        start_nodes,
                    },
                }),
            ))
            .await;
        match result {
            Ok(ResultData::BrowsingContextResult(browsing_context_result)) => {
                match browsing_context_result {
                    BrowsingContextResult::LocateNodesResult(locate_nodes_result) => {
                        Ok(locate_nodes_result)
                    }
                    _ => Err(FindNodesError::CommandResultError(
                        CommandResultError::InvalidResultTypeError(
                            ResultData::BrowsingContextResult(browsing_context_result),
                        ),
                    )),
                }
            }
            Ok(result) => Err(FindNodesError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result),
            )),
            Err(err) => Err(FindNodesError::CommandResultError(
                CommandResultError::SessionSendError(err),
            )),
        }
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
        let result = self
            .send_command(CommandData::ScriptCommand(ScriptCommand::Evaluate(
                Evaluate {
                    method: ScriptEvaluateMethod::ScriptEvaluate,
                    params: EvaluateParameters {
                        expression,
                        target,
                        await_promise,
                        result_ownership,
                        serialization_options,
                        user_activation,
                    },
                },
            )))
            .await;
        match result {
            Ok(ResultData::ScriptResult(script_result)) => match script_result {
                ScriptResult::EvaluateResult(evaluate_result) => match evaluate_result {
                    EvaluateResult::EvaluateResultSuccess(evaluate_result_success) => {
                        Ok(evaluate_result_success)
                    }
                    EvaluateResult::EvaluateResultException(evaluate_result_error) => {
                        Err(EvaluateResultError::ExceptionError(evaluate_result_error))
                    }
                },
                _ => Err(EvaluateResultError::CommandResultError(
                    CommandResultError::InvalidResultTypeError(ResultData::ScriptResult(
                        script_result,
                    )),
                )),
            },
            Ok(result) => Err(EvaluateResultError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result),
            )),
            Err(err) => Err(EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(err),
            )),
        }
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

        let result = self
            .send_command(CommandData::ScriptCommand(ScriptCommand::CallFunction(
                CallFunction {
                    method: ScriptCallFunctionMethod::ScriptCallFunction,
                    params: CallFunctionParameters {
                        function_declaration,
                        await_promise,
                        target,
                        arguments,
                        result_ownership,
                        serialization_options,
                        this,
                        user_activation,
                    },
                },
            )))
            .await;

        match result {
            Ok(ResultData::ScriptResult(script_result)) => {
                let evaluate_result = match script_result {
                    ScriptResult::CallFunctionResult(eval_result) => eval_result,
                    ScriptResult::EvaluateResult(eval_result) => eval_result,
                    _ => return Err(EvaluateResultError::CommandResultError(
                        CommandResultError::InvalidResultTypeError(ResultData::ScriptResult(
                            script_result,
                        )),
                    )),
                };

                match evaluate_result {
                    EvaluateResult::EvaluateResultSuccess(evaluate_result_success) => {
                        Ok(evaluate_result_success)
                    }
                    EvaluateResult::EvaluateResultException(evaluate_result_error) => {
                        Err(EvaluateResultError::ExceptionError(evaluate_result_error))
                    }
                }
            }
            Ok(result) => Err(EvaluateResultError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result),
            )),
            Err(err) => Err(EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(err),
            )),
        }
    }

    /// Add a preload script that will be executed in new contexts
    pub async fn add_preload_script(
        &mut self,
        function_declaration: String,
        arguments: Option<Vec<ChannelValue>>,
        contexts: Option<Vec<String>>,
        user_contexts: Option<Vec<String>>,
        sandbox: Option<String>,
    ) -> Result<PreloadScript, EvaluateResultError> {
        let result = self
            .send_command(CommandData::ScriptCommand(ScriptCommand::AddPreloadScript(
                AddPreloadScript {
                    method: ScriptAddPreloadScriptMethod::ScriptAddPreloadScript,
                    params: AddPreloadScriptParameters {
                        function_declaration,
                        arguments,
                        contexts,
                        user_contexts,
                        sandbox,
                    },
                },
            )))
            .await;

        match result {
            Ok(ResultData::ScriptResult(ScriptResult::AddPreloadScriptResult(script_result))) => {
                Ok(script_result.script)
            }
            Ok(result) => Err(EvaluateResultError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result),
            )),
            Err(err) => Err(EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(err),
            )),
        }
    }

    /// Remove a preload script by its ID
    pub async fn remove_preload_script(
        &mut self,
        script: PreloadScript,
    ) -> Result<(), EvaluateResultError> {
        let result = self
            .send_command(CommandData::ScriptCommand(ScriptCommand::RemovePreloadScript(
                RemovePreloadScript {
                    method: ScriptRemovePreloadScriptMethod::ScriptRemovePreloadScript,
                    params: RemovePreloadScriptParameters { script },
                },
            )))
            .await;

        match result {
            Ok(ResultData::ScriptResult(ScriptResult::RemovePreloadScriptResult(_))) => Ok(()),
            Ok(result) => Err(EvaluateResultError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result),
            )),
            Err(err) => Err(EvaluateResultError::CommandResultError(
                CommandResultError::SessionSendError(err),
            )),
        }
    }

    fn get_context_id(&self, context_index: usize) -> Result<String, ContextIndexError> {
        match self
            .browsing_contexts
            .lock()
            .unwrap()
            .get(self.active_bc_index)
        {
            Some(context) => Ok(context.get_context_id()),
            None => Err(ContextIndexError {}),
        }
    }

    /// Get the active context
    pub fn get_active_context(&self) -> Result<Context, ContextIndexError> {
        match self
            .browsing_contexts
            .lock()
            .unwrap()
            .get(self.active_bc_index)
        {
            Some(context) => Ok(context.clone()),
            None => Err(ContextIndexError {}),
        }
    }

    /// Get the active context ID
    pub fn get_active_context_id(&self) -> Result<BidiBrowsingContext, ContextIndexError> {
        self.get_active_context().map(|c| c.id().to_string())
    }

    /// Add an event handler without sending a subscription command
    /// Returns the handler ID (either provided or generated)
    pub async fn add_event_handler<F, R>(
        &mut self,
        events: HashSet<&str>,
        handler: F,
        handler_id: Option<String>,
    ) -> String
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        self.session.lock().await.add_event_handler(events, handler, handler_id)
    }

    /// Create a new browsing context (tab or window)
    pub async fn create_context(
        &mut self,
        context_type: Option<rustenium_bidi_commands::browsing_context::types::CreateType>,
        reference_context: Option<&rustenium_core::Context>,
        background: bool,
    ) -> Result<rustenium_core::Context, rustenium_core::error::CommandResultError> {
        let mut session = self.session.lock().await;
        rustenium_core::Context::new(&mut *session, context_type, reference_context, background).await
    }

    /// Move mouse to the center of a node
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

    /// Click on a node (scrolls into view and moves mouse first)
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
        // Then click
        self.mouse.click(None, context, options).await?;
        Ok(())
    }

    /// Register a handler to be called for each network request
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
        // Use active context if no contexts provided
        let contexts = match contexts {
            Some(ctxs) => Some(ctxs),
            None => Some(vec![self.get_active_context_id()?]),
        };


        // Add network intercept
        let add_intercept_command = CommandData::NetworkCommand(
            NetworkCommand::AddIntercept(AddIntercept {
                method: NetworkAddInterceptMethod::NetworkAddIntercept,
                params: AddInterceptParameters {
                    phases: vec![InterceptPhase::BeforeRequestSent],
                    url_patterns,
                    contexts: None,
                },
            })
        );

        let result = self.send_command(add_intercept_command).await
            .map_err(|e| InterceptNetworkError::CommandResultError(CommandResultError::SessionSendError(e)))?;

        // Extract intercept ID from result
        let intercept_id = if let ResultData::NetworkResult(NetworkResult::AddInterceptResult(intercept_result)) = result {
            intercept_result.intercept
        } else {
            return Err(InterceptNetworkError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result)
            ));
        };

        // Clone Arc to session for use in handler (cheap - just clones the Arc pointer)
        let session = Arc::clone(&self.session);

        // Subscribe to network.beforeRequestSent events
        let handler = Arc::new(handler);
        self.session.lock().await.subscribe_events(
            HashSet::from(["network.beforeRequestSent"]),
            move |event: Event| {
                let handler = Arc::clone(&handler);
                let session = Arc::clone(&session);
                async move {
                    if let EventData::NetworkEvent(NetworkEvent::BeforeRequestSent(before_request)) = event.event_data {
                        let request = NetworkRequest::new(before_request.params, session);
                        handler(request).await;
                    }
                }
            },
            None,
            None,
        ).await
            .map_err(|e| InterceptNetworkError::CommandResultError(e))?;

        Ok(())
    }

    /// Capture a screenshot of the current browsing context
    /// If `save_path` is provided:
    ///   - If it's a directory, saves with auto-generated filename (screenshot_TIMESTAMP.png)
    ///   - If it's a file path, saves to that exact location
    ///   Returns the final path where the file was saved
    /// Otherwise, returns the base64-encoded image data
    pub async fn screenshot(
        &mut self,
        context_id: Option<BidiBrowsingContext>,
        origin: Option<OriginUnion>,
        format: Option<ImageFormat>,
        clip: Option<ClipRectangle>,
        save_path: Option<&str>,
    ) -> Result<String, crate::error::ScreenshotError> {
        let context_id = context_id.unwrap_or(self.get_active_context_id()?);

        let result = self
            .send_command(CommandData::BrowsingContextCommand(
                BrowsingContextCommand::CaptureScreenshot(CaptureScreenshot {
                    method: BrowsingContextCaptureScreenshotMethod::BrowsingContextCaptureScreenshot,
                    params: CaptureScreenshotParameters {
                        context: context_id,
                        origin,
                        format,
                        clip,
                    },
                }),
            ))
            .await;

        let base64_data = match result {
            Ok(ResultData::BrowsingContextResult(browsing_context_result)) => {
                match browsing_context_result {
                    BrowsingContextResult::CaptureScreenshotResult(screenshot_result) => {
                        screenshot_result.data
                    }
                    _ => return Err(crate::error::ScreenshotError::CommandResultError(
                        CommandResultError::InvalidResultTypeError(
                            ResultData::BrowsingContextResult(browsing_context_result),
                        ),
                    )),
                }
            }
            Ok(result) => return Err(crate::error::ScreenshotError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result),
            )),
            Err(err) => return Err(crate::error::ScreenshotError::CommandResultError(
                CommandResultError::SessionSendError(err),
            )),
        };

        // If save_path is provided, save to file
        if let Some(path) = save_path {
            use std::path::Path;

            let path_obj = Path::new(path);

            // Determine the final file path
            let final_path = if path_obj.is_dir() {
                // Generate timestamp-based filename
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map(|d| d.as_millis())
                    .unwrap_or(0);
                let filename = format!("screenshot_{}.png", timestamp);
                path_obj.join(filename)
            } else {
                // Verify parent directory exists
                if let Some(parent) = path_obj.parent() {
                    if !parent.as_os_str().is_empty() && !parent.exists() {
                        return Err(crate::error::ScreenshotError::InvalidPath(
                            format!("Parent directory does not exist: {}", parent.display())
                        ));
                    }
                }
                path_obj.to_path_buf()
            };

            // Decode base64 and write to file
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

    /// Set the timezone override for the browsing contexts
    ///
    /// # Arguments
    /// * `timezone` - Optional timezone ID (e.g., "America/New_York", "Europe/London"). Pass None to clear the override.
    /// * `contexts` - Optional list of browsing context IDs to apply the override to. If None, applies to the active context.
    /// * `user_contexts` - Optional list of user context IDs
    ///
    /// # Example
    /// ```ignore
    /// // Set timezone to New York
    /// driver.set_timezone_override(Some("America/New_York".to_string()), None, None).await?;
    ///
    /// // Clear timezone override
    /// driver.set_timezone_override(None, None, None).await?;
    /// ```
    pub async fn set_timezone_override(
        &mut self,
        timezone: Option<String>,
        contexts: Option<Vec<BidiBrowsingContext>>,
        user_contexts: Option<Vec<String>>,
    ) -> Result<(), crate::error::EmulationError> {
        use rustenium_bidi_commands::emulation::commands::{
            EmulationSetTimezoneOverrideMethod, SetTimezoneOverride, SetTimezoneOverrideParameters,
        };

        let contexts = match contexts {
            Some(ctxs) => Some(ctxs),
            None => Some(vec![self.get_active_context_id()?]),
        };

        let result = self
            .send_command(CommandData::EmulationCommand(
                EmulationCommand::SetTimezoneOverride(SetTimezoneOverride {
                    method: EmulationSetTimezoneOverrideMethod::EmulationSetTimezoneOverride,
                    params: SetTimezoneOverrideParameters {
                        timezone,
                        contexts,
                        user_contexts,
                    },
                }),
            ))
            .await;

        match result {
            Ok(ResultData::EmptyResult(_)) => Ok(()),
            Ok(result) => Err(crate::error::EmulationError::CommandResultError(
                CommandResultError::InvalidResultTypeError(result),
            )),
            Err(err) => Err(crate::error::EmulationError::CommandResultError(
                CommandResultError::SessionSendError(err),
            )),
        }
    }
}
