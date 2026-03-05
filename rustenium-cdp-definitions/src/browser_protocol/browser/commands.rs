use serde::{Deserialize, Serialize};
#[doc = "Set permission settings for given embedding and embedded origins.\n[setPermission](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-setPermission)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetPermissionParams {
    #[doc = "Descriptor of permission to override."]
    #[serde(rename = "permission")]
    pub permission: super::types::PermissionDescriptor,
    #[doc = "Setting of the permission."]
    #[serde(rename = "setting")]
    pub setting: super::types::PermissionSetting,
    #[doc = "Embedding origin the permission applies to, all origins if not specified."]
    #[serde(rename = "origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub origin: Option<String>,
    #[doc = "Embedded origin the permission applies to. It is ignored unless the embedding origin is\npresent and valid. If the embedding origin is provided but the embedded origin isn't, the\nembedding origin is used as the embedded origin."]
    #[serde(rename = "embeddedOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub embedded_origin: Option<String>,
    #[doc = "Context to override. When omitted, default browser context is used."]
    #[serde(rename = "browserContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub browser_context_id: Option<super::types::BrowserContextId>,
}
impl SetPermissionParams {
    pub fn new(
        permission: impl Into<super::types::PermissionDescriptor>,
        setting: impl Into<super::types::PermissionSetting>,
    ) -> Self {
        Self {
            permission: permission.into(),
            setting: setting.into(),
            origin: None,
            embedded_origin: None,
            browser_context_id: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetPermissionMethod {
    #[serde(rename = "Browser.setPermission")]
    SetPermission,
}
impl SetPermissionMethod {
    pub const IDENTIFIER: &'static str = "Browser.setPermission";
}
#[doc = "Set permission settings for given embedding and embedded origins.\n[setPermission](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-setPermission)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetPermission {
    pub method: SetPermissionMethod,
    pub params: SetPermissionParams,
}
impl super::super::super::CommandResult for SetPermission {
    type Result = super::results::SetPermissionResult;
}
#[doc = "Reset all permission management for all origins.\n[resetPermissions](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-resetPermissions)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ResetPermissionsParams {
    #[doc = "BrowserContext to reset permissions. When omitted, default browser context is used."]
    #[serde(rename = "browserContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub browser_context_id: Option<super::types::BrowserContextId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResetPermissionsMethod {
    #[serde(rename = "Browser.resetPermissions")]
    ResetPermissions,
}
impl ResetPermissionsMethod {
    pub const IDENTIFIER: &'static str = "Browser.resetPermissions";
}
#[doc = "Reset all permission management for all origins.\n[resetPermissions](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-resetPermissions)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ResetPermissions {
    pub method: ResetPermissionsMethod,
    pub params: ResetPermissionsParams,
}
impl super::super::super::CommandResult for ResetPermissions {
    type Result = super::results::ResetPermissionsResult;
}
#[doc = "Set the behavior when downloading a file.\n[setDownloadBehavior](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-setDownloadBehavior)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetDownloadBehaviorParams {
    #[doc = "Whether to allow all or deny all download requests, or use default Chrome behavior if\navailable (otherwise deny). |allowAndName| allows download and names files according to\ntheir download guids."]
    #[serde(rename = "behavior")]
    pub behavior: SetDownloadBehaviorBehavior,
    #[doc = "BrowserContext to set download behavior. When omitted, default browser context is used."]
    #[serde(rename = "browserContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub browser_context_id: Option<super::types::BrowserContextId>,
    #[doc = "The default path to save downloaded files to. This is required if behavior is set to 'allow'\nor 'allowAndName'."]
    #[serde(rename = "downloadPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub download_path: Option<String>,
    #[doc = "Whether to emit download events (defaults to false)."]
    #[serde(rename = "eventsEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub events_enabled: Option<bool>,
}
#[doc = "Whether to allow all or deny all download requests, or use default Chrome behavior if\navailable (otherwise deny). |allowAndName| allows download and names files according to\ntheir download guids."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetDownloadBehaviorBehavior {
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "allowAndName")]
    AllowAndName,
    #[serde(rename = "default")]
    Default,
}
impl SetDownloadBehaviorParams {
    pub fn new(behavior: impl Into<SetDownloadBehaviorBehavior>) -> Self {
        Self {
            behavior: behavior.into(),
            browser_context_id: None,
            download_path: None,
            events_enabled: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDownloadBehaviorMethod {
    #[serde(rename = "Browser.setDownloadBehavior")]
    SetDownloadBehavior,
}
impl SetDownloadBehaviorMethod {
    pub const IDENTIFIER: &'static str = "Browser.setDownloadBehavior";
}
#[doc = "Set the behavior when downloading a file.\n[setDownloadBehavior](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-setDownloadBehavior)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetDownloadBehavior {
    pub method: SetDownloadBehaviorMethod,
    pub params: SetDownloadBehaviorParams,
}
impl super::super::super::CommandResult for SetDownloadBehavior {
    type Result = super::results::SetDownloadBehaviorResult;
}
#[doc = "Cancel a download if in progress\n[cancelDownload](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-cancelDownload)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CancelDownloadParams {
    #[doc = "Global unique identifier of the download."]
    #[serde(rename = "guid")]
    pub guid: String,
    #[doc = "BrowserContext to perform the action in. When omitted, default browser context is used."]
    #[serde(rename = "browserContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub browser_context_id: Option<super::types::BrowserContextId>,
}
impl CancelDownloadParams {
    pub fn new(guid: impl Into<String>) -> Self {
        Self {
            guid: guid.into(),
            browser_context_id: None,
        }
    }
}
impl<T: Into<String>> From<T> for CancelDownloadParams {
    fn from(url: T) -> Self {
        CancelDownloadParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CancelDownloadMethod {
    #[serde(rename = "Browser.cancelDownload")]
    CancelDownload,
}
impl CancelDownloadMethod {
    pub const IDENTIFIER: &'static str = "Browser.cancelDownload";
}
#[doc = "Cancel a download if in progress\n[cancelDownload](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-cancelDownload)"]
#[derive(Debug, Clone, PartialEq)]
pub struct CancelDownload {
    pub method: CancelDownloadMethod,
    pub params: CancelDownloadParams,
}
impl super::super::super::CommandResult for CancelDownload {
    type Result = super::results::CancelDownloadResult;
}
#[doc = "Close browser gracefully.\n[close](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-close)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CloseParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CloseMethod {
    #[serde(rename = "Browser.close")]
    Close,
}
impl CloseMethod {
    pub const IDENTIFIER: &'static str = "Browser.close";
}
#[doc = "Close browser gracefully.\n[close](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-close)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Close {
    pub method: CloseMethod,
    pub params: CloseParams,
}
impl super::super::super::CommandResult for Close {
    type Result = super::results::CloseResult;
}
#[doc = "Crashes browser on the main thread.\n[crash](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-crash)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CrashParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CrashMethod {
    #[serde(rename = "Browser.crash")]
    Crash,
}
impl CrashMethod {
    pub const IDENTIFIER: &'static str = "Browser.crash";
}
#[doc = "Crashes browser on the main thread.\n[crash](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-crash)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Crash {
    pub method: CrashMethod,
    pub params: CrashParams,
}
impl super::super::super::CommandResult for Crash {
    type Result = super::results::CrashResult;
}
#[doc = "Crashes GPU process.\n[crashGpuProcess](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-crashGpuProcess)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CrashGpuProcessParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CrashGpuProcessMethod {
    #[serde(rename = "Browser.crashGpuProcess")]
    CrashGpuProcess,
}
impl CrashGpuProcessMethod {
    pub const IDENTIFIER: &'static str = "Browser.crashGpuProcess";
}
#[doc = "Crashes GPU process.\n[crashGpuProcess](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-crashGpuProcess)"]
#[derive(Debug, Clone, PartialEq)]
pub struct CrashGpuProcess {
    pub method: CrashGpuProcessMethod,
    pub params: CrashGpuProcessParams,
}
impl super::super::super::CommandResult for CrashGpuProcess {
    type Result = super::results::CrashGpuProcessResult;
}
#[doc = "Returns version information.\n[getVersion](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-getVersion)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetVersionParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetVersionMethod {
    #[serde(rename = "Browser.getVersion")]
    GetVersion,
}
impl GetVersionMethod {
    pub const IDENTIFIER: &'static str = "Browser.getVersion";
}
#[doc = "Returns version information.\n[getVersion](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-getVersion)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetVersion {
    pub method: GetVersionMethod,
    pub params: GetVersionParams,
}
impl super::super::super::CommandResult for GetVersion {
    type Result = super::results::GetVersionResult;
}
#[doc = "Returns the command line switches for the browser process if, and only if\n--enable-automation is on the commandline.\n[getBrowserCommandLine](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-getBrowserCommandLine)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetBrowserCommandLineParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetBrowserCommandLineMethod {
    #[serde(rename = "Browser.getBrowserCommandLine")]
    GetBrowserCommandLine,
}
impl GetBrowserCommandLineMethod {
    pub const IDENTIFIER: &'static str = "Browser.getBrowserCommandLine";
}
#[doc = "Returns the command line switches for the browser process if, and only if\n--enable-automation is on the commandline.\n[getBrowserCommandLine](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-getBrowserCommandLine)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetBrowserCommandLine {
    pub method: GetBrowserCommandLineMethod,
    pub params: GetBrowserCommandLineParams,
}
impl super::super::super::CommandResult for GetBrowserCommandLine {
    type Result = super::results::GetBrowserCommandLineResult;
}
#[doc = "Get Chrome histograms.\n[getHistograms](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-getHistograms)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetHistogramsParams {
    #[doc = "Requested substring in name. Only histograms which have query as a\nsubstring in their name are extracted. An empty or absent query returns\nall histograms."]
    #[serde(rename = "query")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub query: Option<String>,
    #[doc = "If true, retrieve delta since last delta call."]
    #[serde(rename = "delta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub delta: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetHistogramsMethod {
    #[serde(rename = "Browser.getHistograms")]
    GetHistograms,
}
impl GetHistogramsMethod {
    pub const IDENTIFIER: &'static str = "Browser.getHistograms";
}
#[doc = "Get Chrome histograms.\n[getHistograms](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-getHistograms)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetHistograms {
    pub method: GetHistogramsMethod,
    pub params: GetHistogramsParams,
}
impl super::super::super::CommandResult for GetHistograms {
    type Result = super::results::GetHistogramsResult;
}
#[doc = "Get a Chrome histogram by name.\n[getHistogram](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-getHistogram)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetHistogramParams {
    #[doc = "Requested histogram name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "If true, retrieve delta since last delta call."]
    #[serde(rename = "delta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub delta: Option<bool>,
}
impl GetHistogramParams {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            delta: None,
        }
    }
}
impl<T: Into<String>> From<T> for GetHistogramParams {
    fn from(url: T) -> Self {
        GetHistogramParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetHistogramMethod {
    #[serde(rename = "Browser.getHistogram")]
    GetHistogram,
}
impl GetHistogramMethod {
    pub const IDENTIFIER: &'static str = "Browser.getHistogram";
}
#[doc = "Get a Chrome histogram by name.\n[getHistogram](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-getHistogram)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetHistogram {
    pub method: GetHistogramMethod,
    pub params: GetHistogramParams,
}
impl super::super::super::CommandResult for GetHistogram {
    type Result = super::results::GetHistogramResult;
}
#[doc = "Get position and size of the browser window.\n[getWindowBounds](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-getWindowBounds)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetWindowBoundsParams {
    #[doc = "Browser window id."]
    #[serde(rename = "windowId")]
    pub window_id: super::types::WindowId,
}
impl GetWindowBoundsParams {
    pub fn new(window_id: impl Into<super::types::WindowId>) -> Self {
        Self {
            window_id: window_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetWindowBoundsMethod {
    #[serde(rename = "Browser.getWindowBounds")]
    GetWindowBounds,
}
impl GetWindowBoundsMethod {
    pub const IDENTIFIER: &'static str = "Browser.getWindowBounds";
}
#[doc = "Get position and size of the browser window.\n[getWindowBounds](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-getWindowBounds)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetWindowBounds {
    pub method: GetWindowBoundsMethod,
    pub params: GetWindowBoundsParams,
}
impl super::super::super::CommandResult for GetWindowBounds {
    type Result = super::results::GetWindowBoundsResult;
}
#[doc = "Get the browser window that contains the devtools target.\n[getWindowForTarget](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-getWindowForTarget)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GetWindowForTargetParams {
    #[doc = "Devtools agent host id. If called as a part of the session, associated targetId is used."]
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub target_id: Option<super::super::target::types::TargetId>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetWindowForTargetMethod {
    #[serde(rename = "Browser.getWindowForTarget")]
    GetWindowForTarget,
}
impl GetWindowForTargetMethod {
    pub const IDENTIFIER: &'static str = "Browser.getWindowForTarget";
}
#[doc = "Get the browser window that contains the devtools target.\n[getWindowForTarget](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-getWindowForTarget)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetWindowForTarget {
    pub method: GetWindowForTargetMethod,
    pub params: GetWindowForTargetParams,
}
impl super::super::super::CommandResult for GetWindowForTarget {
    type Result = super::results::GetWindowForTargetResult;
}
#[doc = "Set position and/or size of the browser window.\n[setWindowBounds](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-setWindowBounds)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetWindowBoundsParams {
    #[doc = "Browser window id."]
    #[serde(rename = "windowId")]
    pub window_id: super::types::WindowId,
    #[doc = "New window bounds. The 'minimized', 'maximized' and 'fullscreen' states cannot be combined\nwith 'left', 'top', 'width' or 'height'. Leaves unspecified fields unchanged."]
    #[serde(rename = "bounds")]
    pub bounds: super::types::Bounds,
}
impl SetWindowBoundsParams {
    pub fn new(
        window_id: impl Into<super::types::WindowId>,
        bounds: impl Into<super::types::Bounds>,
    ) -> Self {
        Self {
            window_id: window_id.into(),
            bounds: bounds.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetWindowBoundsMethod {
    #[serde(rename = "Browser.setWindowBounds")]
    SetWindowBounds,
}
impl SetWindowBoundsMethod {
    pub const IDENTIFIER: &'static str = "Browser.setWindowBounds";
}
#[doc = "Set position and/or size of the browser window.\n[setWindowBounds](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-setWindowBounds)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetWindowBounds {
    pub method: SetWindowBoundsMethod,
    pub params: SetWindowBoundsParams,
}
impl super::super::super::CommandResult for SetWindowBounds {
    type Result = super::results::SetWindowBoundsResult;
}
#[doc = "Set size of the browser contents resizing browser window as necessary.\n[setContentsSize](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-setContentsSize)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SetContentsSizeParams {
    #[doc = "Browser window id."]
    #[serde(rename = "windowId")]
    pub window_id: super::types::WindowId,
    #[doc = "The window contents width in DIP. Assumes current width if omitted.\nMust be specified if 'height' is omitted."]
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub width: Option<i64>,
    #[doc = "The window contents height in DIP. Assumes current height if omitted.\nMust be specified if 'width' is omitted."]
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub height: Option<i64>,
}
impl SetContentsSizeParams {
    pub fn new(window_id: impl Into<super::types::WindowId>) -> Self {
        Self {
            window_id: window_id.into(),
            width: None,
            height: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetContentsSizeMethod {
    #[serde(rename = "Browser.setContentsSize")]
    SetContentsSize,
}
impl SetContentsSizeMethod {
    pub const IDENTIFIER: &'static str = "Browser.setContentsSize";
}
#[doc = "Set size of the browser contents resizing browser window as necessary.\n[setContentsSize](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-setContentsSize)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetContentsSize {
    pub method: SetContentsSizeMethod,
    pub params: SetContentsSizeParams,
}
impl super::super::super::CommandResult for SetContentsSize {
    type Result = super::results::SetContentsSizeResult;
}
#[doc = "Set dock tile details, platform-specific.\n[setDockTile](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-setDockTile)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SetDockTileParams {
    #[serde(rename = "badgeLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub badge_label: Option<String>,
    #[doc = "Png encoded image."]
    #[serde(rename = "image")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub image: Option<super::super::super::Binary>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SetDockTileMethod {
    #[serde(rename = "Browser.setDockTile")]
    SetDockTile,
}
impl SetDockTileMethod {
    pub const IDENTIFIER: &'static str = "Browser.setDockTile";
}
#[doc = "Set dock tile details, platform-specific.\n[setDockTile](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-setDockTile)"]
#[derive(Debug, Clone, PartialEq)]
pub struct SetDockTile {
    pub method: SetDockTileMethod,
    pub params: SetDockTileParams,
}
impl super::super::super::CommandResult for SetDockTile {
    type Result = super::results::SetDockTileResult;
}
#[doc = "Invoke custom browser commands used by telemetry.\n[executeBrowserCommand](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-executeBrowserCommand)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExecuteBrowserCommandParams {
    #[serde(rename = "commandId")]
    pub command_id: super::types::BrowserCommandId,
}
impl ExecuteBrowserCommandParams {
    pub fn new(command_id: impl Into<super::types::BrowserCommandId>) -> Self {
        Self {
            command_id: command_id.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExecuteBrowserCommandMethod {
    #[serde(rename = "Browser.executeBrowserCommand")]
    ExecuteBrowserCommand,
}
impl ExecuteBrowserCommandMethod {
    pub const IDENTIFIER: &'static str = "Browser.executeBrowserCommand";
}
#[doc = "Invoke custom browser commands used by telemetry.\n[executeBrowserCommand](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-executeBrowserCommand)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ExecuteBrowserCommand {
    pub method: ExecuteBrowserCommandMethod,
    pub params: ExecuteBrowserCommandParams,
}
impl super::super::super::CommandResult for ExecuteBrowserCommand {
    type Result = super::results::ExecuteBrowserCommandResult;
}
#[doc = "Allows a site to use privacy sandbox features that require enrollment\nwithout the site actually being enrolled. Only supported on page targets.\n[addPrivacySandboxEnrollmentOverride](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-addPrivacySandboxEnrollmentOverride)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddPrivacySandboxEnrollmentOverrideParams {
    #[serde(rename = "url")]
    pub url: String,
}
impl AddPrivacySandboxEnrollmentOverrideParams {
    pub fn new(url: impl Into<String>) -> Self {
        Self { url: url.into() }
    }
}
impl<T: Into<String>> From<T> for AddPrivacySandboxEnrollmentOverrideParams {
    fn from(url: T) -> Self {
        AddPrivacySandboxEnrollmentOverrideParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddPrivacySandboxEnrollmentOverrideMethod {
    #[serde(rename = "Browser.addPrivacySandboxEnrollmentOverride")]
    AddPrivacySandboxEnrollmentOverride,
}
impl AddPrivacySandboxEnrollmentOverrideMethod {
    pub const IDENTIFIER: &'static str = "Browser.addPrivacySandboxEnrollmentOverride";
}
#[doc = "Allows a site to use privacy sandbox features that require enrollment\nwithout the site actually being enrolled. Only supported on page targets.\n[addPrivacySandboxEnrollmentOverride](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-addPrivacySandboxEnrollmentOverride)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AddPrivacySandboxEnrollmentOverride {
    pub method: AddPrivacySandboxEnrollmentOverrideMethod,
    pub params: AddPrivacySandboxEnrollmentOverrideParams,
}
impl super::super::super::CommandResult for AddPrivacySandboxEnrollmentOverride {
    type Result = super::results::AddPrivacySandboxEnrollmentOverrideResult;
}
#[doc = "Configures encryption keys used with a given privacy sandbox API to talk\nto a trusted coordinator.  Since this is intended for test automation only,\ncoordinatorOrigin must be a .test domain. No existing coordinator\nconfiguration for the origin may exist.\n[addPrivacySandboxCoordinatorKeyConfig](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-addPrivacySandboxCoordinatorKeyConfig)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddPrivacySandboxCoordinatorKeyConfigParams {
    #[serde(rename = "api")]
    pub api: super::types::PrivacySandboxApi,
    #[serde(rename = "coordinatorOrigin")]
    pub coordinator_origin: String,
    #[serde(rename = "keyConfig")]
    pub key_config: String,
    #[doc = "BrowserContext to perform the action in. When omitted, default browser\ncontext is used."]
    #[serde(rename = "browserContextId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub browser_context_id: Option<super::types::BrowserContextId>,
}
impl AddPrivacySandboxCoordinatorKeyConfigParams {
    pub fn new(
        api: impl Into<super::types::PrivacySandboxApi>,
        coordinator_origin: impl Into<String>,
        key_config: impl Into<String>,
    ) -> Self {
        Self {
            api: api.into(),
            coordinator_origin: coordinator_origin.into(),
            key_config: key_config.into(),
            browser_context_id: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AddPrivacySandboxCoordinatorKeyConfigMethod {
    #[serde(rename = "Browser.addPrivacySandboxCoordinatorKeyConfig")]
    AddPrivacySandboxCoordinatorKeyConfig,
}
impl AddPrivacySandboxCoordinatorKeyConfigMethod {
    pub const IDENTIFIER: &'static str = "Browser.addPrivacySandboxCoordinatorKeyConfig";
}
#[doc = "Configures encryption keys used with a given privacy sandbox API to talk\nto a trusted coordinator.  Since this is intended for test automation only,\ncoordinatorOrigin must be a .test domain. No existing coordinator\nconfiguration for the origin may exist.\n[addPrivacySandboxCoordinatorKeyConfig](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#method-addPrivacySandboxCoordinatorKeyConfig)"]
#[derive(Debug, Clone, PartialEq)]
pub struct AddPrivacySandboxCoordinatorKeyConfig {
    pub method: AddPrivacySandboxCoordinatorKeyConfigMethod,
    pub params: AddPrivacySandboxCoordinatorKeyConfigParams,
}
impl super::super::super::CommandResult for AddPrivacySandboxCoordinatorKeyConfig {
    type Result = super::results::AddPrivacySandboxCoordinatorKeyConfigResult;
}
group_enum ! (BrowserCommands { SetPermission (SetPermission) , ResetPermissions (ResetPermissions) , SetDownloadBehavior (SetDownloadBehavior) , CancelDownload (CancelDownload) , Close (Close) , Crash (Crash) , CrashGpuProcess (CrashGpuProcess) , GetVersion (GetVersion) , GetBrowserCommandLine (GetBrowserCommandLine) , GetHistograms (GetHistograms) , GetHistogram (GetHistogram) , GetWindowBounds (GetWindowBounds) , GetWindowForTarget (GetWindowForTarget) , SetWindowBounds (SetWindowBounds) , SetContentsSize (SetContentsSize) , SetDockTile (SetDockTile) , ExecuteBrowserCommand (ExecuteBrowserCommand) , AddPrivacySandboxEnrollmentOverride (AddPrivacySandboxEnrollmentOverride) , AddPrivacySandboxCoordinatorKeyConfig (AddPrivacySandboxCoordinatorKeyConfig) });
