use rustenium_bidi_commands::{CommandData, ResultData};
use crate::contexts::BrowsingContext;
use crate::Session;
use crate::session::SessionSendError;
use crate::transport::ConnectionTransport;

pub struct BidiEvent {
    pub event: String,
}

pub trait EventManager<'a, T: ConnectionTransport<'a>> {
    fn send(&self, command_data:  CommandData) -> Result<ResultData, SessionSendError>;

    fn get_bidi_events(&self) -> Vec<BidiEvent>;

    // I don't know what to do with UserContexts yet
    async fn subscribe_events(&self, events: Vec<String>, contexts: Option<Vec<&BrowsingContext>>, user_contexts: Option<Vec<&str>>) {
        event_result = self.send
    }
}