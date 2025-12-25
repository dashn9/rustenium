use crate::error::{CommandResultError, SessionSendError};
use crate::{impl_has_method, impl_has_method_getter};
use rustenium_bidi_commands::session::commands::{
    SessionSubscribeMethod, SessionUnsubscribeMethod, Subscribe, SubscribeResult,
    SubscriptionRequest, Unsubscribe, UnsubscribeParameters, UnsubscribeResult,
};
use rustenium_bidi_commands::session::types::{
    Subscription, UnsubscribeByAttributesRequest, UnsubscribeByIDRequest,
};
use rustenium_bidi_commands::{
    BrowsingContextEvent, CommandData, Event, EventData, InputEvent, LogEvent, NetworkEvent,
    ResultData, ScriptEvent, SessionCommand, SessionResult,
};
use std::collections::HashSet;
use std::future::{Future};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use std::{fmt, vec};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;

trait HasMethod {
    fn get_method(&self) -> String;
}

trait HasMethodGetter {
    fn get_method(&self) -> String;
}

// Some events do not have context, need to specify to macro which has and which do not
impl_has_method_getter!(
    EventData,
    [
        BrowsingContextEvent,
        ScriptEvent,
        NetworkEvent,
        LogEvent,
        InputEvent
    ]
);

impl_has_method!(
    BrowsingContextEvent,
    [
        ContextCreated,
        ContextDestroyed,
        DomContentLoaded,
        DownloadEnd,
        DownloadWillBegin,
        FragmentNavigated,
        HistoryUpdated,
        Load,
        NavigationAborted,
        NavigationCommitted,
        NavigationFailed,
        NavigationStarted,
        UserPromptClosed,
        UserPromptOpened
    ]
);

impl_has_method!(InputEvent, [FileDialogOpened]);

impl_has_method!(LogEvent, [EntryAdded]);

impl_has_method!(
    NetworkEvent,
    [
        AuthRequired,
        BeforeRequestSent,
        FetchError,
        ResponseCompleted,
        ResponseStarted
    ]
);
impl_has_method!(ScriptEvent, [Message, RealmCreated, RealmDestroyed]);

type BidiEventHandler = Arc<
    Mutex<dyn FnMut(Event) -> Pin<Box<dyn Future<Output=()> + Send>> + Send + Sync + 'static>,
>;
pub struct BidiEvent {
    pub id: String,
    pub events: Vec<String>,
    pub handler: BidiEventHandler,
}

impl fmt::Debug for BidiEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BidiEvent")
            .field("id", &self.id)
            .field("events", &self.events)
            .field("handler", &"<BidiEventHandler>")
            .finish()
    }
}

pub trait EventManagement {
    fn send_event(
        &mut self,
        command_data: CommandData,
    ) -> impl Future<Output = Result<ResultData, SessionSendError>>;

    fn get_bidi_events(&mut self) -> &mut Arc<StdMutex<Vec<BidiEvent>>>;

    fn push_event(&mut self, event: BidiEvent) -> ();

    // I don't know what to do with UserContexts yet
    fn subscribe_events<F, R>(
        &mut self,
        events: HashSet<&str>,
        mut handler: F,
        browsing_contexts: Option<Vec<String>>,
        _user_contexts: Option<Vec<&str>>,
    ) -> impl Future<Output = Result<Option<SubscribeResult>, CommandResultError>>
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output=()> + Send + 'static,
    {
        async move {
        let browsing_context_strings = match &browsing_contexts {
            Some(browsing_contexts) => browsing_contexts.clone(),
            None => vec![],
        };

        // Optimistically push event before sending to avoid race condition
        let temp_id = format!("temp_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        let bidi_event = BidiEvent {
            id: temp_id.clone(),
            events: events
                .clone()
                .into_iter()
                .map(|event| event.to_string())
                .collect(),
            handler: Arc::new(Mutex::new(move |event| Box::pin(handler(event)) as Pin<Box<dyn Future<Output=()> + Send>>)),
        };
        self.push_event(bidi_event);

        let subscribe_event_command =
            CommandData::SessionCommand(SessionCommand::Subscribe(Subscribe {
                method: SessionSubscribeMethod::SessionSubscribe,
                params: SubscriptionRequest {
                    events: events
                        .clone()
                        .into_iter()
                        .map(|event| event.to_string())
                        .collect(),
                    contexts: if browsing_contexts.is_none() {
                        None
                    } else {
                        Some(browsing_context_strings.clone())
                    },
                    user_contexts: None,
                },
            }));
        let event_result = self.send_event(subscribe_event_command).await;
        match event_result {
            Ok(ResultData::SessionResult(session_result)) => match session_result {
                SessionResult::SubscribeResult(subscribe_result) => {
                    // Update temp ID with actual subscription ID
                    let mut bidi_events = self.get_bidi_events().lock().unwrap();
                    if let Some(event) = bidi_events.iter_mut().find(|e| e.id == temp_id) {
                        event.id = subscribe_result.subscription.clone();
                    }
                    Ok(Some(subscribe_result))
                }
                _ => {
                    // Remove on failure
                    let mut bidi_events = self.get_bidi_events().lock().unwrap();
                    bidi_events.retain(|e| e.id != temp_id);
                    Err(CommandResultError::InvalidResultTypeError(
                        ResultData::SessionResult(session_result),
                    ))
                }
            },
            Ok(result) => {
                // Remove on failure
                let mut bidi_events = self.get_bidi_events().lock().unwrap();
                bidi_events.retain(|e| e.id != temp_id);
                Err(CommandResultError::InvalidResultTypeError(result))
            }
            Err(e) => {
                // Remove on failure
                let mut bidi_events = self.get_bidi_events().lock().unwrap();
                bidi_events.retain(|e| e.id != temp_id);
                Err(CommandResultError::SessionSendError(e))
            }
        }
        }
    }

    /// Add an event handler without sending a subscription command
    /// Returns the handler ID (either provided or generated)
    fn add_event_handler<F, R>(
        &mut self,
        events: HashSet<&str>,
        mut handler: F,
        handler_id: Option<String>,
    ) -> String
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output=()> + Send + 'static,
    {
        let id = handler_id.unwrap_or_else(|| {
            format!("handler_{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos())
        });

        let bidi_event = BidiEvent {
            id: id.clone(),
            events: events
                .into_iter()
                .map(|event| event.to_string())
                .collect(),
            handler: Arc::new(Mutex::new(move |event| Box::pin(handler(event)) as Pin<Box<dyn Future<Output=()> + Send>>)),
        };
        self.push_event(bidi_event);

        id
    }

    /// Unsubscribe from events by event names
    fn unsubscribe_events_by_names(
        &mut self,
        events: HashSet<&str>,
    ) -> impl Future<Output = Result<Option<UnsubscribeResult>, CommandResultError>> {
        async move {
        let unsubscribe_command =
            CommandData::SessionCommand(SessionCommand::Unsubscribe(Unsubscribe {
                method: SessionUnsubscribeMethod::SessionUnsubscribe,
                params: UnsubscribeParameters::UnsubscribeByAttributesRequest(
                    UnsubscribeByAttributesRequest {
                        events: events
                            .clone()
                            .into_iter()
                            .map(|event| event.to_string())
                            .collect(),
                    },
                ),
            }));

        let event_result = self.send_event(unsubscribe_command).await;
        match event_result {
            Ok(ResultData::SessionResult(session_result)) => match session_result {
                SessionResult::UnsubscribeResult(unsubscribe_result) => {
                    // Remove the event names from BidiEvents and clean up empty ones
                    let mut bidi_events = self.get_bidi_events().lock().unwrap();

                    // First, remove matching event names from each BidiEvent
                    for bidi_event in bidi_events.iter_mut() {
                        bidi_event.events.retain(|e| !events.contains(e.as_str()));
                    }

                    // Then remove any BidiEvents that have no events left
                    bidi_events.retain(|bidi_event| !bidi_event.events.is_empty());

                    Ok(Some(unsubscribe_result))
                }
                _ => Err(CommandResultError::InvalidResultTypeError(
                    ResultData::SessionResult(session_result),
                )),
            },
            Ok(result) => Err(CommandResultError::InvalidResultTypeError(result)),
            Err(e) => Err(CommandResultError::SessionSendError(e)),
        }
        }
    }

    /// Unsubscribe from events by subscription IDs
    fn unsubscribe_events_by_ids(
        &mut self,
        subscription_ids: Vec<Subscription>,
    ) -> impl Future<Output = Result<Option<UnsubscribeResult>, CommandResultError>> {
        async move {
        let unsubscribe_command =
            CommandData::SessionCommand(SessionCommand::Unsubscribe(Unsubscribe {
                method: SessionUnsubscribeMethod::SessionUnsubscribe,
                params: UnsubscribeParameters::UnsubscribeByIDRequest(UnsubscribeByIDRequest {
                    subscriptions: subscription_ids.clone(),
                }),
            }));

        let event_result = self.send_event(unsubscribe_command).await;
        match event_result {
            Ok(ResultData::EmptyResult(empty_result)) => {
                // Remove the subscriptions from our local tracking
                let mut bidi_events = self.get_bidi_events().lock().unwrap();
                bidi_events.retain(| bidi_event | !subscription_ids.contains(&bidi_event.id));
                Ok(Some(empty_result))
            },
            Ok(result) => Err(CommandResultError::InvalidResultTypeError(result)),
            Err(e) => Err(CommandResultError::SessionSendError(e)),
        }
        }
    }

    fn event_dispatch(&mut self) -> impl Future<Output = (JoinHandle<()>, UnboundedSender<Event>)> {
        async move {
        let (tx, mut rx) = unbounded_channel::<Event>();
        let bidi_events = self.get_bidi_events().clone();
        (
            tokio::spawn(async move {
                while let Some(event) = rx.recv().await {
                    let event_method = event.event_data.get_method().trim_matches('"').to_string();
                    // Manually handling context check was abandoned, too much variation/nesting of context
                    for bidi_event in bidi_events.lock().unwrap().iter() {
                        if bidi_event.events.contains(&event_method) {
                            let ch = Arc::clone(&bidi_event.handler);
                            let ce = event.clone();
                            tokio::spawn(async move {
                                (ch.lock().await)(ce).await;
                            });
                        }
                    }
                }
            }),
            tx,
        )
        }
    }
}
