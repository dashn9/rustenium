use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetForcedColorsModeThemeOverrideParams {
    #[serde(rename = "theme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub theme: Option<super::types::ForcedColorsModeTheme>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetForcedColorsModeThemeOverrideMethod {
    #[serde(rename = "emulation.setForcedColorsModeThemeOverride")]
    SetForcedColorsModeThemeOverride,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetForcedColorsModeThemeOverride {
    pub method: SetForcedColorsModeThemeOverrideMethod,
    pub params: SetForcedColorsModeThemeOverrideParams,
}
impl SetForcedColorsModeThemeOverride {
    pub const IDENTIFIER: &'static str = "emulation.setForcedColorsModeThemeOverride";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetForcedColorsModeThemeOverride {
    type Result = super::results::SetForcedColorsModeThemeOverrideResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGeolocationOverrideParams {
    #[serde(rename = "(error")]
    pub error: super::types::GeolocationPositionError,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl SetGeolocationOverrideParams {
    pub fn new(error: impl Into<super::types::GeolocationPositionError>) -> Self {
        Self {
            error: error.into(),
            contexts: None,
            user_contexts: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetGeolocationOverrideMethod {
    #[serde(rename = "emulation.setGeolocationOverride")]
    SetGeolocationOverride,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetGeolocationOverride {
    pub method: SetGeolocationOverrideMethod,
    pub params: SetGeolocationOverrideParams,
}
impl SetGeolocationOverride {
    pub const IDENTIFIER: &'static str = "emulation.setGeolocationOverride";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetGeolocationOverride {
    type Result = super::results::SetGeolocationOverrideResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetLocaleOverrideParams {
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub locale: Option<String>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetLocaleOverrideMethod {
    #[serde(rename = "emulation.setLocaleOverride")]
    SetLocaleOverride,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetLocaleOverride {
    pub method: SetLocaleOverrideMethod,
    pub params: SetLocaleOverrideParams,
}
impl SetLocaleOverride {
    pub const IDENTIFIER: &'static str = "emulation.setLocaleOverride";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetLocaleOverride {
    type Result = super::results::SetLocaleOverrideResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetNetworkConditionsParams {
    #[serde(rename = "networkConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub network_conditions: Option<super::types::NetworkConditions>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetNetworkConditionsMethod {
    #[serde(rename = "emulation.setNetworkConditions")]
    SetNetworkConditions,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetNetworkConditions {
    pub method: SetNetworkConditionsMethod,
    pub params: SetNetworkConditionsParams,
}
impl SetNetworkConditions {
    pub const IDENTIFIER: &'static str = "emulation.setNetworkConditions";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetNetworkConditions {
    type Result = super::results::SetNetworkConditionsResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetScreenOrientationOverrideParams {
    #[serde(rename = "screenOrientation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub screen_orientation: Option<super::types::ScreenOrientation>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetScreenOrientationOverrideMethod {
    #[serde(rename = "emulation.setScreenOrientationOverride")]
    SetScreenOrientationOverride,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetScreenOrientationOverride {
    pub method: SetScreenOrientationOverrideMethod,
    pub params: SetScreenOrientationOverrideParams,
}
impl SetScreenOrientationOverride {
    pub const IDENTIFIER: &'static str = "emulation.setScreenOrientationOverride";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetScreenOrientationOverride {
    type Result = super::results::SetScreenOrientationOverrideResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetUserAgentOverrideParams {
    #[serde(rename = "userAgent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_agent: Option<String>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetUserAgentOverrideMethod {
    #[serde(rename = "emulation.setUserAgentOverride")]
    SetUserAgentOverride,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetUserAgentOverride {
    pub method: SetUserAgentOverrideMethod,
    pub params: SetUserAgentOverrideParams,
}
impl SetUserAgentOverride {
    pub const IDENTIFIER: &'static str = "emulation.setUserAgentOverride";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetUserAgentOverride {
    type Result = super::results::SetUserAgentOverrideResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetScriptingEnabledParams {
    #[serde(rename = "enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub enabled: Option<bool>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetScriptingEnabledMethod {
    #[serde(rename = "emulation.setScriptingEnabled")]
    SetScriptingEnabled,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetScriptingEnabled {
    pub method: SetScriptingEnabledMethod,
    pub params: SetScriptingEnabledParams,
}
impl SetScriptingEnabled {
    pub const IDENTIFIER: &'static str = "emulation.setScriptingEnabled";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetScriptingEnabled {
    type Result = super::results::SetScriptingEnabledResult;
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetTimezoneOverrideParams {
    #[serde(rename = "timezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetTimezoneOverrideMethod {
    #[serde(rename = "emulation.setTimezoneOverride")]
    SetTimezoneOverride,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetTimezoneOverride {
    pub method: SetTimezoneOverrideMethod,
    pub params: SetTimezoneOverrideParams,
}
impl SetTimezoneOverride {
    pub const IDENTIFIER: &'static str = "emulation.setTimezoneOverride";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SetTimezoneOverride {
    type Result = super::results::SetTimezoneOverrideResult;
}
group_enum ! (EmulationCommand { SetForcedColorsModeThemeOverride (SetForcedColorsModeThemeOverride) , SetGeolocationOverride (SetGeolocationOverride) , SetLocaleOverride (SetLocaleOverride) , SetNetworkConditions (SetNetworkConditions) , SetScreenOrientationOverride (SetScreenOrientationOverride) , SetUserAgentOverride (SetUserAgentOverride) , SetScriptingEnabled (SetScriptingEnabled) , SetTimezoneOverride (SetTimezoneOverride) });
