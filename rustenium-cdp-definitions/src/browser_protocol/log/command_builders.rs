use super::commands::*;
impl StartViolationsReport {
    pub fn builder() -> StartViolationsReportBuilder {
        StartViolationsReportBuilder::default()
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
