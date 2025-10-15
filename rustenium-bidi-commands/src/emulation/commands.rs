// Generated commands for module

use serde::{Serialize, Deserialize};
use crate::browser::types::UserContext;
use crate::browsing_context::types::BrowsingContext;
use crate::EmptyResult;
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmulationCommand {
    SetForcedColorsModeThemeOverride(SetForcedColorsModeThemeOverride),
    SetGeolocationOverride(SetGeolocationOverride),
    SetLocaleOverride(SetLocaleOverride),
    SetNetworkConditions(SetNetworkConditions),
    SetScreenOrientationOverride(SetScreenOrientationOverride),
    SetScriptingEnabled(SetScriptingEnabled),
    SetTimezoneOverride(SetTimezoneOverride),
    SetUserAgentOverride(SetUserAgentOverride),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmulationSetForcedColorsModeThemeOverrideMethod {
    #[serde(rename = "emulation.setForcedColorsModeThemeOverride")]
    EmulationSetForcedColorsModeThemeOverride,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmulationSetGeolocationOverrideMethod {
    #[serde(rename = "emulation.setGeolocationOverride")]
    EmulationSetGeolocationOverride,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmulationSetLocaleOverrideMethod {
    #[serde(rename = "emulation.setLocaleOverride")]
    EmulationSetLocaleOverride,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmulationSetNetworkConditionsMethod {
    #[serde(rename = "emulation.setNetworkConditions")]
    EmulationSetNetworkConditions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmulationSetScreenOrientationOverrideMethod {
    #[serde(rename = "emulation.setScreenOrientationOverride")]
    EmulationSetScreenOrientationOverride,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmulationSetScriptingEnabledMethod {
    #[serde(rename = "emulation.setScriptingEnabled")]
    EmulationSetScriptingEnabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmulationSetTimezoneOverrideMethod {
    #[serde(rename = "emulation.setTimezoneOverride")]
    EmulationSetTimezoneOverride,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmulationSetUserAgentOverrideMethod {
    #[serde(rename = "emulation.setUserAgentOverride")]
    EmulationSetUserAgentOverride,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetForcedColorsModeThemeOverrideParameters {
    #[serde(rename = "theme")]
    pub theme: Option<ForcedColorsModeTheme>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetGeolocationOverrideParameters {
    #[serde(rename = "coordinates_error")]
    pub coordinates_error: Option<CoordinatesErrorUnion>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLocaleOverrideParameters {
    #[serde(rename = "locale")]
    pub locale: Option<String>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct setNetworkConditionsParameters {
    #[serde(rename = "networkConditions")]
    pub network_conditions: Option<NetworkConditions>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetScreenOrientationOverrideParameters {
    #[serde(rename = "screenOrientation")]
    pub screen_orientation: Option<ScreenOrientation>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetScriptingEnabledParameters {
    #[serde(rename = "enabled")]
    pub enabled: Option<bool>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetTimezoneOverrideParameters {
    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetUserAgentOverrideParameters {
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
pub struct SetNetworkConditions {
    #[serde(rename = "method")]
    pub method: EmulationSetNetworkConditionsMethod,
    #[serde(rename = "params")]
    pub params: setNetworkConditionsParameters,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetUserAgentOverride {
    #[serde(rename = "method")]
    pub method: EmulationSetUserAgentOverrideMethod,
    #[serde(rename = "params")]
    pub params: SetUserAgentOverrideParameters,
}

// Generated results

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmulationResult {
    SetForcedColorsModeThemeOverrideResult(SetForcedColorsModeThemeOverrideResult),
    SetGeolocationOverrideResult(SetGeolocationOverrideResult),
    SetLocaleOverrideResult(SetLocaleOverrideResult),
    SetScreenOrientationOverrideResult(SetScreenOrientationOverrideResult),
    SetScriptingEnabledResult(SetScriptingEnabledResult),
    SetTimezoneOverrideResult(SetTimezoneOverrideResult),
    SetUserAgentOverrideResult(SetUserAgentOverrideResult),
}


pub type SetForcedColorsModeThemeOverrideResult = EmptyResult;


pub type SetGeolocationOverrideResult = EmptyResult;


pub type SetLocaleOverrideResult = EmptyResult;


pub type SetScreenOrientationOverrideResult = EmptyResult;


pub type SetScriptingEnabledResult = EmptyResult;


pub type SetTimezoneOverrideResult = EmptyResult;


pub type SetUserAgentOverrideResult = EmptyResult;


