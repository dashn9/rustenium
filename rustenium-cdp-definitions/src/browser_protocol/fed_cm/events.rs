use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DialogShownParams {
    #[serde(rename = "dialogId")]
    pub dialog_id: String,
    #[serde(rename = "dialogType")]
    pub dialog_type: super::types::DialogType,
    #[serde(rename = "accounts")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub accounts: Vec<super::types::Account>,
    #[doc = "These exist primarily so that the caller can verify the\nRP context was used appropriately."]
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "subtitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub subtitle: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DialogShownMethod {
    #[serde(rename = "FedCm.dialogShown")]
    DialogShown,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DialogShown {
    pub method: DialogShownMethod,
    pub params: DialogShownParams,
}
impl DialogShown {
    pub const IDENTIFIER: &'static str = "FedCm.dialogShown";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[doc = "Triggered when a dialog is closed, either by user action, JS abort,\nor a command below.\n[dialogClosed](https://chromedevtools.github.io/devtools-protocol/tot/FedCm/#event-dialogClosed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DialogClosedParams {
    #[serde(rename = "dialogId")]
    pub dialog_id: String,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DialogClosedMethod {
    #[serde(rename = "FedCm.dialogClosed")]
    DialogClosed,
}
#[doc = "Triggered when a dialog is closed, either by user action, JS abort,\nor a command below.\n[dialogClosed](https://chromedevtools.github.io/devtools-protocol/tot/FedCm/#event-dialogClosed)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DialogClosed {
    pub method: DialogClosedMethod,
    pub params: DialogClosedParams,
}
impl DialogClosed {
    pub const IDENTIFIER: &'static str = "FedCm.dialogClosed";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (FedCmEvents { DialogShown (DialogShown) , DialogClosed (DialogClosed) } + identifiable);
