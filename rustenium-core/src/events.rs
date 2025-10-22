use crate::contexts::BrowsingContext;
use crate::error::{CommandResultError, SessionSendError};
use crate::transport::ConnectionTransport;
use crate::{impl_has_method, impl_has_method_getter};
use rustenium_bidi_commands::session::commands::{
    SessionSubscribeMethod, Subscribe, SubscriptionRequest,
};
use rustenium_bidi_commands::{
    BrowsingContextEvent, CommandData, Event, EventData, InputEvent, LogEvent, NetworkEvent,
    ResultData, ScriptEvent, SessionCommand, SessionResult,
};
use std::collections::HashSet;
use std::future::{Future, IntoFuture};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::vec;

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

type BidiEventHandler =
    Arc<dyn Fn(Event) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync + 'static>;
pub struct BidiEvent {
    pub id: String,
    pub events: Vec<String>,
    pub handler: BidiEventHandler,
}

pub trait EventManagement {
    async fn send_event(
        &mut self,
        command_data: CommandData,
    ) -> Result<ResultData, SessionSendError>;

    fn get_bidi_events(&mut self) -> &mut Arc<Mutex<Vec<BidiEvent>>>;

    fn push_event(&mut self, event: BidiEvent) -> ();

    // I don't know what to do with UserContexts yet
    async fn subscribe_events<F, R>(
        &mut self,
        events: HashSet<&str>,
        handler: F,
        browsing_contexts: Option<Vec<&BrowsingContext>>,
        user_contexts: Option<Vec<&str>>,
    ) -> Result<Option<ResultData>, CommandResultError>
    where
        F: Fn(Event) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        let browsing_context_strings = match &browsing_contexts {
            Some(browsing_contexts) => browsing_contexts
                .iter()
                .map(|browsing_context| browsing_context.context.clone())
                .collect(),
            None => vec![],
        };
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
                    let bidi_event = BidiEvent {
                        id: subscribe_result.subscription.clone(),
                        events: events
                            .clone()
                            .into_iter()
                            .map(|event| event.to_string())
                            .collect(),
                        handler: Arc::new(move |event| Box::pin(handler(event))),
                    };
                    self.push_event(bidi_event);
                    Ok(Some(ResultData::SessionResult(
                        SessionResult::SubscribeResult(subscribe_result),
                    )))
                }
                _ => Err(CommandResultError::InvalidResultTypeError(
                    ResultData::SessionResult(session_result),
                )),
            },
            Ok(result) => Err(CommandResultError::InvalidResultTypeError(result)),
            Err(e) => Err(CommandResultError::SessionSendError(e)),
        }
    }

    async fn dispatch_event(&mut self, event: Event) {
        let bidi_events = self.get_bidi_events().clone();
        tokio::task::spawn_blocking(move || {
            let event_method = &event.event_data.get_method();
            // Manually handling context check was abandoned, too much variation/nesting of context
            for bidi_event in bidi_events.lock().unwrap().iter() {
                if bidi_event.events.contains(&event_method) {
                    let ch = Arc::clone(&bidi_event.handler);
                    let ce = event.clone();
                    tokio::spawn(async move {
                        ch(ce).await;
                    });
                }
            }
        });
    }
}
