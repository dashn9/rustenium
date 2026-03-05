use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct BrowserContextId(String);
impl BrowserContextId {
    pub fn new(val: impl Into<String>) -> Self {
        BrowserContextId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for BrowserContextId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<BrowserContextId> for String {
    fn from(el: BrowserContextId) -> String {
        el.0
    }
}
impl From<String> for BrowserContextId {
    fn from(expr: String) -> Self {
        BrowserContextId(expr)
    }
}
impl std::borrow::Borrow<str> for BrowserContextId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl BrowserContextId {
    pub const IDENTIFIER: &'static str = "Browser.BrowserContextID";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Copy, Hash)]
pub struct WindowId(i64);
impl WindowId {
    pub fn new(val: impl Into<i64>) -> Self {
        WindowId(val.into())
    }
    pub fn inner(&self) -> &i64 {
        &self.0
    }
}
impl WindowId {
    pub const IDENTIFIER: &'static str = "Browser.WindowID";
}
#[doc = "The state of the browser window."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WindowState {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "minimized")]
    Minimized,
    #[serde(rename = "maximized")]
    Maximized,
    #[serde(rename = "fullscreen")]
    Fullscreen,
}
#[doc = "Browser window bounds information\n[Bounds](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-Bounds)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Bounds {
    #[doc = "The offset from the left edge of the screen to the window in pixels."]
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub left: Option<i64>,
    #[doc = "The offset from the top edge of the screen to the window in pixels."]
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub top: Option<i64>,
    #[doc = "The window width in pixels."]
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub width: Option<i64>,
    #[doc = "The window height in pixels."]
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub height: Option<i64>,
    #[doc = "The window state. Default to normal."]
    #[serde(rename = "windowState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub window_state: Option<WindowState>,
}
impl Bounds {
    pub const IDENTIFIER: &'static str = "Browser.Bounds";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionType {
    #[serde(rename = "ar")]
    Ar,
    #[serde(rename = "audioCapture")]
    AudioCapture,
    #[serde(rename = "automaticFullscreen")]
    AutomaticFullscreen,
    #[serde(rename = "backgroundFetch")]
    BackgroundFetch,
    #[serde(rename = "backgroundSync")]
    BackgroundSync,
    #[serde(rename = "cameraPanTiltZoom")]
    CameraPanTiltZoom,
    #[serde(rename = "capturedSurfaceControl")]
    CapturedSurfaceControl,
    #[serde(rename = "clipboardReadWrite")]
    ClipboardReadWrite,
    #[serde(rename = "clipboardSanitizedWrite")]
    ClipboardSanitizedWrite,
    #[serde(rename = "displayCapture")]
    DisplayCapture,
    #[serde(rename = "durableStorage")]
    DurableStorage,
    #[serde(rename = "geolocation")]
    Geolocation,
    #[serde(rename = "handTracking")]
    HandTracking,
    #[serde(rename = "idleDetection")]
    IdleDetection,
    #[serde(rename = "keyboardLock")]
    KeyboardLock,
    #[serde(rename = "localFonts")]
    LocalFonts,
    #[serde(rename = "localNetwork")]
    LocalNetwork,
    #[serde(rename = "localNetworkAccess")]
    LocalNetworkAccess,
    #[serde(rename = "loopbackNetwork")]
    LoopbackNetwork,
    #[serde(rename = "midi")]
    Midi,
    #[serde(rename = "midiSysex")]
    MidiSysex,
    #[serde(rename = "nfc")]
    Nfc,
    #[serde(rename = "notifications")]
    Notifications,
    #[serde(rename = "paymentHandler")]
    PaymentHandler,
    #[serde(rename = "periodicBackgroundSync")]
    PeriodicBackgroundSync,
    #[serde(rename = "pointerLock")]
    PointerLock,
    #[serde(rename = "protectedMediaIdentifier")]
    ProtectedMediaIdentifier,
    #[serde(rename = "sensors")]
    Sensors,
    #[serde(rename = "smartCard")]
    SmartCard,
    #[serde(rename = "speakerSelection")]
    SpeakerSelection,
    #[serde(rename = "storageAccess")]
    StorageAccess,
    #[serde(rename = "topLevelStorageAccess")]
    TopLevelStorageAccess,
    #[serde(rename = "videoCapture")]
    VideoCapture,
    #[serde(rename = "vr")]
    Vr,
    #[serde(rename = "wakeLockScreen")]
    WakeLockScreen,
    #[serde(rename = "wakeLockSystem")]
    WakeLockSystem,
    #[serde(rename = "webAppInstallation")]
    WebAppInstallation,
    #[serde(rename = "webPrinting")]
    WebPrinting,
    #[serde(rename = "windowManagement")]
    WindowManagement,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionSetting {
    #[serde(rename = "granted")]
    Granted,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "prompt")]
    Prompt,
}
#[doc = "Definition of PermissionDescriptor defined in the Permissions API:\nhttps://w3c.github.io/permissions/#dom-permissiondescriptor.\n[PermissionDescriptor](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-PermissionDescriptor)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionDescriptor {
    #[doc = "Name of permission.\nSee https://cs.chromium.org/chromium/src/third_party/blink/renderer/modules/permissions/permission_descriptor.idl for valid permission names."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "For \"midi\" permission, may also specify sysex control."]
    #[serde(rename = "sysex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sysex: Option<bool>,
    #[doc = "For \"push\" permission, may specify userVisibleOnly.\nNote that userVisibleOnly = true is the only currently supported type."]
    #[serde(rename = "userVisibleOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_visible_only: Option<bool>,
    #[doc = "For \"clipboard\" permission, may specify allowWithoutSanitization."]
    #[serde(rename = "allowWithoutSanitization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub allow_without_sanitization: Option<bool>,
    #[doc = "For \"fullscreen\" permission, must specify allowWithoutGesture:true."]
    #[serde(rename = "allowWithoutGesture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub allow_without_gesture: Option<bool>,
    #[doc = "For \"camera\" permission, may specify panTiltZoom."]
    #[serde(rename = "panTiltZoom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pan_tilt_zoom: Option<bool>,
}
impl PermissionDescriptor {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            sysex: None,
            user_visible_only: None,
            allow_without_sanitization: None,
            allow_without_gesture: None,
            pan_tilt_zoom: None,
        }
    }
}
impl<T: Into<String>> From<T> for PermissionDescriptor {
    fn from(url: T) -> Self {
        PermissionDescriptor::new(url)
    }
}
impl PermissionDescriptor {
    pub const IDENTIFIER: &'static str = "Browser.PermissionDescriptor";
}
#[doc = "Browser command ids used by executeBrowserCommand."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BrowserCommandId {
    #[serde(rename = "openTabSearch")]
    OpenTabSearch,
    #[serde(rename = "closeTabSearch")]
    CloseTabSearch,
    #[serde(rename = "openGlic")]
    OpenGlic,
}
#[doc = "Chrome histogram bucket.\n[Bucket](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-Bucket)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bucket {
    #[doc = "Minimum value (inclusive)."]
    #[serde(rename = "low")]
    pub low: i64,
    #[doc = "Maximum value (exclusive)."]
    #[serde(rename = "high")]
    pub high: i64,
    #[doc = "Number of samples."]
    #[serde(rename = "count")]
    pub count: i64,
}
impl Bucket {
    pub fn new(low: impl Into<i64>, high: impl Into<i64>, count: impl Into<i64>) -> Self {
        Self {
            low: low.into(),
            high: high.into(),
            count: count.into(),
        }
    }
}
impl Bucket {
    pub const IDENTIFIER: &'static str = "Browser.Bucket";
}
#[doc = "Chrome histogram.\n[Histogram](https://chromedevtools.github.io/devtools-protocol/tot/Browser/#type-Histogram)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Histogram {
    #[doc = "Name."]
    #[serde(rename = "name")]
    pub name: String,
    #[doc = "Sum of sample values."]
    #[serde(rename = "sum")]
    pub sum: i64,
    #[doc = "Total number of samples."]
    #[serde(rename = "count")]
    pub count: i64,
    #[doc = "Buckets."]
    #[serde(rename = "buckets")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub buckets: Vec<Bucket>,
}
impl Histogram {
    pub fn new(
        name: impl Into<String>,
        sum: impl Into<i64>,
        count: impl Into<i64>,
        buckets: Vec<Bucket>,
    ) -> Self {
        Self {
            name: name.into(),
            sum: sum.into(),
            count: count.into(),
            buckets,
        }
    }
}
impl Histogram {
    pub const IDENTIFIER: &'static str = "Browser.Histogram";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrivacySandboxApi {
    #[serde(rename = "BiddingAndAuctionServices")]
    BiddingAndAuctionServices,
    #[serde(rename = "TrustedKeyValue")]
    TrustedKeyValue,
}
group_enum ! (BrowserTypes { BrowserContextId (BrowserContextId) , WindowId (WindowId) , WindowState (WindowState) , Bounds (Bounds) , PermissionType (PermissionType) , PermissionSetting (PermissionSetting) , PermissionDescriptor (PermissionDescriptor) , BrowserCommandId (BrowserCommandId) , Bucket (Bucket) , Histogram (Histogram) , PrivacySandboxApi (PrivacySandboxApi) });
