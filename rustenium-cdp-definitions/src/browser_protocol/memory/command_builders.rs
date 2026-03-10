use super::commands::*;
#[derive(Debug, Clone, Default)]
pub struct GetDomCountersBuilder;
impl GetDomCountersBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> GetDomCounters {
        GetDomCounters {
            method: GetDomCountersMethod::GetDomCounters,
            params: GetDomCountersParams {},
        }
    }
}
impl GetDomCounters {
    pub fn builder() -> GetDomCountersBuilder {
        GetDomCountersBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct GetDomCountersForLeakDetectionBuilder;
impl GetDomCountersForLeakDetectionBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> GetDomCountersForLeakDetection {
        GetDomCountersForLeakDetection {
            method: GetDomCountersForLeakDetectionMethod::GetDomCountersForLeakDetection,
            params: GetDomCountersForLeakDetectionParams {},
        }
    }
}
impl GetDomCountersForLeakDetection {
    pub fn builder() -> GetDomCountersForLeakDetectionBuilder {
        GetDomCountersForLeakDetectionBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct PrepareForLeakDetectionBuilder;
impl PrepareForLeakDetectionBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> PrepareForLeakDetection {
        PrepareForLeakDetection {
            method: PrepareForLeakDetectionMethod::PrepareForLeakDetection,
            params: PrepareForLeakDetectionParams {},
        }
    }
}
impl PrepareForLeakDetection {
    pub fn builder() -> PrepareForLeakDetectionBuilder {
        PrepareForLeakDetectionBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct ForciblyPurgeJavaScriptMemoryBuilder;
impl ForciblyPurgeJavaScriptMemoryBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> ForciblyPurgeJavaScriptMemory {
        ForciblyPurgeJavaScriptMemory {
            method: ForciblyPurgeJavaScriptMemoryMethod::ForciblyPurgeJavaScriptMemory,
            params: ForciblyPurgeJavaScriptMemoryParams {},
        }
    }
}
impl ForciblyPurgeJavaScriptMemory {
    pub fn builder() -> ForciblyPurgeJavaScriptMemoryBuilder {
        ForciblyPurgeJavaScriptMemoryBuilder
    }
}
impl SetPressureNotificationsSuppressed {
    pub fn builder() -> SetPressureNotificationsSuppressedBuilder {
        <SetPressureNotificationsSuppressedBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetPressureNotificationsSuppressedBuilder {
    suppressed: Option<bool>,
}
impl SetPressureNotificationsSuppressedBuilder {
    pub fn suppressed(mut self, suppressed: impl Into<bool>) -> Self {
        self.suppressed = Some(suppressed.into());
        self
    }
    pub fn build(self) -> Result<SetPressureNotificationsSuppressed, String> {
        Ok(SetPressureNotificationsSuppressed {
            method: SetPressureNotificationsSuppressedMethod::SetPressureNotificationsSuppressed,
            params: SetPressureNotificationsSuppressedParams {
                suppressed: self.suppressed.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(suppressed))
                })?,
            },
        })
    }
}
impl SimulatePressureNotification {
    pub fn builder() -> SimulatePressureNotificationBuilder {
        <SimulatePressureNotificationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SimulatePressureNotificationBuilder {
    level: Option<super::types::PressureLevel>,
}
impl SimulatePressureNotificationBuilder {
    pub fn level(mut self, level: impl Into<super::types::PressureLevel>) -> Self {
        self.level = Some(level.into());
        self
    }
    pub fn build(self) -> Result<SimulatePressureNotification, String> {
        Ok(SimulatePressureNotification {
            method: SimulatePressureNotificationMethod::SimulatePressureNotification,
            params: SimulatePressureNotificationParams {
                level: self
                    .level
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(level)))?,
            },
        })
    }
}
impl StartSampling {
    pub fn builder() -> StartSamplingBuilder {
        <StartSamplingBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StartSamplingBuilder {
    sampling_interval: Option<i64>,
    suppress_randomness: Option<bool>,
}
impl StartSamplingBuilder {
    pub fn sampling_interval(mut self, sampling_interval: impl Into<i64>) -> Self {
        self.sampling_interval = Some(sampling_interval.into());
        self
    }
    pub fn suppress_randomness(mut self, suppress_randomness: impl Into<bool>) -> Self {
        self.suppress_randomness = Some(suppress_randomness.into());
        self
    }
    pub fn build(self) -> StartSampling {
        StartSampling {
            method: StartSamplingMethod::StartSampling,
            params: StartSamplingParams {
                sampling_interval: self.sampling_interval,
                suppress_randomness: self.suppress_randomness,
            },
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct StopSamplingBuilder;
impl StopSamplingBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> StopSampling {
        StopSampling {
            method: StopSamplingMethod::StopSampling,
            params: StopSamplingParams {},
        }
    }
}
impl StopSampling {
    pub fn builder() -> StopSamplingBuilder {
        StopSamplingBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct GetAllTimeSamplingProfileBuilder;
impl GetAllTimeSamplingProfileBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> GetAllTimeSamplingProfile {
        GetAllTimeSamplingProfile {
            method: GetAllTimeSamplingProfileMethod::GetAllTimeSamplingProfile,
            params: GetAllTimeSamplingProfileParams {},
        }
    }
}
impl GetAllTimeSamplingProfile {
    pub fn builder() -> GetAllTimeSamplingProfileBuilder {
        GetAllTimeSamplingProfileBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct GetBrowserSamplingProfileBuilder;
impl GetBrowserSamplingProfileBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> GetBrowserSamplingProfile {
        GetBrowserSamplingProfile {
            method: GetBrowserSamplingProfileMethod::GetBrowserSamplingProfile,
            params: GetBrowserSamplingProfileParams {},
        }
    }
}
impl GetBrowserSamplingProfile {
    pub fn builder() -> GetBrowserSamplingProfileBuilder {
        GetBrowserSamplingProfileBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct GetSamplingProfileBuilder;
impl GetSamplingProfileBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> GetSamplingProfile {
        GetSamplingProfile {
            method: GetSamplingProfileMethod::GetSamplingProfile,
            params: GetSamplingProfileParams {},
        }
    }
}
impl GetSamplingProfile {
    pub fn builder() -> GetSamplingProfileBuilder {
        GetSamplingProfileBuilder
    }
}
