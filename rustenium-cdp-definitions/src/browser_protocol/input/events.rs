use serde::{Deserialize, Serialize};
#[doc = "Emitted only when `Input.setInterceptDrags` is enabled. Use this data with `Input.dispatchDragEvent` to\nrestore normal drag and drop behavior.\n[dragIntercepted](https://chromedevtools.github.io/devtools-protocol/tot/Input/#event-dragIntercepted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DragInterceptedParams {
    #[serde(rename = "data")]
    pub data: super::types::DragData,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DragInterceptedMethod {
    #[serde(rename = "Input.dragIntercepted")]
    DragIntercepted,
}
#[doc = "Emitted only when `Input.setInterceptDrags` is enabled. Use this data with `Input.dispatchDragEvent` to\nrestore normal drag and drop behavior.\n[dragIntercepted](https://chromedevtools.github.io/devtools-protocol/tot/Input/#event-dragIntercepted)"]
#[derive(Debug, Clone, PartialEq)]
pub struct DragIntercepted {
    pub method: DragInterceptedMethod,
    pub params: DragInterceptedParams,
}
impl DragIntercepted {
    pub const IDENTIFIER: &'static str = "Input.dragIntercepted";
}
group_enum ! (InputEvents { DragIntercepted (DragIntercepted) });
