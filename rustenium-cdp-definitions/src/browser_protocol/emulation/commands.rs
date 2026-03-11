use serde::{Deserialize, Serialize};
#[doc = "Clears the overridden device metrics.\n[clearDeviceMetricsOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearDeviceMetricsOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearDeviceMetricsOverrideParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearDeviceMetricsOverrideMethod {
    #[serde(rename = "Emulation.clearDeviceMetricsOverride")]
    ClearDeviceMetricsOverride,
}
#[doc = "Clears the overridden device metrics.\n[clearDeviceMetricsOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearDeviceMetricsOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearDeviceMetricsOverride {
    pub method: ClearDeviceMetricsOverrideMethod,
    pub params: ClearDeviceMetricsOverrideParams,
}
impl ClearDeviceMetricsOverride {
    pub const IDENTIFIER: &'static str = "Emulation.clearDeviceMetricsOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ClearDeviceMetricsOverride {
    type Result = super::results::ClearDeviceMetricsOverrideResult;
}
#[doc = "Clears the overridden Geolocation Position and Error.\n[clearGeolocationOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearGeolocationOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearGeolocationOverrideParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearGeolocationOverrideMethod {
    #[serde(rename = "Emulation.clearGeolocationOverride")]
    ClearGeolocationOverride,
}
#[doc = "Clears the overridden Geolocation Position and Error.\n[clearGeolocationOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearGeolocationOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearGeolocationOverride {
    pub method: ClearGeolocationOverrideMethod,
    pub params: ClearGeolocationOverrideParams,
}
impl ClearGeolocationOverride {
    pub const IDENTIFIER: &'static str = "Emulation.clearGeolocationOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ClearGeolocationOverride {
    type Result = super::results::ClearGeolocationOverrideResult;
}
#[doc = "Requests that page scale factor is reset to initial values.\n[resetPageScaleFactor](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-resetPageScaleFactor)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetPageScaleFactorParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResetPageScaleFactorMethod {
    #[serde(rename = "Emulation.resetPageScaleFactor")]
    ResetPageScaleFactor,
}
#[doc = "Requests that page scale factor is reset to initial values.\n[resetPageScaleFactor](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-resetPageScaleFactor)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetPageScaleFactor {
    pub method: ResetPageScaleFactorMethod,
    pub params: ResetPageScaleFactorParams,
}
impl ResetPageScaleFactor {
    pub const IDENTIFIER: &'static str = "Emulation.resetPageScaleFactor";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ResetPageScaleFactor {
    type Result = super::results::ResetPageScaleFactorResult;
}
#[doc = "Enables or disables simulating a focused and active page.\n[setFocusEmulationEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setFocusEmulationEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetFocusEmulationEnabledParams {
    #[doc = "Whether to enable to disable focus emulation."]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
impl SetFocusEmulationEnabledParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self {
            enabled: enabled.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetFocusEmulationEnabledMethod {
    #[serde(rename = "Emulation.setFocusEmulationEnabled")]
    SetFocusEmulationEnabled,
}
#[doc = "Enables or disables simulating a focused and active page.\n[setFocusEmulationEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setFocusEmulationEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetFocusEmulationEnabled {
    pub method: SetFocusEmulationEnabledMethod,
    pub params: SetFocusEmulationEnabledParams,
}
impl SetFocusEmulationEnabled {
    pub const IDENTIFIER: &'static str = "Emulation.setFocusEmulationEnabled";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetFocusEmulationEnabled {
    type Result = super::results::SetFocusEmulationEnabledResult;
}
#[doc = "Automatically render all web contents using a dark theme.\n[setAutoDarkModeOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setAutoDarkModeOverride)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetAutoDarkModeOverrideParams {
    #[doc = "Whether to enable or disable automatic dark mode.\nIf not specified, any existing override will be cleared."]
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enabled: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetAutoDarkModeOverrideMethod {
    #[serde(rename = "Emulation.setAutoDarkModeOverride")]
    SetAutoDarkModeOverride,
}
#[doc = "Automatically render all web contents using a dark theme.\n[setAutoDarkModeOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setAutoDarkModeOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAutoDarkModeOverride {
    pub method: SetAutoDarkModeOverrideMethod,
    pub params: SetAutoDarkModeOverrideParams,
}
impl SetAutoDarkModeOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setAutoDarkModeOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetAutoDarkModeOverride {
    type Result = super::results::SetAutoDarkModeOverrideResult;
}
#[doc = "Enables CPU throttling to emulate slow CPUs.\n[setCPUThrottlingRate](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setCPUThrottlingRate)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCpuThrottlingRateParams {
    #[doc = "Throttling rate as a slowdown factor (1 is no throttle, 2 is 2x slowdown, etc)."]
    #[serde(rename = "rate")]
    pub rate: f64,
}
impl SetCpuThrottlingRateParams {
    pub fn new(rate: impl Into<f64>) -> Self {
        Self { rate: rate.into() }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetCpuThrottlingRateMethod {
    #[serde(rename = "Emulation.setCPUThrottlingRate")]
    SetCpuThrottlingRate,
}
#[doc = "Enables CPU throttling to emulate slow CPUs.\n[setCPUThrottlingRate](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setCPUThrottlingRate)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetCpuThrottlingRate {
    pub method: SetCpuThrottlingRateMethod,
    pub params: SetCpuThrottlingRateParams,
}
impl SetCpuThrottlingRate {
    pub const IDENTIFIER: &'static str = "Emulation.setCPUThrottlingRate";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetCpuThrottlingRate {
    type Result = super::results::SetCpuThrottlingRateResult;
}
#[doc = "Sets or clears an override of the default background color of the frame. This override is used\nif the content does not specify one.\n[setDefaultBackgroundColorOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDefaultBackgroundColorOverride)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDefaultBackgroundColorOverrideParams {
    #[doc = "RGBA of the default background color. If not specified, any existing override will be\ncleared."]
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub color: Option<crate::browser_protocol::dom::types::Rgba>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDefaultBackgroundColorOverrideMethod {
    #[serde(rename = "Emulation.setDefaultBackgroundColorOverride")]
    SetDefaultBackgroundColorOverride,
}
#[doc = "Sets or clears an override of the default background color of the frame. This override is used\nif the content does not specify one.\n[setDefaultBackgroundColorOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDefaultBackgroundColorOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDefaultBackgroundColorOverride {
    pub method: SetDefaultBackgroundColorOverrideMethod,
    pub params: SetDefaultBackgroundColorOverrideParams,
}
impl SetDefaultBackgroundColorOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setDefaultBackgroundColorOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetDefaultBackgroundColorOverride {
    type Result = super::results::SetDefaultBackgroundColorOverrideResult;
}
#[doc = "Overrides the values for env(safe-area-inset-*) and env(safe-area-max-inset-*). Unset values will cause the\nrespective variables to be undefined, even if previously overridden.\n[setSafeAreaInsetsOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setSafeAreaInsetsOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSafeAreaInsetsOverrideParams {
    #[serde(rename = "insets")]
    pub insets: super::types::SafeAreaInsets,
}
impl SetSafeAreaInsetsOverrideParams {
    pub fn new(insets: impl Into<super::types::SafeAreaInsets>) -> Self {
        Self {
            insets: insets.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetSafeAreaInsetsOverrideMethod {
    #[serde(rename = "Emulation.setSafeAreaInsetsOverride")]
    SetSafeAreaInsetsOverride,
}
#[doc = "Overrides the values for env(safe-area-inset-*) and env(safe-area-max-inset-*). Unset values will cause the\nrespective variables to be undefined, even if previously overridden.\n[setSafeAreaInsetsOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setSafeAreaInsetsOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSafeAreaInsetsOverride {
    pub method: SetSafeAreaInsetsOverrideMethod,
    pub params: SetSafeAreaInsetsOverrideParams,
}
impl SetSafeAreaInsetsOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setSafeAreaInsetsOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetSafeAreaInsetsOverride {
    type Result = super::results::SetSafeAreaInsetsOverrideResult;
}
#[doc = "Overrides the values of device screen dimensions (window.screen.width, window.screen.height,\nwindow.innerWidth, window.innerHeight, and \"device-width\"/\"device-height\"-related CSS media\nquery results).\n[setDeviceMetricsOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDeviceMetricsOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDeviceMetricsOverrideParams {
    #[doc = "Overriding width value in pixels (minimum 0, maximum 10000000). 0 disables the override."]
    #[serde(rename = "width")]
    pub width: i64,
    #[doc = "Overriding height value in pixels (minimum 0, maximum 10000000). 0 disables the override."]
    #[serde(rename = "height")]
    pub height: i64,
    #[doc = "Overriding device scale factor value. 0 disables the override."]
    #[serde(rename = "deviceScaleFactor")]
    pub device_scale_factor: f64,
    #[doc = "Whether to emulate mobile device. This includes viewport meta tag, overlay scrollbars, text\nautosizing and more."]
    #[serde(rename = "mobile")]
    pub mobile: bool,
    #[doc = "Scale to apply to resulting view image."]
    #[serde(rename = "scale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scale: Option<f64>,
    #[doc = "Overriding screen width value in pixels (minimum 0, maximum 10000000)."]
    #[serde(rename = "screenWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub screen_width: Option<i64>,
    #[doc = "Overriding screen height value in pixels (minimum 0, maximum 10000000)."]
    #[serde(rename = "screenHeight")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub screen_height: Option<i64>,
    #[doc = "Overriding view X position on screen in pixels (minimum 0, maximum 10000000)."]
    #[serde(rename = "positionX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub position_x: Option<i64>,
    #[doc = "Overriding view Y position on screen in pixels (minimum 0, maximum 10000000)."]
    #[serde(rename = "positionY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub position_y: Option<i64>,
    #[doc = "Do not set visible view size, rely upon explicit setVisibleSize call."]
    #[serde(rename = "dontSetVisibleSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dont_set_visible_size: Option<bool>,
    #[doc = "Screen orientation override."]
    #[serde(rename = "screenOrientation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub screen_orientation: Option<super::types::ScreenOrientation>,
    #[doc = "If set, the visible area of the page will be overridden to this viewport. This viewport\nchange is not observed by the page, e.g. viewport-relative elements do not change positions."]
    #[serde(rename = "viewport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub viewport: Option<crate::browser_protocol::page::types::Viewport>,
    #[doc = "Scrollbar type. Default: `default`."]
    #[serde(rename = "scrollbarType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scrollbar_type: Option<SetDeviceMetricsOverrideScrollbarType>,
}
#[doc = "Scrollbar type. Default: `default`."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetDeviceMetricsOverrideScrollbarType {
    #[doc = "Emulates scrollbars that float over the content, typically appearing\nonly when scrolling."]
    #[serde(rename = "overlay")]
    Overlay,
    #[doc = "Restores the platform's default scrollbar behavior, which might be\nclassic (occupying space within the layout) or overlay, depending\non the platform. Note: if `mobile` is `true`, the default scrollbar type is `overlay`."]
    #[serde(rename = "default")]
    Default,
}
impl SetDeviceMetricsOverrideParams {
    pub fn new(
        width: impl Into<i64>,
        height: impl Into<i64>,
        device_scale_factor: impl Into<f64>,
        mobile: impl Into<bool>,
    ) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
            device_scale_factor: device_scale_factor.into(),
            mobile: mobile.into(),
            scale: None,
            screen_width: None,
            screen_height: None,
            position_x: None,
            position_y: None,
            dont_set_visible_size: None,
            screen_orientation: None,
            viewport: None,
            scrollbar_type: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDeviceMetricsOverrideMethod {
    #[serde(rename = "Emulation.setDeviceMetricsOverride")]
    SetDeviceMetricsOverride,
}
#[doc = "Overrides the values of device screen dimensions (window.screen.width, window.screen.height,\nwindow.innerWidth, window.innerHeight, and \"device-width\"/\"device-height\"-related CSS media\nquery results).\n[setDeviceMetricsOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDeviceMetricsOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDeviceMetricsOverride {
    pub method: SetDeviceMetricsOverrideMethod,
    pub params: SetDeviceMetricsOverrideParams,
}
impl SetDeviceMetricsOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setDeviceMetricsOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetDeviceMetricsOverride {
    type Result = super::results::SetDeviceMetricsOverrideResult;
}
#[doc = "Start reporting the given posture value to the Device Posture API.\nThis override can also be set in setDeviceMetricsOverride().\n[setDevicePostureOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDevicePostureOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDevicePostureOverrideParams {
    #[serde(rename = "posture")]
    pub posture: super::types::DevicePosture,
}
impl SetDevicePostureOverrideParams {
    pub fn new(posture: impl Into<super::types::DevicePosture>) -> Self {
        Self {
            posture: posture.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDevicePostureOverrideMethod {
    #[serde(rename = "Emulation.setDevicePostureOverride")]
    SetDevicePostureOverride,
}
#[doc = "Start reporting the given posture value to the Device Posture API.\nThis override can also be set in setDeviceMetricsOverride().\n[setDevicePostureOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDevicePostureOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDevicePostureOverride {
    pub method: SetDevicePostureOverrideMethod,
    pub params: SetDevicePostureOverrideParams,
}
impl SetDevicePostureOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setDevicePostureOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetDevicePostureOverride {
    type Result = super::results::SetDevicePostureOverrideResult;
}
#[doc = "Clears a device posture override set with either setDeviceMetricsOverride()\nor setDevicePostureOverride() and starts using posture information from the\nplatform again.\nDoes nothing if no override is set.\n[clearDevicePostureOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearDevicePostureOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearDevicePostureOverrideParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearDevicePostureOverrideMethod {
    #[serde(rename = "Emulation.clearDevicePostureOverride")]
    ClearDevicePostureOverride,
}
#[doc = "Clears a device posture override set with either setDeviceMetricsOverride()\nor setDevicePostureOverride() and starts using posture information from the\nplatform again.\nDoes nothing if no override is set.\n[clearDevicePostureOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearDevicePostureOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearDevicePostureOverride {
    pub method: ClearDevicePostureOverrideMethod,
    pub params: ClearDevicePostureOverrideParams,
}
impl ClearDevicePostureOverride {
    pub const IDENTIFIER: &'static str = "Emulation.clearDevicePostureOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ClearDevicePostureOverride {
    type Result = super::results::ClearDevicePostureOverrideResult;
}
#[doc = "Start using the given display features to pupulate the Viewport Segments API.\nThis override can also be set in setDeviceMetricsOverride().\n[setDisplayFeaturesOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDisplayFeaturesOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDisplayFeaturesOverrideParams {
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<super::types::DisplayFeature>,
}
impl SetDisplayFeaturesOverrideParams {
    pub fn new(features: Vec<super::types::DisplayFeature>) -> Self {
        Self { features }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDisplayFeaturesOverrideMethod {
    #[serde(rename = "Emulation.setDisplayFeaturesOverride")]
    SetDisplayFeaturesOverride,
}
#[doc = "Start using the given display features to pupulate the Viewport Segments API.\nThis override can also be set in setDeviceMetricsOverride().\n[setDisplayFeaturesOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDisplayFeaturesOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDisplayFeaturesOverride {
    pub method: SetDisplayFeaturesOverrideMethod,
    pub params: SetDisplayFeaturesOverrideParams,
}
impl SetDisplayFeaturesOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setDisplayFeaturesOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetDisplayFeaturesOverride {
    type Result = super::results::SetDisplayFeaturesOverrideResult;
}
#[doc = "Clears the display features override set with either setDeviceMetricsOverride()\nor setDisplayFeaturesOverride() and starts using display features from the\nplatform again.\nDoes nothing if no override is set.\n[clearDisplayFeaturesOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearDisplayFeaturesOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearDisplayFeaturesOverrideParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearDisplayFeaturesOverrideMethod {
    #[serde(rename = "Emulation.clearDisplayFeaturesOverride")]
    ClearDisplayFeaturesOverride,
}
#[doc = "Clears the display features override set with either setDeviceMetricsOverride()\nor setDisplayFeaturesOverride() and starts using display features from the\nplatform again.\nDoes nothing if no override is set.\n[clearDisplayFeaturesOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearDisplayFeaturesOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearDisplayFeaturesOverride {
    pub method: ClearDisplayFeaturesOverrideMethod,
    pub params: ClearDisplayFeaturesOverrideParams,
}
impl ClearDisplayFeaturesOverride {
    pub const IDENTIFIER: &'static str = "Emulation.clearDisplayFeaturesOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ClearDisplayFeaturesOverride {
    type Result = super::results::ClearDisplayFeaturesOverrideResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetScrollbarsHiddenParams {
    #[doc = "Whether scrollbars should be always hidden."]
    #[serde(rename = "hidden")]
    pub hidden: bool,
}
impl SetScrollbarsHiddenParams {
    pub fn new(hidden: impl Into<bool>) -> Self {
        Self {
            hidden: hidden.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetScrollbarsHiddenMethod {
    #[serde(rename = "Emulation.setScrollbarsHidden")]
    SetScrollbarsHidden,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetScrollbarsHidden {
    pub method: SetScrollbarsHiddenMethod,
    pub params: SetScrollbarsHiddenParams,
}
impl SetScrollbarsHidden {
    pub const IDENTIFIER: &'static str = "Emulation.setScrollbarsHidden";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetScrollbarsHidden {
    type Result = super::results::SetScrollbarsHiddenResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDocumentCookieDisabledParams {
    #[doc = "Whether document.coookie API should be disabled."]
    #[serde(rename = "disabled")]
    pub disabled: bool,
}
impl SetDocumentCookieDisabledParams {
    pub fn new(disabled: impl Into<bool>) -> Self {
        Self {
            disabled: disabled.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDocumentCookieDisabledMethod {
    #[serde(rename = "Emulation.setDocumentCookieDisabled")]
    SetDocumentCookieDisabled,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDocumentCookieDisabled {
    pub method: SetDocumentCookieDisabledMethod,
    pub params: SetDocumentCookieDisabledParams,
}
impl SetDocumentCookieDisabled {
    pub const IDENTIFIER: &'static str = "Emulation.setDocumentCookieDisabled";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetDocumentCookieDisabled {
    type Result = super::results::SetDocumentCookieDisabledResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEmitTouchEventsForMouseParams {
    #[doc = "Whether touch emulation based on mouse input should be enabled."]
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[doc = "Touch/gesture events configuration. Default: current platform."]
    #[serde(rename = "configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub configuration: Option<SetEmitTouchEventsForMouseConfiguration>,
}
#[doc = "Touch/gesture events configuration. Default: current platform."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetEmitTouchEventsForMouseConfiguration {
    #[serde(rename = "mobile")]
    Mobile,
    #[serde(rename = "desktop")]
    Desktop,
}
impl SetEmitTouchEventsForMouseParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self {
            enabled: enabled.into(),
            configuration: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetEmitTouchEventsForMouseMethod {
    #[serde(rename = "Emulation.setEmitTouchEventsForMouse")]
    SetEmitTouchEventsForMouse,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEmitTouchEventsForMouse {
    pub method: SetEmitTouchEventsForMouseMethod,
    pub params: SetEmitTouchEventsForMouseParams,
}
impl SetEmitTouchEventsForMouse {
    pub const IDENTIFIER: &'static str = "Emulation.setEmitTouchEventsForMouse";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetEmitTouchEventsForMouse {
    type Result = super::results::SetEmitTouchEventsForMouseResult;
}
#[doc = "Emulates the given media type or media feature for CSS media queries.\n[setEmulatedMedia](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setEmulatedMedia)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetEmulatedMediaParams {
    #[doc = "Media type to emulate. Empty string disables the override."]
    #[serde(rename = "media")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub media: Option<String>,
    #[doc = "Media features to emulate."]
    #[serde(rename = "features")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub features: Option<Vec<super::types::MediaFeature>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetEmulatedMediaMethod {
    #[serde(rename = "Emulation.setEmulatedMedia")]
    SetEmulatedMedia,
}
#[doc = "Emulates the given media type or media feature for CSS media queries.\n[setEmulatedMedia](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setEmulatedMedia)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEmulatedMedia {
    pub method: SetEmulatedMediaMethod,
    pub params: SetEmulatedMediaParams,
}
impl SetEmulatedMedia {
    pub const IDENTIFIER: &'static str = "Emulation.setEmulatedMedia";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetEmulatedMedia {
    type Result = super::results::SetEmulatedMediaResult;
}
#[doc = "Emulates the given vision deficiency.\n[setEmulatedVisionDeficiency](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setEmulatedVisionDeficiency)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEmulatedVisionDeficiencyParams {
    #[doc = "Vision deficiency to emulate. Order: best-effort emulations come first, followed by any\nphysiologically accurate emulations for medically recognized color vision deficiencies."]
    #[serde(rename = "type")]
    pub r#type: SetEmulatedVisionDeficiencyType,
}
#[doc = "Vision deficiency to emulate. Order: best-effort emulations come first, followed by any\nphysiologically accurate emulations for medically recognized color vision deficiencies."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetEmulatedVisionDeficiencyType {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "blurredVision")]
    BlurredVision,
    #[serde(rename = "reducedContrast")]
    ReducedContrast,
    #[serde(rename = "achromatopsia")]
    Achromatopsia,
    #[serde(rename = "deuteranopia")]
    Deuteranopia,
    #[serde(rename = "protanopia")]
    Protanopia,
    #[serde(rename = "tritanopia")]
    Tritanopia,
}
impl SetEmulatedVisionDeficiencyParams {
    pub fn new(r#type: impl Into<SetEmulatedVisionDeficiencyType>) -> Self {
        Self {
            r#type: r#type.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetEmulatedVisionDeficiencyMethod {
    #[serde(rename = "Emulation.setEmulatedVisionDeficiency")]
    SetEmulatedVisionDeficiency,
}
#[doc = "Emulates the given vision deficiency.\n[setEmulatedVisionDeficiency](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setEmulatedVisionDeficiency)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEmulatedVisionDeficiency {
    pub method: SetEmulatedVisionDeficiencyMethod,
    pub params: SetEmulatedVisionDeficiencyParams,
}
impl SetEmulatedVisionDeficiency {
    pub const IDENTIFIER: &'static str = "Emulation.setEmulatedVisionDeficiency";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetEmulatedVisionDeficiency {
    type Result = super::results::SetEmulatedVisionDeficiencyResult;
}
#[doc = "Emulates the given OS text scale.\n[setEmulatedOSTextScale](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setEmulatedOSTextScale)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetEmulatedOsTextScaleParams {
    #[serde(rename = "scale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scale: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetEmulatedOsTextScaleMethod {
    #[serde(rename = "Emulation.setEmulatedOSTextScale")]
    SetEmulatedOsTextScale,
}
#[doc = "Emulates the given OS text scale.\n[setEmulatedOSTextScale](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setEmulatedOSTextScale)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetEmulatedOsTextScale {
    pub method: SetEmulatedOsTextScaleMethod,
    pub params: SetEmulatedOsTextScaleParams,
}
impl SetEmulatedOsTextScale {
    pub const IDENTIFIER: &'static str = "Emulation.setEmulatedOSTextScale";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetEmulatedOsTextScale {
    type Result = super::results::SetEmulatedOsTextScaleResult;
}
#[doc = "Overrides the Geolocation Position or Error. Omitting latitude, longitude or\naccuracy emulates position unavailable.\n[setGeolocationOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setGeolocationOverride)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetGeolocationOverrideParams {
    #[doc = "Mock latitude"]
    #[serde(rename = "latitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub latitude: Option<f64>,
    #[doc = "Mock longitude"]
    #[serde(rename = "longitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub longitude: Option<f64>,
    #[doc = "Mock accuracy"]
    #[serde(rename = "accuracy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub accuracy: Option<f64>,
    #[doc = "Mock altitude"]
    #[serde(rename = "altitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub altitude: Option<f64>,
    #[doc = "Mock altitudeAccuracy"]
    #[serde(rename = "altitudeAccuracy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub altitude_accuracy: Option<f64>,
    #[doc = "Mock heading"]
    #[serde(rename = "heading")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub heading: Option<f64>,
    #[doc = "Mock speed"]
    #[serde(rename = "speed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub speed: Option<f64>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetGeolocationOverrideMethod {
    #[serde(rename = "Emulation.setGeolocationOverride")]
    SetGeolocationOverride,
}
#[doc = "Overrides the Geolocation Position or Error. Omitting latitude, longitude or\naccuracy emulates position unavailable.\n[setGeolocationOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setGeolocationOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGeolocationOverride {
    pub method: SetGeolocationOverrideMethod,
    pub params: SetGeolocationOverrideParams,
}
impl SetGeolocationOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setGeolocationOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetGeolocationOverride {
    type Result = super::results::SetGeolocationOverrideResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOverriddenSensorInformationParams {
    #[serde(rename = "type")]
    pub r#type: super::types::SensorType,
}
impl GetOverriddenSensorInformationParams {
    pub fn new(r#type: impl Into<super::types::SensorType>) -> Self {
        Self {
            r#type: r#type.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetOverriddenSensorInformationMethod {
    #[serde(rename = "Emulation.getOverriddenSensorInformation")]
    GetOverriddenSensorInformation,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOverriddenSensorInformation {
    pub method: GetOverriddenSensorInformationMethod,
    pub params: GetOverriddenSensorInformationParams,
}
impl GetOverriddenSensorInformation {
    pub const IDENTIFIER: &'static str = "Emulation.getOverriddenSensorInformation";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetOverriddenSensorInformation {
    type Result = super::results::GetOverriddenSensorInformationResult;
}
#[doc = "Overrides a platform sensor of a given type. If |enabled| is true, calls to\nSensor.start() will use a virtual sensor as backend rather than fetching\ndata from a real hardware sensor. Otherwise, existing virtual\nsensor-backend Sensor objects will fire an error event and new calls to\nSensor.start() will attempt to use a real sensor instead.\n[setSensorOverrideEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setSensorOverrideEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSensorOverrideEnabledParams {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "type")]
    pub r#type: super::types::SensorType,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub metadata: Option<super::types::SensorMetadata>,
}
impl SetSensorOverrideEnabledParams {
    pub fn new(enabled: impl Into<bool>, r#type: impl Into<super::types::SensorType>) -> Self {
        Self {
            enabled: enabled.into(),
            r#type: r#type.into(),
            metadata: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetSensorOverrideEnabledMethod {
    #[serde(rename = "Emulation.setSensorOverrideEnabled")]
    SetSensorOverrideEnabled,
}
#[doc = "Overrides a platform sensor of a given type. If |enabled| is true, calls to\nSensor.start() will use a virtual sensor as backend rather than fetching\ndata from a real hardware sensor. Otherwise, existing virtual\nsensor-backend Sensor objects will fire an error event and new calls to\nSensor.start() will attempt to use a real sensor instead.\n[setSensorOverrideEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setSensorOverrideEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSensorOverrideEnabled {
    pub method: SetSensorOverrideEnabledMethod,
    pub params: SetSensorOverrideEnabledParams,
}
impl SetSensorOverrideEnabled {
    pub const IDENTIFIER: &'static str = "Emulation.setSensorOverrideEnabled";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetSensorOverrideEnabled {
    type Result = super::results::SetSensorOverrideEnabledResult;
}
#[doc = "Updates the sensor readings reported by a sensor type previously overridden\nby setSensorOverrideEnabled.\n[setSensorOverrideReadings](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setSensorOverrideReadings)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSensorOverrideReadingsParams {
    #[serde(rename = "type")]
    pub r#type: super::types::SensorType,
    #[serde(rename = "reading")]
    pub reading: super::types::SensorReading,
}
impl SetSensorOverrideReadingsParams {
    pub fn new(
        r#type: impl Into<super::types::SensorType>,
        reading: impl Into<super::types::SensorReading>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            reading: reading.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetSensorOverrideReadingsMethod {
    #[serde(rename = "Emulation.setSensorOverrideReadings")]
    SetSensorOverrideReadings,
}
#[doc = "Updates the sensor readings reported by a sensor type previously overridden\nby setSensorOverrideEnabled.\n[setSensorOverrideReadings](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setSensorOverrideReadings)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSensorOverrideReadings {
    pub method: SetSensorOverrideReadingsMethod,
    pub params: SetSensorOverrideReadingsParams,
}
impl SetSensorOverrideReadings {
    pub const IDENTIFIER: &'static str = "Emulation.setSensorOverrideReadings";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetSensorOverrideReadings {
    type Result = super::results::SetSensorOverrideReadingsResult;
}
#[doc = "Overrides a pressure source of a given type, as used by the Compute\nPressure API, so that updates to PressureObserver.observe() are provided\nvia setPressureStateOverride instead of being retrieved from\nplatform-provided telemetry data.\n[setPressureSourceOverrideEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setPressureSourceOverrideEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPressureSourceOverrideEnabledParams {
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(rename = "source")]
    pub source: super::types::PressureSource,
    #[serde(rename = "metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub metadata: Option<super::types::PressureMetadata>,
}
impl SetPressureSourceOverrideEnabledParams {
    pub fn new(enabled: impl Into<bool>, source: impl Into<super::types::PressureSource>) -> Self {
        Self {
            enabled: enabled.into(),
            source: source.into(),
            metadata: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetPressureSourceOverrideEnabledMethod {
    #[serde(rename = "Emulation.setPressureSourceOverrideEnabled")]
    SetPressureSourceOverrideEnabled,
}
#[doc = "Overrides a pressure source of a given type, as used by the Compute\nPressure API, so that updates to PressureObserver.observe() are provided\nvia setPressureStateOverride instead of being retrieved from\nplatform-provided telemetry data.\n[setPressureSourceOverrideEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setPressureSourceOverrideEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPressureSourceOverrideEnabled {
    pub method: SetPressureSourceOverrideEnabledMethod,
    pub params: SetPressureSourceOverrideEnabledParams,
}
impl SetPressureSourceOverrideEnabled {
    pub const IDENTIFIER: &'static str = "Emulation.setPressureSourceOverrideEnabled";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetPressureSourceOverrideEnabled {
    type Result = super::results::SetPressureSourceOverrideEnabledResult;
}
#[doc = "TODO: OBSOLETE: To remove when setPressureDataOverride is merged.\nProvides a given pressure state that will be processed and eventually be\ndelivered to PressureObserver users. |source| must have been previously\noverridden by setPressureSourceOverrideEnabled.\n[setPressureStateOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setPressureStateOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPressureStateOverrideParams {
    #[serde(rename = "source")]
    pub source: super::types::PressureSource,
    #[serde(rename = "state")]
    pub state: super::types::PressureState,
}
impl SetPressureStateOverrideParams {
    pub fn new(
        source: impl Into<super::types::PressureSource>,
        state: impl Into<super::types::PressureState>,
    ) -> Self {
        Self {
            source: source.into(),
            state: state.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetPressureStateOverrideMethod {
    #[serde(rename = "Emulation.setPressureStateOverride")]
    SetPressureStateOverride,
}
#[doc = "TODO: OBSOLETE: To remove when setPressureDataOverride is merged.\nProvides a given pressure state that will be processed and eventually be\ndelivered to PressureObserver users. |source| must have been previously\noverridden by setPressureSourceOverrideEnabled.\n[setPressureStateOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setPressureStateOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPressureStateOverride {
    pub method: SetPressureStateOverrideMethod,
    pub params: SetPressureStateOverrideParams,
}
impl SetPressureStateOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setPressureStateOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetPressureStateOverride {
    type Result = super::results::SetPressureStateOverrideResult;
}
#[doc = "Provides a given pressure data set that will be processed and eventually be\ndelivered to PressureObserver users. |source| must have been previously\noverridden by setPressureSourceOverrideEnabled.\n[setPressureDataOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setPressureDataOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPressureDataOverrideParams {
    #[serde(rename = "source")]
    pub source: super::types::PressureSource,
    #[serde(rename = "state")]
    pub state: super::types::PressureState,
    #[serde(rename = "ownContributionEstimate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub own_contribution_estimate: Option<f64>,
}
impl SetPressureDataOverrideParams {
    pub fn new(
        source: impl Into<super::types::PressureSource>,
        state: impl Into<super::types::PressureState>,
    ) -> Self {
        Self {
            source: source.into(),
            state: state.into(),
            own_contribution_estimate: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetPressureDataOverrideMethod {
    #[serde(rename = "Emulation.setPressureDataOverride")]
    SetPressureDataOverride,
}
#[doc = "Provides a given pressure data set that will be processed and eventually be\ndelivered to PressureObserver users. |source| must have been previously\noverridden by setPressureSourceOverrideEnabled.\n[setPressureDataOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setPressureDataOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPressureDataOverride {
    pub method: SetPressureDataOverrideMethod,
    pub params: SetPressureDataOverrideParams,
}
impl SetPressureDataOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setPressureDataOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetPressureDataOverride {
    type Result = super::results::SetPressureDataOverrideResult;
}
#[doc = "Overrides the Idle state.\n[setIdleOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setIdleOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetIdleOverrideParams {
    #[doc = "Mock isUserActive"]
    #[serde(rename = "isUserActive")]
    pub is_user_active: bool,
    #[doc = "Mock isScreenUnlocked"]
    #[serde(rename = "isScreenUnlocked")]
    pub is_screen_unlocked: bool,
}
impl SetIdleOverrideParams {
    pub fn new(is_user_active: impl Into<bool>, is_screen_unlocked: impl Into<bool>) -> Self {
        Self {
            is_user_active: is_user_active.into(),
            is_screen_unlocked: is_screen_unlocked.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetIdleOverrideMethod {
    #[serde(rename = "Emulation.setIdleOverride")]
    SetIdleOverride,
}
#[doc = "Overrides the Idle state.\n[setIdleOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setIdleOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetIdleOverride {
    pub method: SetIdleOverrideMethod,
    pub params: SetIdleOverrideParams,
}
impl SetIdleOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setIdleOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetIdleOverride {
    type Result = super::results::SetIdleOverrideResult;
}
#[doc = "Clears Idle state overrides.\n[clearIdleOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearIdleOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearIdleOverrideParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearIdleOverrideMethod {
    #[serde(rename = "Emulation.clearIdleOverride")]
    ClearIdleOverride,
}
#[doc = "Clears Idle state overrides.\n[clearIdleOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearIdleOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClearIdleOverride {
    pub method: ClearIdleOverrideMethod,
    pub params: ClearIdleOverrideParams,
}
impl ClearIdleOverride {
    pub const IDENTIFIER: &'static str = "Emulation.clearIdleOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ClearIdleOverride {
    type Result = super::results::ClearIdleOverrideResult;
}
#[doc = "Sets a specified page scale factor.\n[setPageScaleFactor](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setPageScaleFactor)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPageScaleFactorParams {
    #[doc = "Page scale factor."]
    #[serde(rename = "pageScaleFactor")]
    pub page_scale_factor: f64,
}
impl SetPageScaleFactorParams {
    pub fn new(page_scale_factor: impl Into<f64>) -> Self {
        Self {
            page_scale_factor: page_scale_factor.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetPageScaleFactorMethod {
    #[serde(rename = "Emulation.setPageScaleFactor")]
    SetPageScaleFactor,
}
#[doc = "Sets a specified page scale factor.\n[setPageScaleFactor](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setPageScaleFactor)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPageScaleFactor {
    pub method: SetPageScaleFactorMethod,
    pub params: SetPageScaleFactorParams,
}
impl SetPageScaleFactor {
    pub const IDENTIFIER: &'static str = "Emulation.setPageScaleFactor";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetPageScaleFactor {
    type Result = super::results::SetPageScaleFactorResult;
}
#[doc = "Switches script execution in the page.\n[setScriptExecutionDisabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setScriptExecutionDisabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetScriptExecutionDisabledParams {
    #[doc = "Whether script execution should be disabled in the page."]
    #[serde(rename = "value")]
    pub value: bool,
}
impl SetScriptExecutionDisabledParams {
    pub fn new(value: impl Into<bool>) -> Self {
        Self {
            value: value.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetScriptExecutionDisabledMethod {
    #[serde(rename = "Emulation.setScriptExecutionDisabled")]
    SetScriptExecutionDisabled,
}
#[doc = "Switches script execution in the page.\n[setScriptExecutionDisabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setScriptExecutionDisabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetScriptExecutionDisabled {
    pub method: SetScriptExecutionDisabledMethod,
    pub params: SetScriptExecutionDisabledParams,
}
impl SetScriptExecutionDisabled {
    pub const IDENTIFIER: &'static str = "Emulation.setScriptExecutionDisabled";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetScriptExecutionDisabled {
    type Result = super::results::SetScriptExecutionDisabledResult;
}
#[doc = "Enables touch on platforms which do not support them.\n[setTouchEmulationEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setTouchEmulationEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetTouchEmulationEnabledParams {
    #[doc = "Whether the touch event emulation should be enabled."]
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[doc = "Maximum touch points supported. Defaults to one."]
    #[serde(rename = "maxTouchPoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_touch_points: Option<i64>,
}
impl SetTouchEmulationEnabledParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self {
            enabled: enabled.into(),
            max_touch_points: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetTouchEmulationEnabledMethod {
    #[serde(rename = "Emulation.setTouchEmulationEnabled")]
    SetTouchEmulationEnabled,
}
#[doc = "Enables touch on platforms which do not support them.\n[setTouchEmulationEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setTouchEmulationEnabled)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetTouchEmulationEnabled {
    pub method: SetTouchEmulationEnabledMethod,
    pub params: SetTouchEmulationEnabledParams,
}
impl SetTouchEmulationEnabled {
    pub const IDENTIFIER: &'static str = "Emulation.setTouchEmulationEnabled";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetTouchEmulationEnabled {
    type Result = super::results::SetTouchEmulationEnabledResult;
}
#[doc = "Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets\nthe current virtual time policy.  Note this supersedes any previous time budget.\n[setVirtualTimePolicy](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setVirtualTimePolicy)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetVirtualTimePolicyParams {
    #[serde(rename = "policy")]
    pub policy: super::types::VirtualTimePolicy,
    #[doc = "If set, after this many virtual milliseconds have elapsed virtual time will be paused and a\nvirtualTimeBudgetExpired event is sent."]
    #[serde(rename = "budget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub budget: Option<f64>,
    #[doc = "If set this specifies the maximum number of tasks that can be run before virtual is forced\nforwards to prevent deadlock."]
    #[serde(rename = "maxVirtualTimeTaskStarvationCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub max_virtual_time_task_starvation_count: Option<i64>,
    #[doc = "If set, base::Time::Now will be overridden to initially return this value."]
    #[serde(rename = "initialVirtualTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub initial_virtual_time: Option<crate::browser_protocol::network::types::TimeSinceEpoch>,
}
impl SetVirtualTimePolicyParams {
    pub fn new(policy: impl Into<super::types::VirtualTimePolicy>) -> Self {
        Self {
            policy: policy.into(),
            budget: None,
            max_virtual_time_task_starvation_count: None,
            initial_virtual_time: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetVirtualTimePolicyMethod {
    #[serde(rename = "Emulation.setVirtualTimePolicy")]
    SetVirtualTimePolicy,
}
#[doc = "Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets\nthe current virtual time policy.  Note this supersedes any previous time budget.\n[setVirtualTimePolicy](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setVirtualTimePolicy)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetVirtualTimePolicy {
    pub method: SetVirtualTimePolicyMethod,
    pub params: SetVirtualTimePolicyParams,
}
impl SetVirtualTimePolicy {
    pub const IDENTIFIER: &'static str = "Emulation.setVirtualTimePolicy";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetVirtualTimePolicy {
    type Result = super::results::SetVirtualTimePolicyResult;
}
#[doc = "Overrides default host system locale with the specified one.\n[setLocaleOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setLocaleOverride)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetLocaleOverrideParams {
    #[doc = "ICU style C locale (e.g. \"en_US\"). If not specified or empty, disables the override and\nrestores default host system locale."]
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub locale: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetLocaleOverrideMethod {
    #[serde(rename = "Emulation.setLocaleOverride")]
    SetLocaleOverride,
}
#[doc = "Overrides default host system locale with the specified one.\n[setLocaleOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setLocaleOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLocaleOverride {
    pub method: SetLocaleOverrideMethod,
    pub params: SetLocaleOverrideParams,
}
impl SetLocaleOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setLocaleOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetLocaleOverride {
    type Result = super::results::SetLocaleOverrideResult;
}
#[doc = "Overrides default host system timezone with the specified one.\n[setTimezoneOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setTimezoneOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetTimezoneOverrideParams {
    #[doc = "The timezone identifier. List of supported timezones:\nhttps://source.chromium.org/chromium/chromium/deps/icu.git/+/faee8bc70570192d82d2978a71e2a615788597d1:source/data/misc/metaZones.txt\nIf empty, disables the override and restores default host system timezone."]
    #[serde(rename = "timezoneId")]
    pub timezone_id: String,
}
impl SetTimezoneOverrideParams {
    pub fn new(timezone_id: impl Into<String>) -> Self {
        Self {
            timezone_id: timezone_id.into(),
        }
    }
}
impl<T: Into<String>> From<T> for SetTimezoneOverrideParams {
    fn from(url: T) -> Self {
        SetTimezoneOverrideParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetTimezoneOverrideMethod {
    #[serde(rename = "Emulation.setTimezoneOverride")]
    SetTimezoneOverride,
}
#[doc = "Overrides default host system timezone with the specified one.\n[setTimezoneOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setTimezoneOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetTimezoneOverride {
    pub method: SetTimezoneOverrideMethod,
    pub params: SetTimezoneOverrideParams,
}
impl SetTimezoneOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setTimezoneOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetTimezoneOverride {
    type Result = super::results::SetTimezoneOverrideResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDisabledImageTypesParams {
    #[doc = "Image types to disable."]
    #[serde(rename = "imageTypes")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub image_types: Vec<super::types::DisabledImageType>,
}
impl SetDisabledImageTypesParams {
    pub fn new(image_types: Vec<super::types::DisabledImageType>) -> Self {
        Self { image_types }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDisabledImageTypesMethod {
    #[serde(rename = "Emulation.setDisabledImageTypes")]
    SetDisabledImageTypes,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDisabledImageTypes {
    pub method: SetDisabledImageTypesMethod,
    pub params: SetDisabledImageTypesParams,
}
impl SetDisabledImageTypes {
    pub const IDENTIFIER: &'static str = "Emulation.setDisabledImageTypes";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetDisabledImageTypes {
    type Result = super::results::SetDisabledImageTypesResult;
}
#[doc = "Override the value of navigator.connection.saveData\n[setDataSaverOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDataSaverOverride)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDataSaverOverrideParams {
    #[doc = "Override value. Omitting the parameter disables the override."]
    #[serde(rename = "dataSaverEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data_saver_enabled: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDataSaverOverrideMethod {
    #[serde(rename = "Emulation.setDataSaverOverride")]
    SetDataSaverOverride,
}
#[doc = "Override the value of navigator.connection.saveData\n[setDataSaverOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDataSaverOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDataSaverOverride {
    pub method: SetDataSaverOverrideMethod,
    pub params: SetDataSaverOverrideParams,
}
impl SetDataSaverOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setDataSaverOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetDataSaverOverride {
    type Result = super::results::SetDataSaverOverrideResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHardwareConcurrencyOverrideParams {
    #[doc = "Hardware concurrency to report"]
    #[serde(rename = "hardwareConcurrency")]
    pub hardware_concurrency: i64,
}
impl SetHardwareConcurrencyOverrideParams {
    pub fn new(hardware_concurrency: impl Into<i64>) -> Self {
        Self {
            hardware_concurrency: hardware_concurrency.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetHardwareConcurrencyOverrideMethod {
    #[serde(rename = "Emulation.setHardwareConcurrencyOverride")]
    SetHardwareConcurrencyOverride,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetHardwareConcurrencyOverride {
    pub method: SetHardwareConcurrencyOverrideMethod,
    pub params: SetHardwareConcurrencyOverrideParams,
}
impl SetHardwareConcurrencyOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setHardwareConcurrencyOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetHardwareConcurrencyOverride {
    type Result = super::results::SetHardwareConcurrencyOverrideResult;
}
#[doc = "Allows overriding user agent with the given string.\n`userAgentMetadata` must be set for Client Hint headers to be sent.\n[setUserAgentOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setUserAgentOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetUserAgentOverrideParams {
    #[doc = "User agent to use."]
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    #[doc = "Browser language to emulate."]
    #[serde(rename = "acceptLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub accept_language: Option<String>,
    #[doc = "The platform navigator.platform should return."]
    #[serde(rename = "platform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub platform: Option<String>,
    #[doc = "To be sent in Sec-CH-UA-* headers and returned in navigator.userAgentData"]
    #[serde(rename = "userAgentMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_agent_metadata: Option<super::types::UserAgentMetadata>,
}
impl SetUserAgentOverrideParams {
    pub fn new(user_agent: impl Into<String>) -> Self {
        Self {
            user_agent: user_agent.into(),
            accept_language: None,
            platform: None,
            user_agent_metadata: None,
        }
    }
}
impl<T: Into<String>> From<T> for SetUserAgentOverrideParams {
    fn from(url: T) -> Self {
        SetUserAgentOverrideParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetUserAgentOverrideMethod {
    #[serde(rename = "Emulation.setUserAgentOverride")]
    SetUserAgentOverride,
}
#[doc = "Allows overriding user agent with the given string.\n`userAgentMetadata` must be set for Client Hint headers to be sent.\n[setUserAgentOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setUserAgentOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetUserAgentOverride {
    pub method: SetUserAgentOverrideMethod,
    pub params: SetUserAgentOverrideParams,
}
impl SetUserAgentOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setUserAgentOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetUserAgentOverride {
    type Result = super::results::SetUserAgentOverrideResult;
}
#[doc = "Allows overriding the automation flag.\n[setAutomationOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setAutomationOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAutomationOverrideParams {
    #[doc = "Whether the override should be enabled."]
    #[serde(rename = "enabled")]
    pub enabled: bool,
}
impl SetAutomationOverrideParams {
    pub fn new(enabled: impl Into<bool>) -> Self {
        Self {
            enabled: enabled.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetAutomationOverrideMethod {
    #[serde(rename = "Emulation.setAutomationOverride")]
    SetAutomationOverride,
}
#[doc = "Allows overriding the automation flag.\n[setAutomationOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setAutomationOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetAutomationOverride {
    pub method: SetAutomationOverrideMethod,
    pub params: SetAutomationOverrideParams,
}
impl SetAutomationOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setAutomationOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetAutomationOverride {
    type Result = super::results::SetAutomationOverrideResult;
}
#[doc = "Allows overriding the difference between the small and large viewport sizes, which determine the\nvalue of the `svh` and `lvh` unit, respectively. Only supported for top-level frames.\n[setSmallViewportHeightDifferenceOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setSmallViewportHeightDifferenceOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSmallViewportHeightDifferenceOverrideParams {
    #[doc = "This will cause an element of size 100svh to be `difference` pixels smaller than an element\nof size 100lvh."]
    #[serde(rename = "difference")]
    pub difference: i64,
}
impl SetSmallViewportHeightDifferenceOverrideParams {
    pub fn new(difference: impl Into<i64>) -> Self {
        Self {
            difference: difference.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetSmallViewportHeightDifferenceOverrideMethod {
    #[serde(rename = "Emulation.setSmallViewportHeightDifferenceOverride")]
    SetSmallViewportHeightDifferenceOverride,
}
#[doc = "Allows overriding the difference between the small and large viewport sizes, which determine the\nvalue of the `svh` and `lvh` unit, respectively. Only supported for top-level frames.\n[setSmallViewportHeightDifferenceOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setSmallViewportHeightDifferenceOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetSmallViewportHeightDifferenceOverride {
    pub method: SetSmallViewportHeightDifferenceOverrideMethod,
    pub params: SetSmallViewportHeightDifferenceOverrideParams,
}
impl SetSmallViewportHeightDifferenceOverride {
    pub const IDENTIFIER: &'static str = "Emulation.setSmallViewportHeightDifferenceOverride";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetSmallViewportHeightDifferenceOverride {
    type Result = super::results::SetSmallViewportHeightDifferenceOverrideResult;
}
#[doc = "Returns device's screen configuration. In headful mode, the physical screens configuration is returned,\nwhereas in headless mode, a virtual headless screen configuration is provided instead.\n[getScreenInfos](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-getScreenInfos)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScreenInfosParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetScreenInfosMethod {
    #[serde(rename = "Emulation.getScreenInfos")]
    GetScreenInfos,
}
#[doc = "Returns device's screen configuration. In headful mode, the physical screens configuration is returned,\nwhereas in headless mode, a virtual headless screen configuration is provided instead.\n[getScreenInfos](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-getScreenInfos)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetScreenInfos {
    pub method: GetScreenInfosMethod,
    pub params: GetScreenInfosParams,
}
impl GetScreenInfos {
    pub const IDENTIFIER: &'static str = "Emulation.getScreenInfos";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for GetScreenInfos {
    type Result = super::results::GetScreenInfosResult;
}
#[doc = "Add a new screen to the device. Only supported in headless mode.\n[addScreen](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-addScreen)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddScreenParams {
    #[doc = "Offset of the left edge of the screen in pixels."]
    #[serde(rename = "left")]
    pub left: i64,
    #[doc = "Offset of the top edge of the screen in pixels."]
    #[serde(rename = "top")]
    pub top: i64,
    #[doc = "The width of the screen in pixels."]
    #[serde(rename = "width")]
    pub width: i64,
    #[doc = "The height of the screen in pixels."]
    #[serde(rename = "height")]
    pub height: i64,
    #[doc = "Specifies the screen's work area. Default is entire screen."]
    #[serde(rename = "workAreaInsets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub work_area_insets: Option<super::types::WorkAreaInsets>,
    #[doc = "Specifies the screen's device pixel ratio. Default is 1."]
    #[serde(rename = "devicePixelRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub device_pixel_ratio: Option<f64>,
    #[doc = "Specifies the screen's rotation angle. Available values are 0, 90, 180 and 270. Default is 0."]
    #[serde(rename = "rotation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub rotation: Option<i64>,
    #[doc = "Specifies the screen's color depth in bits. Default is 24."]
    #[serde(rename = "colorDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub color_depth: Option<i64>,
    #[doc = "Specifies the descriptive label for the screen. Default is none."]
    #[serde(rename = "label")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub label: Option<String>,
    #[doc = "Indicates whether the screen is internal to the device or external, attached to the device. Default is false."]
    #[serde(rename = "isInternal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub is_internal: Option<bool>,
}
impl AddScreenParams {
    pub fn new(
        left: impl Into<i64>,
        top: impl Into<i64>,
        width: impl Into<i64>,
        height: impl Into<i64>,
    ) -> Self {
        Self {
            left: left.into(),
            top: top.into(),
            width: width.into(),
            height: height.into(),
            work_area_insets: None,
            device_pixel_ratio: None,
            rotation: None,
            color_depth: None,
            label: None,
            is_internal: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddScreenMethod {
    #[serde(rename = "Emulation.addScreen")]
    AddScreen,
}
#[doc = "Add a new screen to the device. Only supported in headless mode.\n[addScreen](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-addScreen)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddScreen {
    pub method: AddScreenMethod,
    pub params: AddScreenParams,
}
impl AddScreen {
    pub const IDENTIFIER: &'static str = "Emulation.addScreen";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for AddScreen {
    type Result = super::results::AddScreenResult;
}
#[doc = "Remove screen from the device. Only supported in headless mode.\n[removeScreen](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-removeScreen)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveScreenParams {
    #[serde(rename = "screenId")]
    pub screen_id: super::types::ScreenId,
}
impl RemoveScreenParams {
    pub fn new(screen_id: impl Into<super::types::ScreenId>) -> Self {
        Self {
            screen_id: screen_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RemoveScreenMethod {
    #[serde(rename = "Emulation.removeScreen")]
    RemoveScreen,
}
#[doc = "Remove screen from the device. Only supported in headless mode.\n[removeScreen](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-removeScreen)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveScreen {
    pub method: RemoveScreenMethod,
    pub params: RemoveScreenParams,
}
impl RemoveScreen {
    pub const IDENTIFIER: &'static str = "Emulation.removeScreen";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for RemoveScreen {
    type Result = super::results::RemoveScreenResult;
}
group_enum ! (EmulationCommands { ClearDeviceMetricsOverride (ClearDeviceMetricsOverride) , ClearGeolocationOverride (ClearGeolocationOverride) , ResetPageScaleFactor (ResetPageScaleFactor) , SetFocusEmulationEnabled (SetFocusEmulationEnabled) , SetAutoDarkModeOverride (SetAutoDarkModeOverride) , SetCpuThrottlingRate (SetCpuThrottlingRate) , SetDefaultBackgroundColorOverride (SetDefaultBackgroundColorOverride) , SetSafeAreaInsetsOverride (SetSafeAreaInsetsOverride) , SetDeviceMetricsOverride (SetDeviceMetricsOverride) , SetDevicePostureOverride (SetDevicePostureOverride) , ClearDevicePostureOverride (ClearDevicePostureOverride) , SetDisplayFeaturesOverride (SetDisplayFeaturesOverride) , ClearDisplayFeaturesOverride (ClearDisplayFeaturesOverride) , SetScrollbarsHidden (SetScrollbarsHidden) , SetDocumentCookieDisabled (SetDocumentCookieDisabled) , SetEmitTouchEventsForMouse (SetEmitTouchEventsForMouse) , SetEmulatedMedia (SetEmulatedMedia) , SetEmulatedVisionDeficiency (SetEmulatedVisionDeficiency) , SetEmulatedOsTextScale (SetEmulatedOsTextScale) , SetGeolocationOverride (SetGeolocationOverride) , GetOverriddenSensorInformation (GetOverriddenSensorInformation) , SetSensorOverrideEnabled (SetSensorOverrideEnabled) , SetSensorOverrideReadings (SetSensorOverrideReadings) , SetPressureSourceOverrideEnabled (SetPressureSourceOverrideEnabled) , SetPressureStateOverride (SetPressureStateOverride) , SetPressureDataOverride (SetPressureDataOverride) , SetIdleOverride (SetIdleOverride) , ClearIdleOverride (ClearIdleOverride) , SetPageScaleFactor (SetPageScaleFactor) , SetScriptExecutionDisabled (SetScriptExecutionDisabled) , SetTouchEmulationEnabled (SetTouchEmulationEnabled) , SetVirtualTimePolicy (SetVirtualTimePolicy) , SetLocaleOverride (SetLocaleOverride) , SetTimezoneOverride (SetTimezoneOverride) , SetDisabledImageTypes (SetDisabledImageTypes) , SetDataSaverOverride (SetDataSaverOverride) , SetHardwareConcurrencyOverride (SetHardwareConcurrencyOverride) , SetUserAgentOverride (SetUserAgentOverride) , SetAutomationOverride (SetAutomationOverride) , SetSmallViewportHeightDifferenceOverride (SetSmallViewportHeightDifferenceOverride) , GetScreenInfos (GetScreenInfos) , AddScreen (AddScreen) , RemoveScreen (RemoveScreen) } + identifiable);
