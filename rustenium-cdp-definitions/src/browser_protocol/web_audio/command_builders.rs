use super::commands::*;
#[derive(Debug, Clone, Default)]
pub struct EnableBuilder;
impl EnableBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Enable {
        Enable {
            method: EnableMethod::Enable,
            params: EnableParams {},
        }
    }
}
impl Enable {
    pub fn builder() -> EnableBuilder {
        EnableBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct DisableBuilder;
impl DisableBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Disable {
        Disable {
            method: DisableMethod::Disable,
            params: DisableParams {},
        }
    }
}
impl Disable {
    pub fn builder() -> DisableBuilder {
        DisableBuilder
    }
}
impl GetRealtimeData {
    pub fn builder() -> GetRealtimeDataBuilder {
        <GetRealtimeDataBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetRealtimeDataBuilder {
    context_id: Option<super::types::GraphObjectId>,
}
impl GetRealtimeDataBuilder {
    pub fn context_id(mut self, context_id: impl Into<super::types::GraphObjectId>) -> Self {
        self.context_id = Some(context_id.into());
        self
    }
    pub fn build(self) -> Result<GetRealtimeData, String> {
        Ok(GetRealtimeData {
            method: GetRealtimeDataMethod::GetRealtimeData,
            params: GetRealtimeDataParams {
                context_id: self.context_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(context_id))
                })?,
            },
        })
    }
}
