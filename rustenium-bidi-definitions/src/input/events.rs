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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FileDialogOpened {
    pub method: FileDialogOpenedMethod,
    pub params: FileDialogOpenedParams,
}
impl FileDialogOpened {
    pub const IDENTIFIER: &'static str = "input.fileDialogOpened";
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (InputEvent { FileDialogOpened (FileDialogOpened) } + identifiable);
