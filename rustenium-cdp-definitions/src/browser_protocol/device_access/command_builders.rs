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
impl SelectPrompt {
    pub fn builder() -> SelectPromptBuilder {
        <SelectPromptBuilder as Default>::default()
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
        <CancelPromptBuilder as Default>::default()
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
