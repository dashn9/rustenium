use super::commands::*;
impl Install {
    pub fn builder() -> InstallBuilder {
        InstallBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct InstallBuilder {
    extension_data: Option<super::types::ExtensionData>,
}
impl InstallBuilder {
    pub fn extension_data(
        mut self,
        extension_data: impl Into<super::types::ExtensionData>,
    ) -> Self {
        self.extension_data = Some(extension_data.into());
        self
    }
    pub fn build(self) -> Result<Install, String> {
        Ok(Install {
            method: InstallMethod::Install,
            params: InstallParams {
                extension_data: self.extension_data.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(extension_data))
                })?,
            },
        })
    }
}
impl Uninstall {
    pub fn builder() -> UninstallBuilder {
        UninstallBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct UninstallBuilder {
    extension: Option<super::types::Extension>,
}
impl UninstallBuilder {
    pub fn extension(mut self, extension: impl Into<super::types::Extension>) -> Self {
        self.extension = Some(extension.into());
        self
    }
    pub fn build(self) -> Result<Uninstall, String> {
        Ok(Uninstall {
            method: UninstallMethod::Uninstall,
            params: UninstallParams {
                extension: self.extension.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(extension))
                })?,
            },
        })
    }
}
