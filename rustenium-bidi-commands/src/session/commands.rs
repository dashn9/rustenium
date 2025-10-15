// Generated commands for module

use serde::{Serialize, Deserialize};
use crate::browser::types::UserContext;
use crate::browsing_context::types::BrowsingContext;
use crate::EmptyParams;
use crate::EmptyResult;
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
pub enum SessionEndMethod {
    #[serde(rename = "session.end")]
    SessionEnd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionNewMethod {
    #[serde(rename = "session.new")]
    SessionNew,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatusMethod {
    #[serde(rename = "session.status")]
    SessionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionSubscribeMethod {
    #[serde(rename = "session.subscribe")]
    SessionSubscribe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionUnsubscribeMethod {
    #[serde(rename = "session.unsubscribe")]
    SessionUnsubscribe,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<BrowsingContext>>,
    #[serde(rename = "userContexts")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
    EndResult(EndResult),
    NewResult(NewResult),
    StatusResult(StatusResult),
    SubscribeResult(SubscribeResult),
    UnsubscribeResult(UnsubscribeResult),
}


pub type EndResult = EmptyResult;


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

pub type UnsubscribeResult = EmptyResult;


