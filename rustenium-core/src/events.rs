use crate::contexts::BrowsingContext;
use crate::error::{CommandResultError, SessionSendError};
use crate::transport::ConnectionTransport;
use rustenium_bidi_commands::session::commands::{
    SessionSubscribeMethod, Subscribe, SubscriptionRequest,
};
use rustenium_bidi_commands::{CommandData, Event, EventData, ResultData, SessionCommand, SessionResult};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::vec;

pub struct BidiEvent {
    pub id: String,
    pub events: Vec<String>,
    pub handler: Box<dyn Fn(Event) -> ()>,
}
pub trait EventManagement<'a, T: ConnectionTransport<'a>> {
    async fn send(&self, command_data: CommandData) -> Result<ResultData, SessionSendError>;

    fn get_bidi_events(&self) -> &mut Vec<BidiEvent>;

    // I don't know what to do with UserContexts yet
    async fn subscribe_events<F>(
        &self,
        events: HashSet<String>,
        handler: F,
        browsing_contexts: Option<Vec<&BrowsingContext>>,
        user_contexts: Option<Vec<&str>>,
    ) -> Result<Option<ResultData>, CommandResultError>
    where
        F: Fn(Event) -> () + 'static,
    {
        let browsing_context_strings = match &browsing_contexts {
            Some(browsing_contexts) => browsing_contexts
                .iter()
                .map(|browsing_context| browsing_context.context.clone())
                .collect(),
            None => vec![],
        };
        let bidi_events = self.get_bidi_events();
        let subscribe_event_command =
            CommandData::SessionCommand(SessionCommand::Subscribe(Subscribe {
                method: SessionSubscribeMethod::SessionSubscribe,
                params: SubscriptionRequest {
                    events: events.clone().into_iter().collect(),
                    contexts: if browsing_contexts.is_none() {
                        None
                    } else {
                        Some(browsing_context_strings.clone())
                    },
                    user_contexts: None,
                },
            }));
        let event_result = self.send(subscribe_event_command).await;
        match event_result {
            Ok(ResultData::SessionResult(session_result)) => match session_result {
                SessionResult::SubscribeResult(subscribe_result) => {
                    let bidi_event = BidiEvent {
                        id: subscribe_result.subscription.clone(),
                        events: events.into_iter().collect(),
                        handler: Box::new(handler),
                    };
                    bidi_events.push(bidi_event);
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

    async fn dispatch_event(&self, event: Event) {
        let bidi_events = self.get_bidi_events();
        for bidi_event in bidi_events {
            if
        }
    }
}
