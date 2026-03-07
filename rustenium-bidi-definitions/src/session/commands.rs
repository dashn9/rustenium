use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StatusParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatusMethod {
    #[serde(rename = "session.status")]
    Status,
}
impl StatusMethod {
    pub const IDENTIFIER: &'static str = "session.status";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Status {
    pub method: StatusMethod,
    pub params: StatusParams,
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
impl NewMethod {
    pub const IDENTIFIER: &'static str = "session.new";
}
#[derive(Debug, Clone, PartialEq)]
pub struct New {
    pub method: NewMethod,
    pub params: NewParams,
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
impl EndMethod {
    pub const IDENTIFIER: &'static str = "session.end";
}
#[derive(Debug, Clone, PartialEq)]
pub struct End {
    pub method: EndMethod,
    pub params: EndParams,
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
impl SubscribeMethod {
    pub const IDENTIFIER: &'static str = "session.subscribe";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Subscribe {
    pub method: SubscribeMethod,
    pub params: SubscribeParams,
}
impl crate::CommandResult for Subscribe {
    type Result = super::results::SubscribeResult;
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnsubscribeParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnsubscribeMethod {
    #[serde(rename = "session.unsubscribe")]
    Unsubscribe,
}
impl UnsubscribeMethod {
    pub const IDENTIFIER: &'static str = "session.unsubscribe";
}
#[derive(Debug, Clone, PartialEq)]
pub struct Unsubscribe {
    pub method: UnsubscribeMethod,
    pub params: UnsubscribeParams,
}
impl crate::CommandResult for Unsubscribe {
    type Result = super::results::UnsubscribeResult;
}
group_enum ! (SessionCommands { Status (Status) , New (New) , End (End) , Subscribe (Subscribe) , Unsubscribe (Unsubscribe) });
