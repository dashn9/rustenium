// Generated events for module

use serde::{Serialize, Deserialize};
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputEvent {
    FileDialogOpened(FileDialogOpened),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputFileDialogOpenedMethod {
    #[serde(rename = "input.fileDialogOpened")]
    InputFileDialogOpened,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDialogOpened {
    #[serde(rename = "method")]
    pub method: InputFileDialogOpenedMethod,
    #[serde(rename = "params")]
    pub params: FileDialogInfo,
}

