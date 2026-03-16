use crate::error::{CommandResultError, SessionSendError};
use rustenium_bidi_definitions::Command;
use rustenium_bidi_definitions::Event;
use rustenium_bidi_definitions::base::CommandResponse;
use rustenium_bidi_definitions::base::EventResponse;
use rustenium_bidi_definitions::session::command_builders::SubscribeBuilder;
use rustenium_bidi_definitions::session::command_builders::UnsubscribeBuilder;
use rustenium_bidi_definitions::session::results::SubscribeResult;
use rustenium_bidi_definitions::session::results::UnsubscribeResult;
use rustenium_bidi_definitions::session::type_builders::UnsubscribeByAttributesRequestBuilder;
use rustenium_bidi_definitions::session::type_builders::UnsubscribeByIdRequestBuilder;
use rustenium_bidi_definitions::session::types::Subscription;
use rustenium_bidi_definitions::session::types::UnsubscribeParameters;
use std::collections::HashSet;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex as StdMutex;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tokio::task::JoinHandle;

type BidiEventHandler = Arc<
    Mutex<dyn FnMut(Event) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync + 'static>,
>;
pub struct BidiEvent {
    pub id: String,
    pub events: Vec<String>,
    pub handler: BidiEventHandler,
    browsing_contexts: Option<Vec<String>>,
    user_contexts: Option<Vec<String>>,
}

impl BidiEvent {
    pub fn add_browsing_context(&mut self, browsing_context: String) {
        self.browsing_contexts
            .get_or_insert_with(Vec::new)
            .push(browsing_context);
    }

    pub fn add_user_context(&mut self, user_context: String) {
        self.user_contexts
            .get_or_insert_with(Vec::new)
            .push(user_context);
    }
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

pub trait BidiEventManagement {
    fn send_event(
        &mut self,
        command: impl Into<Command>,
    ) -> impl Future<Output = Result<CommandResponse, SessionSendError>>;

    fn get_events(&mut self) -> &mut Arc<StdMutex<Vec<BidiEvent>>>;

    fn push_event(&mut self, event: BidiEvent) -> ();

    fn create_event<F, R, T: BidiEventManagement>(
        &mut self,
        events: HashSet<&str>,
        mut handler: F,
    ) -> BidiEvent
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        let temp_id = format!(
            "temp_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        BidiEvent {
            id: temp_id.clone(),
            events: events
                .clone()
                .into_iter()
                .map(|event| event.to_string())
                .collect(),
            handler: Arc::new(Mutex::new(move |event| {
                Box::pin(handler(event)) as Pin<Box<dyn Future<Output = ()> + Send>>
            })),
            browsing_contexts: None,
            user_contexts: None,
        }
    }
    // I don't know what to do with UserContexts yet
    fn subscribe_events(
        &mut self,
        mut bidi_event: BidiEvent,
    ) -> impl Future<Output = Result<Option<SubscribeResult>, CommandResultError>> {
        async move {
            let mut subscribe_event_command_builder =
                SubscribeBuilder::default().events(bidi_event.events.clone());

            if let Some(browsing_contexts) = bidi_event.browsing_contexts.clone() {
                subscribe_event_command_builder =
                    subscribe_event_command_builder.contexts(browsing_contexts);
            }

            if let Some(user_contexts) = bidi_event.user_contexts.clone() {
                subscribe_event_command_builder =
                    subscribe_event_command_builder.contexts(user_contexts);
            }

            let bidi_event_id = bidi_event.id.to_owned();
            // Optimistically push event before sending to avoid race condition
            self.push_event(bidi_event);
            let event_response = self
                .send_event(subscribe_event_command_builder.build().unwrap())
                .await;
            match event_response {
                Ok(response) => {
                    let mut bidi_events = self.get_events().lock().unwrap();
                    let subscribe_result: SubscribeResult =
                        response.result.clone().try_into().map_err(|_| {
                            // Remove on failure
                            bidi_events.retain(|e| e.id != bidi_event_id);
                            CommandResultError::InvalidResultTypeError(response.result)
                        })?;
                    bidi_events
                        .iter_mut()
                        .filter(|e| e.id == bidi_event_id)
                        .for_each(|e| e.id = subscribe_result.subscription.clone().into());

                    Ok(Some(subscribe_result))
                }
                Err(e) => {
                    // Remove on failure
                    let mut bidi_events = self.get_events().lock().unwrap();
                    bidi_events.retain(|e| e.id != bidi_event_id);
                    Err(CommandResultError::SessionSendError(e))
                }
            }
        }
    }

    /// Add an event handler without sending a subscription command
    /// Returns the handler ID (either provided or generated)
    fn add_event_handler<F, R>(&mut self, events: HashSet<&str>, mut handler: F) -> String
    where
        F: FnMut(Event) -> R + Send + Sync + 'static,
        R: Future<Output = ()> + Send + 'static,
    {
        let id = format!(
            "handler_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );

        let bidi_event = BidiEvent {
            id: id.clone(),
            events: events.into_iter().map(|event| event.to_string()).collect(),
            handler: Arc::new(Mutex::new(move |event| {
                Box::pin(handler(event)) as Pin<Box<dyn Future<Output = ()> + Send>>
            })),
            browsing_contexts: None,
            user_contexts: None,
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
            let unsubscribe_command = UnsubscribeBuilder::default()
                .unsubscribe_parameters(UnsubscribeParameters::UnsubscribeByAttributesRequest(
                    UnsubscribeByAttributesRequestBuilder::default()
                        .events(events.clone().into_iter())
                        .build()
                        .unwrap(),
                ))
                .build()
                .unwrap();

            let event_result = self.send_event(unsubscribe_command).await;
            match event_result {
                Ok(unsubscribe_response) => {
                    let unsubscribe_result: UnsubscribeResult = unsubscribe_response
                        .result
                        .clone()
                        .try_into()
                        .map_err(|_| {
                            CommandResultError::InvalidResultTypeError(unsubscribe_response.result)
                        })?;
                    // Remove the event names from BidiEvents and clean up empty ones
                    let mut bidi_events = self.get_events().lock().unwrap();

                    // First, remove matching event names from each BidiEvent
                    for bidi_event in bidi_events.iter_mut() {
                        bidi_event.events.retain(|e| !events.contains(e.as_str()));
                    }

                    // Then remove any BidiEvents that have no events left
                    bidi_events.retain(|bidi_event| !bidi_event.events.is_empty());

                    Ok(Some(unsubscribe_result))
                }
                Err(e) => Err(CommandResultError::SessionSendError(e)),
            }
        }
    }

    /// Unsubscribe from events by subscription IDs
    fn unsubscribe_events_by_ids(
        &mut self,
        subscription_ids: Vec<Subscription>,
    ) -> impl Future<Output = Result<UnsubscribeResult, CommandResultError>> {
        async move {
            let unsubscribe_command = UnsubscribeBuilder::default()
                .unsubscribe_parameters(UnsubscribeParameters::UnsubscribeByIdRequest(
                    UnsubscribeByIdRequestBuilder::default()
                        .subscriptions(subscription_ids.clone())
                        .build()
                        .unwrap(),
                ))
                .build()
                .unwrap();

            let event_result = self.send_event(unsubscribe_command).await;
            match event_result {
                Ok(response) => {
                    let unsubscribe_result: UnsubscribeResult =
                        response.result.clone().try_into().map_err(|_| {
                            CommandResultError::InvalidResultTypeError(response.result)
                        })?;
                    // Remove the subscriptions from our local tracking
                    let mut bidi_events = self.get_events().lock().unwrap();
                    bidi_events.retain(|bidi_event| {
                        !subscription_ids.contains(&bidi_event.id.clone().into())
                    });
                    Ok(unsubscribe_result)
                }
                Err(e) => Err(CommandResultError::SessionSendError(e)),
            }
        }
    }

    fn event_dispatch(
        &mut self,
    ) -> impl Future<Output = (JoinHandle<()>, UnboundedSender<EventResponse>)> {
        async move {
            let (tx, mut rx) = unbounded_channel::<EventResponse>();
            let bidi_events = self.get_events().clone();
            (
                tokio::spawn(async move {
                    while let Some(event) = rx.recv().await {
                        let event: Event = event.event_data.try_into().unwrap();
                        let event_method = event.identifier().to_string();
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
