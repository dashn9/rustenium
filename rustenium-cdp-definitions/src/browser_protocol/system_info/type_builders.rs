use super::types::*;
impl GpuDevice {
    pub fn builder() -> GpuDeviceBuilder {
        <GpuDeviceBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GpuDeviceBuilder {
    vendor_id: Option<f64>,
    device_id: Option<f64>,
    sub_sys_id: Option<f64>,
    revision: Option<f64>,
    vendor_string: Option<String>,
    device_string: Option<String>,
    driver_vendor: Option<String>,
    driver_version: Option<String>,
}
impl GpuDeviceBuilder {
    pub fn vendor_id(mut self, vendor_id: impl Into<f64>) -> Self {
        self.vendor_id = Some(vendor_id.into());
        self
    }
    pub fn device_id(mut self, device_id: impl Into<f64>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }
    pub fn sub_sys_id(mut self, sub_sys_id: impl Into<f64>) -> Self {
        self.sub_sys_id = Some(sub_sys_id.into());
        self
    }
    pub fn revision(mut self, revision: impl Into<f64>) -> Self {
        self.revision = Some(revision.into());
        self
    }
    pub fn vendor_string(mut self, vendor_string: impl Into<String>) -> Self {
        self.vendor_string = Some(vendor_string.into());
        self
    }
    pub fn device_string(mut self, device_string: impl Into<String>) -> Self {
        self.device_string = Some(device_string.into());
        self
    }
    pub fn driver_vendor(mut self, driver_vendor: impl Into<String>) -> Self {
        self.driver_vendor = Some(driver_vendor.into());
        self
    }
    pub fn driver_version(mut self, driver_version: impl Into<String>) -> Self {
        self.driver_version = Some(driver_version.into());
        self
    }
    pub fn build(self) -> Result<GpuDevice, String> {
        Ok(GpuDevice {
            vendor_id: self
                .vendor_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(vendor_id)))?,
            device_id: self
                .device_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(device_id)))?,
            sub_sys_id: self.sub_sys_id,
            revision: self.revision,
            vendor_string: self.vendor_string.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(vendor_string))
            })?,
            device_string: self.device_string.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(device_string))
            })?,
            driver_vendor: self.driver_vendor.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(driver_vendor))
            })?,
            driver_version: self.driver_version.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(driver_version))
            })?,
        })
    }
}
impl Size {
    pub fn builder() -> SizeBuilder {
        <SizeBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SizeBuilder {
    width: Option<i64>,
    height: Option<i64>,
}
impl SizeBuilder {
    pub fn width(mut self, width: impl Into<i64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn height(mut self, height: impl Into<i64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn build(self) -> Result<Size, String> {
        Ok(Size {
            width: self
                .width
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(width)))?,
            height: self
                .height
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(height)))?,
        })
    }
}
impl VideoDecodeAcceleratorCapability {
    pub fn builder() -> VideoDecodeAcceleratorCapabilityBuilder {
        <VideoDecodeAcceleratorCapabilityBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct VideoDecodeAcceleratorCapabilityBuilder {
    profile: Option<String>,
    max_resolution: Option<Size>,
    min_resolution: Option<Size>,
}
impl VideoDecodeAcceleratorCapabilityBuilder {
    pub fn profile(mut self, profile: impl Into<String>) -> Self {
        self.profile = Some(profile.into());
        self
    }
    pub fn max_resolution(mut self, max_resolution: impl Into<Size>) -> Self {
        self.max_resolution = Some(max_resolution.into());
        self
    }
    pub fn min_resolution(mut self, min_resolution: impl Into<Size>) -> Self {
        self.min_resolution = Some(min_resolution.into());
        self
    }
    pub fn build(self) -> Result<VideoDecodeAcceleratorCapability, String> {
        Ok(VideoDecodeAcceleratorCapability {
            profile: self
                .profile
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(profile)))?,
            max_resolution: self.max_resolution.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(max_resolution))
            })?,
            min_resolution: self.min_resolution.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(min_resolution))
            })?,
        })
    }
}
impl VideoEncodeAcceleratorCapability {
    pub fn builder() -> VideoEncodeAcceleratorCapabilityBuilder {
        <VideoEncodeAcceleratorCapabilityBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct VideoEncodeAcceleratorCapabilityBuilder {
    profile: Option<String>,
    max_resolution: Option<Size>,
    max_framerate_numerator: Option<i64>,
    max_framerate_denominator: Option<i64>,
}
impl VideoEncodeAcceleratorCapabilityBuilder {
    pub fn profile(mut self, profile: impl Into<String>) -> Self {
        self.profile = Some(profile.into());
        self
    }
    pub fn max_resolution(mut self, max_resolution: impl Into<Size>) -> Self {
        self.max_resolution = Some(max_resolution.into());
        self
    }
    pub fn max_framerate_numerator(mut self, max_framerate_numerator: impl Into<i64>) -> Self {
        self.max_framerate_numerator = Some(max_framerate_numerator.into());
        self
    }
    pub fn max_framerate_denominator(mut self, max_framerate_denominator: impl Into<i64>) -> Self {
        self.max_framerate_denominator = Some(max_framerate_denominator.into());
        self
    }
    pub fn build(self) -> Result<VideoEncodeAcceleratorCapability, String> {
        Ok(VideoEncodeAcceleratorCapability {
            profile: self
                .profile
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(profile)))?,
            max_resolution: self.max_resolution.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(max_resolution))
            })?,
            max_framerate_numerator: self.max_framerate_numerator.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(max_framerate_numerator)
                )
            })?,
            max_framerate_denominator: self.max_framerate_denominator.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(max_framerate_denominator)
                )
            })?,
        })
    }
}
impl GpuInfo {
    pub fn builder() -> GpuInfoBuilder {
        <GpuInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct GpuInfoBuilder {
    devices: Option<Vec<GpuDevice>>,
    aux_attributes: Option<serde_json::Value>,
    feature_status: Option<serde_json::Value>,
    driver_bug_workarounds: Option<Vec<String>>,
    video_decoding: Option<Vec<VideoDecodeAcceleratorCapability>>,
    video_encoding: Option<Vec<VideoEncodeAcceleratorCapability>>,
}
impl GpuInfoBuilder {
    pub fn device(mut self, device: impl Into<GpuDevice>) -> Self {
        let v = self.devices.get_or_insert(Vec::new());
        v.push(device.into());
        self
    }
    pub fn devices<I, S>(mut self, devices: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<GpuDevice>,
    {
        let v = self.devices.get_or_insert(Vec::new());
        for val in devices {
            v.push(val.into());
        }
        self
    }
    pub fn aux_attributes(mut self, aux_attributes: impl Into<serde_json::Value>) -> Self {
        self.aux_attributes = Some(aux_attributes.into());
        self
    }
    pub fn feature_status(mut self, feature_status: impl Into<serde_json::Value>) -> Self {
        self.feature_status = Some(feature_status.into());
        self
    }
    pub fn driver_bug_workaround(mut self, driver_bug_workaround: impl Into<String>) -> Self {
        let v = self.driver_bug_workarounds.get_or_insert(Vec::new());
        v.push(driver_bug_workaround.into());
        self
    }
    pub fn driver_bug_workarounds<I, S>(mut self, driver_bug_workarounds: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.driver_bug_workarounds.get_or_insert(Vec::new());
        for val in driver_bug_workarounds {
            v.push(val.into());
        }
        self
    }
    pub fn video_decoding(
        mut self,
        video_decoding: impl Into<VideoDecodeAcceleratorCapability>,
    ) -> Self {
        let v = self.video_decoding.get_or_insert(Vec::new());
        v.push(video_decoding.into());
        self
    }
    pub fn video_decodings<I, S>(mut self, video_decodings: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<VideoDecodeAcceleratorCapability>,
    {
        let v = self.video_decoding.get_or_insert(Vec::new());
        for val in video_decodings {
            v.push(val.into());
        }
        self
    }
    pub fn video_encoding(
        mut self,
        video_encoding: impl Into<VideoEncodeAcceleratorCapability>,
    ) -> Self {
        let v = self.video_encoding.get_or_insert(Vec::new());
        v.push(video_encoding.into());
        self
    }
    pub fn video_encodings<I, S>(mut self, video_encodings: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<VideoEncodeAcceleratorCapability>,
    {
        let v = self.video_encoding.get_or_insert(Vec::new());
        for val in video_encodings {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<GpuInfo, String> {
        Ok(GpuInfo {
            devices: self
                .devices
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(devices)))?,
            aux_attributes: self.aux_attributes,
            feature_status: self.feature_status,
            driver_bug_workarounds: self.driver_bug_workarounds.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(driver_bug_workarounds)
                )
            })?,
            video_decoding: self.video_decoding.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(video_decoding))
            })?,
            video_encoding: self.video_encoding.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(video_encoding))
            })?,
        })
    }
}
impl ProcessInfo {
    pub fn builder() -> ProcessInfoBuilder {
        <ProcessInfoBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ProcessInfoBuilder {
    r#type: Option<String>,
    id: Option<i64>,
    cpu_time: Option<f64>,
}
impl ProcessInfoBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn id(mut self, id: impl Into<i64>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn cpu_time(mut self, cpu_time: impl Into<f64>) -> Self {
        self.cpu_time = Some(cpu_time.into());
        self
    }
    pub fn build(self) -> Result<ProcessInfo, String> {
        Ok(ProcessInfo {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            cpu_time: self
                .cpu_time
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cpu_time)))?,
        })
    }
}
