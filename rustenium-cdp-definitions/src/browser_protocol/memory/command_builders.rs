use super::commands::*;
impl SetPressureNotificationsSuppressed {
    pub fn builder() -> SetPressureNotificationsSuppressedBuilder {
        SetPressureNotificationsSuppressedBuilder::default()
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
        SimulatePressureNotificationBuilder::default()
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
        StartSamplingBuilder::default()
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
