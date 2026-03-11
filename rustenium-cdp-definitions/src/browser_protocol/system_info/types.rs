use serde::{Deserialize, Serialize};
#[doc = "Describes a single graphics processor (GPU).\n[GPUDevice](https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-GPUDevice)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpuDevice {
    #[doc = "PCI ID of the GPU vendor, if available; 0 otherwise."]
    #[serde(rename = "vendorId")]
    pub vendor_id: f64,
    #[doc = "PCI ID of the GPU device, if available; 0 otherwise."]
    #[serde(rename = "deviceId")]
    pub device_id: f64,
    #[doc = "Sub sys ID of the GPU, only available on Windows."]
    #[serde(rename = "subSysId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sub_sys_id: Option<f64>,
    #[doc = "Revision of the GPU, only available on Windows."]
    #[serde(rename = "revision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub revision: Option<f64>,
    #[doc = "String description of the GPU vendor, if the PCI ID is not available."]
    #[serde(rename = "vendorString")]
    pub vendor_string: String,
    #[doc = "String description of the GPU device, if the PCI ID is not available."]
    #[serde(rename = "deviceString")]
    pub device_string: String,
    #[doc = "String description of the GPU driver vendor."]
    #[serde(rename = "driverVendor")]
    pub driver_vendor: String,
    #[doc = "String description of the GPU driver version."]
    #[serde(rename = "driverVersion")]
    pub driver_version: String,
}
impl GpuDevice {
    pub const IDENTIFIER: &'static str = "SystemInfo.GPUDevice";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Describes the width and height dimensions of an entity.\n[Size](https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-Size)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Size {
    #[doc = "Width in pixels."]
    #[serde(rename = "width")]
    pub width: i64,
    #[doc = "Height in pixels."]
    #[serde(rename = "height")]
    pub height: i64,
}
impl Size {
    pub fn new(width: impl Into<i64>, height: impl Into<i64>) -> Self {
        Self {
            width: width.into(),
            height: height.into(),
        }
    }
}
impl Size {
    pub const IDENTIFIER: &'static str = "SystemInfo.Size";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Describes a supported video decoding profile with its associated minimum and\nmaximum resolutions.\n[VideoDecodeAcceleratorCapability](https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-VideoDecodeAcceleratorCapability)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoDecodeAcceleratorCapability {
    #[doc = "Video codec profile that is supported, e.g. VP9 Profile 2."]
    #[serde(rename = "profile")]
    pub profile: String,
    #[doc = "Maximum video dimensions in pixels supported for this |profile|."]
    #[serde(rename = "maxResolution")]
    pub max_resolution: Size,
    #[doc = "Minimum video dimensions in pixels supported for this |profile|."]
    #[serde(rename = "minResolution")]
    pub min_resolution: Size,
}
impl VideoDecodeAcceleratorCapability {
    pub fn new(
        profile: impl Into<String>,
        max_resolution: impl Into<Size>,
        min_resolution: impl Into<Size>,
    ) -> Self {
        Self {
            profile: profile.into(),
            max_resolution: max_resolution.into(),
            min_resolution: min_resolution.into(),
        }
    }
}
impl VideoDecodeAcceleratorCapability {
    pub const IDENTIFIER: &'static str = "SystemInfo.VideoDecodeAcceleratorCapability";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Describes a supported video encoding profile with its associated maximum\nresolution and maximum framerate.\n[VideoEncodeAcceleratorCapability](https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-VideoEncodeAcceleratorCapability)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoEncodeAcceleratorCapability {
    #[doc = "Video codec profile that is supported, e.g H264 Main."]
    #[serde(rename = "profile")]
    pub profile: String,
    #[doc = "Maximum video dimensions in pixels supported for this |profile|."]
    #[serde(rename = "maxResolution")]
    pub max_resolution: Size,
    #[doc = "Maximum encoding framerate in frames per second supported for this\n|profile|, as fraction's numerator and denominator, e.g. 24/1 fps,\n24000/1001 fps, etc."]
    #[serde(rename = "maxFramerateNumerator")]
    pub max_framerate_numerator: i64,
    #[serde(rename = "maxFramerateDenominator")]
    pub max_framerate_denominator: i64,
}
impl VideoEncodeAcceleratorCapability {
    pub fn new(
        profile: impl Into<String>,
        max_resolution: impl Into<Size>,
        max_framerate_numerator: impl Into<i64>,
        max_framerate_denominator: impl Into<i64>,
    ) -> Self {
        Self {
            profile: profile.into(),
            max_resolution: max_resolution.into(),
            max_framerate_numerator: max_framerate_numerator.into(),
            max_framerate_denominator: max_framerate_denominator.into(),
        }
    }
}
impl VideoEncodeAcceleratorCapability {
    pub const IDENTIFIER: &'static str = "SystemInfo.VideoEncodeAcceleratorCapability";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "YUV subsampling type of the pixels of a given image."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubsamplingFormat {
    #[serde(rename = "yuv420")]
    Yuv420,
    #[serde(rename = "yuv422")]
    Yuv422,
    #[serde(rename = "yuv444")]
    Yuv444,
}
#[doc = "Image format of a given image."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImageType {
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "webp")]
    Webp,
    #[serde(rename = "unknown")]
    Unknown,
}
#[doc = "Provides information about the GPU(s) on the system.\n[GPUInfo](https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-GPUInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GpuInfo {
    #[doc = "The graphics devices on the system. Element 0 is the primary GPU."]
    #[serde(rename = "devices")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub devices: Vec<GpuDevice>,
    #[doc = "An optional dictionary of additional GPU related attributes."]
    #[serde(rename = "auxAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub aux_attributes: Option<serde_json::Value>,
    #[doc = "An optional dictionary of graphics features and their status."]
    #[serde(rename = "featureStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub feature_status: Option<serde_json::Value>,
    #[doc = "An optional array of GPU driver bug workarounds."]
    #[serde(rename = "driverBugWorkarounds")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub driver_bug_workarounds: Vec<String>,
    #[doc = "Supported accelerated video decoding capabilities."]
    #[serde(rename = "videoDecoding")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub video_decoding: Vec<VideoDecodeAcceleratorCapability>,
    #[doc = "Supported accelerated video encoding capabilities."]
    #[serde(rename = "videoEncoding")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub video_encoding: Vec<VideoEncodeAcceleratorCapability>,
}
impl GpuInfo {
    pub fn new(
        devices: Vec<GpuDevice>,
        driver_bug_workarounds: Vec<String>,
        video_decoding: Vec<VideoDecodeAcceleratorCapability>,
        video_encoding: Vec<VideoEncodeAcceleratorCapability>,
    ) -> Self {
        Self {
            devices,
            driver_bug_workarounds,
            video_decoding,
            video_encoding,
            aux_attributes: None,
            feature_status: None,
        }
    }
}
impl GpuInfo {
    pub const IDENTIFIER: &'static str = "SystemInfo.GPUInfo";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Represents process info.\n[ProcessInfo](https://chromedevtools.github.io/devtools-protocol/tot/SystemInfo/#type-ProcessInfo)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessInfo {
    #[doc = "Specifies process type."]
    #[serde(rename = "type")]
    pub r#type: String,
    #[doc = "Specifies process id."]
    #[serde(rename = "id")]
    pub id: i64,
    #[doc = "Specifies cumulative CPU usage in seconds across all threads of the\nprocess since the process start."]
    #[serde(rename = "cpuTime")]
    pub cpu_time: f64,
}
impl ProcessInfo {
    pub fn new(r#type: impl Into<String>, id: impl Into<i64>, cpu_time: impl Into<f64>) -> Self {
        Self {
            r#type: r#type.into(),
            id: id.into(),
            cpu_time: cpu_time.into(),
        }
    }
}
impl ProcessInfo {
    pub const IDENTIFIER: &'static str = "SystemInfo.ProcessInfo";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (SystemInfoTypes { GpuDevice (GpuDevice) , Size (Size) , VideoDecodeAcceleratorCapability (VideoDecodeAcceleratorCapability) , VideoEncodeAcceleratorCapability (VideoEncodeAcceleratorCapability) , SubsamplingFormat (SubsamplingFormat) , ImageType (ImageType) , GpuInfo (GpuInfo) , ProcessInfo (ProcessInfo) });
