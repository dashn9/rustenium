use super::types::*;
impl Bounds {
    pub fn builder() -> BoundsBuilder {
        <BoundsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BoundsBuilder {
    left: Option<i64>,
    top: Option<i64>,
    width: Option<i64>,
    height: Option<i64>,
    window_state: Option<WindowState>,
}
impl BoundsBuilder {
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
    pub fn window_state(mut self, window_state: impl Into<WindowState>) -> Self {
        self.window_state = Some(window_state.into());
        self
    }
    pub fn build(self) -> Bounds {
        Bounds {
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
            window_state: self.window_state,
        }
    }
}
impl PermissionDescriptor {
    pub fn builder() -> PermissionDescriptorBuilder {
        <PermissionDescriptorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PermissionDescriptorBuilder {
    name: Option<String>,
    sysex: Option<bool>,
    user_visible_only: Option<bool>,
    allow_without_sanitization: Option<bool>,
    allow_without_gesture: Option<bool>,
    pan_tilt_zoom: Option<bool>,
}
impl PermissionDescriptorBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn sysex(mut self, sysex: impl Into<bool>) -> Self {
        self.sysex = Some(sysex.into());
        self
    }
    pub fn user_visible_only(mut self, user_visible_only: impl Into<bool>) -> Self {
        self.user_visible_only = Some(user_visible_only.into());
        self
    }
    pub fn allow_without_sanitization(
        mut self,
        allow_without_sanitization: impl Into<bool>,
    ) -> Self {
        self.allow_without_sanitization = Some(allow_without_sanitization.into());
        self
    }
    pub fn allow_without_gesture(mut self, allow_without_gesture: impl Into<bool>) -> Self {
        self.allow_without_gesture = Some(allow_without_gesture.into());
        self
    }
    pub fn pan_tilt_zoom(mut self, pan_tilt_zoom: impl Into<bool>) -> Self {
        self.pan_tilt_zoom = Some(pan_tilt_zoom.into());
        self
    }
    pub fn build(self) -> Result<PermissionDescriptor, String> {
        Ok(PermissionDescriptor {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            sysex: self.sysex,
            user_visible_only: self.user_visible_only,
            allow_without_sanitization: self.allow_without_sanitization,
            allow_without_gesture: self.allow_without_gesture,
            pan_tilt_zoom: self.pan_tilt_zoom,
        })
    }
}
impl Bucket {
    pub fn builder() -> BucketBuilder {
        <BucketBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct BucketBuilder {
    low: Option<i64>,
    high: Option<i64>,
    count: Option<i64>,
}
impl BucketBuilder {
    pub fn low(mut self, low: impl Into<i64>) -> Self {
        self.low = Some(low.into());
        self
    }
    pub fn high(mut self, high: impl Into<i64>) -> Self {
        self.high = Some(high.into());
        self
    }
    pub fn count(mut self, count: impl Into<i64>) -> Self {
        self.count = Some(count.into());
        self
    }
    pub fn build(self) -> Result<Bucket, String> {
        Ok(Bucket {
            low: self
                .low
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(low)))?,
            high: self
                .high
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(high)))?,
            count: self
                .count
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(count)))?,
        })
    }
}
impl Histogram {
    pub fn builder() -> HistogramBuilder {
        <HistogramBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct HistogramBuilder {
    name: Option<String>,
    sum: Option<i64>,
    count: Option<i64>,
    buckets: Option<Vec<Bucket>>,
}
impl HistogramBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn sum(mut self, sum: impl Into<i64>) -> Self {
        self.sum = Some(sum.into());
        self
    }
    pub fn count(mut self, count: impl Into<i64>) -> Self {
        self.count = Some(count.into());
        self
    }
    pub fn bucket(mut self, bucket: impl Into<Bucket>) -> Self {
        let v = self.buckets.get_or_insert(Vec::new());
        v.push(bucket.into());
        self
    }
    pub fn buckets<I, S>(mut self, buckets: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Bucket>,
    {
        let v = self.buckets.get_or_insert(Vec::new());
        for val in buckets {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<Histogram, String> {
        Ok(Histogram {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            sum: self
                .sum
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(sum)))?,
            count: self
                .count
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(count)))?,
            buckets: self
                .buckets
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(buckets)))?,
        })
    }
}
