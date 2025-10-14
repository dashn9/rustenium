// Generated commands for module

use serde::{Serialize, Deserialize};
use crate::browser::types::UserContext;
use crate::browsing_context::types::BrowsingContext;
use crate::Extensible;
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionCommand {
    End(End),
    New(New),
    Status(Status),
    Subscribe(Subscribe),
    Unsubscribe(Unsubscribe),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionEndMethod {
    #[serde(rename = "session.End")]
    SessionEnd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionNewMethod {
    #[serde(rename = "session.New")]
    SessionNew,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionStatusMethod {
    #[serde(rename = "session.Status")]
    SessionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionSubscribeMethod {
    #[serde(rename = "session.Subscribe")]
    SessionSubscribe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionUnsubscribeMethod {
    #[serde(rename = "session.Unsubscribe")]
    SessionUnsubscribe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EmptyParams {
    Extensible(Extensible),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewParameters {
    #[serde(rename = "capabilities")]
    pub capabilities: CapabilitiesRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRequest {
    #[serde(rename = "events")]
    pub events: Vec<String>,
    #[serde(rename = "contexts")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    pub user_contexts: Option<Vec<UserContext>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnsubscribeParameters {
    UnsubscribeByAttributesRequest(UnsubscribeByAttributesRequest),
    UnsubscribeByIDRequest(UnsubscribeByIDRequest),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct End {
    #[serde(rename = "method")]
    pub method: SessionEndMethod,
    #[serde(rename = "params")]
    pub params: EmptyParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct New {
    #[serde(rename = "method")]
    pub method: SessionNewMethod,
    #[serde(rename = "params")]
    pub params: NewParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "method")]
    pub method: SessionStatusMethod,
    #[serde(rename = "params")]
    pub params: EmptyParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscribe {
    #[serde(rename = "method")]
    pub method: SessionSubscribeMethod,
    #[serde(rename = "params")]
    pub params: SubscriptionRequest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Unsubscribe {
    #[serde(rename = "method")]
    pub method: SessionUnsubscribeMethod,
    #[serde(rename = "params")]
    pub params: UnsubscribeParameters,
}

// Generated results

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionResult {
    NewResult(NewResult),
    StatusResult(StatusResult),
    SubscribeResult(SubscribeResult),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewResult {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "capabilities")]
    pub capabilities: NewResultCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResult {
    #[serde(rename = "ready")]
    pub ready: bool,
    #[serde(rename = "message")]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscribeResult {
    #[serde(rename = "subscription")]
    pub subscription: Subscription,
}

