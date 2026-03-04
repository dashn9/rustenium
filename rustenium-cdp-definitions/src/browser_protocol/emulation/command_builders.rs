use super::commands::*;
impl SetFocusEmulationEnabled {
    pub fn builder() -> SetFocusEmulationEnabledBuilder {
        SetFocusEmulationEnabledBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetFocusEmulationEnabledBuilder {
    enabled: Option<bool>,
}
impl SetFocusEmulationEnabledBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn build(self) -> Result<SetFocusEmulationEnabled, String> {
        Ok(SetFocusEmulationEnabled {
            method: SetFocusEmulationEnabledMethod::SetFocusEmulationEnabled,
            params: SetFocusEmulationEnabledParams {
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
            },
        })
    }
}
impl SetAutoDarkModeOverride {
    pub fn builder() -> SetAutoDarkModeOverrideBuilder {
        SetAutoDarkModeOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetAutoDarkModeOverrideBuilder {
    enabled: Option<bool>,
}
impl SetAutoDarkModeOverrideBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn build(self) -> SetAutoDarkModeOverride {
        SetAutoDarkModeOverride {
            method: SetAutoDarkModeOverrideMethod::SetAutoDarkModeOverride,
            params: SetAutoDarkModeOverrideParams {
                enabled: self.enabled,
            },
        }
    }
}
impl SetCpuThrottlingRate {
    pub fn builder() -> SetCpuThrottlingRateBuilder {
        SetCpuThrottlingRateBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetCpuThrottlingRateBuilder {
    rate: Option<f64>,
}
impl SetCpuThrottlingRateBuilder {
    pub fn rate(mut self, rate: impl Into<f64>) -> Self {
        self.rate = Some(rate.into());
        self
    }
    pub fn build(self) -> Result<SetCpuThrottlingRate, String> {
        Ok(SetCpuThrottlingRate {
            method: SetCpuThrottlingRateMethod::SetCpuThrottlingRate,
            params: SetCpuThrottlingRateParams {
                rate: self
                    .rate
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(rate)))?,
            },
        })
    }
}
impl SetDefaultBackgroundColorOverride {
    pub fn builder() -> SetDefaultBackgroundColorOverrideBuilder {
        SetDefaultBackgroundColorOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDefaultBackgroundColorOverrideBuilder {
    color: Option<super::super::dom::types::Rgba>,
}
impl SetDefaultBackgroundColorOverrideBuilder {
    pub fn color(mut self, color: impl Into<super::super::dom::types::Rgba>) -> Self {
        self.color = Some(color.into());
        self
    }
    pub fn build(self) -> SetDefaultBackgroundColorOverride {
        SetDefaultBackgroundColorOverride {
            method: SetDefaultBackgroundColorOverrideMethod::SetDefaultBackgroundColorOverride,
            params: SetDefaultBackgroundColorOverrideParams { color: self.color },
        }
    }
}
impl SetSafeAreaInsetsOverride {
    pub fn builder() -> SetSafeAreaInsetsOverrideBuilder {
        SetSafeAreaInsetsOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetSafeAreaInsetsOverrideBuilder {
    insets: Option<super::types::SafeAreaInsets>,
}
impl SetSafeAreaInsetsOverrideBuilder {
    pub fn insets(mut self, insets: impl Into<super::types::SafeAreaInsets>) -> Self {
        self.insets = Some(insets.into());
        self
    }
    pub fn build(self) -> Result<SetSafeAreaInsetsOverride, String> {
        Ok(SetSafeAreaInsetsOverride {
            method: SetSafeAreaInsetsOverrideMethod::SetSafeAreaInsetsOverride,
            params: SetSafeAreaInsetsOverrideParams {
                insets: self
                    .insets
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(insets)))?,
            },
        })
    }
}
impl SetDeviceMetricsOverride {
    pub fn builder() -> SetDeviceMetricsOverrideBuilder {
        SetDeviceMetricsOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDeviceMetricsOverrideBuilder {
    width: Option<i64>,
    height: Option<i64>,
    device_scale_factor: Option<f64>,
    mobile: Option<bool>,
    scale: Option<f64>,
    screen_width: Option<i64>,
    screen_height: Option<i64>,
    position_x: Option<i64>,
    position_y: Option<i64>,
    dont_set_visible_size: Option<bool>,
    screen_orientation: Option<super::types::ScreenOrientation>,
    viewport: Option<super::super::page::types::Viewport>,
    scrollbar_type: Option<SetDeviceMetricsOverrideScrollbarType>,
}
impl SetDeviceMetricsOverrideBuilder {
    pub fn width(mut self, width: impl Into<i64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn height(mut self, height: impl Into<i64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn device_scale_factor(mut self, device_scale_factor: impl Into<f64>) -> Self {
        self.device_scale_factor = Some(device_scale_factor.into());
        self
    }
    pub fn mobile(mut self, mobile: impl Into<bool>) -> Self {
        self.mobile = Some(mobile.into());
        self
    }
    pub fn scale(mut self, scale: impl Into<f64>) -> Self {
        self.scale = Some(scale.into());
        self
    }
    pub fn screen_width(mut self, screen_width: impl Into<i64>) -> Self {
        self.screen_width = Some(screen_width.into());
        self
    }
    pub fn screen_height(mut self, screen_height: impl Into<i64>) -> Self {
        self.screen_height = Some(screen_height.into());
        self
    }
    pub fn position_x(mut self, position_x: impl Into<i64>) -> Self {
        self.position_x = Some(position_x.into());
        self
    }
    pub fn position_y(mut self, position_y: impl Into<i64>) -> Self {
        self.position_y = Some(position_y.into());
        self
    }
    pub fn dont_set_visible_size(mut self, dont_set_visible_size: impl Into<bool>) -> Self {
        self.dont_set_visible_size = Some(dont_set_visible_size.into());
        self
    }
    pub fn screen_orientation(
        mut self,
        screen_orientation: impl Into<super::types::ScreenOrientation>,
    ) -> Self {
        self.screen_orientation = Some(screen_orientation.into());
        self
    }
    pub fn viewport(mut self, viewport: impl Into<super::super::page::types::Viewport>) -> Self {
        self.viewport = Some(viewport.into());
        self
    }
    pub fn scrollbar_type(
        mut self,
        scrollbar_type: impl Into<SetDeviceMetricsOverrideScrollbarType>,
    ) -> Self {
        self.scrollbar_type = Some(scrollbar_type.into());
        self
    }
    pub fn build(self) -> Result<SetDeviceMetricsOverride, String> {
        Ok(SetDeviceMetricsOverride {
            method: SetDeviceMetricsOverrideMethod::SetDeviceMetricsOverride,
            params: SetDeviceMetricsOverrideParams {
                width: self
                    .width
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(width)))?,
                height: self
                    .height
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(height)))?,
                device_scale_factor: self.device_scale_factor.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(device_scale_factor)
                    )
                })?,
                mobile: self
                    .mobile
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(mobile)))?,
                scale: self.scale,
                screen_width: self.screen_width,
                screen_height: self.screen_height,
                position_x: self.position_x,
                position_y: self.position_y,
                dont_set_visible_size: self.dont_set_visible_size,
                screen_orientation: self.screen_orientation,
                viewport: self.viewport,
                scrollbar_type: self.scrollbar_type,
            },
        })
    }
}
impl SetDevicePostureOverride {
    pub fn builder() -> SetDevicePostureOverrideBuilder {
        SetDevicePostureOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDevicePostureOverrideBuilder {
    posture: Option<super::types::DevicePosture>,
}
impl SetDevicePostureOverrideBuilder {
    pub fn posture(mut self, posture: impl Into<super::types::DevicePosture>) -> Self {
        self.posture = Some(posture.into());
        self
    }
    pub fn build(self) -> Result<SetDevicePostureOverride, String> {
        Ok(SetDevicePostureOverride {
            method: SetDevicePostureOverrideMethod::SetDevicePostureOverride,
            params: SetDevicePostureOverrideParams {
                posture: self
                    .posture
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(posture)))?,
            },
        })
    }
}
impl SetDisplayFeaturesOverride {
    pub fn builder() -> SetDisplayFeaturesOverrideBuilder {
        SetDisplayFeaturesOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDisplayFeaturesOverrideBuilder {
    features: Option<Vec<super::types::DisplayFeature>>,
}
impl SetDisplayFeaturesOverrideBuilder {
    pub fn feature(mut self, feature: impl Into<super::types::DisplayFeature>) -> Self {
        let v = self.features.get_or_insert(Vec::new());
        v.push(feature.into());
        self
    }
    pub fn features<I, S>(mut self, features: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::DisplayFeature>,
    {
        let v = self.features.get_or_insert(Vec::new());
        for val in features {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetDisplayFeaturesOverride, String> {
        Ok(SetDisplayFeaturesOverride {
            method: SetDisplayFeaturesOverrideMethod::SetDisplayFeaturesOverride,
            params: SetDisplayFeaturesOverrideParams {
                features: self.features.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(features))
                })?,
            },
        })
    }
}
impl SetScrollbarsHidden {
    pub fn builder() -> SetScrollbarsHiddenBuilder {
        SetScrollbarsHiddenBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetScrollbarsHiddenBuilder {
    hidden: Option<bool>,
}
impl SetScrollbarsHiddenBuilder {
    pub fn hidden(mut self, hidden: impl Into<bool>) -> Self {
        self.hidden = Some(hidden.into());
        self
    }
    pub fn build(self) -> Result<SetScrollbarsHidden, String> {
        Ok(SetScrollbarsHidden {
            method: SetScrollbarsHiddenMethod::SetScrollbarsHidden,
            params: SetScrollbarsHiddenParams {
                hidden: self
                    .hidden
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(hidden)))?,
            },
        })
    }
}
impl SetDocumentCookieDisabled {
    pub fn builder() -> SetDocumentCookieDisabledBuilder {
        SetDocumentCookieDisabledBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDocumentCookieDisabledBuilder {
    disabled: Option<bool>,
}
impl SetDocumentCookieDisabledBuilder {
    pub fn disabled(mut self, disabled: impl Into<bool>) -> Self {
        self.disabled = Some(disabled.into());
        self
    }
    pub fn build(self) -> Result<SetDocumentCookieDisabled, String> {
        Ok(SetDocumentCookieDisabled {
            method: SetDocumentCookieDisabledMethod::SetDocumentCookieDisabled,
            params: SetDocumentCookieDisabledParams {
                disabled: self.disabled.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(disabled))
                })?,
            },
        })
    }
}
impl SetEmitTouchEventsForMouse {
    pub fn builder() -> SetEmitTouchEventsForMouseBuilder {
        SetEmitTouchEventsForMouseBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetEmitTouchEventsForMouseBuilder {
    enabled: Option<bool>,
    configuration: Option<SetEmitTouchEventsForMouseConfiguration>,
}
impl SetEmitTouchEventsForMouseBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn configuration(
        mut self,
        configuration: impl Into<SetEmitTouchEventsForMouseConfiguration>,
    ) -> Self {
        self.configuration = Some(configuration.into());
        self
    }
    pub fn build(self) -> Result<SetEmitTouchEventsForMouse, String> {
        Ok(SetEmitTouchEventsForMouse {
            method: SetEmitTouchEventsForMouseMethod::SetEmitTouchEventsForMouse,
            params: SetEmitTouchEventsForMouseParams {
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
                configuration: self.configuration,
            },
        })
    }
}
impl SetEmulatedMedia {
    pub fn builder() -> SetEmulatedMediaBuilder {
        SetEmulatedMediaBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetEmulatedMediaBuilder {
    media: Option<String>,
    features: Option<Vec<super::types::MediaFeature>>,
}
impl SetEmulatedMediaBuilder {
    pub fn media(mut self, media: impl Into<String>) -> Self {
        self.media = Some(media.into());
        self
    }
    pub fn feature(mut self, feature: impl Into<super::types::MediaFeature>) -> Self {
        let v = self.features.get_or_insert(Vec::new());
        v.push(feature.into());
        self
    }
    pub fn features<I, S>(mut self, features: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::MediaFeature>,
    {
        let v = self.features.get_or_insert(Vec::new());
        for val in features {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> SetEmulatedMedia {
        SetEmulatedMedia {
            method: SetEmulatedMediaMethod::SetEmulatedMedia,
            params: SetEmulatedMediaParams {
                media: self.media,
                features: self.features,
            },
        }
    }
}
impl SetEmulatedVisionDeficiency {
    pub fn builder() -> SetEmulatedVisionDeficiencyBuilder {
        SetEmulatedVisionDeficiencyBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetEmulatedVisionDeficiencyBuilder {
    r#type: Option<SetEmulatedVisionDeficiencyType>,
}
impl SetEmulatedVisionDeficiencyBuilder {
    pub fn r#type(mut self, r#type: impl Into<SetEmulatedVisionDeficiencyType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<SetEmulatedVisionDeficiency, String> {
        Ok(SetEmulatedVisionDeficiency {
            method: SetEmulatedVisionDeficiencyMethod::SetEmulatedVisionDeficiency,
            params: SetEmulatedVisionDeficiencyParams {
                r#type: self
                    .r#type
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            },
        })
    }
}
impl SetEmulatedOsTextScale {
    pub fn builder() -> SetEmulatedOsTextScaleBuilder {
        SetEmulatedOsTextScaleBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetEmulatedOsTextScaleBuilder {
    scale: Option<f64>,
}
impl SetEmulatedOsTextScaleBuilder {
    pub fn scale(mut self, scale: impl Into<f64>) -> Self {
        self.scale = Some(scale.into());
        self
    }
    pub fn build(self) -> SetEmulatedOsTextScale {
        SetEmulatedOsTextScale {
            method: SetEmulatedOsTextScaleMethod::SetEmulatedOsTextScale,
            params: SetEmulatedOsTextScaleParams { scale: self.scale },
        }
    }
}
impl SetGeolocationOverride {
    pub fn builder() -> SetGeolocationOverrideBuilder {
        SetGeolocationOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetGeolocationOverrideBuilder {
    latitude: Option<f64>,
    longitude: Option<f64>,
    accuracy: Option<f64>,
    altitude: Option<f64>,
    altitude_accuracy: Option<f64>,
    heading: Option<f64>,
    speed: Option<f64>,
}
impl SetGeolocationOverrideBuilder {
    pub fn latitude(mut self, latitude: impl Into<f64>) -> Self {
        self.latitude = Some(latitude.into());
        self
    }
    pub fn longitude(mut self, longitude: impl Into<f64>) -> Self {
        self.longitude = Some(longitude.into());
        self
    }
    pub fn accuracy(mut self, accuracy: impl Into<f64>) -> Self {
        self.accuracy = Some(accuracy.into());
        self
    }
    pub fn altitude(mut self, altitude: impl Into<f64>) -> Self {
        self.altitude = Some(altitude.into());
        self
    }
    pub fn altitude_accuracy(mut self, altitude_accuracy: impl Into<f64>) -> Self {
        self.altitude_accuracy = Some(altitude_accuracy.into());
        self
    }
    pub fn heading(mut self, heading: impl Into<f64>) -> Self {
        self.heading = Some(heading.into());
        self
    }
    pub fn speed(mut self, speed: impl Into<f64>) -> Self {
        self.speed = Some(speed.into());
        self
    }
    pub fn build(self) -> SetGeolocationOverride {
        SetGeolocationOverride {
            method: SetGeolocationOverrideMethod::SetGeolocationOverride,
            params: SetGeolocationOverrideParams {
                latitude: self.latitude,
                longitude: self.longitude,
                accuracy: self.accuracy,
                altitude: self.altitude,
                altitude_accuracy: self.altitude_accuracy,
                heading: self.heading,
                speed: self.speed,
            },
        }
    }
}
impl GetOverriddenSensorInformation {
    pub fn builder() -> GetOverriddenSensorInformationBuilder {
        GetOverriddenSensorInformationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct GetOverriddenSensorInformationBuilder {
    r#type: Option<super::types::SensorType>,
}
impl GetOverriddenSensorInformationBuilder {
    pub fn r#type(mut self, r#type: impl Into<super::types::SensorType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<GetOverriddenSensorInformation, String> {
        Ok(GetOverriddenSensorInformation {
            method: GetOverriddenSensorInformationMethod::GetOverriddenSensorInformation,
            params: GetOverriddenSensorInformationParams {
                r#type: self
                    .r#type
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            },
        })
    }
}
impl SetSensorOverrideEnabled {
    pub fn builder() -> SetSensorOverrideEnabledBuilder {
        SetSensorOverrideEnabledBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetSensorOverrideEnabledBuilder {
    enabled: Option<bool>,
    r#type: Option<super::types::SensorType>,
    metadata: Option<super::types::SensorMetadata>,
}
impl SetSensorOverrideEnabledBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<super::types::SensorType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn metadata(mut self, metadata: impl Into<super::types::SensorMetadata>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }
    pub fn build(self) -> Result<SetSensorOverrideEnabled, String> {
        Ok(SetSensorOverrideEnabled {
            method: SetSensorOverrideEnabledMethod::SetSensorOverrideEnabled,
            params: SetSensorOverrideEnabledParams {
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
                r#type: self
                    .r#type
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
                metadata: self.metadata,
            },
        })
    }
}
impl SetSensorOverrideReadings {
    pub fn builder() -> SetSensorOverrideReadingsBuilder {
        SetSensorOverrideReadingsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetSensorOverrideReadingsBuilder {
    r#type: Option<super::types::SensorType>,
    reading: Option<super::types::SensorReading>,
}
impl SetSensorOverrideReadingsBuilder {
    pub fn r#type(mut self, r#type: impl Into<super::types::SensorType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn reading(mut self, reading: impl Into<super::types::SensorReading>) -> Self {
        self.reading = Some(reading.into());
        self
    }
    pub fn build(self) -> Result<SetSensorOverrideReadings, String> {
        Ok(SetSensorOverrideReadings {
            method: SetSensorOverrideReadingsMethod::SetSensorOverrideReadings,
            params: SetSensorOverrideReadingsParams {
                r#type: self
                    .r#type
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
                reading: self
                    .reading
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(reading)))?,
            },
        })
    }
}
impl SetPressureSourceOverrideEnabled {
    pub fn builder() -> SetPressureSourceOverrideEnabledBuilder {
        SetPressureSourceOverrideEnabledBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetPressureSourceOverrideEnabledBuilder {
    enabled: Option<bool>,
    source: Option<super::types::PressureSource>,
    metadata: Option<super::types::PressureMetadata>,
}
impl SetPressureSourceOverrideEnabledBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn source(mut self, source: impl Into<super::types::PressureSource>) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn metadata(mut self, metadata: impl Into<super::types::PressureMetadata>) -> Self {
        self.metadata = Some(metadata.into());
        self
    }
    pub fn build(self) -> Result<SetPressureSourceOverrideEnabled, String> {
        Ok(SetPressureSourceOverrideEnabled {
            method: SetPressureSourceOverrideEnabledMethod::SetPressureSourceOverrideEnabled,
            params: SetPressureSourceOverrideEnabledParams {
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
                source: self
                    .source
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(source)))?,
                metadata: self.metadata,
            },
        })
    }
}
impl SetPressureStateOverride {
    pub fn builder() -> SetPressureStateOverrideBuilder {
        SetPressureStateOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetPressureStateOverrideBuilder {
    source: Option<super::types::PressureSource>,
    state: Option<super::types::PressureState>,
}
impl SetPressureStateOverrideBuilder {
    pub fn source(mut self, source: impl Into<super::types::PressureSource>) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn state(mut self, state: impl Into<super::types::PressureState>) -> Self {
        self.state = Some(state.into());
        self
    }
    pub fn build(self) -> Result<SetPressureStateOverride, String> {
        Ok(SetPressureStateOverride {
            method: SetPressureStateOverrideMethod::SetPressureStateOverride,
            params: SetPressureStateOverrideParams {
                source: self
                    .source
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(source)))?,
                state: self
                    .state
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(state)))?,
            },
        })
    }
}
impl SetPressureDataOverride {
    pub fn builder() -> SetPressureDataOverrideBuilder {
        SetPressureDataOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetPressureDataOverrideBuilder {
    source: Option<super::types::PressureSource>,
    state: Option<super::types::PressureState>,
    own_contribution_estimate: Option<f64>,
}
impl SetPressureDataOverrideBuilder {
    pub fn source(mut self, source: impl Into<super::types::PressureSource>) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn state(mut self, state: impl Into<super::types::PressureState>) -> Self {
        self.state = Some(state.into());
        self
    }
    pub fn own_contribution_estimate(mut self, own_contribution_estimate: impl Into<f64>) -> Self {
        self.own_contribution_estimate = Some(own_contribution_estimate.into());
        self
    }
    pub fn build(self) -> Result<SetPressureDataOverride, String> {
        Ok(SetPressureDataOverride {
            method: SetPressureDataOverrideMethod::SetPressureDataOverride,
            params: SetPressureDataOverrideParams {
                source: self
                    .source
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(source)))?,
                state: self
                    .state
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(state)))?,
                own_contribution_estimate: self.own_contribution_estimate,
            },
        })
    }
}
impl SetIdleOverride {
    pub fn builder() -> SetIdleOverrideBuilder {
        SetIdleOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetIdleOverrideBuilder {
    is_user_active: Option<bool>,
    is_screen_unlocked: Option<bool>,
}
impl SetIdleOverrideBuilder {
    pub fn is_user_active(mut self, is_user_active: impl Into<bool>) -> Self {
        self.is_user_active = Some(is_user_active.into());
        self
    }
    pub fn is_screen_unlocked(mut self, is_screen_unlocked: impl Into<bool>) -> Self {
        self.is_screen_unlocked = Some(is_screen_unlocked.into());
        self
    }
    pub fn build(self) -> Result<SetIdleOverride, String> {
        Ok(SetIdleOverride {
            method: SetIdleOverrideMethod::SetIdleOverride,
            params: SetIdleOverrideParams {
                is_user_active: self.is_user_active.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(is_user_active))
                })?,
                is_screen_unlocked: self.is_screen_unlocked.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(is_screen_unlocked)
                    )
                })?,
            },
        })
    }
}
impl SetPageScaleFactor {
    pub fn builder() -> SetPageScaleFactorBuilder {
        SetPageScaleFactorBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetPageScaleFactorBuilder {
    page_scale_factor: Option<f64>,
}
impl SetPageScaleFactorBuilder {
    pub fn page_scale_factor(mut self, page_scale_factor: impl Into<f64>) -> Self {
        self.page_scale_factor = Some(page_scale_factor.into());
        self
    }
    pub fn build(self) -> Result<SetPageScaleFactor, String> {
        Ok(SetPageScaleFactor {
            method: SetPageScaleFactorMethod::SetPageScaleFactor,
            params: SetPageScaleFactorParams {
                page_scale_factor: self.page_scale_factor.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(page_scale_factor)
                    )
                })?,
            },
        })
    }
}
impl SetScriptExecutionDisabled {
    pub fn builder() -> SetScriptExecutionDisabledBuilder {
        SetScriptExecutionDisabledBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetScriptExecutionDisabledBuilder {
    value: Option<bool>,
}
impl SetScriptExecutionDisabledBuilder {
    pub fn value(mut self, value: impl Into<bool>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<SetScriptExecutionDisabled, String> {
        Ok(SetScriptExecutionDisabled {
            method: SetScriptExecutionDisabledMethod::SetScriptExecutionDisabled,
            params: SetScriptExecutionDisabledParams {
                value: self
                    .value
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
            },
        })
    }
}
impl SetTouchEmulationEnabled {
    pub fn builder() -> SetTouchEmulationEnabledBuilder {
        SetTouchEmulationEnabledBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetTouchEmulationEnabledBuilder {
    enabled: Option<bool>,
    max_touch_points: Option<i64>,
}
impl SetTouchEmulationEnabledBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn max_touch_points(mut self, max_touch_points: impl Into<i64>) -> Self {
        self.max_touch_points = Some(max_touch_points.into());
        self
    }
    pub fn build(self) -> Result<SetTouchEmulationEnabled, String> {
        Ok(SetTouchEmulationEnabled {
            method: SetTouchEmulationEnabledMethod::SetTouchEmulationEnabled,
            params: SetTouchEmulationEnabledParams {
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
                max_touch_points: self.max_touch_points,
            },
        })
    }
}
impl SetVirtualTimePolicy {
    pub fn builder() -> SetVirtualTimePolicyBuilder {
        SetVirtualTimePolicyBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetVirtualTimePolicyBuilder {
    policy: Option<super::types::VirtualTimePolicy>,
    budget: Option<f64>,
    max_virtual_time_task_starvation_count: Option<i64>,
    initial_virtual_time: Option<super::super::network::types::TimeSinceEpoch>,
}
impl SetVirtualTimePolicyBuilder {
    pub fn policy(mut self, policy: impl Into<super::types::VirtualTimePolicy>) -> Self {
        self.policy = Some(policy.into());
        self
    }
    pub fn budget(mut self, budget: impl Into<f64>) -> Self {
        self.budget = Some(budget.into());
        self
    }
    pub fn max_virtual_time_task_starvation_count(
        mut self,
        max_virtual_time_task_starvation_count: impl Into<i64>,
    ) -> Self {
        self.max_virtual_time_task_starvation_count =
            Some(max_virtual_time_task_starvation_count.into());
        self
    }
    pub fn initial_virtual_time(
        mut self,
        initial_virtual_time: impl Into<super::super::network::types::TimeSinceEpoch>,
    ) -> Self {
        self.initial_virtual_time = Some(initial_virtual_time.into());
        self
    }
    pub fn build(self) -> Result<SetVirtualTimePolicy, String> {
        Ok(SetVirtualTimePolicy {
            method: SetVirtualTimePolicyMethod::SetVirtualTimePolicy,
            params: SetVirtualTimePolicyParams {
                policy: self
                    .policy
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(policy)))?,
                budget: self.budget,
                max_virtual_time_task_starvation_count: self.max_virtual_time_task_starvation_count,
                initial_virtual_time: self.initial_virtual_time,
            },
        })
    }
}
impl SetLocaleOverride {
    pub fn builder() -> SetLocaleOverrideBuilder {
        SetLocaleOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetLocaleOverrideBuilder {
    locale: Option<String>,
}
impl SetLocaleOverrideBuilder {
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }
    pub fn build(self) -> SetLocaleOverride {
        SetLocaleOverride {
            method: SetLocaleOverrideMethod::SetLocaleOverride,
            params: SetLocaleOverrideParams {
                locale: self.locale,
            },
        }
    }
}
impl SetTimezoneOverride {
    pub fn builder() -> SetTimezoneOverrideBuilder {
        SetTimezoneOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetTimezoneOverrideBuilder {
    timezone_id: Option<String>,
}
impl SetTimezoneOverrideBuilder {
    pub fn timezone_id(mut self, timezone_id: impl Into<String>) -> Self {
        self.timezone_id = Some(timezone_id.into());
        self
    }
    pub fn build(self) -> Result<SetTimezoneOverride, String> {
        Ok(SetTimezoneOverride {
            method: SetTimezoneOverrideMethod::SetTimezoneOverride,
            params: SetTimezoneOverrideParams {
                timezone_id: self.timezone_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(timezone_id))
                })?,
            },
        })
    }
}
impl SetDisabledImageTypes {
    pub fn builder() -> SetDisabledImageTypesBuilder {
        SetDisabledImageTypesBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDisabledImageTypesBuilder {
    image_types: Option<Vec<super::types::DisabledImageType>>,
}
impl SetDisabledImageTypesBuilder {
    pub fn image_type(mut self, image_type: impl Into<super::types::DisabledImageType>) -> Self {
        let v = self.image_types.get_or_insert(Vec::new());
        v.push(image_type.into());
        self
    }
    pub fn image_types<I, S>(mut self, image_types: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::DisabledImageType>,
    {
        let v = self.image_types.get_or_insert(Vec::new());
        for val in image_types {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetDisabledImageTypes, String> {
        Ok(SetDisabledImageTypes {
            method: SetDisabledImageTypesMethod::SetDisabledImageTypes,
            params: SetDisabledImageTypesParams {
                image_types: self.image_types.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(image_types))
                })?,
            },
        })
    }
}
impl SetDataSaverOverride {
    pub fn builder() -> SetDataSaverOverrideBuilder {
        SetDataSaverOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDataSaverOverrideBuilder {
    data_saver_enabled: Option<bool>,
}
impl SetDataSaverOverrideBuilder {
    pub fn data_saver_enabled(mut self, data_saver_enabled: impl Into<bool>) -> Self {
        self.data_saver_enabled = Some(data_saver_enabled.into());
        self
    }
    pub fn build(self) -> SetDataSaverOverride {
        SetDataSaverOverride {
            method: SetDataSaverOverrideMethod::SetDataSaverOverride,
            params: SetDataSaverOverrideParams {
                data_saver_enabled: self.data_saver_enabled,
            },
        }
    }
}
impl SetHardwareConcurrencyOverride {
    pub fn builder() -> SetHardwareConcurrencyOverrideBuilder {
        SetHardwareConcurrencyOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetHardwareConcurrencyOverrideBuilder {
    hardware_concurrency: Option<i64>,
}
impl SetHardwareConcurrencyOverrideBuilder {
    pub fn hardware_concurrency(mut self, hardware_concurrency: impl Into<i64>) -> Self {
        self.hardware_concurrency = Some(hardware_concurrency.into());
        self
    }
    pub fn build(self) -> Result<SetHardwareConcurrencyOverride, String> {
        Ok(SetHardwareConcurrencyOverride {
            method: SetHardwareConcurrencyOverrideMethod::SetHardwareConcurrencyOverride,
            params: SetHardwareConcurrencyOverrideParams {
                hardware_concurrency: self.hardware_concurrency.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(hardware_concurrency)
                    )
                })?,
            },
        })
    }
}
impl SetUserAgentOverride {
    pub fn builder() -> SetUserAgentOverrideBuilder {
        SetUserAgentOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetUserAgentOverrideBuilder {
    user_agent: Option<String>,
    accept_language: Option<String>,
    platform: Option<String>,
    user_agent_metadata: Option<super::types::UserAgentMetadata>,
}
impl SetUserAgentOverrideBuilder {
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
    pub fn accept_language(mut self, accept_language: impl Into<String>) -> Self {
        self.accept_language = Some(accept_language.into());
        self
    }
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
    pub fn user_agent_metadata(
        mut self,
        user_agent_metadata: impl Into<super::types::UserAgentMetadata>,
    ) -> Self {
        self.user_agent_metadata = Some(user_agent_metadata.into());
        self
    }
    pub fn build(self) -> Result<SetUserAgentOverride, String> {
        Ok(SetUserAgentOverride {
            method: SetUserAgentOverrideMethod::SetUserAgentOverride,
            params: SetUserAgentOverrideParams {
                user_agent: self.user_agent.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(user_agent))
                })?,
                accept_language: self.accept_language,
                platform: self.platform,
                user_agent_metadata: self.user_agent_metadata,
            },
        })
    }
}
impl SetAutomationOverride {
    pub fn builder() -> SetAutomationOverrideBuilder {
        SetAutomationOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetAutomationOverrideBuilder {
    enabled: Option<bool>,
}
impl SetAutomationOverrideBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn build(self) -> Result<SetAutomationOverride, String> {
        Ok(SetAutomationOverride {
            method: SetAutomationOverrideMethod::SetAutomationOverride,
            params: SetAutomationOverrideParams {
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
            },
        })
    }
}
impl SetSmallViewportHeightDifferenceOverride {
    pub fn builder() -> SetSmallViewportHeightDifferenceOverrideBuilder {
        SetSmallViewportHeightDifferenceOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetSmallViewportHeightDifferenceOverrideBuilder {
    difference: Option<i64>,
}
impl SetSmallViewportHeightDifferenceOverrideBuilder {
    pub fn difference(mut self, difference: impl Into<i64>) -> Self {
        self.difference = Some(difference.into());
        self
    }
    pub fn build(self) -> Result<SetSmallViewportHeightDifferenceOverride, String> {
        Ok (SetSmallViewportHeightDifferenceOverride { method : SetSmallViewportHeightDifferenceOverrideMethod :: SetSmallViewportHeightDifferenceOverride , params : SetSmallViewportHeightDifferenceOverrideParams { difference : self . difference . ok_or_else (|| format ! ("Field `{}` is mandatory." , std :: stringify ! (difference))) ? , } , })
    }
}
impl AddScreen {
    pub fn builder() -> AddScreenBuilder {
        AddScreenBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AddScreenBuilder {
    left: Option<i64>,
    top: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
    work_area_insets: Option<super::types::WorkAreaInsets>,
    device_pixel_ratio: Option<f64>,
    rotation: Option<i64>,
    color_depth: Option<i64>,
    label: Option<String>,
    is_internal: Option<bool>,
}
impl AddScreenBuilder {
    pub fn left(mut self, left: impl Into<i64>) -> Self {
        self.left = Some(left.into());
        self
    }
    pub fn top(mut self, top: impl Into<i64>) -> Self {
        self.top = Some(top.into());
        self
    }
    pub fn width(mut self, width: impl Into<i64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn height(mut self, height: impl Into<i64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn work_area_insets(
        mut self,
        work_area_insets: impl Into<super::types::WorkAreaInsets>,
    ) -> Self {
        self.work_area_insets = Some(work_area_insets.into());
        self
    }
    pub fn device_pixel_ratio(mut self, device_pixel_ratio: impl Into<f64>) -> Self {
        self.device_pixel_ratio = Some(device_pixel_ratio.into());
        self
    }
    pub fn rotation(mut self, rotation: impl Into<i64>) -> Self {
        self.rotation = Some(rotation.into());
        self
    }
    pub fn color_depth(mut self, color_depth: impl Into<i64>) -> Self {
        self.color_depth = Some(color_depth.into());
        self
    }
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn is_internal(mut self, is_internal: impl Into<bool>) -> Self {
        self.is_internal = Some(is_internal.into());
        self
    }
    pub fn build(self) -> Result<AddScreen, String> {
        Ok(AddScreen {
            method: AddScreenMethod::AddScreen,
            params: AddScreenParams {
                left: self
                    .left
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(left)))?,
                top: self
                    .top
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(top)))?,
                width: self
                    .width
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(width)))?,
                height: self
                    .height
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(height)))?,
                work_area_insets: self.work_area_insets,
                device_pixel_ratio: self.device_pixel_ratio,
                rotation: self.rotation,
                color_depth: self.color_depth,
                label: self.label,
                is_internal: self.is_internal,
            },
        })
    }
}
impl RemoveScreen {
    pub fn builder() -> RemoveScreenBuilder {
        RemoveScreenBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RemoveScreenBuilder {
    screen_id: Option<super::types::ScreenId>,
}
impl RemoveScreenBuilder {
    pub fn screen_id(mut self, screen_id: impl Into<super::types::ScreenId>) -> Self {
        self.screen_id = Some(screen_id.into());
        self
    }
    pub fn build(self) -> Result<RemoveScreen, String> {
        Ok(RemoveScreen {
            method: RemoveScreenMethod::RemoveScreen,
            params: RemoveScreenParams {
                screen_id: self.screen_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(screen_id))
                })?,
            },
        })
    }
}
