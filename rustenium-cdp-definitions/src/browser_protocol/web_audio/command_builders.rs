use super::commands::*;
impl GetRealtimeData {
    pub fn builder() -> GetRealtimeDataBuilder {
        GetRealtimeDataBuilder::default()
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
