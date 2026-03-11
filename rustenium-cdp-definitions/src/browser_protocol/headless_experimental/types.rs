use serde::{Deserialize, Serialize};
#[doc = "Encoding options for a screenshot.\n[ScreenshotParams](https://chromedevtools.github.io/devtools-protocol/tot/HeadlessExperimental/#type-ScreenshotParams)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScreenshotParams {
    #[doc = "Image compression format (defaults to png)."]
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub format: Option<ScreenshotParamsFormat>,
    #[doc = "Compression quality from range [0..100] (jpeg and webp only)."]
    #[serde(rename = "quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub quality: Option<i64>,
    #[doc = "Optimize image encoding for speed, not for resulting size (defaults to false)"]
    #[serde(rename = "optimizeForSpeed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub optimize_for_speed: Option<bool>,
}
#[doc = "Image compression format (defaults to png)."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScreenshotParamsFormat {
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
}
impl ScreenshotParams {
    pub const IDENTIFIER: &'static str = "HeadlessExperimental.ScreenshotParams";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (HeadlessExperimentalTypes { ScreenshotParams (ScreenshotParams) });
