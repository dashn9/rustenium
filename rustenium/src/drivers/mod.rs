mod bidi;
pub mod chrome;

pub use chrome::ChromeDriver;
use rustenium_bidi_commands::browsing_context::types::CreateType;
use rustenium_bidi_commands::session::commands::SubscribeResult;
use rustenium_bidi_commands::{BrowsingContextEvent, EventData};
use rustenium_core::contexts::BrowsingContext;
use rustenium_core::events::EventManagement;
use rustenium_core::session::SessionConnectionType;
use rustenium_core::{process::Process, transport::ConnectionTransport, Session};
use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;
use crate::error::ContextCreationListenError;

pub struct Driver<T: ConnectionTransport> {
    pub exe_path: String,
    pub flags: Vec<String>,
    pub session: Option<Session<T>>,
    browsing_contexts: Arc<Mutex<Vec<BrowsingContext>>>,
    driver_process: Option<Process>,
}

impl<T: ConnectionTransport> Driver<T> {
    async fn new_session(
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
                    _ => {},
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
}
