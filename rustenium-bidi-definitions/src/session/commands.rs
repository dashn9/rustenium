use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusMethod {
    #[serde(rename = "session.status")]
    Status,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub method: StatusMethod,
    pub params: StatusParams,
}
impl Status {
    pub const IDENTIFIER: &'static str = "session.status";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
impl crate::CommandResult for Status {
    type Result = super::results::StatusResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewParams {
    #[serde(rename = "capabilities")]
    pub capabilities: super::types::CapabilitiesRequest,
}
impl NewParams {
    pub fn new(capabilities: impl Into<super::types::CapabilitiesRequest>) -> Self {
        Self {
            capabilities: capabilities.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NewMethod {
    #[serde(rename = "session.new")]
    New,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct New {
    pub method: NewMethod,
    pub params: NewParams,
}
impl New {
    pub const IDENTIFIER: &'static str = "session.new";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
impl crate::CommandResult for New {
    type Result = super::results::NewResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EndMethod {
    #[serde(rename = "session.end")]
    End,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct End {
    pub method: EndMethod,
    pub params: EndParams,
}
impl End {
    pub const IDENTIFIER: &'static str = "session.end";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
impl crate::CommandResult for End {
    type Result = super::results::EndResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribeParams {
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<String>,
    #[serde(rename = "contexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub contexts: Option<Vec<crate::browsing_context::types::BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub user_contexts: Option<Vec<crate::browser::types::UserContext>>,
}
impl SubscribeParams {
    pub fn new(events: Vec<String>) -> Self {
        Self {
            events,
            contexts: None,
            user_contexts: None,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SubscribeMethod {
    #[serde(rename = "session.subscribe")]
    Subscribe,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Subscribe {
    pub method: SubscribeMethod,
    pub params: SubscribeParams,
}
impl Subscribe {
    pub const IDENTIFIER: &'static str = "session.subscribe";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
impl crate::CommandResult for Subscribe {
    type Result = super::results::SubscribeResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribeParams {
    #[serde(flatten)]
    #[serde(default)]
    pub unsubscribe_parameters: super::types::UnsubscribeParameters,
}
impl UnsubscribeParams {
    pub fn new(unsubscribe_parameters: impl Into<super::types::UnsubscribeParameters>) -> Self {
        Self {
            unsubscribe_parameters: unsubscribe_parameters.into(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnsubscribeMethod {
    #[serde(rename = "session.unsubscribe")]
    Unsubscribe,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Unsubscribe {
    pub method: UnsubscribeMethod,
    pub params: UnsubscribeParams,
}
impl Unsubscribe {
    pub const IDENTIFIER: &'static str = "session.unsubscribe";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
}
impl crate::CommandResult for Unsubscribe {
    type Result = super::results::UnsubscribeResult;
}
group_enum ! (SessionCommand { Status (Status) , New (New) , End (End) , Subscribe (Subscribe) , Unsubscribe (Unsubscribe) });
