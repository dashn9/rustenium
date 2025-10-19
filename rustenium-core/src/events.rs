use rustenium_bidi_commands::{CommandData, Event, ResultData, SessionCommand, SessionResult};
use rustenium_bidi_commands::session::commands::{SessionSubscribeMethod, Subscribe, SubscriptionRequest};
use crate::contexts::BrowsingContext;
use crate::error::{CommandResultError, SessionSendError};
use crate::transport::ConnectionTransport;

pub struct BidiEvent {
    pub event: String,
    pub is_subscribed: bool,
    pub handlers: Vec<Box<dyn FnMut(Event) -> ()>>
}

pub trait EventManagement<'a, T: ConnectionTransport<'a>> {
    async fn send(&self, command_data:  CommandData) -> Result<ResultData, SessionSendError>;

    fn get_bidi_events(&self) -> Vec<BidiEvent>;

    // I don't know what to do with UserContexts yet
    async fn subscribe_events(&self, events: Vec<String>, browsing_contexts: Option<Vec<&BrowsingContext>>, user_contexts: Option<Vec<&str>>) -> Result<ResultData, CommandResultError> {

        let subscribe_event_command = CommandData::SessionCommand(SessionCommand::Subscribe(Subscribe {
            method: SessionSubscribeMethod::SessionSubscribe,
            params: SubscriptionRequest {
                events,
                contexts: match browsing_contexts {
                    Some(browsing_contexts) => Some(browsing_contexts.iter().map(|browsing_context| browsing_context.context.clone()).collect()),
                    None => None
                },
                user_contexts: None,
            }
        }));
        let event_result = self.send(subscribe_event_command).await;
        match event_result {
            Ok(ResultData::SessionResult(session_result)) => match session_result {
                SessionResult::SubscribeResult(subscribe_result) => {
                    Ok(ResultData::SessionResult(SessionResult::SubscribeResult(subscribe_result)))
                },
                _ => Err(CommandResultError::InvalidResultTypeError(ResultData::SessionResult(session_result)))
            },
            Ok(result) => Err(CommandResultError::InvalidResultTypeError(result)),
            Err(e) => Err(CommandResultError::SessionSendError(e)),
        }
    }
}