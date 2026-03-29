use rustenium_cdp_definitions::browser_protocol::target::types::TargetId;

pub trait Page {
    fn target_id(&self) -> &TargetId;
}
