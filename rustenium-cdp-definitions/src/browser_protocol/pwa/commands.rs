use serde::{Deserialize, Serialize};
#[doc = "Returns the following OS state for the given manifest id.\n[getOsAppState](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#method-getOsAppState)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GetOsAppStateParams {
    #[doc = "The id from the webapp's manifest file, commonly it's the url of the\nsite installing the webapp. See\nhttps://web.dev/learn/pwa/web-app-manifest."]
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
}
impl GetOsAppStateParams {
    pub fn new(manifest_id: impl Into<String>) -> Self {
        Self {
            manifest_id: manifest_id.into(),
        }
    }
}
impl<T: Into<String>> From<T> for GetOsAppStateParams {
    fn from(url: T) -> Self {
        GetOsAppStateParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GetOsAppStateMethod {
    #[serde(rename = "PWA.getOsAppState")]
    GetOsAppState,
}
impl GetOsAppStateMethod {
    pub const IDENTIFIER: &'static str = "PWA.getOsAppState";
}
#[doc = "Returns the following OS state for the given manifest id.\n[getOsAppState](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#method-getOsAppState)"]
#[derive(Debug, Clone, PartialEq)]
pub struct GetOsAppState {
    pub method: GetOsAppStateMethod,
    pub params: GetOsAppStateParams,
}
#[doc = "Installs the given manifest identity, optionally using the given installUrlOrBundleUrl\n\nIWA-specific install description:\nmanifestId corresponds to isolated-app:// + web_package::SignedWebBundleId\n\nFile installation mode:\nThe installUrlOrBundleUrl can be either file:// or http(s):// pointing\nto a signed web bundle (.swbn). In this case SignedWebBundleId must correspond to\nThe .swbn file's signing key.\n\nDev proxy installation mode:\ninstallUrlOrBundleUrl must be http(s):// that serves dev mode IWA.\nweb_package::SignedWebBundleId must be of type dev proxy.\n\nThe advantage of dev proxy mode is that all changes to IWA\nautomatically will be reflected in the running app without\nreinstallation.\n\nTo generate bundle id for proxy mode:\n1. Generate 32 random bytes.\n2. Add a specific suffix at the end following the documentation\nhttps://github.com/WICG/isolated-web-apps/blob/main/Scheme.md#suffix\n3. Encode the entire sequence using Base32 without padding.\n\nIf Chrome is not in IWA dev\nmode, the installation will fail, regardless of the state of the allowlist.\n[install](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#method-install)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InstallParams {
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
    #[doc = "The location of the app or bundle overriding the one derived from the\nmanifestId."]
    #[serde(rename = "installUrlOrBundleUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub install_url_or_bundle_url: Option<String>,
}
impl InstallParams {
    pub fn new(manifest_id: impl Into<String>) -> Self {
        Self {
            manifest_id: manifest_id.into(),
            install_url_or_bundle_url: None,
        }
    }
}
impl<T: Into<String>> From<T> for InstallParams {
    fn from(url: T) -> Self {
        InstallParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InstallMethod {
    #[serde(rename = "PWA.install")]
    Install,
}
impl InstallMethod {
    pub const IDENTIFIER: &'static str = "PWA.install";
}
#[doc = "Installs the given manifest identity, optionally using the given installUrlOrBundleUrl\n\nIWA-specific install description:\nmanifestId corresponds to isolated-app:// + web_package::SignedWebBundleId\n\nFile installation mode:\nThe installUrlOrBundleUrl can be either file:// or http(s):// pointing\nto a signed web bundle (.swbn). In this case SignedWebBundleId must correspond to\nThe .swbn file's signing key.\n\nDev proxy installation mode:\ninstallUrlOrBundleUrl must be http(s):// that serves dev mode IWA.\nweb_package::SignedWebBundleId must be of type dev proxy.\n\nThe advantage of dev proxy mode is that all changes to IWA\nautomatically will be reflected in the running app without\nreinstallation.\n\nTo generate bundle id for proxy mode:\n1. Generate 32 random bytes.\n2. Add a specific suffix at the end following the documentation\nhttps://github.com/WICG/isolated-web-apps/blob/main/Scheme.md#suffix\n3. Encode the entire sequence using Base32 without padding.\n\nIf Chrome is not in IWA dev\nmode, the installation will fail, regardless of the state of the allowlist.\n[install](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#method-install)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Install {
    pub method: InstallMethod,
    pub params: InstallParams,
}
#[doc = "Uninstalls the given manifest_id and closes any opened app windows.\n[uninstall](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#method-uninstall)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UninstallParams {
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
}
impl UninstallParams {
    pub fn new(manifest_id: impl Into<String>) -> Self {
        Self {
            manifest_id: manifest_id.into(),
        }
    }
}
impl<T: Into<String>> From<T> for UninstallParams {
    fn from(url: T) -> Self {
        UninstallParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UninstallMethod {
    #[serde(rename = "PWA.uninstall")]
    Uninstall,
}
impl UninstallMethod {
    pub const IDENTIFIER: &'static str = "PWA.uninstall";
}
#[doc = "Uninstalls the given manifest_id and closes any opened app windows.\n[uninstall](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#method-uninstall)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Uninstall {
    pub method: UninstallMethod,
    pub params: UninstallParams,
}
#[doc = "Launches the installed web app, or an url in the same web app instead of the\ndefault start url if it is provided. Returns a page Target.TargetID which\ncan be used to attach to via Target.attachToTarget or similar APIs.\n[launch](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#method-launch)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaunchParams {
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url: Option<String>,
}
impl LaunchParams {
    pub fn new(manifest_id: impl Into<String>) -> Self {
        Self {
            manifest_id: manifest_id.into(),
            url: None,
        }
    }
}
impl<T: Into<String>> From<T> for LaunchParams {
    fn from(url: T) -> Self {
        LaunchParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LaunchMethod {
    #[serde(rename = "PWA.launch")]
    Launch,
}
impl LaunchMethod {
    pub const IDENTIFIER: &'static str = "PWA.launch";
}
#[doc = "Launches the installed web app, or an url in the same web app instead of the\ndefault start url if it is provided. Returns a page Target.TargetID which\ncan be used to attach to via Target.attachToTarget or similar APIs.\n[launch](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#method-launch)"]
#[derive(Debug, Clone, PartialEq)]
pub struct Launch {
    pub method: LaunchMethod,
    pub params: LaunchParams,
}
#[doc = "Opens one or more local files from an installed web app identified by its\nmanifestId. The web app needs to have file handlers registered to process\nthe files. The API returns one or more page Target.TargetIDs which can be\nused to attach to via Target.attachToTarget or similar APIs.\nIf some files in the parameters cannot be handled by the web app, they will\nbe ignored. If none of the files can be handled, this API returns an error.\nIf no files are provided as the parameter, this API also returns an error.\n\nAccording to the definition of the file handlers in the manifest file, one\nTarget.TargetID may represent a page handling one or more files. The order\nof the returned Target.TargetIDs is not guaranteed.\n\nTODO(crbug.com/339454034): Check the existences of the input files.\n[launchFilesInApp](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#method-launchFilesInApp)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LaunchFilesInAppParams {
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub files: Vec<String>,
}
impl LaunchFilesInAppParams {
    pub fn new(manifest_id: impl Into<String>, files: Vec<String>) -> Self {
        Self {
            manifest_id: manifest_id.into(),
            files,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LaunchFilesInAppMethod {
    #[serde(rename = "PWA.launchFilesInApp")]
    LaunchFilesInApp,
}
impl LaunchFilesInAppMethod {
    pub const IDENTIFIER: &'static str = "PWA.launchFilesInApp";
}
#[doc = "Opens one or more local files from an installed web app identified by its\nmanifestId. The web app needs to have file handlers registered to process\nthe files. The API returns one or more page Target.TargetIDs which can be\nused to attach to via Target.attachToTarget or similar APIs.\nIf some files in the parameters cannot be handled by the web app, they will\nbe ignored. If none of the files can be handled, this API returns an error.\nIf no files are provided as the parameter, this API also returns an error.\n\nAccording to the definition of the file handlers in the manifest file, one\nTarget.TargetID may represent a page handling one or more files. The order\nof the returned Target.TargetIDs is not guaranteed.\n\nTODO(crbug.com/339454034): Check the existences of the input files.\n[launchFilesInApp](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#method-launchFilesInApp)"]
#[derive(Debug, Clone, PartialEq)]
pub struct LaunchFilesInApp {
    pub method: LaunchFilesInAppMethod,
    pub params: LaunchFilesInAppParams,
}
#[doc = "Opens the current page in its web app identified by the manifest id, needs\nto be called on a page target. This function returns immediately without\nwaiting for the app to finish loading.\n[openCurrentPageInApp](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#method-openCurrentPageInApp)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenCurrentPageInAppParams {
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
}
impl OpenCurrentPageInAppParams {
    pub fn new(manifest_id: impl Into<String>) -> Self {
        Self {
            manifest_id: manifest_id.into(),
        }
    }
}
impl<T: Into<String>> From<T> for OpenCurrentPageInAppParams {
    fn from(url: T) -> Self {
        OpenCurrentPageInAppParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OpenCurrentPageInAppMethod {
    #[serde(rename = "PWA.openCurrentPageInApp")]
    OpenCurrentPageInApp,
}
impl OpenCurrentPageInAppMethod {
    pub const IDENTIFIER: &'static str = "PWA.openCurrentPageInApp";
}
#[doc = "Opens the current page in its web app identified by the manifest id, needs\nto be called on a page target. This function returns immediately without\nwaiting for the app to finish loading.\n[openCurrentPageInApp](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#method-openCurrentPageInApp)"]
#[derive(Debug, Clone, PartialEq)]
pub struct OpenCurrentPageInApp {
    pub method: OpenCurrentPageInAppMethod,
    pub params: OpenCurrentPageInAppParams,
}
#[doc = "Changes user settings of the web app identified by its manifestId. If the\napp was not installed, this command returns an error. Unset parameters will\nbe ignored; unrecognized values will cause an error.\n\nUnlike the ones defined in the manifest files of the web apps, these\nsettings are provided by the browser and controlled by the users, they\nimpact the way the browser handling the web apps.\n\nSee the comment of each parameter.\n[changeAppUserSettings](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#method-changeAppUserSettings)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChangeAppUserSettingsParams {
    #[serde(rename = "manifestId")]
    pub manifest_id: String,
    #[doc = "If user allows the links clicked on by the user in the app's scope, or\nextended scope if the manifest has scope extensions and the flags\n`DesktopPWAsLinkCapturingWithScopeExtensions` and\n`WebAppEnableScopeExtensions` are enabled.\n\nNote, the API does not support resetting the linkCapturing to the\ninitial value, uninstalling and installing the web app again will reset\nit.\n\nTODO(crbug.com/339453269): Setting this value on ChromeOS is not\nsupported yet."]
    #[serde(rename = "linkCapturing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub link_capturing: Option<bool>,
    #[serde(rename = "displayMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub display_mode: Option<super::types::DisplayMode>,
}
impl ChangeAppUserSettingsParams {
    pub fn new(manifest_id: impl Into<String>) -> Self {
        Self {
            manifest_id: manifest_id.into(),
            link_capturing: None,
            display_mode: None,
        }
    }
}
impl<T: Into<String>> From<T> for ChangeAppUserSettingsParams {
    fn from(url: T) -> Self {
        ChangeAppUserSettingsParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChangeAppUserSettingsMethod {
    #[serde(rename = "PWA.changeAppUserSettings")]
    ChangeAppUserSettings,
}
impl ChangeAppUserSettingsMethod {
    pub const IDENTIFIER: &'static str = "PWA.changeAppUserSettings";
}
#[doc = "Changes user settings of the web app identified by its manifestId. If the\napp was not installed, this command returns an error. Unset parameters will\nbe ignored; unrecognized values will cause an error.\n\nUnlike the ones defined in the manifest files of the web apps, these\nsettings are provided by the browser and controlled by the users, they\nimpact the way the browser handling the web apps.\n\nSee the comment of each parameter.\n[changeAppUserSettings](https://chromedevtools.github.io/devtools-protocol/tot/PWA/#method-changeAppUserSettings)"]
#[derive(Debug, Clone, PartialEq)]
pub struct ChangeAppUserSettings {
    pub method: ChangeAppUserSettingsMethod,
    pub params: ChangeAppUserSettingsParams,
}
group_enum ! (Command { GetOsAppState (GetOsAppState) , Install (Install) , Uninstall (Uninstall) , Launch (Launch) , LaunchFilesInApp (LaunchFilesInApp) , OpenCurrentPageInApp (OpenCurrentPageInApp) , ChangeAppUserSettings (ChangeAppUserSettings) });
