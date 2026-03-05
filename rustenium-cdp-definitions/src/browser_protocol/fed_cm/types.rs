use serde::{Deserialize, Serialize};
#[doc = "Whether this is a sign-up or sign-in action for this account, i.e.\nwhether this account has ever been used to sign in to this RP before."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LoginState {
    #[serde(rename = "SignIn")]
    SignIn,
    #[serde(rename = "SignUp")]
    SignUp,
}
#[doc = "The types of FedCM dialogs."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DialogType {
    #[serde(rename = "AccountChooser")]
    AccountChooser,
    #[serde(rename = "AutoReauthn")]
    AutoReauthn,
    #[serde(rename = "ConfirmIdpLogin")]
    ConfirmIdpLogin,
    #[serde(rename = "Error")]
    Error,
}
#[doc = "The buttons on the FedCM dialog."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DialogButton {
    #[serde(rename = "ConfirmIdpLoginContinue")]
    ConfirmIdpLoginContinue,
    #[serde(rename = "ErrorGotIt")]
    ErrorGotIt,
    #[serde(rename = "ErrorMoreDetails")]
    ErrorMoreDetails,
}
#[doc = "The URLs that each account has"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AccountUrlType {
    #[serde(rename = "TermsOfService")]
    TermsOfService,
    #[serde(rename = "PrivacyPolicy")]
    PrivacyPolicy,
}
#[doc = "Corresponds to IdentityRequestAccount\n[Account](https://chromedevtools.github.io/devtools-protocol/tot/FedCm/#type-Account)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "givenName")]
    pub given_name: String,
    #[serde(rename = "pictureUrl")]
    pub picture_url: String,
    #[serde(rename = "idpConfigUrl")]
    pub idp_config_url: String,
    #[serde(rename = "idpLoginUrl")]
    pub idp_login_url: String,
    #[serde(rename = "loginState")]
    pub login_state: LoginState,
    #[doc = "These two are only set if the loginState is signUp"]
    #[serde(rename = "termsOfServiceUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub terms_of_service_url: Option<String>,
    #[serde(rename = "privacyPolicyUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub privacy_policy_url: Option<String>,
}
impl Account {
    pub const IDENTIFIER: &'static str = "FedCm.Account";
}
group_enum ! (FedCmTypes { LoginState (LoginState) , DialogType (DialogType) , DialogButton (DialogButton) , AccountUrlType (AccountUrlType) , Account (Account) });
