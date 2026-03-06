use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ForcedColorsModeTheme {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeolocationCoordinates {
    #[serde(rename = "latitude")]
    pub latitude: f64,
    #[serde(rename = "longitude")]
    pub longitude: f64,
    #[serde(rename = "accuracy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_geolocation_coordinates_accuracy")]
    #[serde_valid::validate(minimum = 0f64)]
    pub accuracy: Option<f64>,
    #[serde(rename = "altitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub altitude: Option<f64>,
    #[serde(rename = "altitudeAccuracy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde_valid::validate(minimum = 0f64)]
    pub altitude_accuracy: Option<f64>,
    #[serde(rename = "heading")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub heading: Option<f64>,
    #[serde(rename = "speed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde_valid::validate(minimum = 0f64)]
    pub speed: Option<f64>,
}
fn default_geolocation_coordinates_accuracy() -> Option<f64> {
    Some(1f64)
}
impl GeolocationCoordinates {
    pub fn new(latitude: impl Into<f64>, longitude: impl Into<f64>) -> Self {
        Self {
            latitude: latitude.into(),
            longitude: longitude.into(),
            accuracy: None,
            altitude: None,
            altitude_accuracy: None,
            heading: None,
            speed: None,
        }
    }
}
impl GeolocationCoordinates {
    pub const IDENTIFIER: &'static str = "emulation.GeolocationCoordinates";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeolocationPositionError {
    #[serde(rename = "type")]
    pub r#type: String,
}
impl GeolocationPositionError {
    pub fn new(r#type: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
        }
    }
}
impl<T: Into<String>> From<T> for GeolocationPositionError {
    fn from(url: T) -> Self {
        GeolocationPositionError::new(url)
    }
}
impl GeolocationPositionError {
    pub const IDENTIFIER: &'static str = "emulation.GeolocationPositionError";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct NetworkConditions(NetworkConditionsOffline);
impl NetworkConditions {
    pub fn new(val: impl Into<NetworkConditionsOffline>) -> Self {
        NetworkConditions(val.into())
    }
    pub fn inner(&self) -> &NetworkConditionsOffline {
        &self.0
    }
}
impl NetworkConditions {
    pub const IDENTIFIER: &'static str = "emulation.NetworkConditions";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NetworkConditionsOffline {
    #[serde(rename = "type")]
    pub r#type: String,
}
impl NetworkConditionsOffline {
    pub fn new(r#type: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
        }
    }
}
impl<T: Into<String>> From<T> for NetworkConditionsOffline {
    fn from(url: T) -> Self {
        NetworkConditionsOffline::new(url)
    }
}
impl NetworkConditionsOffline {
    pub const IDENTIFIER: &'static str = "emulation.NetworkConditionsOffline";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScreenOrientationNatural {
    #[serde(rename = "portrait")]
    Portrait,
    #[serde(rename = "landscape")]
    Landscape,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScreenOrientationType {
    #[serde(rename = "portrait-primary")]
    PortraitPrimary,
    #[serde(rename = "portrait-secondary")]
    PortraitSecondary,
    #[serde(rename = "landscape-primary")]
    LandscapePrimary,
    #[serde(rename = "landscape-secondary")]
    LandscapeSecondary,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreenOrientation {
    #[serde(rename = "natural")]
    pub natural: ScreenOrientationNatural,
    #[serde(rename = "type")]
    pub r#type: ScreenOrientationType,
}
impl ScreenOrientation {
    pub fn new(
        natural: impl Into<ScreenOrientationNatural>,
        r#type: impl Into<ScreenOrientationType>,
    ) -> Self {
        Self {
            natural: natural.into(),
            r#type: r#type.into(),
        }
    }
}
impl ScreenOrientation {
    pub const IDENTIFIER: &'static str = "emulation.ScreenOrientation";
}
group_enum ! (EmulationTypes { ForcedColorsModeTheme (ForcedColorsModeTheme) , GeolocationCoordinates (GeolocationCoordinates) , GeolocationPositionError (GeolocationPositionError) , NetworkConditions (NetworkConditions) , NetworkConditionsOffline (NetworkConditionsOffline) , ScreenOrientationNatural (ScreenOrientationNatural) , ScreenOrientationType (ScreenOrientationType) , ScreenOrientation (ScreenOrientation) });
