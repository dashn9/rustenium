use super::commands::*;
impl Enable {
    pub fn builder() -> EnableBuilder {
        <EnableBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct EnableBuilder {
    disable_rejection_delay: Option<bool>,
}
impl EnableBuilder {
    pub fn disable_rejection_delay(mut self, disable_rejection_delay: impl Into<bool>) -> Self {
        self.disable_rejection_delay = Some(disable_rejection_delay.into());
        self
    }
    pub fn build(self) -> Enable {
        Enable {
            method: EnableMethod::Enable,
            params: EnableParams {
                disable_rejection_delay: self.disable_rejection_delay,
            },
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct DisableBuilder;
impl DisableBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> Disable {
        Disable {
            method: DisableMethod::Disable,
            params: DisableParams {},
        }
    }
}
impl Disable {
    pub fn builder() -> DisableBuilder {
        DisableBuilder
    }
}
impl SelectAccount {
    pub fn builder() -> SelectAccountBuilder {
        <SelectAccountBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct SelectAccountBuilder {
    dialog_id: Option<String>,
    account_index: Option<i64>,
}
impl SelectAccountBuilder {
    pub fn dialog_id(mut self, dialog_id: impl Into<String>) -> Self {
        self.dialog_id = Some(dialog_id.into());
        self
    }
    pub fn account_index(mut self, account_index: impl Into<i64>) -> Self {
        self.account_index = Some(account_index.into());
        self
    }
    pub fn build(self) -> Result<SelectAccount, String> {
        Ok(SelectAccount {
            method: SelectAccountMethod::SelectAccount,
            params: SelectAccountParams {
                dialog_id: self.dialog_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(dialog_id))
                })?,
                account_index: self.account_index.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(account_index))
                })?,
            },
        })
    }
}
impl ClickDialogButton {
    pub fn builder() -> ClickDialogButtonBuilder {
        <ClickDialogButtonBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ClickDialogButtonBuilder {
    dialog_id: Option<String>,
    dialog_button: Option<super::types::DialogButton>,
}
impl ClickDialogButtonBuilder {
    pub fn dialog_id(mut self, dialog_id: impl Into<String>) -> Self {
        self.dialog_id = Some(dialog_id.into());
        self
    }
    pub fn dialog_button(mut self, dialog_button: impl Into<super::types::DialogButton>) -> Self {
        self.dialog_button = Some(dialog_button.into());
        self
    }
    pub fn build(self) -> Result<ClickDialogButton, String> {
        Ok(ClickDialogButton {
            method: ClickDialogButtonMethod::ClickDialogButton,
            params: ClickDialogButtonParams {
                dialog_id: self.dialog_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(dialog_id))
                })?,
                dialog_button: self.dialog_button.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(dialog_button))
                })?,
            },
        })
    }
}
impl OpenUrl {
    pub fn builder() -> OpenUrlBuilder {
        <OpenUrlBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct OpenUrlBuilder {
    dialog_id: Option<String>,
    account_index: Option<i64>,
    account_url_type: Option<super::types::AccountUrlType>,
}
impl OpenUrlBuilder {
    pub fn dialog_id(mut self, dialog_id: impl Into<String>) -> Self {
        self.dialog_id = Some(dialog_id.into());
        self
    }
    pub fn account_index(mut self, account_index: impl Into<i64>) -> Self {
        self.account_index = Some(account_index.into());
        self
    }
    pub fn account_url_type(
        mut self,
        account_url_type: impl Into<super::types::AccountUrlType>,
    ) -> Self {
        self.account_url_type = Some(account_url_type.into());
        self
    }
    pub fn build(self) -> Result<OpenUrl, String> {
        Ok(OpenUrl {
            method: OpenUrlMethod::OpenUrl,
            params: OpenUrlParams {
                dialog_id: self.dialog_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(dialog_id))
                })?,
                account_index: self.account_index.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(account_index))
                })?,
                account_url_type: self.account_url_type.ok_or_else(|| {
                    format!(
                        "Field `{}` is mandatory.",
                        std::stringify!(account_url_type)
                    )
                })?,
            },
        })
    }
}
impl DismissDialog {
    pub fn builder() -> DismissDialogBuilder {
        <DismissDialogBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct DismissDialogBuilder {
    dialog_id: Option<String>,
    trigger_cooldown: Option<bool>,
}
impl DismissDialogBuilder {
    pub fn dialog_id(mut self, dialog_id: impl Into<String>) -> Self {
        self.dialog_id = Some(dialog_id.into());
        self
    }
    pub fn trigger_cooldown(mut self, trigger_cooldown: impl Into<bool>) -> Self {
        self.trigger_cooldown = Some(trigger_cooldown.into());
        self
    }
    pub fn build(self) -> Result<DismissDialog, String> {
        Ok(DismissDialog {
            method: DismissDialogMethod::DismissDialog,
            params: DismissDialogParams {
                dialog_id: self.dialog_id.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(dialog_id))
                })?,
                trigger_cooldown: self.trigger_cooldown,
            },
        })
    }
}
#[derive(Debug, Clone, Default)]
pub struct ResetCooldownBuilder;
impl ResetCooldownBuilder {
    pub fn new() -> Self {
        Self
    }
    pub fn build(self) -> ResetCooldown {
        ResetCooldown {
            method: ResetCooldownMethod::ResetCooldown,
            params: ResetCooldownParams {},
        }
    }
}
impl ResetCooldown {
    pub fn builder() -> ResetCooldownBuilder {
        ResetCooldownBuilder
    }
}
