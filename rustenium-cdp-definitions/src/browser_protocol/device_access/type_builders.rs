use super::types::*;
impl PromptDevice {
    pub fn builder() -> PromptDeviceBuilder {
        <PromptDeviceBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PromptDeviceBuilder {
    id: Option<DeviceId>,
    name: Option<String>,
}
impl PromptDeviceBuilder {
    pub fn id(mut self, id: impl Into<DeviceId>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn build(self) -> Result<PromptDevice, String> {
        Ok(PromptDevice {
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
        })
    }
}
