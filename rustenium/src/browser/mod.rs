mod bidi;
pub mod chrome;

pub use chrome::ChromeDriver;
use rustenium_bidi_commands::browsing_context::types::CreateType;
use rustenium_bidi_commands::{BrowsingContextEvent, EventData};
use rustenium_core::contexts::BrowsingContext;
use rustenium_core::error::CommandResultError;
use rustenium_core::events::EventManagement;
use rustenium_core::session::SessionConnectionType;
use rustenium_core::{process::Process, transport::ConnectionTransport, Session};
use std::collections::HashSet;
use std::error::Error;
use std::sync::{Arc, Mutex};

pub struct Driver<'a, T: ConnectionTransport<'a>> {
    pub exe_path: &'static str,
    pub flags: Vec<String>,
    pub session: Option<Session<'a, T>>,
    browsing_contexts: Arc<Mutex<Vec<BrowsingContext>>>,
    driver_process: Option<Process>,
}

impl<'a, T: ConnectionTransport<'a>> Driver<'a, T> {
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

    pub async fn listen_to_context_creation(&mut self) -> Result<(), CommandResultError> {
        let session = self.session.as_mut().unwrap();
        let browsing_contexts = self.browsing_contexts.clone();
        let result = self
            .session
            .as_mut()
            .unwrap()
            .subscribe_events(
                HashSet::from(["browsingContext.contextCreated"]),
                move |event| {
                    let bc = browsing_contexts.clone();
                    async move {
                        if let EventData::BrowsingContextEvent(BrowsingContextEvent::ContextCreated(
                                                                   context,
                                                               )) = event.event_data
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

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
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
