use serde::{Deserialize, Serialize};
#[doc = "Clears the overridden Device Orientation.\n[clearDeviceOrientationOverride](https://chromedevtools.github.io/devtools-protocol/tot/DeviceOrientation/#method-clearDeviceOrientationOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearDeviceOrientationOverrideParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearDeviceOrientationOverrideMethod {
    #[serde(rename = "DeviceOrientation.clearDeviceOrientationOverride")]
    ClearDeviceOrientationOverride,
}
#[doc = "Clears the overridden Device Orientation.\n[clearDeviceOrientationOverride](https://chromedevtools.github.io/devtools-protocol/tot/DeviceOrientation/#method-clearDeviceOrientationOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearDeviceOrientationOverride {
    pub method: ClearDeviceOrientationOverrideMethod,
    pub params: ClearDeviceOrientationOverrideParams,
}
impl ClearDeviceOrientationOverride {
    pub const IDENTIFIER: &'static str = "DeviceOrientation.clearDeviceOrientationOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ClearDeviceOrientationOverride {
    type Result = super::results::ClearDeviceOrientationOverrideResult;
}
#[doc = "Overrides the Device Orientation.\n[setDeviceOrientationOverride](https://chromedevtools.github.io/devtools-protocol/tot/DeviceOrientation/#method-setDeviceOrientationOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDeviceOrientationOverrideParams {
    #[doc = "Mock alpha"]
    #[serde(rename = "alpha")]
    pub alpha: f64,
    #[doc = "Mock beta"]
    #[serde(rename = "beta")]
    pub beta: f64,
    #[doc = "Mock gamma"]
    #[serde(rename = "gamma")]
    pub gamma: f64,
}
impl SetDeviceOrientationOverrideParams {
    pub fn new(alpha: impl Into<f64>, beta: impl Into<f64>, gamma: impl Into<f64>) -> Self {
        Self {
            alpha: alpha.into(),
            beta: beta.into(),
            gamma: gamma.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDeviceOrientationOverrideMethod {
    #[serde(rename = "DeviceOrientation.setDeviceOrientationOverride")]
    SetDeviceOrientationOverride,
}
#[doc = "Overrides the Device Orientation.\n[setDeviceOrientationOverride](https://chromedevtools.github.io/devtools-protocol/tot/DeviceOrientation/#method-setDeviceOrientationOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDeviceOrientationOverride {
    pub method: SetDeviceOrientationOverrideMethod,
    pub params: SetDeviceOrientationOverrideParams,
}
impl SetDeviceOrientationOverride {
    pub const IDENTIFIER: &'static str = "DeviceOrientation.setDeviceOrientationOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetDeviceOrientationOverride {
    type Result = super::results::SetDeviceOrientationOverrideResult;
}
group_enum ! (DeviceOrientationCommands { ClearDeviceOrientationOverride (ClearDeviceOrientationOverride) , SetDeviceOrientationOverride (SetDeviceOrientationOverride) } + identifiable);
