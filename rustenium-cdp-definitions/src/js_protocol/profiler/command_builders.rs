use super::commands::*;
impl SetSamplingInterval {
    pub fn builder() -> SetSamplingIntervalBuilder {
        <SetSamplingIntervalBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetSamplingIntervalBuilder {
    interval: Option<i64>,
}
impl SetSamplingIntervalBuilder {
    pub fn interval(mut self, interval: impl Into<i64>) -> Self {
        self.interval = Some(interval.into());
        self
    }
    pub fn build(self) -> Result<SetSamplingInterval, String> {
        Ok(SetSamplingInterval {
            method: SetSamplingIntervalMethod::SetSamplingInterval,
            params: SetSamplingIntervalParams {
                interval: self.interval.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(interval))
                })?,
            },
        })
    }
}
impl StartPreciseCoverage {
    pub fn builder() -> StartPreciseCoverageBuilder {
        <StartPreciseCoverageBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StartPreciseCoverageBuilder {
    call_count: Option<bool>,
    detailed: Option<bool>,
    allow_triggered_updates: Option<bool>,
}
impl StartPreciseCoverageBuilder {
    pub fn call_count(mut self, call_count: impl Into<bool>) -> Self {
        self.call_count = Some(call_count.into());
        self
    }
    pub fn detailed(mut self, detailed: impl Into<bool>) -> Self {
        self.detailed = Some(detailed.into());
        self
    }
    pub fn allow_triggered_updates(mut self, allow_triggered_updates: impl Into<bool>) -> Self {
        self.allow_triggered_updates = Some(allow_triggered_updates.into());
        self
    }
    pub fn build(self) -> StartPreciseCoverage {
        StartPreciseCoverage {
            method: StartPreciseCoverageMethod::StartPreciseCoverage,
            params: StartPreciseCoverageParams {
                call_count: self.call_count,
                detailed: self.detailed,
                allow_triggered_updates: self.allow_triggered_updates,
            },
        }
    }
}
