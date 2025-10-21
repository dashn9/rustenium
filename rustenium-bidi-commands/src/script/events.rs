// Generated events for module

use serde::{Serialize, Deserialize};
use super::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScriptEvent {
    Message(Message),
    RealmCreated(RealmCreated),
    RealmDestroyed(RealmDestroyed),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptMessageMethod {
    #[serde(rename = "script.message")]
    ScriptMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptRealmCreatedMethod {
    #[serde(rename = "script.realmCreated")]
    ScriptRealmCreated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScriptRealmDestroyedMethod {
    #[serde(rename = "script.realmDestroyed")]
    ScriptRealmDestroyed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "method")]
    pub method: ScriptMessageMethod,
    #[serde(rename = "params")]
    pub params: MessageParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmCreated {
    #[serde(rename = "method")]
    pub method: ScriptRealmCreatedMethod,
    #[serde(rename = "params")]
    pub params: RealmInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealmDestroyed {
    #[serde(rename = "method")]
    pub method: ScriptRealmDestroyedMethod,
    #[serde(rename = "params")]
    pub params: RealmDestroyedParameters,
}

