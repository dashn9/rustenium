use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BeginFrameResult {
    #[doc = "Whether the BeginFrame resulted in damage and, thus, a new frame was committed to the\ndisplay. Reported for diagnostic uses, may be removed in the future."]
    #[serde(rename = "hasDamage")]
    pub has_damage: bool,
    #[doc = "Base64-encoded image data of the screenshot, if one was requested and successfully taken."]
    #[serde(rename = "screenshotData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub screenshot_data: Option<crate::Binary>,
}
impl TryFrom<serde_json::Value> for BeginFrameResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DisableResult {}
impl TryFrom<serde_json::Value> for DisableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableResult {}
impl TryFrom<serde_json::Value> for EnableResult {
    type Error = serde_json::Error;
    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        serde_json::from_value(value)
    }
}
