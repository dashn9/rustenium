use rustenium_core::{
    process::Process,
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
    Session,
};
use std::error::Error;

pub use crate::drivers::chrome::ChromeDriver;
use crate::error::{
    ContextCreationListenError, ContextIndexError, EvaluateResultError, FindNodesError,
    OpenUrlError,
};
use rustenium_bidi_commands::browsing_context::types::{
    BrowsingContext as BidiBrowsingContext, CreateType, Locator, ReadinessState,
};
use rustenium_bidi_commands::script::commands::{
    CallFunction, CallFunctionParameters, EvaluateResult, ScriptCallFunctionMethod,
};
use rustenium_bidi_commands::session::commands::SubscribeResult;
use rustenium_bidi_commands::{
    BrowsingContextCommand, BrowsingContextEvent, BrowsingContextResult, CommandData, EventData,
    ResultData, ScriptCommand, ScriptResult, SessionResult,
};
use rustenium_core::contexts::BrowsingContext;
use rustenium_core::events::EventManagement;
use rustenium_core::session::SessionConnectionType;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

use crate::nodes::{Node, NodePosition};
use rustenium_bidi_commands::browsing_context::commands::{
    BrowsingContextLocateNodesMethod, BrowsingContextNavigateMethod, LocateNodes,
    LocateNodesParameters, LocateNodesResult, Navigate, NavigateParameters, NavigateResult,
};
use rustenium_bidi_commands::script::commands::{
    Evaluate, EvaluateParameters, ScriptEvaluateMethod,
};
use rustenium_bidi_commands::script::types::{
    ContextTarget, EvaluateResultSuccess, LocalValue, PrimitiveProtocolValue, RemoteReference,
    RemoteValue, ResultOwnership, SerializationOptions, SharedReference, Target,
};
use rustenium_core::error::{CommandResultError, SessionSendError};
use tokio::io;

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
    ) -> (Session<WebsocketConnectionTransport>, Process) {
        let driver_process = Process::create(driver_config.exe_path(), driver_config.flags());
        let mut session = Session::<T>::ws_new(connection_transport_config).await;
        session
            .create_new_bidi_session(session_connection_type)
            .await;
        (session, driver_process)
    }
}

pub struct BidiDriver<T: ConnectionTransport> {
    pub exe_path: String,
    pub flags: Vec<String>,
    pub session: Session<T>,
    pub active_bc_index: usize,
    pub browsing_contexts: Arc<Mutex<Vec<BrowsingContext>>>,
    pub driver_process: Process,
}

impl<T: ConnectionTransport> BidiDriver<T> {
    pub async fn new_session(
        &mut self,
        connection_type: SessionConnectionType,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub async fn listen_to_context_creation(
        &mut self,
    ) -> Result<Option<SubscribeResult>, ContextCreationListenError> {
        let browsing_contexts = self.browsing_contexts.clone();
        let result = self
            .session
            .subscribe_events(
                HashSet::from(["browsingContext.contextCreated"]),
                move |event| {
                    let bc = browsing_contexts.clone();
                    async move {
                        if let EventData::BrowsingContextEvent(
                            BrowsingContextEvent::ContextCreated(context),
                        ) = event.event_data
                        {
                            bc.lock().unwrap().push(BrowsingContext {
                                r#type: CreateType::Window,
                                context: context.params.context,
                            });
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
        let browsing_context = BrowsingContext::new(&mut self.session, None, None, false)
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
            .session
            .send(CommandData::BrowsingContextCommand(
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
            .session
            .send(CommandData::BrowsingContextCommand(
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

    pub async fn get_node_position(
        &mut self,
        shared_reference: LocalValue,
        locator: &Locator,
    ) -> Result<Option<NodePosition>, EvaluateResultError> {
        let mut script = "";
        if let Locator::CssLocator(locator) = locator {
            script = "function() {
    if (!this) {
        return null;
    }
    const rect = this.getBoundingClientRect();
    const scroll_x = window.pageXOffset || document.documentElement.scrollLeft;
    const scroll_y = window.pageYOffset || document.documentElement.scrollTop;

    return JSON.stringify({
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
        scroll_x: rect.x + scroll_x,
        scroll_y: rect.y + scroll_y
    });
}";
        }
        let result = self
            .call_function(
                script.to_string(),
                false,
                None,
                None,
                None,
                None,
                Some(shared_reference),
                None,
            )
            .await?;
        if let RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(rv_sv)) =
            result.clone().result
        {
            let position: Option<NodePosition> = serde_json::from_str(rv_sv.value.as_str()).ok();
            return Ok(position);
        }
        Ok(None)
    }

    pub async fn get_node_inner_text(
        &mut self,
        shared_reference: LocalValue,
    ) -> Result<String, EvaluateResultError> {
        let script = "function() { return this.innerText || ''; }";

        let result = self
            .call_function(
                script.to_string(),
                false,
                None,
                None,
                None,
                None,
                Some(shared_reference),
                None,
            )
            .await?;

        if let RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(rv_sv)) =
            result.result
        {
            return Ok(rv_sv.value);
        }
        Ok(String::new())
    }

    pub async fn get_node_text_content(
        &mut self,
        shared_reference: LocalValue,
    ) -> Result<String, EvaluateResultError> {
        let script = "function() { return this.textContent || ''; }";

        let result = self
            .call_function(
                script.to_string(),
                false,
                None,
                None,
                None,
                None,
                Some(shared_reference),
                None,
            )
            .await?;

        if let RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(rv_sv)) =
            result.result
        {
            return Ok(rv_sv.value);
        }
        Ok(String::new())
    }

    pub async fn get_node_inner_html(
        &mut self,
        shared_reference: LocalValue,
    ) -> Result<String, EvaluateResultError> {
        let script = "function() { return this.innerHTML || ''; }";

        let result = self
            .call_function(
                script.to_string(),
                false,
                None,
                None,
                None,
                None,
                Some(shared_reference),
                None,
            )
            .await?;

        if let RemoteValue::PrimitiveProtocolValue(PrimitiveProtocolValue::StringValue(rv_sv)) =
            result.result
        {
            return Ok(rv_sv.value);
        }
        Ok(String::new())
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
            .session
            .send(CommandData::ScriptCommand(ScriptCommand::Evaluate(
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
            .session
            .send(CommandData::ScriptCommand(ScriptCommand::CallFunction(
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
            Ok(ResultData::ScriptResult(script_result)) => match script_result {
                ScriptResult::CallFunctionResult(evaluate_result) => match evaluate_result {
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
    fn get_active_context_id(&self) -> Result<String, ContextIndexError> {
        self.get_context_id(self.active_bc_index)
    }
}
