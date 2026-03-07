use super::types::*;
impl Account {
    pub fn builder() -> AccountBuilder {
        <AccountBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct AccountBuilder {
    account_id: Option<String>,
    email: Option<String>,
    name: Option<String>,
    given_name: Option<String>,
    picture_url: Option<String>,
    idp_config_url: Option<String>,
    idp_login_url: Option<String>,
    login_state: Option<LoginState>,
    terms_of_service_url: Option<String>,
    privacy_policy_url: Option<String>,
}
impl AccountBuilder {
    pub fn account_id(mut self, account_id: impl Into<String>) -> Self {
        self.account_id = Some(account_id.into());
        self
    }
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn given_name(mut self, given_name: impl Into<String>) -> Self {
        self.given_name = Some(given_name.into());
        self
    }
    pub fn picture_url(mut self, picture_url: impl Into<String>) -> Self {
        self.picture_url = Some(picture_url.into());
        self
    }
    pub fn idp_config_url(mut self, idp_config_url: impl Into<String>) -> Self {
        self.idp_config_url = Some(idp_config_url.into());
        self
    }
    pub fn idp_login_url(mut self, idp_login_url: impl Into<String>) -> Self {
        self.idp_login_url = Some(idp_login_url.into());
        self
    }
    pub fn login_state(mut self, login_state: impl Into<LoginState>) -> Self {
        self.login_state = Some(login_state.into());
        self
    }
    pub fn terms_of_service_url(mut self, terms_of_service_url: impl Into<String>) -> Self {
        self.terms_of_service_url = Some(terms_of_service_url.into());
        self
    }
    pub fn privacy_policy_url(mut self, privacy_policy_url: impl Into<String>) -> Self {
        self.privacy_policy_url = Some(privacy_policy_url.into());
        self
    }
    pub fn build(self) -> Result<Account, String> {
        Ok(Account {
            account_id: self
                .account_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(account_id)))?,
            email: self
                .email
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(email)))?,
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            given_name: self
                .given_name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(given_name)))?,
            picture_url: self
                .picture_url
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(picture_url)))?,
            idp_config_url: self.idp_config_url.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(idp_config_url))
            })?,
            idp_login_url: self.idp_login_url.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(idp_login_url))
            })?,
            login_state: self
                .login_state
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(login_state)))?,
            terms_of_service_url: self.terms_of_service_url,
            privacy_policy_url: self.privacy_policy_url,
        })
    }
}
