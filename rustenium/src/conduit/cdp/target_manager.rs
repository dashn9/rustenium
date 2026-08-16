//! Target tracking and attachment.
//!
//! A *target* is anything Chrome will let you attach a debugger to — a page, a
//! worker, a cross-origin iframe, an extension background page. Each is an
//! independent execution context, often in its own process, so there is no single
//! "debuggee": every one has to be addressed separately.
//!
//! Three things are deliberately distinct here, and conflating them is what makes
//! multi-target support impossible:
//!
//! * **Connection** — one socket. Transport only.
//! * **Target** — a thing in the browser. Exists whether or not you are attached,
//!   and outlives any particular attachment.
//! * **Session** — an *attachment* to a target, named by `sessionId`. One target
//!   can have several; detach and reattach yields a new session id for the same
//!   target. Anything you want to remember about a target must therefore be keyed
//!   on `target_id`, never on the session.
//!
//! `TargetManager` owns the browser-scoped connection, keeps the target registry,
//! and owns the attach lifecycle. Consumers do not send `Target.setAutoAttach` or
//! `Runtime.runIfWaitingForDebugger` themselves — see [`TargetManager::add_init_script`].

use std::collections::HashMap;
use std::sync::{Arc, Mutex as StdMutex};

use rustenium_cdp_definitions::Command;
use rustenium_cdp_definitions::browser_protocol::page::commands::{
    AddScriptToEvaluateOnNewDocument, AddScriptToEvaluateOnNewDocumentMethod,
    AddScriptToEvaluateOnNewDocumentParams,
};
use rustenium_cdp_definitions::browser_protocol::target::command_builders::SetDiscoverTargetsBuilder;
use rustenium_cdp_definitions::browser_protocol::target::commands::{
    AttachToTarget, GetTargets, SetAutoAttach,
};
use rustenium_cdp_definitions::browser_protocol::target::events::{
    AttachedToTarget, DetachedFromTarget,
};
use rustenium_cdp_definitions::browser_protocol::target::results::{
    AttachToTargetResult, GetTargetsResult,
};
use rustenium_cdp_definitions::browser_protocol::target::types::{TargetId, TargetInfo};
use rustenium_cdp_definitions::js_protocol::runtime::command_builders::EvaluateBuilder;
use rustenium_cdp_definitions::js_protocol::runtime::commands::RunIfWaitingForDebugger;
use rustenium_core::CdpEventManagement;
use rustenium_core::error::CdpSessionSendError;
use rustenium_core::session::CdpSession;
use rustenium_core::transport::ConnectionTransport;
use tokio::sync::Mutex as TokioMutex;

/// The kinds of target we treat differently. Everything else is `Other` — still
/// tracked, but nothing is installed into it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetKind {
    /// Carries a document: a tab, or a cross-origin iframe. An OOPIF is its own
    /// target; a same-origin iframe is *not* — it is a frame inside its parent's
    /// target and shares that session, which is why one registration on the parent
    /// already covers it.
    Document,
    /// Dedicated, shared, or service worker. Service workers are browser-scoped:
    /// they outlive every tab and attach under the browser session, so they belong
    /// to no tab and only a browser-scoped connection ever sees them.
    Worker,
    Other,
}

impl TargetKind {
    fn from_type(target_type: &str) -> Self {
        match target_type {
            "page" | "iframe" => TargetKind::Document,
            "worker" | "service_worker" | "shared_worker" => TargetKind::Worker,
            _ => TargetKind::Other,
        }
    }
}

/// Queue a command at a session without waiting for the reply.
///
/// The attach path is latency-critical in a way the rest of the client is not: the
/// target stays frozen until we release it, and page code is often waiting on that
/// target with a timeout of its own — a few seconds is a realistic budget for a
/// worker that a page has just started and expects an answer from.
///
/// `session.lock().await.send_to(..).await` holds the mutex across the whole round
/// trip, so every attaching target serialises behind every other one — and at
/// startup that includes extensions, browser UI and the new-tab page. That blows
/// the budget easily, and the visible symptom is a worker the page gives up on.
/// CDP processes messages per session in order, so dropping the wait preserves
/// ordering (setup before resume) without the queueing.
async fn fire<T: ConnectionTransport + Send + Sync>(
    session: &Arc<TokioMutex<CdpSession<T>>>,
    session_id: &str,
    command: impl Into<Command>,
) {
    let _ = session
        .lock()
        .await
        .send_and_get_receiver_for(command, Some(session_id.to_string()))
        .await;
}

/// Install one script into one attached target.
///
/// Documents take a preload, which persists across navigations. Workers have a
/// single script lifetime and no Page domain at all — `addScriptToEvaluateOnNewDocument`
/// does not exist for them — so the only way in is to evaluate while the target is
/// frozen, before it has run anything.
async fn install<T: ConnectionTransport + Send + Sync>(
    session: &Arc<TokioMutex<CdpSession<T>>>,
    session_id: &str,
    kind: TargetKind,
    script: &InitScript,
) -> Result<(), CdpSessionSendError> {
    let source = match kind {
        TargetKind::Document => script.page.clone(),
        TargetKind::Worker => script.worker.clone(),
        TargetKind::Other => None,
    };
    let Some(source) = source else { return Ok(()) };
    let sid = Some(session_id.to_string());

    match kind {
        TargetKind::Document => {
            let cmd = AddScriptToEvaluateOnNewDocument {
                method: AddScriptToEvaluateOnNewDocumentMethod::AddScriptToEvaluateOnNewDocument,
                params: AddScriptToEvaluateOnNewDocumentParams::new(source),
            };
            session.lock().await.send_to(cmd, sid).await.map(|_| ())
        }
        TargetKind::Worker => {
            let cmd = EvaluateBuilder::default()
                .expression(source)
                .build()
                .unwrap();
            session.lock().await.send_to(cmd, sid).await.map(|_| ())
        }
        TargetKind::Other => Ok(()),
    }
}

/// A target in the browser, and our attachment to it if we have one.
#[derive(Debug, Clone)]
pub struct Target {
    pub target_id: TargetId,
    pub target_type: String,
    pub url: String,
    /// `None` when the target is known but not attached — discovery reports
    /// targets we never attach to.
    pub session_id: Option<String>,
    /// The session that observed this target attaching, which is what makes the
    /// tree navigable: a page's workers arrive on the page's session.
    pub parent_session_id: Option<String>,
}

impl Target {
    pub fn kind(&self) -> TargetKind {
        TargetKind::from_type(&self.target_type)
    }
}

/// Scripts to install into every target as it attaches, before it runs anything.
///
/// Two variants because worker scope is a different realm: it has no `window` or
/// `document`, and its navigator interface is `WorkerNavigator`, so a script
/// written for a page throws on its first statement in a worker.
#[derive(Debug, Clone, Default)]
pub struct InitScript {
    pub page: Option<String>,
    pub worker: Option<String>,
}

pub struct TargetManager<T: ConnectionTransport + Send + Sync> {
    /// The connection. Shared with the page-scoped adapter, which sets its own
    /// `session_id` on it — so every command issued here passes `None` explicitly
    /// to stay addressed at the browser session, where `Target.*` and service
    /// workers live. Never use plain `send()` from this type.
    session: Arc<TokioMutex<CdpSession<T>>>,
    targets: Arc<StdMutex<HashMap<TargetId, Target>>>,
    init_scripts: Arc<StdMutex<Vec<InitScript>>>,
    /// Sent to every target as it attaches, before any init script. Emulation
    /// overrides live here: they configure a session, so applying one only to the
    /// page leaves other targets computing their values from unconfigured state.
    target_commands: Arc<StdMutex<Vec<Command>>>,
}

// Hand-written rather than derived: every field is an Arc, so cloning a manager
// is cheap and shares state. #[derive(Clone)] would demand `T: Clone`, which the
// transport is not.
impl<T: ConnectionTransport + Send + Sync> Clone for TargetManager<T> {
    fn clone(&self) -> Self {
        Self {
            session: self.session.clone(),
            targets: self.targets.clone(),
            init_scripts: self.init_scripts.clone(),
            target_commands: self.target_commands.clone(),
        }
    }
}

impl<T: ConnectionTransport + Send + Sync + 'static> TargetManager<T> {
    pub fn new(session: Arc<TokioMutex<CdpSession<T>>>) -> Self {
        Self {
            session,
            targets: Arc::new(StdMutex::new(HashMap::new())),
            init_scripts: Arc::new(StdMutex::new(Vec::new())),
            target_commands: Arc::new(StdMutex::new(Vec::new())),
        }
    }

    /// Send a command to every target — those already attached, and every one that
    /// attaches later, before its init scripts run.
    ///
    /// This is how emulation reaches workers. `Emulation.setUserAgentOverride`,
    /// `setTimezoneOverride` and `setLocaleOverride` each configure one session,
    /// so sending them only to the page leaves every other target deriving its
    /// timezone, locale and client hints from unconfigured state — a browser whose
    /// targets disagree with one another.
    ///
    /// Failures are logged at debug, not warn: page-only commands (device metrics,
    /// touch emulation) are expected to be rejected by worker targets.
    pub async fn add_target_command(&self, command: Command) {
        let existing: Vec<Target> = self
            .targets
            .lock()
            .unwrap()
            .values()
            .filter(|t| t.session_id.is_some())
            .cloned()
            .collect();

        for target in existing {
            let Some(sid) = target.session_id.as_deref() else { continue };
            if let Err(e) = self
                .session
                .lock()
                .await
                .send_to(command.clone(), Some(sid.to_string()))
                .await
            {
                tracing::debug!(
                    "[TargetManager] target command rejected by {}: {}",
                    target.target_type,
                    e
                );
            }
        }

        self.target_commands.lock().unwrap().push(command);
    }

    /// Install a script into every target — those already attached, and every one
    /// that attaches later.
    ///
    /// This is the whole consumer-facing API for injection: no session ids, no
    /// attach events, no resume call.
    ///
    /// It applies to existing targets immediately *and* stores the script, so
    /// registration order stops mattering. Without the first half, anything
    /// registered after startup would silently miss the tab that is already open.
    pub async fn add_init_script(&self, script: InitScript) {
        let existing: Vec<Target> = self
            .targets
            .lock()
            .unwrap()
            .values()
            .filter(|t| t.session_id.is_some())
            .cloned()
            .collect();

        for target in existing {
            let Some(sid) = target.session_id.as_deref() else {
                continue;
            };
            if let Err(e) = install(&self.session, sid, target.kind(), &script).await {
                tracing::warn!(
                    "[TargetManager] init script failed for {}: {}",
                    target.target_type,
                    e
                );
            }
        }

        self.init_scripts.lock().unwrap().push(script);
    }

    /// Look a target up by its stable id.
    pub fn target_by_id(&self, target_id: &TargetId) -> Option<Target> {
        self.targets.lock().unwrap().get(target_id).cloned()
    }

    pub fn targets(&self) -> Vec<Target> {
        self.targets.lock().unwrap().values().cloned().collect()
    }

    /// Targets that attached under `parent_session_id` — a page's workers and
    /// OOPIFs. This is what a `workers()` accessor is built from.
    pub fn child_targets(&self, parent_session_id: &str) -> Vec<Target> {
        self.targets
            .lock()
            .unwrap()
            .values()
            .filter(|t| t.parent_session_id.as_deref() == Some(parent_session_id))
            .cloned()
            .collect()
    }

    /// Start discovery and attachment. Idempotent per manager.
    pub async fn initialize(&mut self) -> Result<(), CdpSessionSendError> {
        self.watch_attachments().await;

        let discover = SetDiscoverTargetsBuilder::default()
            .discover(true)
            .build()
            .unwrap();
        self.session.lock().await.send_to(discover, None).await?;

        // Browser-scoped, so service workers are included. Chrome also reports
        // targets that already exist, not just new ones.
        let auto_attach = SetAutoAttach::builder()
            .auto_attach(true)
            .wait_for_debugger_on_start(true)
            .flatten(true)
            .build()
            .unwrap();
        self.session.lock().await.send_to(auto_attach, None).await?;

        Ok(())
    }

    /// Attach to the first page target and return its session id, so a page-scoped
    /// API can be bound to it. Connecting to the browser endpoint gives no page
    /// session for free — that is the trade for being able to see everything else.
    pub async fn attach_to_first_page(&mut self) -> Result<Option<String>, CdpSessionSendError> {
        let response = self
            .session
            .lock()
            .await
            .send_to(GetTargets::builder().build(), None)
            .await?;
        let Ok(result) = GetTargetsResult::try_from(response.result) else {
            return Ok(None);
        };

        let Some(page) = result
            .target_infos
            .into_iter()
            .find(|info: &TargetInfo| info.r#type == "page")
        else {
            return Ok(None);
        };

        let attach = AttachToTarget::builder()
            .target_id(page.target_id.clone())
            .flatten(true)
            .build()
            .unwrap();
        let response = self.session.lock().await.send_to(attach, None).await?;
        let Ok(result) = AttachToTargetResult::try_from(response.result) else {
            return Ok(None);
        };

        Ok(Some(result.session_id.as_ref().to_string()))
    }

    async fn watch_attachments(&mut self) {
        let session = self.session.clone();
        let targets = self.targets.clone();
        let init_scripts = self.init_scripts.clone();
        let target_commands = self.target_commands.clone();

        self.session.lock().await.add_event_handler(
            [AttachedToTarget::IDENTIFIER],
            move |event| {
                let session = session.clone();
                let targets = targets.clone();
                let init_scripts = init_scripts.clone();
                let target_commands = target_commands.clone();
                async move {
                    // The envelope's sessionId is the PARENT — whichever session
                    // observed this attach. The child's own id is in params. That
                    // pairing is what makes the target tree navigable, so capture
                    // it before try_into_event consumes the event.
                    let parent_session_id = event.session_id.clone();
                    let Ok(attached) = event.try_into_event::<AttachedToTarget>() else {
                        return;
                    };
                    let params = attached.params;
                    let info = params.target_info;
                    let child = params.session_id.as_ref().to_string();

                    let target = Target {
                        target_id: info.target_id.clone(),
                        target_type: info.r#type.clone(),
                        url: info.url.clone(),
                        session_id: Some(child.clone()),
                        parent_session_id,
                    };
                    let kind = target.kind();
                    targets
                        .lock()
                        .unwrap()
                        .insert(info.target_id.clone(), target);

                    tracing::debug!(
                        "[TargetManager] attached: type={}, url={}, waiting={}",
                        info.r#type,
                        info.url,
                        params.waiting_for_debugger
                    );

                    // Everything below happens while the target is frozen. This is
                    // the only window in which what we install is invisible to the
                    // target's own code.
                    let commands = target_commands.lock().unwrap().clone();
                    for command in commands {
                        fire(&session, &child, command).await;
                    }

                    let scripts = init_scripts.lock().unwrap().clone();
                    for script in &scripts {
                        let source = match kind {
                            TargetKind::Document => script.page.clone(),
                            TargetKind::Worker => script.worker.clone(),
                            TargetKind::Other => None,
                        };
                        let Some(source) = source else { continue };
                        match kind {
                            TargetKind::Document => {
                                fire(&session, &child, AddScriptToEvaluateOnNewDocument {
                                    method: AddScriptToEvaluateOnNewDocumentMethod::AddScriptToEvaluateOnNewDocument,
                                    params: AddScriptToEvaluateOnNewDocumentParams::new(source),
                                }).await
                            }
                            TargetKind::Worker => {
                                fire(&session, &child, EvaluateBuilder::default()
                                    .expression(source).build().unwrap()).await
                            }
                            TargetKind::Other => {}
                        }
                    }

                    // Re-arm BEFORE releasing. Auto-attach is scoped to the session
                    // it was sent on and is not inherited, so without this a worker
                    // spawned in the target's first tick is never seen.
                    let rearm = SetAutoAttach::builder()
                        .auto_attach(true)
                        .wait_for_debugger_on_start(true)
                        .flatten(true)
                        .build()
                        .unwrap();
                    fire(&session, &child, rearm).await;

                    // Unconditional. A target left waiting for the debugger never
                    // runs at all, so a failure anywhere above must not be allowed
                    // to strand it — the symptom is a page that never finishes
                    // loading.
                    fire(&session, &child, RunIfWaitingForDebugger::builder().build()).await;
                }
            },
        );

        let targets = self.targets.clone();
        self.session.lock().await.add_event_handler(
            [DetachedFromTarget::IDENTIFIER],
            move |event| {
                let targets = targets.clone();
                async move {
                    if let Ok(detached) = event.try_into_event::<DetachedFromTarget>() {
                        let sid = detached.params.session_id.as_ref().to_string();
                        targets
                            .lock()
                            .unwrap()
                            .retain(|_, t| t.session_id.as_deref() != Some(sid.as_str()));
                    }
                }
            },
        );
    }
}
