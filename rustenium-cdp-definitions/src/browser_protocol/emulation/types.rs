use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SafeAreaInsets {
    #[doc = "Overrides safe-area-inset-top."]
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub top: Option<i64>,
    #[doc = "Overrides safe-area-max-inset-top."]
    #[serde(rename = "topMax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub top_max: Option<i64>,
    #[doc = "Overrides safe-area-inset-left."]
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub left: Option<i64>,
    #[doc = "Overrides safe-area-max-inset-left."]
    #[serde(rename = "leftMax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub left_max: Option<i64>,
    #[doc = "Overrides safe-area-inset-bottom."]
    #[serde(rename = "bottom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bottom: Option<i64>,
    #[doc = "Overrides safe-area-max-inset-bottom."]
    #[serde(rename = "bottomMax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bottom_max: Option<i64>,
    #[doc = "Overrides safe-area-inset-right."]
    #[serde(rename = "right")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub right: Option<i64>,
    #[doc = "Overrides safe-area-max-inset-right."]
    #[serde(rename = "rightMax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub right_max: Option<i64>,
}
impl SafeAreaInsets {
    pub const IDENTIFIER: &'static str = "Emulation.SafeAreaInsets";
}
#[doc = "Screen orientation.\n[ScreenOrientation](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-ScreenOrientation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreenOrientation {
    #[doc = "Orientation type."]
    #[serde(rename = "type")]
    pub r#type: ScreenOrientationType,
    #[doc = "Orientation angle."]
    #[serde(rename = "angle")]
    pub angle: i64,
}
#[doc = "Orientation type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScreenOrientationType {
    #[serde(rename = "portraitPrimary")]
    PortraitPrimary,
    #[serde(rename = "portraitSecondary")]
    PortraitSecondary,
    #[serde(rename = "landscapePrimary")]
    LandscapePrimary,
    #[serde(rename = "landscapeSecondary")]
    LandscapeSecondary,
}
impl ScreenOrientation {
    pub fn new(r#type: impl Into<ScreenOrientationType>, angle: impl Into<i64>) -> Self {
        Self {
            r#type: r#type.into(),
            angle: angle.into(),
        }
    }
}
impl ScreenOrientation {
    pub const IDENTIFIER: &'static str = "Emulation.ScreenOrientation";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisplayFeature {
    #[doc = "Orientation of a display feature in relation to screen"]
    #[serde(rename = "orientation")]
    pub orientation: DisplayFeatureOrientation,
    #[doc = "The offset from the screen origin in either the x (for vertical\norientation) or y (for horizontal orientation) direction."]
    #[serde(rename = "offset")]
    pub offset: i64,
    #[doc = "A display feature may mask content such that it is not physically\ndisplayed - this length along with the offset describes this area.\nA display feature that only splits content will have a 0 mask_length."]
    #[serde(rename = "maskLength")]
    pub mask_length: i64,
}
#[doc = "Orientation of a display feature in relation to screen"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisplayFeatureOrientation {
    #[serde(rename = "vertical")]
    Vertical,
    #[serde(rename = "horizontal")]
    Horizontal,
}
impl DisplayFeature {
    pub fn new(
        orientation: impl Into<DisplayFeatureOrientation>,
        offset: impl Into<i64>,
        mask_length: impl Into<i64>,
    ) -> Self {
        Self {
            orientation: orientation.into(),
            offset: offset.into(),
            mask_length: mask_length.into(),
        }
    }
}
impl DisplayFeature {
    pub const IDENTIFIER: &'static str = "Emulation.DisplayFeature";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DevicePosture {
    #[doc = "Current posture of the device"]
    #[serde(rename = "type")]
    pub r#type: DevicePostureType,
}
#[doc = "Current posture of the device"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DevicePostureType {
    #[serde(rename = "continuous")]
    Continuous,
    #[serde(rename = "folded")]
    Folded,
}
impl DevicePosture {
    pub fn new(r#type: impl Into<DevicePostureType>) -> Self {
        Self {
            r#type: r#type.into(),
        }
    }
}
impl DevicePosture {
    pub const IDENTIFIER: &'static str = "Emulation.DevicePosture";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaFeature {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl MediaFeature {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
impl MediaFeature {
    pub const IDENTIFIER: &'static str = "Emulation.MediaFeature";
}
#[doc = "advance: If the scheduler runs out of immediate work, the virtual time base may fast forward to\nallow the next delayed task (if any) to run; pause: The virtual time base may not advance;\npauseIfNetworkFetchesPending: The virtual time base may not advance if there are any pending\nresource fetches."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VirtualTimePolicy {
    #[serde(rename = "advance")]
    Advance,
    #[serde(rename = "pause")]
    Pause,
    #[serde(rename = "pauseIfNetworkFetchesPending")]
    PauseIfNetworkFetchesPending,
}
#[doc = "Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints\n[UserAgentBrandVersion](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-UserAgentBrandVersion)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserAgentBrandVersion {
    #[serde(rename = "brand")]
    pub brand: String,
    #[serde(rename = "version")]
    pub version: String,
}
impl UserAgentBrandVersion {
    pub fn new(brand: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            brand: brand.into(),
            version: version.into(),
        }
    }
}
impl UserAgentBrandVersion {
    pub const IDENTIFIER: &'static str = "Emulation.UserAgentBrandVersion";
}
#[doc = "Used to specify User Agent Client Hints to emulate. See https://wicg.github.io/ua-client-hints\nMissing optional values will be filled in by the target with what it would normally use.\n[UserAgentMetadata](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-UserAgentMetadata)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserAgentMetadata {
    #[doc = "Brands appearing in Sec-CH-UA."]
    #[serde(rename = "brands")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub brands: Option<Vec<UserAgentBrandVersion>>,
    #[doc = "Brands appearing in Sec-CH-UA-Full-Version-List."]
    #[serde(rename = "fullVersionList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub full_version_list: Option<Vec<UserAgentBrandVersion>>,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "platformVersion")]
    pub platform_version: String,
    #[serde(rename = "architecture")]
    pub architecture: String,
    #[serde(rename = "model")]
    pub model: String,
    #[serde(rename = "mobile")]
    pub mobile: bool,
    #[serde(rename = "bitness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bitness: Option<String>,
    #[serde(rename = "wow64")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub wow64: Option<bool>,
    #[doc = "Used to specify User Agent form-factor values.\nSee https://wicg.github.io/ua-client-hints/#sec-ch-ua-form-factors"]
    #[serde(rename = "formFactors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub form_factors: Option<Vec<String>>,
}
impl UserAgentMetadata {
    pub const IDENTIFIER: &'static str = "Emulation.UserAgentMetadata";
}
#[doc = "Used to specify sensor types to emulate.\nSee https://w3c.github.io/sensors/#automation for more information."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SensorType {
    #[serde(rename = "absolute-orientation")]
    AbsoluteOrientation,
    #[serde(rename = "accelerometer")]
    Accelerometer,
    #[serde(rename = "ambient-light")]
    AmbientLight,
    #[serde(rename = "gravity")]
    Gravity,
    #[serde(rename = "gyroscope")]
    Gyroscope,
    #[serde(rename = "linear-acceleration")]
    LinearAcceleration,
    #[serde(rename = "magnetometer")]
    Magnetometer,
    #[serde(rename = "relative-orientation")]
    RelativeOrientation,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SensorMetadata {
    #[serde(rename = "available")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub available: Option<bool>,
    #[serde(rename = "minimumFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub minimum_frequency: Option<f64>,
    #[serde(rename = "maximumFrequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub maximum_frequency: Option<f64>,
}
impl SensorMetadata {
    pub const IDENTIFIER: &'static str = "Emulation.SensorMetadata";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorReadingSingle {
    #[serde(rename = "value")]
    pub value: f64,
}
impl SensorReadingSingle {
    pub fn new(value: impl Into<f64>) -> Self {
        Self {
            value: value.into(),
        }
    }
}
impl SensorReadingSingle {
    pub const IDENTIFIER: &'static str = "Emulation.SensorReadingSingle";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorReadingXyz {
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "y")]
    pub y: f64,
    #[serde(rename = "z")]
    pub z: f64,
}
impl SensorReadingXyz {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}
impl SensorReadingXyz {
    pub const IDENTIFIER: &'static str = "Emulation.SensorReadingXYZ";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SensorReadingQuaternion {
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "y")]
    pub y: f64,
    #[serde(rename = "z")]
    pub z: f64,
    #[serde(rename = "w")]
    pub w: f64,
}
impl SensorReadingQuaternion {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>, z: impl Into<f64>, w: impl Into<f64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            w: w.into(),
        }
    }
}
impl SensorReadingQuaternion {
    pub const IDENTIFIER: &'static str = "Emulation.SensorReadingQuaternion";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SensorReading {
    #[serde(rename = "single")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub single: Option<SensorReadingSingle>,
    #[serde(rename = "xyz")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub xyz: Option<SensorReadingXyz>,
    #[serde(rename = "quaternion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub quaternion: Option<SensorReadingQuaternion>,
}
impl SensorReading {
    pub const IDENTIFIER: &'static str = "Emulation.SensorReading";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PressureSource {
    #[serde(rename = "cpu")]
    Cpu,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PressureState {
    #[serde(rename = "nominal")]
    Nominal,
    #[serde(rename = "fair")]
    Fair,
    #[serde(rename = "serious")]
    Serious,
    #[serde(rename = "critical")]
    Critical,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PressureMetadata {
    #[serde(rename = "available")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub available: Option<bool>,
}
impl PressureMetadata {
    pub const IDENTIFIER: &'static str = "Emulation.PressureMetadata";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct WorkAreaInsets {
    #[doc = "Work area top inset in pixels. Default is 0;"]
    #[serde(rename = "top")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub top: Option<i64>,
    #[doc = "Work area left inset in pixels. Default is 0;"]
    #[serde(rename = "left")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub left: Option<i64>,
    #[doc = "Work area bottom inset in pixels. Default is 0;"]
    #[serde(rename = "bottom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub bottom: Option<i64>,
    #[doc = "Work area right inset in pixels. Default is 0;"]
    #[serde(rename = "right")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub right: Option<i64>,
}
impl WorkAreaInsets {
    pub const IDENTIFIER: &'static str = "Emulation.WorkAreaInsets";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct ScreenId(String);
impl ScreenId {
    pub fn new(val: impl Into<String>) -> Self {
        ScreenId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for ScreenId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<ScreenId> for String {
    fn from(el: ScreenId) -> String {
        el.0
    }
}
impl From<String> for ScreenId {
    fn from(expr: String) -> Self {
        ScreenId(expr)
    }
}
impl std::borrow::Borrow<str> for ScreenId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl ScreenId {
    pub const IDENTIFIER: &'static str = "Emulation.ScreenId";
}
#[doc = "Screen information similar to the one returned by window.getScreenDetails() method,\nsee https://w3c.github.io/window-management/#screendetailed.\n[ScreenInfo](https://chromedevtools.github.io/devtools-protocol/tot/Emulation/#type-ScreenInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScreenInfo {
    #[doc = "Offset of the left edge of the screen."]
    #[serde(rename = "left")]
    pub left: i64,
    #[doc = "Offset of the top edge of the screen."]
    #[serde(rename = "top")]
    pub top: i64,
    #[doc = "Width of the screen."]
    #[serde(rename = "width")]
    pub width: i64,
    #[doc = "Height of the screen."]
    #[serde(rename = "height")]
    pub height: i64,
    #[doc = "Offset of the left edge of the available screen area."]
    #[serde(rename = "availLeft")]
    pub avail_left: i64,
    #[doc = "Offset of the top edge of the available screen area."]
    #[serde(rename = "availTop")]
    pub avail_top: i64,
    #[doc = "Width of the available screen area."]
    #[serde(rename = "availWidth")]
    pub avail_width: i64,
    #[doc = "Height of the available screen area."]
    #[serde(rename = "availHeight")]
    pub avail_height: i64,
    #[doc = "Specifies the screen's device pixel ratio."]
    #[serde(rename = "devicePixelRatio")]
    pub device_pixel_ratio: f64,
    #[doc = "Specifies the screen's orientation."]
    #[serde(rename = "orientation")]
    pub orientation: ScreenOrientation,
    #[doc = "Specifies the screen's color depth in bits."]
    #[serde(rename = "colorDepth")]
    pub color_depth: i64,
    #[doc = "Indicates whether the device has multiple screens."]
    #[serde(rename = "isExtended")]
    pub is_extended: bool,
    #[doc = "Indicates whether the screen is internal to the device or external, attached to the device."]
    #[serde(rename = "isInternal")]
    pub is_internal: bool,
    #[doc = "Indicates whether the screen is set as the the operating system primary screen."]
    #[serde(rename = "isPrimary")]
    pub is_primary: bool,
    #[doc = "Specifies the descriptive label for the screen."]
    #[serde(rename = "label")]
    pub label: String,
    #[doc = "Specifies the unique identifier of the screen."]
    #[serde(rename = "id")]
    pub id: ScreenId,
}
impl ScreenInfo {
    pub const IDENTIFIER: &'static str = "Emulation.ScreenInfo";
}
#[doc = "Enum of image types that can be disabled."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DisabledImageType {
    #[serde(rename = "avif")]
    Avif,
    #[serde(rename = "jxl")]
    Jxl,
    #[serde(rename = "webp")]
    Webp,
}
group_enum ! (Type { SafeAreaInsets (SafeAreaInsets) , ScreenOrientation (ScreenOrientation) , DisplayFeature (DisplayFeature) , DevicePosture (DevicePosture) , MediaFeature (MediaFeature) , VirtualTimePolicy (VirtualTimePolicy) , UserAgentBrandVersion (UserAgentBrandVersion) , UserAgentMetadata (UserAgentMetadata) , SensorType (SensorType) , SensorMetadata (SensorMetadata) , SensorReadingSingle (SensorReadingSingle) , SensorReadingXyz (SensorReadingXyz) , SensorReadingQuaternion (SensorReadingQuaternion) , SensorReading (SensorReading) , PressureSource (PressureSource) , PressureState (PressureState) , PressureMetadata (PressureMetadata) , WorkAreaInsets (WorkAreaInsets) , ScreenId (ScreenId) , ScreenInfo (ScreenInfo) , DisabledImageType (DisabledImageType) });
