// Generated types for module

use serde::{Serialize, Deserialize};
use serde_valid::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ForcedColorsModeTheme {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct GeolocationCoordinates {
    #[serde(rename = "latitude")]
    #[validate(minimum = -90.0)]
    #[validate(maximum = 90.0)]
    pub latitude: f64,
    #[serde(rename = "longitude")]
    #[validate(minimum = -180.0)]
    #[validate(maximum = 180.0)]
    pub longitude: f64,
    #[serde(rename = "accuracy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 0.0)]
    pub accuracy: Option<f64>,
    #[serde(rename = "altitude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub altitude: Option<Option<f64>>,
    #[serde(rename = "altitudeAccuracy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 0.0)]
    pub altitude_accuracy: Option<Option<f64>>,
    #[serde(rename = "heading")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 360.0)]
    pub heading: Option<Option<f64>>,
    #[serde(rename = "speed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[validate(minimum = 0.0)]
    pub speed: Option<Option<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PositionUnavailableEnum {
    #[serde(rename = "positionUnavailable")]
    PositionUnavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeolocationPositionError {
    #[serde(rename = "type")]
    pub r#type: PositionUnavailableEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CoordinatesErrorUnion {
    GeolocationCoordinates(GeolocationCoordinates),
    GeolocationPositionError(GeolocationPositionError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScreenOrientationNatural {
    #[serde(rename = "portrait")]
    Portrait,
    #[serde(rename = "landscape")]
    Landscape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenOrientation {
    #[serde(rename = "natural")]
    pub natural: ScreenOrientationNatural,
    #[serde(rename = "type")]
    pub r#type: ScreenOrientationType,
}

