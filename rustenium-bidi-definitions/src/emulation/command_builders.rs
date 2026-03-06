use super::commands::*;
impl SetForcedColorsModeThemeOverride {
    pub fn builder() -> SetForcedColorsModeThemeOverrideBuilder {
        SetForcedColorsModeThemeOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetForcedColorsModeThemeOverrideBuilder {
    theme: Option<super::types::ForcedColorsModeTheme>,
    contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl SetForcedColorsModeThemeOverrideBuilder {
    pub fn theme(mut self, theme: impl Into<super::types::ForcedColorsModeTheme>) -> Self {
        self.theme = Some(theme.into());
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        let v = self.contexts.get_or_insert(Vec::new());
        v.push(context.into());
        self
    }
    pub fn contexts<I, S>(mut self, contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browsing_context::types::BrowsingContext>,
    {
        let v = self.contexts.get_or_insert(Vec::new());
        for val in contexts {
            v.push(val.into());
        }
        self
    }
    pub fn user_context(
        mut self,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        let v = self.user_contexts.get_or_insert(Vec::new());
        v.push(user_context.into());
        self
    }
    pub fn user_contexts<I, S>(mut self, user_contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser::types::UserContext>,
    {
        let v = self.user_contexts.get_or_insert(Vec::new());
        for val in user_contexts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> SetForcedColorsModeThemeOverride {
        SetForcedColorsModeThemeOverride {
            method: SetForcedColorsModeThemeOverrideMethod::SetForcedColorsModeThemeOverride,
            params: SetForcedColorsModeThemeOverrideParams {
                theme: self.theme,
                contexts: self.contexts,
                user_contexts: self.user_contexts,
            },
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
    coordinates: Option<super::types::GeolocationCoordinates>,
    error: Option<super::types::GeolocationPositionError>,
    contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl SetGeolocationOverrideBuilder {
    pub fn coordinates(
        mut self,
        coordinates: impl Into<super::types::GeolocationCoordinates>,
    ) -> Self {
        self.coordinates = Some(coordinates.into());
        self
    }
    pub fn error(mut self, error: impl Into<super::types::GeolocationPositionError>) -> Self {
        self.error = Some(error.into());
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        let v = self.contexts.get_or_insert(Vec::new());
        v.push(context.into());
        self
    }
    pub fn contexts<I, S>(mut self, contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browsing_context::types::BrowsingContext>,
    {
        let v = self.contexts.get_or_insert(Vec::new());
        for val in contexts {
            v.push(val.into());
        }
        self
    }
    pub fn user_context(
        mut self,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        let v = self.user_contexts.get_or_insert(Vec::new());
        v.push(user_context.into());
        self
    }
    pub fn user_contexts<I, S>(mut self, user_contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser::types::UserContext>,
    {
        let v = self.user_contexts.get_or_insert(Vec::new());
        for val in user_contexts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<SetGeolocationOverride, String> {
        Ok(SetGeolocationOverride {
            method: SetGeolocationOverrideMethod::SetGeolocationOverride,
            params: SetGeolocationOverrideParams {
                coordinates: self.coordinates,
                error: self
                    .error
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(error)))?,
                contexts: self.contexts,
                user_contexts: self.user_contexts,
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
    contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl SetLocaleOverrideBuilder {
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        let v = self.contexts.get_or_insert(Vec::new());
        v.push(context.into());
        self
    }
    pub fn contexts<I, S>(mut self, contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browsing_context::types::BrowsingContext>,
    {
        let v = self.contexts.get_or_insert(Vec::new());
        for val in contexts {
            v.push(val.into());
        }
        self
    }
    pub fn user_context(
        mut self,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        let v = self.user_contexts.get_or_insert(Vec::new());
        v.push(user_context.into());
        self
    }
    pub fn user_contexts<I, S>(mut self, user_contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser::types::UserContext>,
    {
        let v = self.user_contexts.get_or_insert(Vec::new());
        for val in user_contexts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> SetLocaleOverride {
        SetLocaleOverride {
            method: SetLocaleOverrideMethod::SetLocaleOverride,
            params: SetLocaleOverrideParams {
                locale: self.locale,
                contexts: self.contexts,
                user_contexts: self.user_contexts,
            },
        }
    }
}
impl SetNetworkConditions {
    pub fn builder() -> SetNetworkConditionsBuilder {
        SetNetworkConditionsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetNetworkConditionsBuilder {
    network_conditions: Option<super::types::NetworkConditions>,
    contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl SetNetworkConditionsBuilder {
    pub fn network_conditions(
        mut self,
        network_conditions: impl Into<super::types::NetworkConditions>,
    ) -> Self {
        self.network_conditions = Some(network_conditions.into());
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        let v = self.contexts.get_or_insert(Vec::new());
        v.push(context.into());
        self
    }
    pub fn contexts<I, S>(mut self, contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browsing_context::types::BrowsingContext>,
    {
        let v = self.contexts.get_or_insert(Vec::new());
        for val in contexts {
            v.push(val.into());
        }
        self
    }
    pub fn user_context(
        mut self,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        let v = self.user_contexts.get_or_insert(Vec::new());
        v.push(user_context.into());
        self
    }
    pub fn user_contexts<I, S>(mut self, user_contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser::types::UserContext>,
    {
        let v = self.user_contexts.get_or_insert(Vec::new());
        for val in user_contexts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> SetNetworkConditions {
        SetNetworkConditions {
            method: SetNetworkConditionsMethod::SetNetworkConditions,
            params: SetNetworkConditionsParams {
                network_conditions: self.network_conditions,
                contexts: self.contexts,
                user_contexts: self.user_contexts,
            },
        }
    }
}
impl SetScreenOrientationOverride {
    pub fn builder() -> SetScreenOrientationOverrideBuilder {
        SetScreenOrientationOverrideBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetScreenOrientationOverrideBuilder {
    screen_orientation: Option<super::types::ScreenOrientation>,
    contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl SetScreenOrientationOverrideBuilder {
    pub fn screen_orientation(
        mut self,
        screen_orientation: impl Into<super::types::ScreenOrientation>,
    ) -> Self {
        self.screen_orientation = Some(screen_orientation.into());
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        let v = self.contexts.get_or_insert(Vec::new());
        v.push(context.into());
        self
    }
    pub fn contexts<I, S>(mut self, contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browsing_context::types::BrowsingContext>,
    {
        let v = self.contexts.get_or_insert(Vec::new());
        for val in contexts {
            v.push(val.into());
        }
        self
    }
    pub fn user_context(
        mut self,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        let v = self.user_contexts.get_or_insert(Vec::new());
        v.push(user_context.into());
        self
    }
    pub fn user_contexts<I, S>(mut self, user_contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser::types::UserContext>,
    {
        let v = self.user_contexts.get_or_insert(Vec::new());
        for val in user_contexts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> SetScreenOrientationOverride {
        SetScreenOrientationOverride {
            method: SetScreenOrientationOverrideMethod::SetScreenOrientationOverride,
            params: SetScreenOrientationOverrideParams {
                screen_orientation: self.screen_orientation,
                contexts: self.contexts,
                user_contexts: self.user_contexts,
            },
        }
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
    contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl SetUserAgentOverrideBuilder {
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        let v = self.contexts.get_or_insert(Vec::new());
        v.push(context.into());
        self
    }
    pub fn contexts<I, S>(mut self, contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browsing_context::types::BrowsingContext>,
    {
        let v = self.contexts.get_or_insert(Vec::new());
        for val in contexts {
            v.push(val.into());
        }
        self
    }
    pub fn user_context(
        mut self,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        let v = self.user_contexts.get_or_insert(Vec::new());
        v.push(user_context.into());
        self
    }
    pub fn user_contexts<I, S>(mut self, user_contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser::types::UserContext>,
    {
        let v = self.user_contexts.get_or_insert(Vec::new());
        for val in user_contexts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> SetUserAgentOverride {
        SetUserAgentOverride {
            method: SetUserAgentOverrideMethod::SetUserAgentOverride,
            params: SetUserAgentOverrideParams {
                user_agent: self.user_agent,
                contexts: self.contexts,
                user_contexts: self.user_contexts,
            },
        }
    }
}
impl SetScriptingEnabled {
    pub fn builder() -> SetScriptingEnabledBuilder {
        SetScriptingEnabledBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetScriptingEnabledBuilder {
    enabled: Option<bool>,
    contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl SetScriptingEnabledBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        let v = self.contexts.get_or_insert(Vec::new());
        v.push(context.into());
        self
    }
    pub fn contexts<I, S>(mut self, contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browsing_context::types::BrowsingContext>,
    {
        let v = self.contexts.get_or_insert(Vec::new());
        for val in contexts {
            v.push(val.into());
        }
        self
    }
    pub fn user_context(
        mut self,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        let v = self.user_contexts.get_or_insert(Vec::new());
        v.push(user_context.into());
        self
    }
    pub fn user_contexts<I, S>(mut self, user_contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser::types::UserContext>,
    {
        let v = self.user_contexts.get_or_insert(Vec::new());
        for val in user_contexts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> SetScriptingEnabled {
        SetScriptingEnabled {
            method: SetScriptingEnabledMethod::SetScriptingEnabled,
            params: SetScriptingEnabledParams {
                enabled: self.enabled,
                contexts: self.contexts,
                user_contexts: self.user_contexts,
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
    timezone: Option<String>,
    contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl SetTimezoneOverrideBuilder {
    pub fn timezone(mut self, timezone: impl Into<String>) -> Self {
        self.timezone = Some(timezone.into());
        self
    }
    pub fn context(
        mut self,
        context: impl Into<crate::browsing_context::types::BrowsingContext>,
    ) -> Self {
        let v = self.contexts.get_or_insert(Vec::new());
        v.push(context.into());
        self
    }
    pub fn contexts<I, S>(mut self, contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browsing_context::types::BrowsingContext>,
    {
        let v = self.contexts.get_or_insert(Vec::new());
        for val in contexts {
            v.push(val.into());
        }
        self
    }
    pub fn user_context(
        mut self,
        user_context: impl Into<crate::browser::types::UserContext>,
    ) -> Self {
        let v = self.user_contexts.get_or_insert(Vec::new());
        v.push(user_context.into());
        self
    }
    pub fn user_contexts<I, S>(mut self, user_contexts: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<crate::browser::types::UserContext>,
    {
        let v = self.user_contexts.get_or_insert(Vec::new());
        for val in user_contexts {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> SetTimezoneOverride {
        SetTimezoneOverride {
            method: SetTimezoneOverrideMethod::SetTimezoneOverride,
            params: SetTimezoneOverrideParams {
                timezone: self.timezone,
                contexts: self.contexts,
                user_contexts: self.user_contexts,
            },
        }
    }
}
