use super::types::*;
impl SafeAreaInsets {
    pub fn builder() -> SafeAreaInsetsBuilder {
        SafeAreaInsetsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SafeAreaInsetsBuilder {
    top: Option<i64>,
    top_max: Option<i64>,
    left: Option<i64>,
    left_max: Option<i64>,
    bottom: Option<i64>,
    bottom_max: Option<i64>,
    right: Option<i64>,
    right_max: Option<i64>,
}
impl SafeAreaInsetsBuilder {
    pub fn top(mut self, top: impl Into<i64>) -> Self {
        self.top = Some(top.into());
        self
    }
    pub fn top_max(mut self, top_max: impl Into<i64>) -> Self {
        self.top_max = Some(top_max.into());
        self
    }
    pub fn left(mut self, left: impl Into<i64>) -> Self {
        self.left = Some(left.into());
        self
    }
    pub fn left_max(mut self, left_max: impl Into<i64>) -> Self {
        self.left_max = Some(left_max.into());
        self
    }
    pub fn bottom(mut self, bottom: impl Into<i64>) -> Self {
        self.bottom = Some(bottom.into());
        self
    }
    pub fn bottom_max(mut self, bottom_max: impl Into<i64>) -> Self {
        self.bottom_max = Some(bottom_max.into());
        self
    }
    pub fn right(mut self, right: impl Into<i64>) -> Self {
        self.right = Some(right.into());
        self
    }
    pub fn right_max(mut self, right_max: impl Into<i64>) -> Self {
        self.right_max = Some(right_max.into());
        self
    }
    pub fn build(self) -> SafeAreaInsets {
        SafeAreaInsets {
            top: self.top,
            top_max: self.top_max,
            left: self.left,
            left_max: self.left_max,
            bottom: self.bottom,
            bottom_max: self.bottom_max,
            right: self.right,
            right_max: self.right_max,
        }
    }
}
impl ScreenOrientation {
    pub fn builder() -> ScreenOrientationBuilder {
        ScreenOrientationBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ScreenOrientationBuilder {
    r#type: Option<ScreenOrientationType>,
    angle: Option<i64>,
}
impl ScreenOrientationBuilder {
    pub fn r#type(mut self, r#type: impl Into<ScreenOrientationType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn angle(mut self, angle: impl Into<i64>) -> Self {
        self.angle = Some(angle.into());
        self
    }
    pub fn build(self) -> Result<ScreenOrientation, String> {
        Ok(ScreenOrientation {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            angle: self
                .angle
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(angle)))?,
        })
    }
}
impl DisplayFeature {
    pub fn builder() -> DisplayFeatureBuilder {
        DisplayFeatureBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DisplayFeatureBuilder {
    orientation: Option<DisplayFeatureOrientation>,
    offset: Option<i64>,
    mask_length: Option<i64>,
}
impl DisplayFeatureBuilder {
    pub fn orientation(mut self, orientation: impl Into<DisplayFeatureOrientation>) -> Self {
        self.orientation = Some(orientation.into());
        self
    }
    pub fn offset(mut self, offset: impl Into<i64>) -> Self {
        self.offset = Some(offset.into());
        self
    }
    pub fn mask_length(mut self, mask_length: impl Into<i64>) -> Self {
        self.mask_length = Some(mask_length.into());
        self
    }
    pub fn build(self) -> Result<DisplayFeature, String> {
        Ok(DisplayFeature {
            orientation: self
                .orientation
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(orientation)))?,
            offset: self
                .offset
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(offset)))?,
            mask_length: self
                .mask_length
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(mask_length)))?,
        })
    }
}
impl DevicePosture {
    pub fn builder() -> DevicePostureBuilder {
        DevicePostureBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DevicePostureBuilder {
    r#type: Option<DevicePostureType>,
}
impl DevicePostureBuilder {
    pub fn r#type(mut self, r#type: impl Into<DevicePostureType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<DevicePosture, String> {
        Ok(DevicePosture {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl MediaFeature {
    pub fn builder() -> MediaFeatureBuilder {
        MediaFeatureBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct MediaFeatureBuilder {
    name: Option<String>,
    value: Option<String>,
}
impl MediaFeatureBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<MediaFeature, String> {
        Ok(MediaFeature {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl UserAgentBrandVersion {
    pub fn builder() -> UserAgentBrandVersionBuilder {
        UserAgentBrandVersionBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct UserAgentBrandVersionBuilder {
    brand: Option<String>,
    version: Option<String>,
}
impl UserAgentBrandVersionBuilder {
    pub fn brand(mut self, brand: impl Into<String>) -> Self {
        self.brand = Some(brand.into());
        self
    }
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }
    pub fn build(self) -> Result<UserAgentBrandVersion, String> {
        Ok(UserAgentBrandVersion {
            brand: self
                .brand
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(brand)))?,
            version: self
                .version
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(version)))?,
        })
    }
}
impl UserAgentMetadata {
    pub fn builder() -> UserAgentMetadataBuilder {
        UserAgentMetadataBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct UserAgentMetadataBuilder {
    brands: Option<Vec<UserAgentBrandVersion>>,
    full_version_list: Option<Vec<UserAgentBrandVersion>>,
    platform: Option<String>,
    platform_version: Option<String>,
    architecture: Option<String>,
    model: Option<String>,
    mobile: Option<bool>,
    bitness: Option<String>,
    wow64: Option<bool>,
    form_factors: Option<Vec<String>>,
}
impl UserAgentMetadataBuilder {
    pub fn brand(mut self, brand: impl Into<UserAgentBrandVersion>) -> Self {
        let v = self.brands.get_or_insert(Vec::new());
        v.push(brand.into());
        self
    }
    pub fn brands<I, S>(mut self, brands: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<UserAgentBrandVersion>,
    {
        let v = self.brands.get_or_insert(Vec::new());
        for val in brands {
            v.push(val.into());
        }
        self
    }
    pub fn full_version_list(
        mut self,
        full_version_list: impl Into<UserAgentBrandVersion>,
    ) -> Self {
        let v = self.full_version_list.get_or_insert(Vec::new());
        v.push(full_version_list.into());
        self
    }
    pub fn full_version_lists<I, S>(mut self, full_version_lists: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<UserAgentBrandVersion>,
    {
        let v = self.full_version_list.get_or_insert(Vec::new());
        for val in full_version_lists {
            v.push(val.into());
        }
        self
    }
    pub fn platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
    pub fn platform_version(mut self, platform_version: impl Into<String>) -> Self {
        self.platform_version = Some(platform_version.into());
        self
    }
    pub fn architecture(mut self, architecture: impl Into<String>) -> Self {
        self.architecture = Some(architecture.into());
        self
    }
    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = Some(model.into());
        self
    }
    pub fn mobile(mut self, mobile: impl Into<bool>) -> Self {
        self.mobile = Some(mobile.into());
        self
    }
    pub fn bitness(mut self, bitness: impl Into<String>) -> Self {
        self.bitness = Some(bitness.into());
        self
    }
    pub fn wow64(mut self, wow64: impl Into<bool>) -> Self {
        self.wow64 = Some(wow64.into());
        self
    }
    pub fn form_factor(mut self, form_factor: impl Into<String>) -> Self {
        let v = self.form_factors.get_or_insert(Vec::new());
        v.push(form_factor.into());
        self
    }
    pub fn form_factors<I, S>(mut self, form_factors: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.form_factors.get_or_insert(Vec::new());
        for val in form_factors {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<UserAgentMetadata, String> {
        Ok(UserAgentMetadata {
            brands: self.brands,
            full_version_list: self.full_version_list,
            platform: self
                .platform
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(platform)))?,
            platform_version: self.platform_version.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(platform_version)
                )
            })?,
            architecture: self.architecture.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(architecture))
            })?,
            model: self
                .model
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(model)))?,
            mobile: self
                .mobile
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(mobile)))?,
            bitness: self.bitness,
            wow64: self.wow64,
            form_factors: self.form_factors,
        })
    }
}
impl SensorMetadata {
    pub fn builder() -> SensorMetadataBuilder {
        SensorMetadataBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SensorMetadataBuilder {
    available: Option<bool>,
    minimum_frequency: Option<f64>,
    maximum_frequency: Option<f64>,
}
impl SensorMetadataBuilder {
    pub fn available(mut self, available: impl Into<bool>) -> Self {
        self.available = Some(available.into());
        self
    }
    pub fn minimum_frequency(mut self, minimum_frequency: impl Into<f64>) -> Self {
        self.minimum_frequency = Some(minimum_frequency.into());
        self
    }
    pub fn maximum_frequency(mut self, maximum_frequency: impl Into<f64>) -> Self {
        self.maximum_frequency = Some(maximum_frequency.into());
        self
    }
    pub fn build(self) -> SensorMetadata {
        SensorMetadata {
            available: self.available,
            minimum_frequency: self.minimum_frequency,
            maximum_frequency: self.maximum_frequency,
        }
    }
}
impl SensorReadingSingle {
    pub fn builder() -> SensorReadingSingleBuilder {
        SensorReadingSingleBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SensorReadingSingleBuilder {
    value: Option<f64>,
}
impl SensorReadingSingleBuilder {
    pub fn value(mut self, value: impl Into<f64>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<SensorReadingSingle, String> {
        Ok(SensorReadingSingle {
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl SensorReadingXyz {
    pub fn builder() -> SensorReadingXyzBuilder {
        SensorReadingXyzBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SensorReadingXyzBuilder {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
}
impl SensorReadingXyzBuilder {
    pub fn x(mut self, x: impl Into<f64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<f64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn z(mut self, z: impl Into<f64>) -> Self {
        self.z = Some(z.into());
        self
    }
    pub fn build(self) -> Result<SensorReadingXyz, String> {
        Ok(SensorReadingXyz {
            x: self
                .x
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
            y: self
                .y
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
            z: self
                .z
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(z)))?,
        })
    }
}
impl SensorReadingQuaternion {
    pub fn builder() -> SensorReadingQuaternionBuilder {
        SensorReadingQuaternionBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SensorReadingQuaternionBuilder {
    x: Option<f64>,
    y: Option<f64>,
    z: Option<f64>,
    w: Option<f64>,
}
impl SensorReadingQuaternionBuilder {
    pub fn x(mut self, x: impl Into<f64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<f64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn z(mut self, z: impl Into<f64>) -> Self {
        self.z = Some(z.into());
        self
    }
    pub fn w(mut self, w: impl Into<f64>) -> Self {
        self.w = Some(w.into());
        self
    }
    pub fn build(self) -> Result<SensorReadingQuaternion, String> {
        Ok(SensorReadingQuaternion {
            x: self
                .x
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
            y: self
                .y
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
            z: self
                .z
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(z)))?,
            w: self
                .w
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(w)))?,
        })
    }
}
impl SensorReading {
    pub fn builder() -> SensorReadingBuilder {
        SensorReadingBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SensorReadingBuilder {
    single: Option<SensorReadingSingle>,
    xyz: Option<SensorReadingXyz>,
    quaternion: Option<SensorReadingQuaternion>,
}
impl SensorReadingBuilder {
    pub fn single(mut self, single: impl Into<SensorReadingSingle>) -> Self {
        self.single = Some(single.into());
        self
    }
    pub fn xyz(mut self, xyz: impl Into<SensorReadingXyz>) -> Self {
        self.xyz = Some(xyz.into());
        self
    }
    pub fn quaternion(mut self, quaternion: impl Into<SensorReadingQuaternion>) -> Self {
        self.quaternion = Some(quaternion.into());
        self
    }
    pub fn build(self) -> SensorReading {
        SensorReading {
            single: self.single,
            xyz: self.xyz,
            quaternion: self.quaternion,
        }
    }
}
impl PressureMetadata {
    pub fn builder() -> PressureMetadataBuilder {
        PressureMetadataBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct PressureMetadataBuilder {
    available: Option<bool>,
}
impl PressureMetadataBuilder {
    pub fn available(mut self, available: impl Into<bool>) -> Self {
        self.available = Some(available.into());
        self
    }
    pub fn build(self) -> PressureMetadata {
        PressureMetadata {
            available: self.available,
        }
    }
}
impl WorkAreaInsets {
    pub fn builder() -> WorkAreaInsetsBuilder {
        WorkAreaInsetsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct WorkAreaInsetsBuilder {
    top: Option<i64>,
    left: Option<i64>,
    bottom: Option<i64>,
    right: Option<i64>,
}
impl WorkAreaInsetsBuilder {
    pub fn top(mut self, top: impl Into<i64>) -> Self {
        self.top = Some(top.into());
        self
    }
    pub fn left(mut self, left: impl Into<i64>) -> Self {
        self.left = Some(left.into());
        self
    }
    pub fn bottom(mut self, bottom: impl Into<i64>) -> Self {
        self.bottom = Some(bottom.into());
        self
    }
    pub fn right(mut self, right: impl Into<i64>) -> Self {
        self.right = Some(right.into());
        self
    }
    pub fn build(self) -> WorkAreaInsets {
        WorkAreaInsets {
            top: self.top,
            left: self.left,
            bottom: self.bottom,
            right: self.right,
        }
    }
}
impl ScreenInfo {
    pub fn builder() -> ScreenInfoBuilder {
        ScreenInfoBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ScreenInfoBuilder {
    left: Option<i64>,
    top: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
    avail_left: Option<i64>,
    avail_top: Option<i64>,
    avail_width: Option<i64>,
    avail_height: Option<i64>,
    device_pixel_ratio: Option<f64>,
    orientation: Option<ScreenOrientation>,
    color_depth: Option<i64>,
    is_extended: Option<bool>,
    is_internal: Option<bool>,
    is_primary: Option<bool>,
    label: Option<String>,
    id: Option<ScreenId>,
}
impl ScreenInfoBuilder {
    pub fn left(mut self, left: impl Into<i64>) -> Self {
        self.left = Some(left.into());
        self
    }
    pub fn top(mut self, top: impl Into<i64>) -> Self {
        self.top = Some(top.into());
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
    pub fn avail_left(mut self, avail_left: impl Into<i64>) -> Self {
        self.avail_left = Some(avail_left.into());
        self
    }
    pub fn avail_top(mut self, avail_top: impl Into<i64>) -> Self {
        self.avail_top = Some(avail_top.into());
        self
    }
    pub fn avail_width(mut self, avail_width: impl Into<i64>) -> Self {
        self.avail_width = Some(avail_width.into());
        self
    }
    pub fn avail_height(mut self, avail_height: impl Into<i64>) -> Self {
        self.avail_height = Some(avail_height.into());
        self
    }
    pub fn device_pixel_ratio(mut self, device_pixel_ratio: impl Into<f64>) -> Self {
        self.device_pixel_ratio = Some(device_pixel_ratio.into());
        self
    }
    pub fn orientation(mut self, orientation: impl Into<ScreenOrientation>) -> Self {
        self.orientation = Some(orientation.into());
        self
    }
    pub fn color_depth(mut self, color_depth: impl Into<i64>) -> Self {
        self.color_depth = Some(color_depth.into());
        self
    }
    pub fn is_extended(mut self, is_extended: impl Into<bool>) -> Self {
        self.is_extended = Some(is_extended.into());
        self
    }
    pub fn is_internal(mut self, is_internal: impl Into<bool>) -> Self {
        self.is_internal = Some(is_internal.into());
        self
    }
    pub fn is_primary(mut self, is_primary: impl Into<bool>) -> Self {
        self.is_primary = Some(is_primary.into());
        self
    }
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn id(mut self, id: impl Into<ScreenId>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn build(self) -> Result<ScreenInfo, String> {
        Ok(ScreenInfo {
            left: self
                .left
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(left)))?,
            top: self
                .top
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(top)))?,
            width: self
                .width
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(width)))?,
            height: self
                .height
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(height)))?,
            avail_left: self
                .avail_left
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(avail_left)))?,
            avail_top: self
                .avail_top
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(avail_top)))?,
            avail_width: self
                .avail_width
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(avail_width)))?,
            avail_height: self.avail_height.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(avail_height))
            })?,
            device_pixel_ratio: self.device_pixel_ratio.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(device_pixel_ratio)
                )
            })?,
            orientation: self
                .orientation
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(orientation)))?,
            color_depth: self
                .color_depth
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(color_depth)))?,
            is_extended: self
                .is_extended
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(is_extended)))?,
            is_internal: self
                .is_internal
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(is_internal)))?,
            is_primary: self
                .is_primary
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(is_primary)))?,
            label: self
                .label
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(label)))?,
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
        })
    }
}
