use crate::conduit::cdp::target_manager::{Target, TargetKind, TargetManager};
use rustenium_cdp_definitions::browser_protocol::target::types::TargetId;
use rustenium_core::WebsocketConnectionTransport;

/// A tab, and the targets that hang off it.
///
/// A tab *is* a target — the browser tracks it by `target_id`, which is stable for
/// its lifetime. Its session id is not: detaching and reattaching yields a new one
/// for the same tab, which is why nothing here is keyed on the session.
pub struct ChromeTab {
    target_id: TargetId,
    manager: TargetManager<WebsocketConnectionTransport>,
}

impl ChromeTab {
    pub fn new(target_id: TargetId, manager: TargetManager<WebsocketConnectionTransport>) -> Self {
        Self { target_id, manager }
    }

    pub fn target_id(&self) -> &TargetId {
        &self.target_id
    }

    /// The current attachment, if this tab is attached. `None` once it detaches.
    pub fn session_id(&self) -> Option<String> {
        self.manager
            .target_by_id(&self.target_id)
            .and_then(|t| t.session_id)
    }

    /// Workers owned by this tab: dedicated and shared.
    ///
    /// Service workers are deliberately absent. They are browser-scoped — shared
    /// between tabs and outliving all of them — so they attach under the browser
    /// session and belong to no tab. Use `ChromeBrowser::workers()` for those.
    pub fn workers(&self) -> Vec<Target> {
        self.child_targets()
            .into_iter()
            .filter(|t| t.kind() == TargetKind::Worker)
            .collect()
    }

    /// Cross-origin iframes belonging to this tab, each of which is its own target.
    ///
    /// This is deliberately *not* called `frames()`: same-origin iframes are not
    /// targets at all — they are frames inside this tab's own target, sharing its
    /// session — so they can never appear here. A complete frame list needs a
    /// frame tracker driven by `Page.frameAttached`, which does not exist yet.
    pub fn oopif_targets(&self) -> Vec<Target> {
        self.child_targets()
            .into_iter()
            .filter(|t| t.kind() == TargetKind::Document)
            .collect()
    }

    fn child_targets(&self) -> Vec<Target> {
        match self.session_id() {
            Some(sid) => self.manager.child_targets(&sid),
            None => Vec::new(),
        }
    }
}
