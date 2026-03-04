use super::commands::*;
impl SelectPrompt {
    pub fn builder() -> SelectPromptBuilder {
        SelectPromptBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SelectPromptBuilder {
    id: Option<super::types::RequestId>,
    device_id: Option<super::types::DeviceId>,
}
impl SelectPromptBuilder {
    pub fn id(mut self, id: impl Into<super::types::RequestId>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn device_id(mut self, device_id: impl Into<super::types::DeviceId>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }
    pub fn build(self) -> Result<SelectPrompt, String> {
        Ok(SelectPrompt {
            method: SelectPromptMethod::SelectPrompt,
            params: SelectPromptParams {
                id: self
                    .id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
                device_id: self.device_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(device_id))
                })?,
            },
        })
    }
}
impl CancelPrompt {
    pub fn builder() -> CancelPromptBuilder {
        CancelPromptBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct CancelPromptBuilder {
    id: Option<super::types::RequestId>,
}
impl CancelPromptBuilder {
    pub fn id(mut self, id: impl Into<super::types::RequestId>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn build(self) -> Result<CancelPrompt, String> {
        Ok(CancelPrompt {
            method: CancelPromptMethod::CancelPrompt,
            params: CancelPromptParams {
                id: self
                    .id
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            },
        })
    }
}
