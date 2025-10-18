mod browser;
mod bidi;

use std::error::Error;
use rustenium_core::{process::Process, transport::ConnectionTransport, Session};

pub use bidi::chrome::ChromeDriver;
use rustenium_core::contexts::BrowsingContext;
use rustenium_core::session::SessionConnectionType;
pub struct Driver<'a, T: ConnectionTransport<'a>>  {
    pub exe_path: &'static str,
    pub flags: Vec<String>,
    pub session: Option<Session<'a, T>>,
    browsing_contexts: Vec<BrowsingContext>,
    driver_process: Option<Process>,
}

impl<'a, T: ConnectionTransport<'a>> Driver<'a, T> {
    async fn new_session(&mut self, connection_type: SessionConnectionType) -> Result<(), Box<dyn Error>> {
        self.session.as_mut().unwrap().create_new_bidi_session(connection_type).await;
        Ok(())
    }
    async fn new_browsing_context(&mut self) -> bool {
        let browsing_context = BrowsingContext::new(self.session.as_mut().unwrap(), None, None, false).await.unwrap();
        self.browsing_contexts.push(browsing_context);
        true
    }
}