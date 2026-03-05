use serde::{Deserialize, Serialize};
#[doc = "Clears the overridden device metrics.\n[clearDeviceMetricsOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearDeviceMetricsOverride)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearDeviceMetricsOverrideParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearDeviceMetricsOverrideMethod {
    #[serde(rename = "Emulation.clearDeviceMetricsOverride")]
    ClearDeviceMetricsOverride,
}
impl ClearDeviceMetricsOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.clearDeviceMetricsOverride";
}
#[doc = "Clears the overridden device metrics.\n[clearDeviceMetricsOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearDeviceMetricsOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearDeviceMetricsOverride {
    pub method: ClearDeviceMetricsOverrideMethod,
    pub params: ClearDeviceMetricsOverrideParams,
}
#[doc = "Clears the overridden Geolocation Position and Error.\n[clearGeolocationOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearGeolocationOverride)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearGeolocationOverrideParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearGeolocationOverrideMethod {
    #[serde(rename = "Emulation.clearGeolocationOverride")]
    ClearGeolocationOverride,
}
impl ClearGeolocationOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.clearGeolocationOverride";
}
#[doc = "Clears the overridden Geolocation Position and Error.\n[clearGeolocationOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearGeolocationOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearGeolocationOverride {
    pub method: ClearGeolocationOverrideMethod,
    pub params: ClearGeolocationOverrideParams,
}
#[doc = "Requests that page scale factor is reset to initial values.\n[resetPageScaleFactor](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-resetPageScaleFactor)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResetPageScaleFactorParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResetPageScaleFactorMethod {
    #[serde(rename = "Emulation.resetPageScaleFactor")]
    ResetPageScaleFactor,
}
impl ResetPageScaleFactorMethod {
    pub const IDENTIFIER: &'static str = "Emulation.resetPageScaleFactor";
}
#[doc = "Requests that page scale factor is reset to initial values.\n[resetPageScaleFactor](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-resetPageScaleFactor)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ResetPageScaleFactor {
    pub method: ResetPageScaleFactorMethod,
    pub params: ResetPageScaleFactorParams,
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
impl SetFocusEmulationEnabledMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setFocusEmulationEnabled";
}
#[doc = "Enables or disables simulating a focused and active page.\n[setFocusEmulationEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setFocusEmulationEnabled)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetFocusEmulationEnabled {
    pub method: SetFocusEmulationEnabledMethod,
    pub params: SetFocusEmulationEnabledParams,
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
impl SetAutoDarkModeOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setAutoDarkModeOverride";
}
#[doc = "Automatically render all web contents using a dark theme.\n[setAutoDarkModeOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setAutoDarkModeOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetAutoDarkModeOverride {
    pub method: SetAutoDarkModeOverrideMethod,
    pub params: SetAutoDarkModeOverrideParams,
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
impl SetCpuThrottlingRateMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setCPUThrottlingRate";
}
#[doc = "Enables CPU throttling to emulate slow CPUs.\n[setCPUThrottlingRate](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setCPUThrottlingRate)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetCpuThrottlingRate {
    pub method: SetCpuThrottlingRateMethod,
    pub params: SetCpuThrottlingRateParams,
}
#[doc = "Sets or clears an override of the default background color of the frame. This override is used\nif the content does not specify one.\n[setDefaultBackgroundColorOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDefaultBackgroundColorOverride)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDefaultBackgroundColorOverrideParams {
    #[doc = "RGBA of the default background color. If not specified, any existing override will be\ncleared."]
    #[serde(rename = "color")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub color: Option<super::super::dom::types::Rgba>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDefaultBackgroundColorOverrideMethod {
    #[serde(rename = "Emulation.setDefaultBackgroundColorOverride")]
    SetDefaultBackgroundColorOverride,
}
impl SetDefaultBackgroundColorOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setDefaultBackgroundColorOverride";
}
#[doc = "Sets or clears an override of the default background color of the frame. This override is used\nif the content does not specify one.\n[setDefaultBackgroundColorOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDefaultBackgroundColorOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetDefaultBackgroundColorOverride {
    pub method: SetDefaultBackgroundColorOverrideMethod,
    pub params: SetDefaultBackgroundColorOverrideParams,
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
impl SetSafeAreaInsetsOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setSafeAreaInsetsOverride";
}
#[doc = "Overrides the values for env(safe-area-inset-*) and env(safe-area-max-inset-*). Unset values will cause the\nrespective variables to be undefined, even if previously overridden.\n[setSafeAreaInsetsOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setSafeAreaInsetsOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetSafeAreaInsetsOverride {
    pub method: SetSafeAreaInsetsOverrideMethod,
    pub params: SetSafeAreaInsetsOverrideParams,
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
    pub viewport: Option<super::super::page::types::Viewport>,
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
impl SetDeviceMetricsOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setDeviceMetricsOverride";
}
#[doc = "Overrides the values of device screen dimensions (window.screen.width, window.screen.height,\nwindow.innerWidth, window.innerHeight, and \"device-width\"/\"device-height\"-related CSS media\nquery results).\n[setDeviceMetricsOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDeviceMetricsOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetDeviceMetricsOverride {
    pub method: SetDeviceMetricsOverrideMethod,
    pub params: SetDeviceMetricsOverrideParams,
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
impl SetDevicePostureOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setDevicePostureOverride";
}
#[doc = "Start reporting the given posture value to the Device Posture API.\nThis override can also be set in setDeviceMetricsOverride().\n[setDevicePostureOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDevicePostureOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetDevicePostureOverride {
    pub method: SetDevicePostureOverrideMethod,
    pub params: SetDevicePostureOverrideParams,
}
#[doc = "Clears a device posture override set with either setDeviceMetricsOverride()\nor setDevicePostureOverride() and starts using posture information from the\nplatform again.\nDoes nothing if no override is set.\n[clearDevicePostureOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearDevicePostureOverride)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearDevicePostureOverrideParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearDevicePostureOverrideMethod {
    #[serde(rename = "Emulation.clearDevicePostureOverride")]
    ClearDevicePostureOverride,
}
impl ClearDevicePostureOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.clearDevicePostureOverride";
}
#[doc = "Clears a device posture override set with either setDeviceMetricsOverride()\nor setDevicePostureOverride() and starts using posture information from the\nplatform again.\nDoes nothing if no override is set.\n[clearDevicePostureOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearDevicePostureOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearDevicePostureOverride {
    pub method: ClearDevicePostureOverrideMethod,
    pub params: ClearDevicePostureOverrideParams,
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
impl SetDisplayFeaturesOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setDisplayFeaturesOverride";
}
#[doc = "Start using the given display features to pupulate the Viewport Segments API.\nThis override can also be set in setDeviceMetricsOverride().\n[setDisplayFeaturesOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDisplayFeaturesOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetDisplayFeaturesOverride {
    pub method: SetDisplayFeaturesOverrideMethod,
    pub params: SetDisplayFeaturesOverrideParams,
}
#[doc = "Clears the display features override set with either setDeviceMetricsOverride()\nor setDisplayFeaturesOverride() and starts using display features from the\nplatform again.\nDoes nothing if no override is set.\n[clearDisplayFeaturesOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearDisplayFeaturesOverride)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearDisplayFeaturesOverrideParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearDisplayFeaturesOverrideMethod {
    #[serde(rename = "Emulation.clearDisplayFeaturesOverride")]
    ClearDisplayFeaturesOverride,
}
impl ClearDisplayFeaturesOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.clearDisplayFeaturesOverride";
}
#[doc = "Clears the display features override set with either setDeviceMetricsOverride()\nor setDisplayFeaturesOverride() and starts using display features from the\nplatform again.\nDoes nothing if no override is set.\n[clearDisplayFeaturesOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearDisplayFeaturesOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearDisplayFeaturesOverride {
    pub method: ClearDisplayFeaturesOverrideMethod,
    pub params: ClearDisplayFeaturesOverrideParams,
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
impl SetScrollbarsHiddenMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setScrollbarsHidden";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetScrollbarsHidden {
    pub method: SetScrollbarsHiddenMethod,
    pub params: SetScrollbarsHiddenParams,
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
impl SetDocumentCookieDisabledMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setDocumentCookieDisabled";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetDocumentCookieDisabled {
    pub method: SetDocumentCookieDisabledMethod,
    pub params: SetDocumentCookieDisabledParams,
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
impl SetEmitTouchEventsForMouseMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setEmitTouchEventsForMouse";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetEmitTouchEventsForMouse {
    pub method: SetEmitTouchEventsForMouseMethod,
    pub params: SetEmitTouchEventsForMouseParams,
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
impl SetEmulatedMediaMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setEmulatedMedia";
}
#[doc = "Emulates the given media type or media feature for CSS media queries.\n[setEmulatedMedia](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setEmulatedMedia)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetEmulatedMedia {
    pub method: SetEmulatedMediaMethod,
    pub params: SetEmulatedMediaParams,
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
impl SetEmulatedVisionDeficiencyMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setEmulatedVisionDeficiency";
}
#[doc = "Emulates the given vision deficiency.\n[setEmulatedVisionDeficiency](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setEmulatedVisionDeficiency)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetEmulatedVisionDeficiency {
    pub method: SetEmulatedVisionDeficiencyMethod,
    pub params: SetEmulatedVisionDeficiencyParams,
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
impl SetEmulatedOsTextScaleMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setEmulatedOSTextScale";
}
#[doc = "Emulates the given OS text scale.\n[setEmulatedOSTextScale](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setEmulatedOSTextScale)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetEmulatedOsTextScale {
    pub method: SetEmulatedOsTextScaleMethod,
    pub params: SetEmulatedOsTextScaleParams,
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
impl SetGeolocationOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setGeolocationOverride";
}
#[doc = "Overrides the Geolocation Position or Error. Omitting latitude, longitude or\naccuracy emulates position unavailable.\n[setGeolocationOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setGeolocationOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetGeolocationOverride {
    pub method: SetGeolocationOverrideMethod,
    pub params: SetGeolocationOverrideParams,
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
impl GetOverriddenSensorInformationMethod {
    pub const IDENTIFIER: &'static str = "Emulation.getOverriddenSensorInformation";
}
#[derive(Debug, Clone, PartialEq)]
pub struct GetOverriddenSensorInformation {
    pub method: GetOverriddenSensorInformationMethod,
    pub params: GetOverriddenSensorInformationParams,
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
impl SetSensorOverrideEnabledMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setSensorOverrideEnabled";
}
#[doc = "Overrides a platform sensor of a given type. If |enabled| is true, calls to\nSensor.start() will use a virtual sensor as backend rather than fetching\ndata from a real hardware sensor. Otherwise, existing virtual\nsensor-backend Sensor objects will fire an error event and new calls to\nSensor.start() will attempt to use a real sensor instead.\n[setSensorOverrideEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setSensorOverrideEnabled)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetSensorOverrideEnabled {
    pub method: SetSensorOverrideEnabledMethod,
    pub params: SetSensorOverrideEnabledParams,
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
impl SetSensorOverrideReadingsMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setSensorOverrideReadings";
}
#[doc = "Updates the sensor readings reported by a sensor type previously overridden\nby setSensorOverrideEnabled.\n[setSensorOverrideReadings](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setSensorOverrideReadings)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetSensorOverrideReadings {
    pub method: SetSensorOverrideReadingsMethod,
    pub params: SetSensorOverrideReadingsParams,
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
impl SetPressureSourceOverrideEnabledMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setPressureSourceOverrideEnabled";
}
#[doc = "Overrides a pressure source of a given type, as used by the Compute\nPressure API, so that updates to PressureObserver.observe() are provided\nvia setPressureStateOverride instead of being retrieved from\nplatform-provided telemetry data.\n[setPressureSourceOverrideEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setPressureSourceOverrideEnabled)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetPressureSourceOverrideEnabled {
    pub method: SetPressureSourceOverrideEnabledMethod,
    pub params: SetPressureSourceOverrideEnabledParams,
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
impl SetPressureStateOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setPressureStateOverride";
}
#[doc = "TODO: OBSOLETE: To remove when setPressureDataOverride is merged.\nProvides a given pressure state that will be processed and eventually be\ndelivered to PressureObserver users. |source| must have been previously\noverridden by setPressureSourceOverrideEnabled.\n[setPressureStateOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setPressureStateOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetPressureStateOverride {
    pub method: SetPressureStateOverrideMethod,
    pub params: SetPressureStateOverrideParams,
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
impl SetPressureDataOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setPressureDataOverride";
}
#[doc = "Provides a given pressure data set that will be processed and eventually be\ndelivered to PressureObserver users. |source| must have been previously\noverridden by setPressureSourceOverrideEnabled.\n[setPressureDataOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setPressureDataOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetPressureDataOverride {
    pub method: SetPressureDataOverrideMethod,
    pub params: SetPressureDataOverrideParams,
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
impl SetIdleOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setIdleOverride";
}
#[doc = "Overrides the Idle state.\n[setIdleOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setIdleOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetIdleOverride {
    pub method: SetIdleOverrideMethod,
    pub params: SetIdleOverrideParams,
}
#[doc = "Clears Idle state overrides.\n[clearIdleOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearIdleOverride)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ClearIdleOverrideParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClearIdleOverrideMethod {
    #[serde(rename = "Emulation.clearIdleOverride")]
    ClearIdleOverride,
}
impl ClearIdleOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.clearIdleOverride";
}
#[doc = "Clears Idle state overrides.\n[clearIdleOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-clearIdleOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ClearIdleOverride {
    pub method: ClearIdleOverrideMethod,
    pub params: ClearIdleOverrideParams,
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
impl SetPageScaleFactorMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setPageScaleFactor";
}
#[doc = "Sets a specified page scale factor.\n[setPageScaleFactor](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setPageScaleFactor)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetPageScaleFactor {
    pub method: SetPageScaleFactorMethod,
    pub params: SetPageScaleFactorParams,
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
impl SetScriptExecutionDisabledMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setScriptExecutionDisabled";
}
#[doc = "Switches script execution in the page.\n[setScriptExecutionDisabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setScriptExecutionDisabled)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetScriptExecutionDisabled {
    pub method: SetScriptExecutionDisabledMethod,
    pub params: SetScriptExecutionDisabledParams,
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
impl SetTouchEmulationEnabledMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setTouchEmulationEnabled";
}
#[doc = "Enables touch on platforms which do not support them.\n[setTouchEmulationEnabled](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setTouchEmulationEnabled)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetTouchEmulationEnabled {
    pub method: SetTouchEmulationEnabledMethod,
    pub params: SetTouchEmulationEnabledParams,
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
    pub initial_virtual_time: Option<super::super::network::types::TimeSinceEpoch>,
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
impl SetVirtualTimePolicyMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setVirtualTimePolicy";
}
#[doc = "Turns on virtual time for all frames (replacing real-time with a synthetic time source) and sets\nthe current virtual time policy.  Note this supersedes any previous time budget.\n[setVirtualTimePolicy](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setVirtualTimePolicy)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetVirtualTimePolicy {
    pub method: SetVirtualTimePolicyMethod,
    pub params: SetVirtualTimePolicyParams,
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
impl SetLocaleOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setLocaleOverride";
}
#[doc = "Overrides default host system locale with the specified one.\n[setLocaleOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setLocaleOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetLocaleOverride {
    pub method: SetLocaleOverrideMethod,
    pub params: SetLocaleOverrideParams,
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
impl SetTimezoneOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setTimezoneOverride";
}
#[doc = "Overrides default host system timezone with the specified one.\n[setTimezoneOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setTimezoneOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetTimezoneOverride {
    pub method: SetTimezoneOverrideMethod,
    pub params: SetTimezoneOverrideParams,
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
impl SetDisabledImageTypesMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setDisabledImageTypes";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetDisabledImageTypes {
    pub method: SetDisabledImageTypesMethod,
    pub params: SetDisabledImageTypesParams,
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
impl SetDataSaverOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setDataSaverOverride";
}
#[doc = "Override the value of navigator.connection.saveData\n[setDataSaverOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setDataSaverOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetDataSaverOverride {
    pub method: SetDataSaverOverrideMethod,
    pub params: SetDataSaverOverrideParams,
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
impl SetHardwareConcurrencyOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setHardwareConcurrencyOverride";
}
#[derive(Debug, Clone, PartialEq)]
pub struct SetHardwareConcurrencyOverride {
    pub method: SetHardwareConcurrencyOverrideMethod,
    pub params: SetHardwareConcurrencyOverrideParams,
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
impl SetUserAgentOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setUserAgentOverride";
}
#[doc = "Allows overriding user agent with the given string.\n`userAgentMetadata` must be set for Client Hint headers to be sent.\n[setUserAgentOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setUserAgentOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetUserAgentOverride {
    pub method: SetUserAgentOverrideMethod,
    pub params: SetUserAgentOverrideParams,
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
impl SetAutomationOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setAutomationOverride";
}
#[doc = "Allows overriding the automation flag.\n[setAutomationOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setAutomationOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetAutomationOverride {
    pub method: SetAutomationOverrideMethod,
    pub params: SetAutomationOverrideParams,
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
impl SetSmallViewportHeightDifferenceOverrideMethod {
    pub const IDENTIFIER: &'static str = "Emulation.setSmallViewportHeightDifferenceOverride";
}
#[doc = "Allows overriding the difference between the small and large viewport sizes, which determine the\nvalue of the `svh` and `lvh` unit, respectively. Only supported for top-level frames.\n[setSmallViewportHeightDifferenceOverride](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-setSmallViewportHeightDifferenceOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetSmallViewportHeightDifferenceOverride {
    pub method: SetSmallViewportHeightDifferenceOverrideMethod,
    pub params: SetSmallViewportHeightDifferenceOverrideParams,
}
#[doc = "Returns device's screen configuration. In headful mode, the physical screens configuration is returned,\nwhereas in headless mode, a virtual headless screen configuration is provided instead.\n[getScreenInfos](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-getScreenInfos)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetScreenInfosParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetScreenInfosMethod {
    #[serde(rename = "Emulation.getScreenInfos")]
    GetScreenInfos,
}
impl GetScreenInfosMethod {
    pub const IDENTIFIER: &'static str = "Emulation.getScreenInfos";
}
#[doc = "Returns device's screen configuration. In headful mode, the physical screens configuration is returned,\nwhereas in headless mode, a virtual headless screen configuration is provided instead.\n[getScreenInfos](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-getScreenInfos)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetScreenInfos {
    pub method: GetScreenInfosMethod,
    pub params: GetScreenInfosParams,
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
impl AddScreenMethod {
    pub const IDENTIFIER: &'static str = "Emulation.addScreen";
}
#[doc = "Add a new screen to the device. Only supported in headless mode.\n[addScreen](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-addScreen)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AddScreen {
    pub method: AddScreenMethod,
    pub params: AddScreenParams,
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
impl RemoveScreenMethod {
    pub const IDENTIFIER: &'static str = "Emulation.removeScreen";
}
#[doc = "Remove screen from the device. Only supported in headless mode.\n[removeScreen](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#method-removeScreen)"]
#[derive(Debug, Clone, PartialEq)]
pub struct RemoveScreen {
    pub method: RemoveScreenMethod,
    pub params: RemoveScreenParams,
}
group_enum ! (EmulationCommands { ClearDeviceMetricsOverride (ClearDeviceMetricsOverride) , ClearGeolocationOverride (ClearGeolocationOverride) , ResetPageScaleFactor (ResetPageScaleFactor) , SetFocusEmulationEnabled (SetFocusEmulationEnabled) , SetAutoDarkModeOverride (SetAutoDarkModeOverride) , SetCpuThrottlingRate (SetCpuThrottlingRate) , SetDefaultBackgroundColorOverride (SetDefaultBackgroundColorOverride) , SetSafeAreaInsetsOverride (SetSafeAreaInsetsOverride) , SetDeviceMetricsOverride (SetDeviceMetricsOverride) , SetDevicePostureOverride (SetDevicePostureOverride) , ClearDevicePostureOverride (ClearDevicePostureOverride) , SetDisplayFeaturesOverride (SetDisplayFeaturesOverride) , ClearDisplayFeaturesOverride (ClearDisplayFeaturesOverride) , SetScrollbarsHidden (SetScrollbarsHidden) , SetDocumentCookieDisabled (SetDocumentCookieDisabled) , SetEmitTouchEventsForMouse (SetEmitTouchEventsForMouse) , SetEmulatedMedia (SetEmulatedMedia) , SetEmulatedVisionDeficiency (SetEmulatedVisionDeficiency) , SetEmulatedOsTextScale (SetEmulatedOsTextScale) , SetGeolocationOverride (SetGeolocationOverride) , GetOverriddenSensorInformation (GetOverriddenSensorInformation) , SetSensorOverrideEnabled (SetSensorOverrideEnabled) , SetSensorOverrideReadings (SetSensorOverrideReadings) , SetPressureSourceOverrideEnabled (SetPressureSourceOverrideEnabled) , SetPressureStateOverride (SetPressureStateOverride) , SetPressureDataOverride (SetPressureDataOverride) , SetIdleOverride (SetIdleOverride) , ClearIdleOverride (ClearIdleOverride) , SetPageScaleFactor (SetPageScaleFactor) , SetScriptExecutionDisabled (SetScriptExecutionDisabled) , SetTouchEmulationEnabled (SetTouchEmulationEnabled) , SetVirtualTimePolicy (SetVirtualTimePolicy) , SetLocaleOverride (SetLocaleOverride) , SetTimezoneOverride (SetTimezoneOverride) , SetDisabledImageTypes (SetDisabledImageTypes) , SetDataSaverOverride (SetDataSaverOverride) , SetHardwareConcurrencyOverride (SetHardwareConcurrencyOverride) , SetUserAgentOverride (SetUserAgentOverride) , SetAutomationOverride (SetAutomationOverride) , SetSmallViewportHeightDifferenceOverride (SetSmallViewportHeightDifferenceOverride) , GetScreenInfos (GetScreenInfos) , AddScreen (AddScreen) , RemoveScreen (RemoveScreen) });
