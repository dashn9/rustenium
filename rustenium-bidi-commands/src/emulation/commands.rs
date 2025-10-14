// Generated commands for module

use serde::{Serialize, Deserialize};
use crate::browser::types::UserContext;
use crate::browsing_context::types::BrowsingContext;
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmulationCommand {
    SetForcedColorsModeThemeOverride(SetForcedColorsModeThemeOverride),
    SetGeolocationOverride(SetGeolocationOverride),
    SetLocaleOverride(SetLocaleOverride),
    SetScreenOrientationOverride(SetScreenOrientationOverride),
    SetScriptingEnabled(SetScriptingEnabled),
    SetTimezoneOverride(SetTimezoneOverride),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmulationSetForcedColorsModeThemeOverrideMethod {
    #[serde(rename = "emulation.SetForcedColorsModeThemeOverride")]
    EmulationSetForcedColorsModeThemeOverride,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmulationSetGeolocationOverrideMethod {
    #[serde(rename = "emulation.SetGeolocationOverride")]
    EmulationSetGeolocationOverride,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmulationSetLocaleOverrideMethod {
    #[serde(rename = "emulation.SetLocaleOverride")]
    EmulationSetLocaleOverride,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmulationSetScreenOrientationOverrideMethod {
    #[serde(rename = "emulation.SetScreenOrientationOverride")]
    EmulationSetScreenOrientationOverride,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmulationSetScriptingEnabledMethod {
    #[serde(rename = "emulation.SetScriptingEnabled")]
    EmulationSetScriptingEnabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmulationSetTimezoneOverrideMethod {
    #[serde(rename = "emulation.SetTimezoneOverride")]
    EmulationSetTimezoneOverride,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetForcedColorsModeThemeOverrideParameters {
    #[serde(rename = "theme")]
    pub theme: Option<ForcedColorsModeTheme>,
    #[serde(rename = "contexts")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetGeolocationOverrideParameters {
    #[serde(rename = "coordinates_error")]
    pub coordinates_error: Option<CoordinatesErrorUnion>,
    #[serde(rename = "contexts")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLocaleOverrideParameters {
    #[serde(rename = "locale")]
    pub locale: Option<String>,
    #[serde(rename = "contexts")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetScreenOrientationOverrideParameters {
    #[serde(rename = "screenOrientation")]
    pub screen_orientation: Option<ScreenOrientation>,
    #[serde(rename = "contexts")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetScriptingEnabledParameters {
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "contexts")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetTimezoneOverrideParameters {
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    #[serde(rename = "contexts")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetForcedColorsModeThemeOverride {
    #[serde(rename = "method")]
    pub method: EmulationSetForcedColorsModeThemeOverrideMethod,
    #[serde(rename = "params")]
    pub params: SetForcedColorsModeThemeOverrideParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetGeolocationOverride {
    #[serde(rename = "method")]
    pub method: EmulationSetGeolocationOverrideMethod,
    #[serde(rename = "params")]
    pub params: SetGeolocationOverrideParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLocaleOverride {
    #[serde(rename = "method")]
    pub method: EmulationSetLocaleOverrideMethod,
    #[serde(rename = "params")]
    pub params: SetLocaleOverrideParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetScreenOrientationOverride {
    #[serde(rename = "method")]
    pub method: EmulationSetScreenOrientationOverrideMethod,
    #[serde(rename = "params")]
    pub params: SetScreenOrientationOverrideParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetScriptingEnabled {
    #[serde(rename = "method")]
    pub method: EmulationSetScriptingEnabledMethod,
    #[serde(rename = "params")]
    pub params: SetScriptingEnabledParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetTimezoneOverride {
    #[serde(rename = "method")]
    pub method: EmulationSetTimezoneOverrideMethod,
    #[serde(rename = "params")]
    pub params: SetTimezoneOverrideParameters,
}

