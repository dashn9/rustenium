use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeginFrameReturns {
    #[doc = "Whether the BeginFrame resulted in damage and, thus, a new frame was committed to the\ndisplay. Reported for diagnostic uses, may be removed in the future."]
    #[serde(rename = "hasDamage")]
    pub has_damage: bool,
    #[doc = "Base64-encoded image data of the screenshot, if one was requested and successfully taken."]
    #[serde(rename = "screenshotData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub screenshot_data: Option<super::super::super::Binary>,
}
