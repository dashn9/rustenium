use super::commands::*;
impl GetFeatureState {
    pub fn builder() -> GetFeatureStateBuilder {
        GetFeatureStateBuilder::default()
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
