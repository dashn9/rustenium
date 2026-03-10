use super::types::*;
impl GeolocationCoordinates {
    pub fn builder() -> GeolocationCoordinatesBuilder {
        <GeolocationCoordinatesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GeolocationCoordinatesBuilder {
    latitude: Option<f64>,
    longitude: Option<f64>,
    accuracy: Option<f64>,
    altitude: Option<f64>,
    altitude_accuracy: Option<f64>,
    heading: Option<f64>,
    speed: Option<f64>,
}
impl GeolocationCoordinatesBuilder {
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
    pub fn build(self) -> Result<GeolocationCoordinates, String> {
        Ok(GeolocationCoordinates {
            latitude: self
                .latitude
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(latitude)))?,
            longitude: self
                .longitude
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(longitude)))?,
            accuracy: self.accuracy,
            altitude: self.altitude,
            altitude_accuracy: self.altitude_accuracy,
            heading: self.heading,
            speed: self.speed,
        })
    }
}
impl GeolocationPositionError {
    pub fn builder() -> GeolocationPositionErrorBuilder {
        <GeolocationPositionErrorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GeolocationPositionErrorBuilder {
    r#type: Option<GeolocationPositionErrorType>,
}
impl GeolocationPositionErrorBuilder {
    pub fn r#type(mut self, r#type: impl Into<GeolocationPositionErrorType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<GeolocationPositionError, String> {
        Ok(GeolocationPositionError {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl NetworkConditionsOffline {
    pub fn builder() -> NetworkConditionsOfflineBuilder {
        <NetworkConditionsOfflineBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct NetworkConditionsOfflineBuilder {
    r#type: Option<NetworkConditionsOfflineType>,
}
impl NetworkConditionsOfflineBuilder {
    pub fn r#type(mut self, r#type: impl Into<NetworkConditionsOfflineType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<NetworkConditionsOffline, String> {
        Ok(NetworkConditionsOffline {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl ScreenOrientation {
    pub fn builder() -> ScreenOrientationBuilder {
        <ScreenOrientationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ScreenOrientationBuilder {
    natural: Option<ScreenOrientationNatural>,
    r#type: Option<ScreenOrientationType>,
}
impl ScreenOrientationBuilder {
    pub fn natural(mut self, natural: impl Into<ScreenOrientationNatural>) -> Self {
        self.natural = Some(natural.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<ScreenOrientationType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<ScreenOrientation, String> {
        Ok(ScreenOrientation {
            natural: self
                .natural
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(natural)))?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
