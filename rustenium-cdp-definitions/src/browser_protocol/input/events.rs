use serde::{Deserialize, Serialize};
#[doc = "Emitted only when `Input.setInterceptDrags` is enabled. Use this data with `Input.dispatchDragEvent` to\nrestore normal drag and drop behavior.\n[dragIntercepted](https://chromedevtools.github.io/devtools-protocol/tot/Input/#event-dragIntercepted)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DragIntercepted {
    #[serde(rename = "data")]
    pub data: super::types::DragData,
}
impl DragIntercepted {
    pub const IDENTIFIER: &'static str = "Input.dragIntercepted";
}
group_enum ! (InputEvents { DragIntercepted (DragIntercepted) });
