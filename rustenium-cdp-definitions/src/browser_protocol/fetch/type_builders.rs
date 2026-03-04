use super::types::*;
impl RequestPattern {
    pub fn builder() -> RequestPatternBuilder {
        RequestPatternBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct RequestPatternBuilder {
    url_pattern: Option<String>,
    resource_type: Option<super::super::network::types::ResourceType>,
    request_stage: Option<RequestStage>,
}
impl RequestPatternBuilder {
    pub fn url_pattern(mut self, url_pattern: impl Into<String>) -> Self {
        self.url_pattern = Some(url_pattern.into());
        self
    }
    pub fn resource_type(
        mut self,
        resource_type: impl Into<super::super::network::types::ResourceType>,
    ) -> Self {
        self.resource_type = Some(resource_type.into());
        self
    }
    pub fn request_stage(mut self, request_stage: impl Into<RequestStage>) -> Self {
        self.request_stage = Some(request_stage.into());
        self
    }
    pub fn build(self) -> RequestPattern {
        RequestPattern {
            url_pattern: self.url_pattern,
            resource_type: self.resource_type,
            request_stage: self.request_stage,
        }
    }
}
impl HeaderEntry {
    pub fn builder() -> HeaderEntryBuilder {
        HeaderEntryBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct HeaderEntryBuilder {
    name: Option<String>,
    value: Option<String>,
}
impl HeaderEntryBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<HeaderEntry, String> {
        Ok(HeaderEntry {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl AuthChallenge {
    pub fn builder() -> AuthChallengeBuilder {
        AuthChallengeBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AuthChallengeBuilder {
    source: Option<AuthChallengeSource>,
    origin: Option<String>,
    scheme: Option<String>,
    realm: Option<String>,
}
impl AuthChallengeBuilder {
    pub fn source(mut self, source: impl Into<AuthChallengeSource>) -> Self {
        self.source = Some(source.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<String>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn scheme(mut self, scheme: impl Into<String>) -> Self {
        self.scheme = Some(scheme.into());
        self
    }
    pub fn realm(mut self, realm: impl Into<String>) -> Self {
        self.realm = Some(realm.into());
        self
    }
    pub fn build(self) -> Result<AuthChallenge, String> {
        Ok(AuthChallenge {
            source: self.source,
            origin: self
                .origin
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(origin)))?,
            scheme: self
                .scheme
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(scheme)))?,
            realm: self
                .realm
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(realm)))?,
        })
    }
}
impl AuthChallengeResponse {
    pub fn builder() -> AuthChallengeResponseBuilder {
        AuthChallengeResponseBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct AuthChallengeResponseBuilder {
    response: Option<AuthChallengeResponseResponse>,
    username: Option<String>,
    password: Option<String>,
}
impl AuthChallengeResponseBuilder {
    pub fn response(mut self, response: impl Into<AuthChallengeResponseResponse>) -> Self {
        self.response = Some(response.into());
        self
    }
    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }
    pub fn build(self) -> Result<AuthChallengeResponse, String> {
        Ok(AuthChallengeResponse {
            response: self
                .response
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(response)))?,
            username: self.username,
            password: self.password,
        })
    }
}
