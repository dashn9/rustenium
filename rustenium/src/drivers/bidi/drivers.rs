use std::error::Error;
use rustenium_core::{
    process::Process,
    transport::{ConnectionTransport, ConnectionTransportConfig, WebsocketConnectionTransport},
    Session,
};


pub use crate::drivers::chrome::ChromeDriver;
use rustenium_bidi_commands::browsing_context::types::{CreateType, ReadinessState, BrowsingContext as BidiBrowsingContext};
use rustenium_bidi_commands::session::commands::SubscribeResult;
use rustenium_bidi_commands::{BrowsingContextEvent, EventData};
use rustenium_core::contexts::BrowsingContext;
use rustenium_core::events::EventManagement;
use rustenium_core::session::SessionConnectionType;
use std::collections::{HashSet};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;
use crate::error::{ContextCreationListenError, ContextIndexError};

use tokio::io;

fn is_connection_refused(e: &reqwest::Error) -> bool {
    if let Some(io_err) = e.source().and_then(|s| s.downcast_ref::<io::Error>()) {
        return io_err.kind() == io::ErrorKind::ConnectionRefused;
    }
    false
}

pub trait BidiDrive<T: ConnectionTransport> {
    fn exe_path(&self) -> &str;
    fn flags(&self) -> Vec<String>;
    async fn start(&self, connection_transport_config: &ConnectionTransportConfig) -> (Session<WebsocketConnectionTransport>, Process) {
        let driver_process = Process::create(self.exe_path(), self.flags());
        let session =
            Session::<T>::ws_new(connection_transport_config)
                .await;
        (session, driver_process)
    }
}


pub struct BidiDriver<T: ConnectionTransport> {
    pub exe_path: String,
    pub flags: Vec<String>,
    pub session: Option<Session<T>>,
    pub active_bc_index: usize,
    pub browsing_contexts: Arc<Mutex<Vec<BrowsingContext>>>,
    pub driver_process: Option<Process>,
}

impl<T: ConnectionTransport> BidiDriver<T> {
    pub async fn new_session(
        &mut self,
        connection_type: SessionConnectionType,
    ) -> Result<(), Box<dyn Error>> {
        self.session
            .as_mut()
            .unwrap()
            .create_new_bidi_session(connection_type)
            .await;
        Ok(())
    }

    pub async fn listen_to_context_creation(
        &mut self,
    ) -> Result<Option<SubscribeResult>, ContextCreationListenError> {
        let session = self.session.as_mut().unwrap();
        let browsing_contexts = self.browsing_contexts.clone();
        let result = session
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
                match session
                    .unsubscribe_events_by_ids(vec![result.subscription.clone()]).await {
                    Err(error) => return Err(ContextCreationListenError::CommandResultError(error)),
                    Ok(None) => return Ok(None),
                    _ => {}
                }
            };
        } else {}
        match result {
            Err(error) => Err(ContextCreationListenError::CommandResultError(error)),
            Ok(result) => Ok(result),
        }
    }

    async fn new_browsing_context(&mut self) -> bool {
        let browsing_context =
            BrowsingContext::new(self.session.as_mut().unwrap(), None, None, false)
                .await
                .unwrap();
        self.browsing_contexts
            .lock()
            .unwrap()
            .push(browsing_context);
        true
    }

    async fn get

    fn open_url(&self, url: String, wait: Option<ReadinessState>, context_id: Option<BidiBrowsingContext>) -> Result<> {
        let context_id = context_id.unwrap_or(self.get_active_context_id()?);
        let wait = wait.unwrap_or(ReadinessState::Complete);
        let result = self.session.
    }

    fn get_context_id(&self, context_index: usize) -> Result<String, ContextIndexError> {
        match self.browsing_contexts.lock().unwrap().get(self.active_bc_index) {
            Some(context) => {
                Ok(context.get_context_id())
            }
            None => {
                Err(ContextIndexError {})
            }
        }
    }
    fn get_active_context_id(&self) -> Result<String, ContextIndexError> {
        self.get_context_id(self.active_bc_index)
    }
}
