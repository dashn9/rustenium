mod bidi;
pub mod chrome;

use rustenium_core::{process::Process, transport::ConnectionTransport, Session};
use std::collections::HashSet;
use std::error::Error;

pub use chrome::ChromeDriver;
use rustenium_bidi_commands::{BrowsingContextEvent, EventData};
use rustenium_core::contexts::BrowsingContext;
use rustenium_core::error::CommandResultError;
use rustenium_core::events::EventManagement;
use rustenium_core::session::SessionConnectionType;
pub struct Driver<'a, T: ConnectionTransport<'a>> {
    pub exe_path: &'static str,
    pub flags: Vec<String>,
    pub session: Option<Session<'a, T>>,
    browsing_contexts: Vec<BrowsingContext>,
    driver_process: Option<Process>,
}

impl<'a, T: ConnectionTransport<'a>> Driver<'a, T>
{
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
        let browsing_context_mut = &mut self.browsing_contexts;
        let result = self
            .session
            .as_mut()
            .unwrap()
            .subscribe_events(
                HashSet::from(["browsingContext.contextCreated"]),
                |event| async {
                    if let EventData::BrowsingContextEvent(BrowsingContextEvent::ContextCreated(
                        context,
                    )) = event.event_data
                    {

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
        self.browsing_contexts.push(browsing_context);
        true
    }
}
