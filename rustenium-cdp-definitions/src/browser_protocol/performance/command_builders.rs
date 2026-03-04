use super::commands::*;
impl Enable {
    pub fn builder() -> EnableBuilder {
        EnableBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct EnableBuilder {
    time_domain: Option<EnableTimeDomain>,
}
impl EnableBuilder {
    pub fn time_domain(mut self, time_domain: impl Into<EnableTimeDomain>) -> Self {
        self.time_domain = Some(time_domain.into());
        self
    }
    pub fn build(self) -> Enable {
        Enable {
            method: EnableMethod::Enable,
            params: EnableParams {
                time_domain: self.time_domain,
            },
        }
    }
}
