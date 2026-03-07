use super::commands::*;
impl GetOsAppState {
    pub fn builder() -> GetOsAppStateBuilder {
        <GetOsAppStateBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetOsAppStateBuilder {
    manifest_id: Option<String>,
}
impl GetOsAppStateBuilder {
    pub fn manifest_id(mut self, manifest_id: impl Into<String>) -> Self {
        self.manifest_id = Some(manifest_id.into());
        self
    }
    pub fn build(self) -> Result<GetOsAppState, String> {
        Ok(GetOsAppState {
            method: GetOsAppStateMethod::GetOsAppState,
            params: GetOsAppStateParams {
                manifest_id: self.manifest_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(manifest_id))
                })?,
            },
        })
    }
}
impl Install {
    pub fn builder() -> InstallBuilder {
        <InstallBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct InstallBuilder {
    manifest_id: Option<String>,
    install_url_or_bundle_url: Option<String>,
}
impl InstallBuilder {
    pub fn manifest_id(mut self, manifest_id: impl Into<String>) -> Self {
        self.manifest_id = Some(manifest_id.into());
        self
    }
    pub fn install_url_or_bundle_url(
        mut self,
        install_url_or_bundle_url: impl Into<String>,
    ) -> Self {
        self.install_url_or_bundle_url = Some(install_url_or_bundle_url.into());
        self
    }
    pub fn build(self) -> Result<Install, String> {
        Ok(Install {
            method: InstallMethod::Install,
            params: InstallParams {
                manifest_id: self.manifest_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(manifest_id))
                })?,
                install_url_or_bundle_url: self.install_url_or_bundle_url,
            },
        })
    }
}
impl Uninstall {
    pub fn builder() -> UninstallBuilder {
        <UninstallBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct UninstallBuilder {
    manifest_id: Option<String>,
}
impl UninstallBuilder {
    pub fn manifest_id(mut self, manifest_id: impl Into<String>) -> Self {
        self.manifest_id = Some(manifest_id.into());
        self
    }
    pub fn build(self) -> Result<Uninstall, String> {
        Ok(Uninstall {
            method: UninstallMethod::Uninstall,
            params: UninstallParams {
                manifest_id: self.manifest_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(manifest_id))
                })?,
            },
        })
    }
}
impl Launch {
    pub fn builder() -> LaunchBuilder {
        <LaunchBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct LaunchBuilder {
    manifest_id: Option<String>,
    url: Option<String>,
}
impl LaunchBuilder {
    pub fn manifest_id(mut self, manifest_id: impl Into<String>) -> Self {
        self.manifest_id = Some(manifest_id.into());
        self
    }
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn build(self) -> Result<Launch, String> {
        Ok(Launch {
            method: LaunchMethod::Launch,
            params: LaunchParams {
                manifest_id: self.manifest_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(manifest_id))
                })?,
                url: self.url,
            },
        })
    }
}
impl LaunchFilesInApp {
    pub fn builder() -> LaunchFilesInAppBuilder {
        <LaunchFilesInAppBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct LaunchFilesInAppBuilder {
    manifest_id: Option<String>,
    files: Option<Vec<String>>,
}
impl LaunchFilesInAppBuilder {
    pub fn manifest_id(mut self, manifest_id: impl Into<String>) -> Self {
        self.manifest_id = Some(manifest_id.into());
        self
    }
    pub fn file(mut self, file: impl Into<String>) -> Self {
        let v = self.files.get_or_insert(Vec::new());
        v.push(file.into());
        self
    }
    pub fn files<I, S>(mut self, files: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.files.get_or_insert(Vec::new());
        for val in files {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<LaunchFilesInApp, String> {
        Ok(LaunchFilesInApp {
            method: LaunchFilesInAppMethod::LaunchFilesInApp,
            params: LaunchFilesInAppParams {
                manifest_id: self.manifest_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(manifest_id))
                })?,
                files: self
                    .files
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(files)))?,
            },
        })
    }
}
impl OpenCurrentPageInApp {
    pub fn builder() -> OpenCurrentPageInAppBuilder {
        <OpenCurrentPageInAppBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct OpenCurrentPageInAppBuilder {
    manifest_id: Option<String>,
}
impl OpenCurrentPageInAppBuilder {
    pub fn manifest_id(mut self, manifest_id: impl Into<String>) -> Self {
        self.manifest_id = Some(manifest_id.into());
        self
    }
    pub fn build(self) -> Result<OpenCurrentPageInApp, String> {
        Ok(OpenCurrentPageInApp {
            method: OpenCurrentPageInAppMethod::OpenCurrentPageInApp,
            params: OpenCurrentPageInAppParams {
                manifest_id: self.manifest_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(manifest_id))
                })?,
            },
        })
    }
}
impl ChangeAppUserSettings {
    pub fn builder() -> ChangeAppUserSettingsBuilder {
        <ChangeAppUserSettingsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ChangeAppUserSettingsBuilder {
    manifest_id: Option<String>,
    link_capturing: Option<bool>,
    display_mode: Option<super::types::DisplayMode>,
}
impl ChangeAppUserSettingsBuilder {
    pub fn manifest_id(mut self, manifest_id: impl Into<String>) -> Self {
        self.manifest_id = Some(manifest_id.into());
        self
    }
    pub fn link_capturing(mut self, link_capturing: impl Into<bool>) -> Self {
        self.link_capturing = Some(link_capturing.into());
        self
    }
    pub fn display_mode(mut self, display_mode: impl Into<super::types::DisplayMode>) -> Self {
        self.display_mode = Some(display_mode.into());
        self
    }
    pub fn build(self) -> Result<ChangeAppUserSettings, String> {
        Ok(ChangeAppUserSettings {
            method: ChangeAppUserSettingsMethod::ChangeAppUserSettings,
            params: ChangeAppUserSettingsParams {
                manifest_id: self.manifest_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(manifest_id))
                })?,
                link_capturing: self.link_capturing,
                display_mode: self.display_mode,
            },
        })
    }
}
