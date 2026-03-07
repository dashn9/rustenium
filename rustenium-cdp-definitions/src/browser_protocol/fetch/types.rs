use serde::{Deserialize, Serialize};
#[doc = "Unique request identifier.\nNote that this does not identify individual HTTP requests that are part of\na network request.\n[RequestId](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#type-RequestId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct RequestId(String);
impl RequestId {
    pub fn new(val: impl Into<String>) -> Self {
        RequestId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for RequestId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<RequestId> for String {
    fn from(el: RequestId) -> String {
        el.0
    }
}
impl From<String> for RequestId {
    fn from(expr: String) -> Self {
        RequestId(expr)
    }
}
impl std::borrow::Borrow<str> for RequestId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl RequestId {
    pub const IDENTIFIER: &'static str = "Fetch.RequestId";
}
#[doc = "Stages of the request to handle. Request will intercept before the request is\nsent. Response will intercept after the response is received (but before response\nbody is received)."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RequestStage {
    #[serde(rename = "Request")]
    Request,
    #[serde(rename = "Response")]
    Response,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestPattern {
    #[doc = "Wildcards (`'*'` -> zero or more, `'?'` -> exactly one) are allowed. Escape character is\nbackslash. Omitting is equivalent to `\"*\"`."]
    #[serde(rename = "urlPattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub url_pattern: Option<String>,
    #[doc = "If set, only requests for matching resource types will be intercepted."]
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub resource_type: Option<crate::browser_protocol::network::types::ResourceType>,
    #[doc = "Stage at which to begin intercepting requests. Default is Request."]
    #[serde(rename = "requestStage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub request_stage: Option<RequestStage>,
}
impl RequestPattern {
    pub const IDENTIFIER: &'static str = "Fetch.RequestPattern";
}
#[doc = "Response HTTP header entry\n[HeaderEntry](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#type-HeaderEntry)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeaderEntry {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl HeaderEntry {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
impl HeaderEntry {
    pub const IDENTIFIER: &'static str = "Fetch.HeaderEntry";
}
#[doc = "Authorization challenge for HTTP status code 401 or 407.\n[AuthChallenge](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#type-AuthChallenge)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthChallenge {
    #[doc = "Source of the authentication challenge."]
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub source: Option<AuthChallengeSource>,
    #[doc = "Origin of the challenger."]
    #[serde(rename = "origin")]
    pub origin: String,
    #[doc = "The authentication scheme used, such as basic or digest"]
    #[serde(rename = "scheme")]
    pub scheme: String,
    #[doc = "The realm of the challenge. May be empty."]
    #[serde(rename = "realm")]
    pub realm: String,
}
#[doc = "Source of the authentication challenge."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthChallengeSource {
    #[serde(rename = "Server")]
    Server,
    #[serde(rename = "Proxy")]
    Proxy,
}
impl AuthChallenge {
    pub fn new(
        origin: impl Into<String>,
        scheme: impl Into<String>,
        realm: impl Into<String>,
    ) -> Self {
        Self {
            origin: origin.into(),
            scheme: scheme.into(),
            realm: realm.into(),
            source: None,
        }
    }
}
impl AuthChallenge {
    pub const IDENTIFIER: &'static str = "Fetch.AuthChallenge";
}
#[doc = "Response to an AuthChallenge.\n[AuthChallengeResponse](https://chromedevtools.github.io/devtools-protocol/tot/Fetch/#type-AuthChallengeResponse)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthChallengeResponse {
    #[doc = "The decision on what to do in response to the authorization challenge.  Default means\ndeferring to the default behavior of the net stack, which will likely either the Cancel\nauthentication or display a popup dialog box."]
    #[serde(rename = "response")]
    pub response: AuthChallengeResponseResponse,
    #[doc = "The username to provide, possibly empty. Should only be set if response is\nProvideCredentials."]
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub username: Option<String>,
    #[doc = "The password to provide, possibly empty. Should only be set if response is\nProvideCredentials."]
    #[serde(rename = "password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub password: Option<String>,
}
#[doc = "The decision on what to do in response to the authorization challenge.  Default means\ndeferring to the default behavior of the net stack, which will likely either the Cancel\nauthentication or display a popup dialog box."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AuthChallengeResponseResponse {
    #[serde(rename = "Default")]
    Default,
    #[serde(rename = "CancelAuth")]
    CancelAuth,
    #[serde(rename = "ProvideCredentials")]
    ProvideCredentials,
}
impl AuthChallengeResponse {
    pub fn new(response: impl Into<AuthChallengeResponseResponse>) -> Self {
        Self {
            response: response.into(),
            username: None,
            password: None,
        }
    }
}
impl AuthChallengeResponse {
    pub const IDENTIFIER: &'static str = "Fetch.AuthChallengeResponse";
}
group_enum ! (FetchTypes { RequestId (RequestId) , RequestStage (RequestStage) , RequestPattern (RequestPattern) , HeaderEntry (HeaderEntry) , AuthChallenge (AuthChallenge) , AuthChallengeResponse (AuthChallengeResponse) });
