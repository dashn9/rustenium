use super::commands::*;
#[derive(Debug, Clone, Default)]
pub struct GetInfoBuilder;
impl GetInfoBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> GetInfo {
        GetInfo {
            method: GetInfoMethod::GetInfo,
            params: GetInfoParams {},
        }
    }
}
impl GetInfo {
    pub fn builder() -> GetInfoBuilder {
        GetInfoBuilder
    }
}
impl GetFeatureState {
    pub fn builder() -> GetFeatureStateBuilder {
        <GetFeatureStateBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetFeatureStateBuilder {
    feature_state: Option<String>,
}
impl GetFeatureStateBuilder {
    pub fn feature_state(mut self, feature_state: impl Into<String>) -> Self {
        self.feature_state = Some(feature_state.into());
        self
    }
    pub fn build(self) -> Result<GetFeatureState, String> {
        Ok(GetFeatureState {
            method: GetFeatureStateMethod::GetFeatureState,
            params: GetFeatureStateParams {
                feature_state: self.feature_state.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(feature_state))
                })?,
            },
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct GetProcessInfoBuilder;
impl GetProcessInfoBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> GetProcessInfo {
        GetProcessInfo {
            method: GetProcessInfoMethod::GetProcessInfo,
            params: GetProcessInfoParams {},
        }
    }
}
impl GetProcessInfo {
    pub fn builder() -> GetProcessInfoBuilder {
        GetProcessInfoBuilder
    }
}
