use super::commands::*;
impl BeginFrame {
    pub fn builder() -> BeginFrameBuilder {
        BeginFrameBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct BeginFrameBuilder {
    frame_time_ticks: Option<f64>,
    interval: Option<f64>,
    no_display_updates: Option<bool>,
    screenshot: Option<super::types::ScreenshotParams>,
}
impl BeginFrameBuilder {
    pub fn frame_time_ticks(mut self, frame_time_ticks: impl Into<f64>) -> Self {
        self.frame_time_ticks = Some(frame_time_ticks.into());
        self
    }
    pub fn interval(mut self, interval: impl Into<f64>) -> Self {
        self.interval = Some(interval.into());
        self
    }
    pub fn no_display_updates(mut self, no_display_updates: impl Into<bool>) -> Self {
        self.no_display_updates = Some(no_display_updates.into());
        self
    }
    pub fn screenshot(mut self, screenshot: impl Into<super::types::ScreenshotParams>) -> Self {
        self.screenshot = Some(screenshot.into());
        self
    }
    pub fn build(self) -> BeginFrame {
        BeginFrame {
            method: BeginFrameMethod::BeginFrame,
            params: BeginFrameParams {
                frame_time_ticks: self.frame_time_ticks,
                interval: self.interval,
                no_display_updates: self.no_display_updates,
                screenshot: self.screenshot,
            },
        }
    }
}
