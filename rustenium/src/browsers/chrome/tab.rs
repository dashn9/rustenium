use rustenium_cdp_definitions::browser_protocol::target::types::TargetId;
use crate::domain::cdp::page::Page;

pub struct ChromeTab {
    target_id: TargetId,
}

impl ChromeTab {
    pub fn new(target_id: TargetId) -> Self {
        Self { target_id }
    }
}

impl Page for ChromeTab {
    fn target_id(&self) -> &TargetId {
        &self.target_id
    }
}
