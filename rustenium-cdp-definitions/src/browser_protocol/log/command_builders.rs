use super::commands::*;
#[derive(Debug, Clone, Default)]
pub struct ClearBuilder;
impl ClearBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Clear {
        Clear {
            method: ClearMethod::Clear,
            params: ClearParams {},
        }
    }
}
impl Clear {
    pub fn builder() -> ClearBuilder {
        ClearBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct DisableBuilder;
impl DisableBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Disable {
        Disable {
            method: DisableMethod::Disable,
            params: DisableParams {},
        }
    }
}
impl Disable {
    pub fn builder() -> DisableBuilder {
        DisableBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct EnableBuilder;
impl EnableBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Enable {
        Enable {
            method: EnableMethod::Enable,
            params: EnableParams {},
        }
    }
}
impl Enable {
    pub fn builder() -> EnableBuilder {
        EnableBuilder
    }
}
impl StartViolationsReport {
    pub fn builder() -> StartViolationsReportBuilder {
        <StartViolationsReportBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StartViolationsReportBuilder {
    config: Option<Vec<super::types::ViolationSetting>>,
}
impl StartViolationsReportBuilder {
    pub fn config(mut self, config: impl Into<super::types::ViolationSetting>) -> Self {
        let v = self.config.get_or_insert(Vec::new());
        v.push(config.into());
        self
    }
    pub fn configs<I, S>(mut self, configs: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::ViolationSetting>,
    {
        let v = self.config.get_or_insert(Vec::new());
        for val in configs {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<StartViolationsReport, String> {
        Ok(StartViolationsReport {
            method: StartViolationsReportMethod::StartViolationsReport,
            params: StartViolationsReportParams {
                config: self
                    .config
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(config)))?,
            },
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct StopViolationsReportBuilder;
impl StopViolationsReportBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> StopViolationsReport {
        StopViolationsReport {
            method: StopViolationsReportMethod::StopViolationsReport,
            params: StopViolationsReportParams {},
        }
    }
}
impl StopViolationsReport {
    pub fn builder() -> StopViolationsReportBuilder {
        StopViolationsReportBuilder
    }
}
