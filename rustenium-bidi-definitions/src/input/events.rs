use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileDialogOpenedParams {
    #[serde(rename = "context")]
    pub context: crate::browsing_context::types::BrowsingContext,
    #[serde(rename = "element")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub element: Option<crate::script::types::SharedReference>,
    #[serde(rename = "multiple")]
    pub multiple: bool,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FileDialogOpenedMethod {
    #[serde(rename = "input.fileDialogOpened")]
    FileDialogOpened,
}
impl FileDialogOpenedMethod {
    pub const IDENTIFIER: &'static str = "input.fileDialogOpened";
}
#[derive(Debug, Clone, PartialEq)]
pub struct FileDialogOpened {
    pub method: FileDialogOpenedMethod,
    pub params: FileDialogOpenedParams,
}
group_enum ! (InputEvents { FileDialogOpened (FileDialogOpened) });
