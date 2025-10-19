use std::collections::{HashMap, HashSet};
use rustenium_bidi_commands::{CommandData, Event, ResultData, SessionCommand, SessionResult};
use rustenium_bidi_commands::session::commands::{SessionSubscribeMethod, Subscribe, SubscriptionRequest};
use crate::contexts::BrowsingContext;
use crate::error::{CommandResultError, SessionSendError};
use crate::transport::ConnectionTransport;

pub struct BidiEvent {
    pub id: String,
    pub handlers: Vec<Box<dyn FnMut(Event) -> ()>>,
    pub contexts: Vec<String>
}

pub trait EventManagement<'a, T: ConnectionTransport<'a>> {
    async fn send(&self, command_data:  CommandData) -> Result<ResultData, SessionSendError>;

    fn get_bidi_events(&self) -> &mut HashMap<String, BidiEvent>;

    // I don't know what to do with UserContexts yet
    async fn subscribe_events(&self, events: HashSet<String>, handlers: Vec<Box<dyn FnMut(Event) -> ()>>, browsing_contexts: Option<Vec<&BrowsingContext>>, user_contexts: Option<Vec<&str>>) -> Result<Option<ResultData>, CommandResultError> {
        let browsing_context_strings = match browsing_contexts {
            Some(browsing_contexts) => Some(browsing_contexts.iter().map(|browsing_context| browsing_context.context.clone()).collect()),
            None => None
        };
        let bidi_events = self.get_bidi_events();
        let mut events_to_subscribe = Vec::<String>::new();
        for event in events.iter() {
            if let Some(event) = bidi_events.get_mut(event) {
                &mut event.handlers.extend(handlers.iter().clone().collect());
            } else {
                events_to_subscribe.push(event.to_string());
            }
        }
        let subscribe_event_command = CommandData::SessionCommand(SessionCommand::Subscribe(Subscribe {
            method: SessionSubscribeMethod::SessionSubscribe,
            params: SubscriptionRequest {
                events: events_to_subscribe.clone(),
                contexts: browsing_context_strings.clone(),
                user_contexts: None,
            }
        }));
        let event_result = self.send(subscribe_event_command).await;
        match event_result {
            Ok(ResultData::SessionResult(session_result)) => match session_result {
                SessionResult::SubscribeResult(subscribe_result) => {
                    events_to_subscribe.iter().for_each(|&event| {
                        let bidi_event = BidiEvent {
                        id: subscribe_result.subscription.clone(),
                        handlers,
                        contexts: browsing_context_strings.unwrap_or(Vec::new())
                    };
                        bidi_events.insert(event, bidi_event);
                    });
                    Ok(Some(ResultData::SessionResult(SessionResult::SubscribeResult(subscribe_result))))
                },
                _ => Err(CommandResultError::InvalidResultTypeError(ResultData::SessionResult(session_result)))
            },
            Ok(result) => Err(CommandResultError::InvalidResultTypeError(result)),
            Err(e) => Err(CommandResultError::SessionSendError(e)),
        }
    }
}