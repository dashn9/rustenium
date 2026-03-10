use super::commands::*;
impl SetPermission {
    pub fn builder() -> SetPermissionBuilder {
        <SetPermissionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetPermissionBuilder {
    permission: Option<super::types::PermissionDescriptor>,
    setting: Option<super::types::PermissionSetting>,
    origin: Option<String>,
    embedded_origin: Option<String>,
    browser_context_id: Option<super::types::BrowserContextId>,
}
impl SetPermissionBuilder {
    pub fn permission(mut self, permission: impl Into<super::types::PermissionDescriptor>) -> Self {
        self.permission = Some(permission.into());
        self
    }
    pub fn setting(mut self, setting: impl Into<super::types::PermissionSetting>) -> Self {
        self.setting = Some(setting.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn embedded_origin(mut self, embedded_origin: impl Into<String>) -> Self {
        self.embedded_origin = Some(embedded_origin.into());
        self
    }
    pub fn browser_context_id(
        mut self,
        browser_context_id: impl Into<super::types::BrowserContextId>,
    ) -> Self {
        self.browser_context_id = Some(browser_context_id.into());
        self
    }
    pub fn build(self) -> Result<SetPermission, String> {
        Ok(SetPermission {
            method: SetPermissionMethod::SetPermission,
            params: SetPermissionParams {
                permission: self.permission.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(permission))
                })?,
                setting: self
                    .setting
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(setting)))?,
                origin: self.origin,
                embedded_origin: self.embedded_origin,
                browser_context_id: self.browser_context_id.map(Box::new),
            },
        })
    }
}
impl ResetPermissions {
    pub fn builder() -> ResetPermissionsBuilder {
        <ResetPermissionsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ResetPermissionsBuilder {
    browser_context_id: Option<super::types::BrowserContextId>,
}
impl ResetPermissionsBuilder {
    pub fn browser_context_id(
        mut self,
        browser_context_id: impl Into<super::types::BrowserContextId>,
    ) -> Self {
        self.browser_context_id = Some(browser_context_id.into());
        self
    }
    pub fn build(self) -> ResetPermissions {
        ResetPermissions {
            method: ResetPermissionsMethod::ResetPermissions,
            params: ResetPermissionsParams {
                browser_context_id: self.browser_context_id.map(Box::new),
            },
        }
    }
}
impl SetDownloadBehavior {
    pub fn builder() -> SetDownloadBehaviorBuilder {
        <SetDownloadBehaviorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDownloadBehaviorBuilder {
    behavior: Option<SetDownloadBehaviorBehavior>,
    browser_context_id: Option<super::types::BrowserContextId>,
    download_path: Option<String>,
    events_enabled: Option<bool>,
}
impl SetDownloadBehaviorBuilder {
    pub fn behavior(mut self, behavior: impl Into<SetDownloadBehaviorBehavior>) -> Self {
        self.behavior = Some(behavior.into());
        self
    }
    pub fn browser_context_id(
        mut self,
        browser_context_id: impl Into<super::types::BrowserContextId>,
    ) -> Self {
        self.browser_context_id = Some(browser_context_id.into());
        self
    }
    pub fn download_path(mut self, download_path: impl Into<String>) -> Self {
        self.download_path = Some(download_path.into());
        self
    }
    pub fn events_enabled(mut self, events_enabled: impl Into<bool>) -> Self {
        self.events_enabled = Some(events_enabled.into());
        self
    }
    pub fn build(self) -> Result<SetDownloadBehavior, String> {
        Ok(SetDownloadBehavior {
            method: SetDownloadBehaviorMethod::SetDownloadBehavior,
            params: SetDownloadBehaviorParams {
                behavior: self.behavior.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(behavior))
                })?,
                browser_context_id: self.browser_context_id.map(Box::new),
                download_path: self.download_path,
                events_enabled: self.events_enabled,
            },
        })
    }
}
impl CancelDownload {
    pub fn builder() -> CancelDownloadBuilder {
        <CancelDownloadBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct CancelDownloadBuilder {
    guid: Option<String>,
    browser_context_id: Option<super::types::BrowserContextId>,
}
impl CancelDownloadBuilder {
    pub fn guid(mut self, guid: impl Into<String>) -> Self {
        self.guid = Some(guid.into());
        self
    }
    pub fn browser_context_id(
        mut self,
        browser_context_id: impl Into<super::types::BrowserContextId>,
    ) -> Self {
        self.browser_context_id = Some(browser_context_id.into());
        self
    }
    pub fn build(self) -> Result<CancelDownload, String> {
        Ok(CancelDownload {
            method: CancelDownloadMethod::CancelDownload,
            params: CancelDownloadParams {
                guid: self
                    .guid
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(guid)))?,
                browser_context_id: self.browser_context_id.map(Box::new),
            },
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct CloseBuilder;
impl CloseBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Close {
        Close {
            method: CloseMethod::Close,
            params: CloseParams {},
        }
    }
}
impl Close {
    pub fn builder() -> CloseBuilder {
        CloseBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct CrashBuilder;
impl CrashBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Crash {
        Crash {
            method: CrashMethod::Crash,
            params: CrashParams {},
        }
    }
}
impl Crash {
    pub fn builder() -> CrashBuilder {
        CrashBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct CrashGpuProcessBuilder;
impl CrashGpuProcessBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> CrashGpuProcess {
        CrashGpuProcess {
            method: CrashGpuProcessMethod::CrashGpuProcess,
            params: CrashGpuProcessParams {},
        }
    }
}
impl CrashGpuProcess {
    pub fn builder() -> CrashGpuProcessBuilder {
        CrashGpuProcessBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct GetVersionBuilder;
impl GetVersionBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> GetVersion {
        GetVersion {
            method: GetVersionMethod::GetVersion,
            params: GetVersionParams {},
        }
    }
}
impl GetVersion {
    pub fn builder() -> GetVersionBuilder {
        GetVersionBuilder
    }
}
#[derive(Debug, Clone, Default)]
pub struct GetBrowserCommandLineBuilder;
impl GetBrowserCommandLineBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> GetBrowserCommandLine {
        GetBrowserCommandLine {
            method: GetBrowserCommandLineMethod::GetBrowserCommandLine,
            params: GetBrowserCommandLineParams {},
        }
    }
}
impl GetBrowserCommandLine {
    pub fn builder() -> GetBrowserCommandLineBuilder {
        GetBrowserCommandLineBuilder
    }
}
impl GetHistograms {
    pub fn builder() -> GetHistogramsBuilder {
        <GetHistogramsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetHistogramsBuilder {
    query: Option<String>,
    delta: Option<bool>,
}
impl GetHistogramsBuilder {
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }
    pub fn delta(mut self, delta: impl Into<bool>) -> Self {
        self.delta = Some(delta.into());
        self
    }
    pub fn build(self) -> GetHistograms {
        GetHistograms {
            method: GetHistogramsMethod::GetHistograms,
            params: GetHistogramsParams {
                query: self.query,
                delta: self.delta,
            },
        }
    }
}
impl GetHistogram {
    pub fn builder() -> GetHistogramBuilder {
        <GetHistogramBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetHistogramBuilder {
    name: Option<String>,
    delta: Option<bool>,
}
impl GetHistogramBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn delta(mut self, delta: impl Into<bool>) -> Self {
        self.delta = Some(delta.into());
        self
    }
    pub fn build(self) -> Result<GetHistogram, String> {
        Ok(GetHistogram {
            method: GetHistogramMethod::GetHistogram,
            params: GetHistogramParams {
                name: self
                    .name
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
                delta: self.delta,
            },
        })
    }
}
impl GetWindowBounds {
    pub fn builder() -> GetWindowBoundsBuilder {
        <GetWindowBoundsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetWindowBoundsBuilder {
    window_id: Option<super::types::WindowId>,
}
impl GetWindowBoundsBuilder {
    pub fn window_id(mut self, window_id: impl Into<super::types::WindowId>) -> Self {
        self.window_id = Some(window_id.into());
        self
    }
    pub fn build(self) -> Result<GetWindowBounds, String> {
        Ok(GetWindowBounds {
            method: GetWindowBoundsMethod::GetWindowBounds,
            params: GetWindowBoundsParams {
                window_id: self.window_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(window_id))
                })?,
            },
        })
    }
}
impl GetWindowForTarget {
    pub fn builder() -> GetWindowForTargetBuilder {
        <GetWindowForTargetBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GetWindowForTargetBuilder {
    target_id: Option<crate::browser_protocol::target::types::TargetId>,
}
impl GetWindowForTargetBuilder {
    pub fn target_id(
        mut self,
        target_id: impl Into<crate::browser_protocol::target::types::TargetId>,
    ) -> Self {
        self.target_id = Some(target_id.into());
        self
    }
    pub fn build(self) -> GetWindowForTarget {
        GetWindowForTarget {
            method: GetWindowForTargetMethod::GetWindowForTarget,
            params: GetWindowForTargetParams {
                target_id: self.target_id,
            },
        }
    }
}
impl SetWindowBounds {
    pub fn builder() -> SetWindowBoundsBuilder {
        <SetWindowBoundsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetWindowBoundsBuilder {
    window_id: Option<super::types::WindowId>,
    bounds: Option<super::types::Bounds>,
}
impl SetWindowBoundsBuilder {
    pub fn window_id(mut self, window_id: impl Into<super::types::WindowId>) -> Self {
        self.window_id = Some(window_id.into());
        self
    }
    pub fn bounds(mut self, bounds: impl Into<super::types::Bounds>) -> Self {
        self.bounds = Some(bounds.into());
        self
    }
    pub fn build(self) -> Result<SetWindowBounds, String> {
        Ok(SetWindowBounds {
            method: SetWindowBoundsMethod::SetWindowBounds,
            params: SetWindowBoundsParams {
                window_id: self.window_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(window_id))
                })?,
                bounds: self
                    .bounds
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(bounds)))?,
            },
        })
    }
}
impl SetContentsSize {
    pub fn builder() -> SetContentsSizeBuilder {
        <SetContentsSizeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetContentsSizeBuilder {
    window_id: Option<super::types::WindowId>,
    width: Option<i64>,
    height: Option<i64>,
}
impl SetContentsSizeBuilder {
    pub fn window_id(mut self, window_id: impl Into<super::types::WindowId>) -> Self {
        self.window_id = Some(window_id.into());
        self
    }
    pub fn width(mut self, width: impl Into<i64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn height(mut self, height: impl Into<i64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn build(self) -> Result<SetContentsSize, String> {
        Ok(SetContentsSize {
            method: SetContentsSizeMethod::SetContentsSize,
            params: SetContentsSizeParams {
                window_id: self.window_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(window_id))
                })?,
                width: self.width,
                height: self.height,
            },
        })
    }
}
impl SetDockTile {
    pub fn builder() -> SetDockTileBuilder {
        <SetDockTileBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SetDockTileBuilder {
    badge_label: Option<String>,
    image: Option<crate::Binary>,
}
impl SetDockTileBuilder {
    pub fn badge_label(mut self, badge_label: impl Into<String>) -> Self {
        self.badge_label = Some(badge_label.into());
        self
    }
    pub fn image(mut self, image: impl Into<crate::Binary>) -> Self {
        self.image = Some(image.into());
        self
    }
    pub fn build(self) -> SetDockTile {
        SetDockTile {
            method: SetDockTileMethod::SetDockTile,
            params: SetDockTileParams {
                badge_label: self.badge_label,
                image: self.image,
            },
        }
    }
}
impl ExecuteBrowserCommand {
    pub fn builder() -> ExecuteBrowserCommandBuilder {
        <ExecuteBrowserCommandBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ExecuteBrowserCommandBuilder {
    command_id: Option<super::types::BrowserCommandId>,
}
impl ExecuteBrowserCommandBuilder {
    pub fn command_id(mut self, command_id: impl Into<super::types::BrowserCommandId>) -> Self {
        self.command_id = Some(command_id.into());
        self
    }
    pub fn build(self) -> Result<ExecuteBrowserCommand, String> {
        Ok(ExecuteBrowserCommand {
            method: ExecuteBrowserCommandMethod::ExecuteBrowserCommand,
            params: ExecuteBrowserCommandParams {
                command_id: self.command_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(command_id))
                })?,
            },
        })
    }
}
impl AddPrivacySandboxEnrollmentOverride {
    pub fn builder() -> AddPrivacySandboxEnrollmentOverrideBuilder {
        <AddPrivacySandboxEnrollmentOverrideBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddPrivacySandboxEnrollmentOverrideBuilder {
    url: Option<String>,
}
impl AddPrivacySandboxEnrollmentOverrideBuilder {
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
    pub fn build(self) -> Result<AddPrivacySandboxEnrollmentOverride, String> {
        Ok(AddPrivacySandboxEnrollmentOverride {
            method: AddPrivacySandboxEnrollmentOverrideMethod::AddPrivacySandboxEnrollmentOverride,
            params: AddPrivacySandboxEnrollmentOverrideParams {
                url: self
                    .url
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(url)))?,
            },
        })
    }
}
impl AddPrivacySandboxCoordinatorKeyConfig {
    pub fn builder() -> AddPrivacySandboxCoordinatorKeyConfigBuilder {
        <AddPrivacySandboxCoordinatorKeyConfigBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AddPrivacySandboxCoordinatorKeyConfigBuilder {
    api: Option<super::types::PrivacySandboxApi>,
    coordinator_origin: Option<String>,
    key_config: Option<String>,
    browser_context_id: Option<super::types::BrowserContextId>,
}
impl AddPrivacySandboxCoordinatorKeyConfigBuilder {
    pub fn api(mut self, api: impl Into<super::types::PrivacySandboxApi>) -> Self {
        self.api = Some(api.into());
        self
    }
    pub fn coordinator_origin(mut self, coordinator_origin: impl Into<String>) -> Self {
        self.coordinator_origin = Some(coordinator_origin.into());
        self
    }
    pub fn key_config(mut self, key_config: impl Into<String>) -> Self {
        self.key_config = Some(key_config.into());
        self
    }
    pub fn browser_context_id(
        mut self,
        browser_context_id: impl Into<super::types::BrowserContextId>,
    ) -> Self {
        self.browser_context_id = Some(browser_context_id.into());
        self
    }
    pub fn build(self) -> Result<AddPrivacySandboxCoordinatorKeyConfig, String> {
        Ok(AddPrivacySandboxCoordinatorKeyConfig {
            method:
                AddPrivacySandboxCoordinatorKeyConfigMethod::AddPrivacySandboxCoordinatorKeyConfig,
            params: AddPrivacySandboxCoordinatorKeyConfigParams {
                api: self
                    .api
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(api)))?,
                coordinator_origin: self.coordinator_origin.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(coordinator_origin)
                    )
                })?,
                key_config: self.key_config.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(key_config))
                })?,
                browser_context_id: self.browser_context_id.map(Box::new),
            },
        })
    }
}
