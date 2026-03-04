use super::commands::*;
impl SetDeviceOrientationOverride {
    pub fn builder() -> SetDeviceOrientationOverrideBuilder {
        SetDeviceOrientationOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDeviceOrientationOverrideBuilder {
    alpha: Option<f64>,
    beta: Option<f64>,
    gamma: Option<f64>,
}
impl SetDeviceOrientationOverrideBuilder {
    pub fn alpha(mut self, alpha: impl Into<f64>) -> Self {
        self.alpha = Some(alpha.into());
        self
    }
    pub fn beta(mut self, beta: impl Into<f64>) -> Self {
        self.beta = Some(beta.into());
        self
    }
    pub fn gamma(mut self, gamma: impl Into<f64>) -> Self {
        self.gamma = Some(gamma.into());
        self
    }
    pub fn build(self) -> Result<SetDeviceOrientationOverride, String> {
        Ok(SetDeviceOrientationOverride {
            method: SetDeviceOrientationOverrideMethod::SetDeviceOrientationOverride,
            params: SetDeviceOrientationOverrideParams {
                alpha: self
                    .alpha
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(alpha)))?,
                beta: self
                    .beta
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(beta)))?,
                gamma: self
                    .gamma
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(gamma)))?,
            },
        })
    }
}
