use super::types::*;
impl ScreenshotParams {
    pub fn builder() -> ScreenshotParamsBuilder {
        <ScreenshotParamsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ScreenshotParamsBuilder {
    format: Option<ScreenshotParamsFormat>,
    quality: Option<i64>,
    optimize_for_speed: Option<bool>,
}
impl ScreenshotParamsBuilder {
    pub fn format(mut self, format: impl Into<ScreenshotParamsFormat>) -> Self {
        self.format = Some(format.into());
        self
    }
    pub fn quality(mut self, quality: impl Into<i64>) -> Self {
        self.quality = Some(quality.into());
        self
    }
    pub fn optimize_for_speed(mut self, optimize_for_speed: impl Into<bool>) -> Self {
        self.optimize_for_speed = Some(optimize_for_speed.into());
        self
    }
    pub fn build(self) -> ScreenshotParams {
        ScreenshotParams {
            format: self.format,
            quality: self.quality,
            optimize_for_speed: self.optimize_for_speed,
        }
    }
}
