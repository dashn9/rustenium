use rustenium_bidi_definitions::emulation::commands::SetTimezoneOverride;
use rustenium_bidi_definitions::script::commands::{
    AddPreloadScript, CallFunction, Evaluate, RemovePreloadScript,
};
use crate::domain::context::BrowsingContext as Context;
use rustenium_core::{
    BidiSession, NetworkRequest,
    process::Process,
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
};

use crate::error::bidi::{
    ContextCreationError, ContextIndexError, EmulationError,
    EvaluateResultError, FindNodesError, InterceptNetworkError, NavigateError, ScreenshotError
};
use rustenium_bidi_definitions::Command;
use rustenium_bidi_definitions::Event;
use rustenium_bidi_definitions::browsing_context::commands::{
    CaptureScreenshot, Create, LocateNodes, Navigate
};
use rustenium_bidi_definitions::browsing_context::results::{
    CaptureScreenshotResult, CreateResult, LocateNodesResult, NavigateResult,
};
use rustenium_bidi_definitions::network::command_builders::AddInterceptBuilder;
use rustenium_bidi_definitions::network::events::NetworkEvent;
use rustenium_bidi_definitions::network::results::AddInterceptResult;
use rustenium_bidi_definitions::network::types::{InterceptPhase, UrlPattern};
use rustenium_bidi_definitions::script::results::AddPreloadScriptResult;
use rustenium_bidi_definitions::script::types::{
    EvaluateResultException, EvaluateResultSuccess,
    PreloadScript,
};
use rustenium_bidi_definitions::session::results::SubscribeResult;
use rustenium_bidi_definitions::session::types::CapabilitiesRequest;
use rustenium_bidi_definitions::{
    base::CommandResponse,
    browsing_context::{
        events::ContextCreated,
        types::{
            BrowsingContext as BidiBrowsingContext, CreateType,
        },
    },
};
use rustenium_core::error::{CommandResultError, SessionSendError};
use rustenium_core::events::BidiEventManagement;
use rustenium_core::session::SessionConnectionType;
use std::collections::HashSet;
use std::future::Future;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::Mutex as TokioMutex;
use tokio::time::sleep;

use crate::input::{BidiMouse, BidiKeyboard, HumanMouse};

pub struct OnRequestBuilder<'a, T: ConnectionTransport + Send + Sync, F> {
    driver: &'a mut BidiDriver<T>,
    handler: F,
    url_patterns: Option<Vec<UrlPattern>>,
    contexts: Option<Vec<String>>,
}

impl<'a, T, F> OnRequestBuilder<'a, T, F>
where
    T: ConnectionTransport + Send + Sync + 'static,
{
    pub fn new(driver: &'a mut BidiDriver<T>, handler: F) -> Self {
        Self {
            driver,
            handler,
            url_patterns: None,
            contexts: None,
        }
    }

    pub fn url_patterns(mut self, patterns: Vec<UrlPattern>) -> Self {
        self.url_patterns = Some(patterns);
        self
    }

    pub fn contexts(mut self, contexts: Vec<String>) -> Self {
        self.contexts = Some(contexts);
        self
    }

    pub async fn execute<Fut>(self) -> Result<(), InterceptNetworkError>
    where
        F: Fn(NetworkRequest<T>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let session = Arc::clone(&self.driver.session);
        let handler = Arc::new(self.handler);
        self.driver
            .on_network(
                vec![InterceptPhase::BeforeRequestSent],
                vec!["network.beforeRequestSent"],
                move |event: Event| {
                    let handler = Arc::clone(&handler);
                    let session = Arc::clone(&session);
                    async move {
                        if let Event::Network(NetworkEvent::BeforeRequestSent(before_request)) =
                            event
                        {
                            let request = NetworkRequest::new(before_request.params, session);
                            handler(request).await;
                        }
                    }
                },
                self.url_patterns,
                self.contexts,
            )
            .await
    }
}

pub struct AuthenticateBuilder<'a, T: ConnectionTransport + Send + Sync> {
    driver: &'a mut BidiDriver<T>,
    username: String,
    password: String,
    url_patterns: Option<Vec<UrlPattern>>,
    contexts: Option<Vec<String>>,
}

impl<'a, T> AuthenticateBuilder<'a, T>
where
    T: ConnectionTransport + Send + Sync + 'static,
{
    pub fn new(
        driver: &'a mut BidiDriver<T>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> Self {
        Self {
            driver,
            username: username.into(),
            password: password.into(),
            url_patterns: None,
            contexts: None,
        }
    }

    pub fn url_patterns(mut self, patterns: Vec<UrlPattern>) -> Self {
        self.url_patterns = Some(patterns);
        self
    }

    pub fn contexts(mut self, contexts: Vec<String>) -> Self {
        self.contexts = Some(contexts);
        self
    }

    pub async fn execute(self) -> Result<(), InterceptNetworkError> {
        use rustenium_bidi_definitions::network::types::{AuthCredentials, AuthCredentialsType};

        let username = self.username;
        let password = self.password;
        let session = Arc::clone(&self.driver.session);

        tracing::debug!("Activated authentication handler");
        self.driver
            .on_network(
                vec![
                    InterceptPhase::AuthRequired,
                    InterceptPhase::ResponseStarted,
                ],
                vec!["network.authRequired"],
                move |event: Event| {
                    let session = Arc::clone(&session);
                    let username = username.clone();
                    let password = password.clone();
                    tracing::trace!("Authentication handler invoked");
                    async move {
                        tracing::trace!("Processing authentication request");
                        if let Event::Network(NetworkEvent::AuthRequired(auth_required)) = event {
                            let request =
                                NetworkRequest::from_auth_required(auth_required.params, session);
                            let credentials = AuthCredentials {
                                r#type: AuthCredentialsType::Password,
                                username,
                                password,
                            };
                            let _ = request.continue_with_auth(credentials).await;
                        }
                    }
                },
                self.url_patterns,
                self.contexts,
            )
            .await
    }
}

pub trait DriverConfiguration {
    fn exe_path(&self) -> &str;
    fn flags(&self) -> Vec<String>;
}

pub struct BidiDriver<T: ConnectionTransport + Send + Sync> {
    pub exe_path: String,
    pub flags: Vec<String>,
    pub session: Arc<TokioMutex<BidiSession<T>>>,
    pub active_bc_index: usize,
    pub browsing_contexts: Arc<Mutex<Vec<Context>>>,
    pub driver_process: Process,
    pub mouse: Arc<BidiMouse<T>>,
    pub human_mouse: Arc<HumanMouse<BidiMouse<T>>>,
    pub keyboard: Arc<BidiKeyboard<T>>,
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
        let mouse = Arc::new(BidiMouse::new(session.clone()));
        let human_mouse = Arc::new(HumanMouse::new(BidiMouse::new(session.clone())));
        let keyboard = Arc::new(BidiKeyboard::new(session.clone()));

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

    pub async fn send_command(
        &mut self,
        command: impl Into<Command>,
    ) -> Result<CommandResponse, SessionSendError> {
        let rx = {
            let mut session = self.session.lock().await;
            session.send_and_get_receiver(command).await
        };

        match tokio::time::timeout(Duration::from_secs(100), rx).await {
            Ok(Ok(command_result)) => match command_result {
                rustenium_core::CommandResponseState::Success(response) => Ok(response),
                rustenium_core::CommandResponseState::Error(err) => {
                    Err(SessionSendError::ErrorResponse(err))
                }
            },
            Ok(Err(err)) => panic!("A recv error occurred: {}", err),
            Err(_) => Err(SessionSendError::ResponseReceiveTimeoutError(
                rustenium_core::error::ResponseReceiveTimeoutError,
            )),
        }
    }

    pub async fn listen_to_context_creation(
        &mut self,
    ) -> Result<Option<SubscribeResult>, ContextCreationError> {
        let browsing_contexts = self.browsing_contexts.clone();
        let events = HashSet::from(["browsingContext.contextCreated"]);
        let handler = move |event: Event| {
            let bc = browsing_contexts.clone();
            async move {
                if let Ok(context) = TryInto::<ContextCreated>::try_into(event) {
                    tracing::debug!("[BiDiDriver]: BrowsingContext Created: ID: {}", context.params.context.as_ref());
                    bc.lock()
                        .unwrap()
                        .push(Context::from_id(context.params.context, CreateType::Tab));
                }
            }
        };

        let bidi_event = self
            .session
            .lock()
            .await
            .create_event::<_, _, BidiSession<T>>(events, handler);
        let result = self.session.lock().await.subscribe_events(bidi_event).await;
        // Wait for 2s, to allow current BrowsingContext be updated via the event.
        sleep(Duration::from_millis(800)).await;
        match result {
            Err(error) => Err(ContextCreationError::CommandResultError(error)),
            Ok(result) => Ok(result),
        }
    }

    pub async fn navigate(&mut self, navigate: Navigate) -> Result<NavigateResult, NavigateError> {
        // navigate.params.context = navigate.params.context.unwrap_or(self.get_active_context_id()?);

        let result_value = self
            .send_command(navigate)
            .await
            .map_err(|err| {
                NavigateError::CommandResultError(CommandResultError::SessionSendError(err))
            })?
            .result;

        NavigateResult::try_from(result_value.clone()).map_err(|_| {
            NavigateError::CommandResultError(CommandResultError::InvalidResultTypeError(
                result_value,
            ))
        })
    }

    pub async fn find_nodes(
        &mut self,
        locator: LocateNodes,
    ) -> Result<LocateNodesResult, FindNodesError> {
        let result_value = self
            .send_command(locator)
            .await
            .map_err(|err| {
                FindNodesError::CommandResultError(CommandResultError::SessionSendError(err))
            })?
            .result;

        LocateNodesResult::try_from(result_value.clone()).map_err(|_| {
            FindNodesError::CommandResultError(CommandResultError::InvalidResultTypeError(
                result_value,
            ))
        })
    }

    pub async fn evaluate_script(
        &mut self,
        evaluate: Evaluate,
    ) -> Result<EvaluateResultSuccess, EvaluateResultError> {
        let result_value = self
            .send_command(evaluate)
            .await
            .map_err(|err| {
                EvaluateResultError::CommandResultError(CommandResultError::SessionSendError(err))
            })?
            .result;

        if let Ok(success) = serde_json::from_value::<EvaluateResultSuccess>(result_value.clone()) {
            return Ok(success);
        }
        if let Ok(exception) =
            serde_json::from_value::<EvaluateResultException>(result_value.clone())
        {
            return Err(EvaluateResultError::ExceptionError(exception));
        }

        Err(EvaluateResultError::CommandResultError(
            CommandResultError::InvalidResultTypeError(result_value),
        ))
    }

    pub async fn call_function(
        &mut self,
        call_function: CallFunction,
    ) -> Result<EvaluateResultSuccess, EvaluateResultError> {
        let result_value = self
            .send_command(call_function)
            .await
            .map_err(|err| {
                EvaluateResultError::CommandResultError(CommandResultError::SessionSendError(err))
            })?
            .result;

        if let Ok(success) = serde_json::from_value::<EvaluateResultSuccess>(result_value.clone()) {
            return Ok(success);
        }
        if let Ok(exception) =
            serde_json::from_value::<EvaluateResultException>(result_value.clone())
        {
            return Err(EvaluateResultError::ExceptionError(exception));
        }

        Err(EvaluateResultError::CommandResultError(
            CommandResultError::InvalidResultTypeError(result_value),
        ))
    }

    pub async fn add_preload_script(
        &mut self,
        add_preload_script: AddPreloadScript,
    ) -> Result<PreloadScript, EvaluateResultError> {
        let result_value = self
            .send_command(add_preload_script)
            .await
            .map_err(|err| {
                EvaluateResultError::CommandResultError(CommandResultError::SessionSendError(err))
            })?
            .result;

        let add_result = AddPreloadScriptResult::try_from(result_value.clone()).map_err(|_| {
            EvaluateResultError::CommandResultError(CommandResultError::InvalidResultTypeError(
                result_value,
            ))
        })?;

        Ok(add_result.script)
    }

    pub async fn remove_preload_script(
        &mut self,
        remove_preload_script: RemovePreloadScript,
    ) -> Result<(), EvaluateResultError> {
        self.send_command(remove_preload_script)
            .await
            .map_err(|err| {
                EvaluateResultError::CommandResultError(CommandResultError::SessionSendError(err))
            })?;

        Ok(())
    }

    pub fn get_active_context_id(&self) -> Result<BidiBrowsingContext, ContextIndexError> {
        let contexts = self.browsing_contexts.lock().unwrap();
        match contexts.get(self.active_bc_index) {
            Some(context) => Ok(context.id().clone()),
            None => Err(ContextIndexError {}),
        }
    }

    pub async fn add_event_handler<F, R>(&mut self, events: HashSet<&str>, handler: F) -> String
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        self.session.lock().await.add_event_handler(events, handler)
    }

    pub async fn create_context(
        &mut self,
        context_create: Create,
    ) -> Result<Context, ContextCreationError> {
        let context_type = context_create.params.r#type.clone();
        let response = self.send_command(context_create).await.map_err(|err| {
            ContextCreationError::CommandResultError(CommandResultError::SessionSendError(err))
        })?;

        let result: CreateResult = response.result.clone().try_into().map_err(|_| {
            ContextCreationError::CommandResultError(CommandResultError::InvalidResultTypeError(
                response.result,
            ))
        })?;

        let bc = Context::from_id(result.context, context_type);

        Ok(bc)
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
        R: Future<Output = ()> + Send + 'static,
    {
        let mut builder = AddInterceptBuilder::default();
        builder = builder.phases(phases);
        if let Some(patterns) = url_patterns {
            builder = builder.url_patterns(patterns);
        }
        let add_intercept_command = builder.build().unwrap();

        let result_value = self
            .send_command(add_intercept_command)
            .await
            .map_err(|e| {
                InterceptNetworkError::CommandResultError(CommandResultError::SessionSendError(e))
            })?
            .result;

        let _ = AddInterceptResult::try_from(result_value.clone()).map_err(|_| {
            InterceptNetworkError::CommandResultError(CommandResultError::InvalidResultTypeError(
                result_value,
            ))
        })?;

        let events = HashSet::from_iter(event_names);
        let bidi_event = self
            .session
            .lock()
            .await
            .create_event::<_, _, BidiSession<T>>(events, handler);
        self.session
            .lock()
            .await
            .subscribe_events(bidi_event)
            .await
            .map_err(|e| InterceptNetworkError::CommandResultError(e))?;

        Ok(())
    }

    pub fn on_request<F>(&mut self, handler: F) -> OnRequestBuilder<'_, T, F> {
        OnRequestBuilder::new(self, handler)
    }

    pub fn authenticate(
        &mut self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> AuthenticateBuilder<'_, T> {
        AuthenticateBuilder::new(self, username, password)
    }

    pub async fn screenshot(
        &mut self,
        capture_screenshot: CaptureScreenshot,
        save_path: Option<&str>,
    ) -> Result<String, ScreenshotError> {
        let result_value = self
            .send_command(capture_screenshot)
            .await
            .map_err(|err| {
                ScreenshotError::CommandResultError(
                    CommandResultError::SessionSendError(err),
                )
            })?
            .result;

        let screenshot_result =
            CaptureScreenshotResult::try_from(result_value.clone()).map_err(|_| {
                ScreenshotError::CommandResultError(
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
                        return Err(ScreenshotError::InvalidPath(format!(
                            "Parent directory does not exist: {}",
                            parent.display()
                        )));
                    }
                }
                path_obj.to_path_buf()
            };

            use base64::{Engine as _, engine::general_purpose};
            let decoded = general_purpose::STANDARD
                .decode(&base64_data)
                .map_err(|e| ScreenshotError::Base64DecodeError(e.to_string()))?;

            std::fs::write(&final_path, decoded)
                .map_err(|e| ScreenshotError::FileWriteError(e.to_string()))?;

            Ok(final_path.to_string_lossy().to_string())
        } else {
            Ok(base64_data)
        }
    }

    pub async fn set_timezone_override(
        &mut self,
        set_timezone_override: SetTimezoneOverride,
    ) -> Result<(), EmulationError> {
        self.send_command(set_timezone_override)
            .await
            .map_err(|err| {
                EmulationError::CommandResultError(CommandResultError::SessionSendError(err))
            })?;

        Ok(())
    }

    pub async fn end_session(&mut self) -> Result<(), SessionSendError> {
        self.session.lock().await.end_session().await?;
        Ok(())
    }
}

pub async fn start_bidi_driver(
    driver_config: &impl DriverConfiguration,
    connection_transport_config: &ConnectionTransportConfig,
    session_connection_type: SessionConnectionType,
    capabilities: CapabilitiesRequest,
) -> (
    Arc<TokioMutex<BidiSession<WebsocketConnectionTransport>>>,
    Process,
) {
    let driver_process = Process::create(driver_config.exe_path(), driver_config.flags());
    let mut session =
        BidiSession::<WebsocketConnectionTransport>::ws_new(connection_transport_config).await;
    session
        .initialize(session_connection_type, capabilities)
        .await;
    (Arc::new(TokioMutex::new(session)), driver_process)
}
