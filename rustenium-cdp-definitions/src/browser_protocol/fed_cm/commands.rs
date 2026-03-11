use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnableParams {
    #[doc = "Allows callers to disable the promise rejection delay that would\nnormally happen, if this is unimportant to what's being tested.\n(step 4 of https://fedidcg.github.io/FedCM/#browser-api-rp-sign-in)"]
    #[serde(rename = "disableRejectionDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub disable_rejection_delay: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnableMethod {
    #[serde(rename = "FedCm.enable")]
    Enable,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Enable {
    pub method: EnableMethod,
    pub params: EnableParams,
}
impl Enable {
    pub const IDENTIFIER: &'static str = "FedCm.enable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Enable {
    type Result = super::results::EnableResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DisableParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DisableMethod {
    #[serde(rename = "FedCm.disable")]
    Disable,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Disable {
    pub method: DisableMethod,
    pub params: DisableParams,
}
impl Disable {
    pub const IDENTIFIER: &'static str = "FedCm.disable";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for Disable {
    type Result = super::results::DisableResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectAccountParams {
    #[serde(rename = "dialogId")]
    pub dialog_id: String,
    #[serde(rename = "accountIndex")]
    pub account_index: i64,
}
impl SelectAccountParams {
    pub fn new(dialog_id: impl Into<String>, account_index: impl Into<i64>) -> Self {
        Self {
            dialog_id: dialog_id.into(),
            account_index: account_index.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SelectAccountMethod {
    #[serde(rename = "FedCm.selectAccount")]
    SelectAccount,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectAccount {
    pub method: SelectAccountMethod,
    pub params: SelectAccountParams,
}
impl SelectAccount {
    pub const IDENTIFIER: &'static str = "FedCm.selectAccount";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for SelectAccount {
    type Result = super::results::SelectAccountResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClickDialogButtonParams {
    #[serde(rename = "dialogId")]
    pub dialog_id: String,
    #[serde(rename = "dialogButton")]
    pub dialog_button: super::types::DialogButton,
}
impl ClickDialogButtonParams {
    pub fn new(
        dialog_id: impl Into<String>,
        dialog_button: impl Into<super::types::DialogButton>,
    ) -> Self {
        Self {
            dialog_id: dialog_id.into(),
            dialog_button: dialog_button.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ClickDialogButtonMethod {
    #[serde(rename = "FedCm.clickDialogButton")]
    ClickDialogButton,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClickDialogButton {
    pub method: ClickDialogButtonMethod,
    pub params: ClickDialogButtonParams,
}
impl ClickDialogButton {
    pub const IDENTIFIER: &'static str = "FedCm.clickDialogButton";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ClickDialogButton {
    type Result = super::results::ClickDialogButtonResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenUrlParams {
    #[serde(rename = "dialogId")]
    pub dialog_id: String,
    #[serde(rename = "accountIndex")]
    pub account_index: i64,
    #[serde(rename = "accountUrlType")]
    pub account_url_type: super::types::AccountUrlType,
}
impl OpenUrlParams {
    pub fn new(
        dialog_id: impl Into<String>,
        account_index: impl Into<i64>,
        account_url_type: impl Into<super::types::AccountUrlType>,
    ) -> Self {
        Self {
            dialog_id: dialog_id.into(),
            account_index: account_index.into(),
            account_url_type: account_url_type.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OpenUrlMethod {
    #[serde(rename = "FedCm.openUrl")]
    OpenUrl,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenUrl {
    pub method: OpenUrlMethod,
    pub params: OpenUrlParams,
}
impl OpenUrl {
    pub const IDENTIFIER: &'static str = "FedCm.openUrl";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for OpenUrl {
    type Result = super::results::OpenUrlResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DismissDialogParams {
    #[serde(rename = "dialogId")]
    pub dialog_id: String,
    #[serde(rename = "triggerCooldown")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub trigger_cooldown: Option<bool>,
}
impl DismissDialogParams {
    pub fn new(dialog_id: impl Into<String>) -> Self {
        Self {
            dialog_id: dialog_id.into(),
            trigger_cooldown: None,
        }
    }
}
impl<T: Into<String>> From<T> for DismissDialogParams {
    fn from(url: T) -> Self {
        DismissDialogParams::new(url)
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DismissDialogMethod {
    #[serde(rename = "FedCm.dismissDialog")]
    DismissDialog,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DismissDialog {
    pub method: DismissDialogMethod,
    pub params: DismissDialogParams,
}
impl DismissDialog {
    pub const IDENTIFIER: &'static str = "FedCm.dismissDialog";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for DismissDialog {
    type Result = super::results::DismissDialogResult;
}
#[doc = "Resets the cooldown time, if any, to allow the next FedCM call to show\na dialog even if one was recently dismissed by the user.\n[resetCooldown](https://chromedevtools.github.io/devtools-protocol/tot/FedCm/#method-resetCooldown)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetCooldownParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ResetCooldownMethod {
    #[serde(rename = "FedCm.resetCooldown")]
    ResetCooldown,
}
#[doc = "Resets the cooldown time, if any, to allow the next FedCM call to show\na dialog even if one was recently dismissed by the user.\n[resetCooldown](https://chromedevtools.github.io/devtools-protocol/tot/FedCm/#method-resetCooldown)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetCooldown {
    pub method: ResetCooldownMethod,
    pub params: ResetCooldownParams,
}
impl ResetCooldown {
    pub const IDENTIFIER: &'static str = "FedCm.resetCooldown";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
impl crate::CommandResult for ResetCooldown {
    type Result = super::results::ResetCooldownResult;
}
group_enum ! (FedCmCommands { Enable (Enable) , Disable (Disable) , SelectAccount (SelectAccount) , ClickDialogButton (ClickDialogButton) , OpenUrl (OpenUrl) , DismissDialog (DismissDialog) , ResetCooldown (ResetCooldown) });
