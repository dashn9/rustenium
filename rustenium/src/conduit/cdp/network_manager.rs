//! Request interception over the CDP `Fetch` domain.
//!
//! Ported from Puppeteer's `NetworkManager`, which owns the pieces that cannot
//! live on an individual request: the merged `Fetch.enable` configuration, the
//! handler list, and the authentication bookkeeping.
//!
//! Three things here exist because the protocol punishes their absence:
//!
//! * **`Fetch.enable` replaces, it does not merge.** A second call with new
//!   patterns silently discards the first call's, and clears
//!   `handleAuthRequests` with it — so registering a URL filter after
//!   [`NetworkManager::authenticate`] would turn authentication off. Puppeteer
//!   sidesteps this by always sending `[{urlPattern: '*'}]`; supporting real
//!   patterns means merging them here instead.
//! * **A pause that is never answered blocks the page forever.** Enabling
//!   `handleAuthRequests` also pauses every ordinary request, so credentials on
//!   their own would deadlock the browser without an auto-continue behind them.
//! * **`Fetch.authRequired` repeats on rejected credentials.** Answering with
//!   the same wrong password each time loops until the request times out, so a
//!   second challenge for one request is cancelled rather than retried.

use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex as StdMutex};

use rustenium_cdp_definitions::Command;
use rustenium_cdp_definitions::base::EventResponse;
use rustenium_cdp_definitions::browser_protocol::fetch::commands::{Disable, Enable};
use rustenium_cdp_definitions::browser_protocol::fetch::events::{AuthRequired, RequestPaused};
use rustenium_cdp_definitions::browser_protocol::fetch::types::{
    AuthChallengeResponseResponse, RequestPattern, RequestStage,
};
use rustenium_cdp_definitions::browser_protocol::network::types::ResourceType;
use rustenium_core::error::CdpCommandResultError;
use rustenium_core::session::CdpSession;
use rustenium_core::transport::ConnectionTransport;
use rustenium_core::{CdpEventManagement, CdpHttpRequest, HttpRequest};
use tokio::sync::Mutex as TokioMutex;

use crate::conduit::cdp::target_manager::TargetManager;
use crate::error::cdp::InterceptNetworkError;

type RequestHandler<T> =
    Arc<dyn Fn(Arc<CdpHttpRequest<T>>) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

#[derive(Debug, Clone, Default)]
pub struct RequestInterceptOptions {
    /// Which requests to pause. `None` pauses every request.
    pub patterns: Option<Vec<RequestPattern>>,
    /// Also pause once the response headers have arrived, the only stage at
    /// which the body can be read or the response replaced.
    pub intercept_response: bool,
    /// Intercept in every target rather than only the page — out-of-process
    /// iframes and workers issue requests the page session never sees.
    pub all_targets: bool,
}

impl RequestInterceptOptions {
    /// Wildcards are `*` for any run of characters and `?` for exactly one.
    pub fn url_patterns<I, S>(mut self, patterns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.patterns = Some(
            patterns
                .into_iter()
                .map(|pattern| RequestPattern::builder().url_pattern(pattern).build())
                .collect(),
        );
        self
    }

    pub fn resource_types<I>(mut self, types: I) -> Self
    where
        I: IntoIterator<Item = ResourceType>,
    {
        self.patterns = Some(
            types
                .into_iter()
                .map(|resource_type| {
                    RequestPattern::builder()
                        .resource_type(resource_type)
                        .build()
                })
                .collect(),
        );
        self
    }

    pub fn patterns(mut self, patterns: Vec<RequestPattern>) -> Self {
        self.patterns = Some(patterns);
        self
    }

    pub fn intercept_response(mut self, intercept_response: bool) -> Self {
        self.intercept_response = intercept_response;
        self
    }

    pub fn all_targets(mut self, all_targets: bool) -> Self {
        self.all_targets = all_targets;
        self
    }
}

#[derive(Debug, Clone, Default)]
struct FetchConfig {
    /// `None` means every request. A pattern list can only narrow, so the union
    /// of "everything" with any list is still everything.
    patterns: Option<Vec<RequestPattern>>,
    handle_auth_requests: bool,
    intercept_response: bool,
    all_targets: bool,
    enabled: bool,
}

impl FetchConfig {
    fn merge(&mut self, options: &RequestInterceptOptions) {
        match (&mut self.patterns, &options.patterns) {
            (_, None) => self.patterns = None,
            (None, Some(_)) => {}
            (Some(existing), Some(added)) => existing.extend(added.iter().cloned()),
        }
        self.intercept_response |= options.intercept_response;
        self.all_targets |= options.all_targets;
    }

    /// A pattern carries a single stage, so asking for both means emitting each
    /// pattern twice.
    fn command(&self) -> Enable {
        let mut builder = Enable::builder();
        if self.handle_auth_requests {
            builder = builder.handle_auth_requests(true);
        }

        let base = self
            .patterns
            .clone()
            .unwrap_or_else(|| vec![RequestPattern::builder().url_pattern("*").build()]);

        let mut patterns = Vec::with_capacity(base.len() * 2);
        for pattern in base {
            if self.intercept_response {
                let mut response = pattern.clone();
                response.request_stage = Some(RequestStage::Response);
                patterns.push(response);
            }
            let mut request = pattern;
            request.request_stage = Some(RequestStage::Request);
            patterns.push(request);
        }

        builder.patterns(patterns).build()
    }
}

#[derive(Debug, Clone)]
struct Credentials {
    username: String,
    password: String,
}

/// Owns request interception for one CDP connection.
pub struct NetworkManager<T: ConnectionTransport + Send + Sync> {
    session: Arc<TokioMutex<CdpSession<T>>>,
    target_manager: Option<TargetManager<T>>,
    config: Arc<StdMutex<FetchConfig>>,
    /// Copy-on-write: registration builds a new list, dispatch clones one Arc.
    /// Dispatch runs per intercepted request, so it must not allocate, and a
    /// std lock is right here because it is never held across an await.
    handlers: Arc<StdMutex<Arc<Vec<RequestHandler<T>>>>>,
    credentials: Arc<StdMutex<Option<Credentials>>>,
    /// Request ids already answered with credentials once, so a rejected
    /// password cancels instead of looping.
    attempted_authentications: Arc<StdMutex<HashSet<String>>>,
    listening: Arc<AtomicBool>,
}

// Every field is shared, so a clone is a handle rather than a copy. Derived
// Clone would demand `T: Clone`, which a transport is not.
impl<T: ConnectionTransport + Send + Sync> Clone for NetworkManager<T> {
    fn clone(&self) -> Self {
        Self {
            session: self.session.clone(),
            target_manager: self.target_manager.clone(),
            config: self.config.clone(),
            handlers: self.handlers.clone(),
            credentials: self.credentials.clone(),
            attempted_authentications: self.attempted_authentications.clone(),
            listening: self.listening.clone(),
        }
    }
}

impl<T: ConnectionTransport + Send + Sync + 'static> NetworkManager<T> {
    pub fn new(
        session: Arc<TokioMutex<CdpSession<T>>>,
        target_manager: Option<TargetManager<T>>,
    ) -> Self {
        Self {
            session,
            target_manager,
            config: Arc::new(StdMutex::new(FetchConfig::default())),
            handlers: Arc::new(StdMutex::new(Arc::new(Vec::new()))),
            credentials: Arc::new(StdMutex::new(None)),
            attempted_authentications: Arc::new(StdMutex::new(HashSet::new())),
            listening: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn on_request<F, Fut>(
        &self,
        handler: F,
        options: RequestInterceptOptions,
    ) -> Result<(), InterceptNetworkError>
    where
        F: Fn(Arc<CdpHttpRequest<T>>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let handler: RequestHandler<T> = Arc::new(move |request| Box::pin(handler(request)));
        {
            let mut handlers = self.handlers.lock().unwrap();
            let mut next = Vec::with_capacity(handlers.len() + 1);
            next.extend(handlers.iter().cloned());
            next.push(handler);
            *handlers = Arc::new(next);
        }
        self.config.lock().unwrap().merge(&options);
        self.watch().await;
        self.apply().await
    }

    pub async fn authenticate(
        &self,
        username: impl Into<String>,
        password: impl Into<String>,
        options: RequestInterceptOptions,
    ) -> Result<(), InterceptNetworkError> {
        *self.credentials.lock().unwrap() = Some(Credentials {
            username: username.into(),
            password: password.into(),
        });
        {
            let mut config = self.config.lock().unwrap();
            config.handle_auth_requests = true;
            config.merge(&options);
        }
        self.watch().await;
        self.apply().await
    }

    /// Stops interception and releases the handlers.
    ///
    /// Requests paused before this lands are still waiting on an answer, so they
    /// are let through first; `Fetch.disable` alone would strand them.
    pub async fn disable(&self) -> Result<(), InterceptNetworkError> {
        *self.handlers.lock().unwrap() = Arc::new(Vec::new());
        self.credentials.lock().unwrap().take();
        self.attempted_authentications.lock().unwrap().clear();

        let was_enabled = {
            let mut config = self.config.lock().unwrap();
            let was_enabled = config.enabled;
            *config = FetchConfig::default();
            was_enabled
        };
        if !was_enabled {
            return Ok(());
        }

        self.send(Disable::builder().build()).await
    }

    async fn apply(&self) -> Result<(), InterceptNetworkError> {
        let (command, all_targets) = {
            let mut config = self.config.lock().unwrap();
            config.enabled = true;
            (config.command(), config.all_targets)
        };

        if let (true, Some(target_manager)) = (all_targets, self.target_manager.as_ref()) {
            // Reaches every attached target and every one that attaches later,
            // which is this codebase's equivalent of Puppeteer re-sending
            // `Fetch.enable` from `addClient` for each new session.
            target_manager.add_target_command(command.into()).await;
            return Ok(());
        }

        self.send(command).await
    }

    async fn send(&self, command: impl Into<Command>) -> Result<(), InterceptNetworkError> {
        let rx = {
            let mut session = self.session.lock().await;
            let session_id = session.session_id.clone();
            session.send_and_get_receiver_for(command, session_id).await
        };
        CdpSession::<T>::await_response(rx)
            .await
            .map(|_| ())
            .map_err(|error| {
                InterceptNetworkError::CommandResultError(CdpCommandResultError::SessionSendError(
                    error,
                ))
            })
    }

    /// Registers the `Fetch` event handlers exactly once.
    ///
    /// Registration happens before `Fetch.enable` is sent, because the browser
    /// starts pausing the moment it is enabled and a pause with nobody listening
    /// is a stalled page.
    async fn watch(&self) {
        if self.listening.swap(true, Ordering::AcqRel) {
            return;
        }

        let manager = self.clone();
        self.session
            .lock()
            .await
            .add_event_handler([RequestPaused::IDENTIFIER], move |event| {
                let manager = manager.clone();
                async move { manager.on_request_paused(event).await }
            });

        let manager = self.clone();
        self.session
            .lock()
            .await
            .add_event_handler([AuthRequired::IDENTIFIER], move |event| {
                let manager = manager.clone();
                async move { manager.on_auth_required(event).await }
            });
    }

    async fn on_request_paused(&self, event: EventResponse) {
        let Some(request) = CdpHttpRequest::from_event(event, self.session.clone()) else {
            return;
        };
        let request = Arc::new(request);

        let handlers = {
            let handlers = self.handlers.lock().unwrap();
            Arc::clone(&handlers)
        };

        // Interception is on for credentials alone, so nothing would answer this
        // pause and the request would hang.
        if handlers.is_empty() {
            let _ = request.continue_().await;
            return;
        }

        for handler in handlers.iter() {
            handler(request.clone()).await;
        }

        // Handlers have all returned without resolving. Puppeteer leaves the
        // request paused here and the page hangs on it; letting it through is
        // the only outcome that cannot deadlock the browser.
        if !request.is_handled() {
            tracing::warn!(
                "[NetworkManager] no handler resolved {}, continuing it",
                request.url()
            );
            let _ = request.continue_().await;
        }
    }

    async fn on_auth_required(&self, event: EventResponse) {
        let Some(request) = CdpHttpRequest::from_event(event, self.session.clone()) else {
            return;
        };

        let credentials = self.credentials.lock().unwrap().clone();
        let first_attempt = self
            .attempted_authentications
            .lock()
            .unwrap()
            .insert(request.request_id().as_ref().to_string());

        let result = match credentials {
            // A repeat challenge for one request means the credentials were
            // rejected. Sending them again just loops.
            _ if !first_attempt => {
                request
                    .continue_with_no_auth(AuthChallengeResponseResponse::CancelAuth)
                    .await
            }
            Some(credentials) => {
                request
                    .continue_with_auth(credentials.username, credentials.password)
                    .await
            }
            // Interception is on for requests, not credentials: hand the
            // challenge back to the browser to deal with.
            None => {
                request
                    .continue_with_no_auth(AuthChallengeResponseResponse::Default)
                    .await
            }
        };

        if let Err(error) = result {
            tracing::debug!("[NetworkManager] auth response failed: {}", error);
        }
    }
}
