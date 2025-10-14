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

fn geolocation_coordinates_default_accuracy() -> f64 {
    1.0
}

fn geolocation_coordinates_default_altitude() -> Option<f64> {
    None
}

fn geolocation_coordinates_default_altitude_accuracy() -> Option<f64> {
    None
}

fn geolocation_coordinates_default_heading() -> Option<f64> {
    None
}

fn geolocation_coordinates_default_speed() -> Option<f64> {
    None
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
    #[validate(minimum = 0.0)]
    #[serde(default = "geolocation_coordinates_default_accuracy")]
    pub accuracy: f64,
    #[serde(rename = "altitude")]
    #[serde(default = "geolocation_coordinates_default_altitude")]
    pub altitude: Option<f64>,
    #[serde(rename = "altitudeAccuracy")]
    #[validate(minimum = 0.0)]
    #[serde(default = "geolocation_coordinates_default_altitude_accuracy")]
    pub altitude_accuracy: Option<f64>,
    #[serde(rename = "heading")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 360.0)]
    #[serde(default = "geolocation_coordinates_default_heading")]
    pub heading: Option<f64>,
    #[serde(rename = "speed")]
    #[validate(minimum = 0.0)]
    #[serde(default = "geolocation_coordinates_default_speed")]
    pub speed: Option<f64>,
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

