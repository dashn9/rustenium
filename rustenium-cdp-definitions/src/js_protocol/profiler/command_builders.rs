use super::commands::*;
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
#[derive(Debug, Clone, Default)]
pub struct GetBestEffortCoverageBuilder;
impl GetBestEffortCoverageBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> GetBestEffortCoverage {
        GetBestEffortCoverage {
            method: GetBestEffortCoverageMethod::GetBestEffortCoverage,
            params: GetBestEffortCoverageParams {},
        }
    }
}
impl GetBestEffortCoverage {
    pub fn builder() -> GetBestEffortCoverageBuilder {
        GetBestEffortCoverageBuilder
    }
}
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
#[derive(Debug, Clone, Default)]
pub struct StartBuilder;
impl StartBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Start {
        Start {
            method: StartMethod::Start,
            params: StartParams {},
        }
    }
}
impl Start {
    pub fn builder() -> StartBuilder {
        StartBuilder
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
#[derive(Debug, Clone, Default)]
pub struct StopBuilder;
impl StopBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Stop {
        Stop {
            method: StopMethod::Stop,
            params: StopParams {},
        }
    }
}
impl Stop {
    pub fn builder() -> StopBuilder {
        StopBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct StopPreciseCoverageBuilder;
impl StopPreciseCoverageBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> StopPreciseCoverage {
        StopPreciseCoverage {
            method: StopPreciseCoverageMethod::StopPreciseCoverage,
            params: StopPreciseCoverageParams {},
        }
    }
}
impl StopPreciseCoverage {
    pub fn builder() -> StopPreciseCoverageBuilder {
        StopPreciseCoverageBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct TakePreciseCoverageBuilder;
impl TakePreciseCoverageBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> TakePreciseCoverage {
        TakePreciseCoverage {
            method: TakePreciseCoverageMethod::TakePreciseCoverage,
            params: TakePreciseCoverageParams {},
        }
    }
}
impl TakePreciseCoverage {
    pub fn builder() -> TakePreciseCoverageBuilder {
        TakePreciseCoverageBuilder
    }
}
