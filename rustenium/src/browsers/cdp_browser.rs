use rustenium_cdp_definitions::Command;
use rustenium_cdp_definitions::base::CommandResponse;
use rustenium_cdp_definitions::browser_protocol::page::commands::Navigate;
use rustenium_cdp_definitions::browser_protocol::page::results::NavigateResult;
use rustenium_cdp_definitions::browser_protocol::page::types::{TransitionType, ReferrerPolicy};
use rustenium_core::error::CdpSessionSendError;
use rustenium_core::transport::WebsocketConnectionTransport;
use crate::conduit::cdp::adapter::CdpAdapter;
use crate::error::cdp::NavigateError;

#[derive(Debug, Clone, Default)]
pub struct NavigateOptions {
    pub referrer: Option<String>,
    pub transition_type: Option<TransitionType>,
    pub referrer_policy: Option<ReferrerPolicy>,
}

#[derive(Default, Clone)]
pub struct NavigateOptionsBuilder {
    referrer: Option<String>,
    transition_type: Option<TransitionType>,
    referrer_policy: Option<ReferrerPolicy>,
}

impl NavigateOptionsBuilder {
    pub fn referrer(mut self, v: impl Into<String>) -> Self { self.referrer = Some(v.into()); self }
    pub fn transition_type(mut self, v: impl Into<TransitionType>) -> Self { self.transition_type = Some(v.into()); self }
    pub fn referrer_policy(mut self, v: impl Into<ReferrerPolicy>) -> Self { self.referrer_policy = Some(v.into()); self }
    pub fn build(self) -> NavigateOptions {
        NavigateOptions {
            referrer: self.referrer,
            transition_type: self.transition_type,
            referrer_policy: self.referrer_policy,
        }
    }
}

pub trait CdpBrowser {
    fn adapter(&self) -> &CdpAdapter<WebsocketConnectionTransport>;

    fn adapter_mut(&mut self) -> &mut CdpAdapter<WebsocketConnectionTransport>;

    fn navigate(
        &mut self,
        url: impl Into<String>,
    ) -> impl Future<Output = Result<NavigateResult, NavigateError>> {
        self.navigate_with_options(url, NavigateOptions::default())
    }

    fn navigate_with_options(
        &mut self,
        url: impl Into<String>,
        options: NavigateOptions,
    ) -> impl Future<Output = Result<NavigateResult, NavigateError>> {
        let mut builder = Navigate::builder().url(url);
        if let Some(referrer) = options.referrer {
            builder = builder.referrer(referrer);
        }
        if let Some(transition_type) = options.transition_type {
            builder = builder.transition_type(transition_type);
        }
        if let Some(referrer_policy) = options.referrer_policy {
            builder = builder.referrer_policy(referrer_policy);
        }
        let command = builder.build().unwrap();
        let adapter = self.adapter_mut();
        async move {
            adapter.navigate(command).await
        }
    }

    fn send_command(
        &mut self,
        command: Command,
    ) -> impl Future<Output = Result<CommandResponse, CdpSessionSendError>> + Send {
        let adapter = self.adapter_mut();
        async move { adapter.send_command(command).await }
    }
}
