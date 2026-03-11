use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MessageParams {
    #[serde(rename = "channel")]
    pub channel: super::types::Channel,
    #[serde(rename = "data")]
    pub data: super::types::RemoteValue,
    #[serde(rename = "source")]
    pub source: super::types::Source,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageMethod {
    #[serde(rename = "script.message")]
    Message,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub method: MessageMethod,
    pub params: MessageParams,
}
impl Message {
    pub const IDENTIFIER: &'static str = "script.message";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealmCreatedParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RealmCreatedMethod {
    #[serde(rename = "script.realmCreated")]
    RealmCreated,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealmCreated {
    pub method: RealmCreatedMethod,
    pub params: RealmCreatedParams,
}
impl RealmCreated {
    pub const IDENTIFIER: &'static str = "script.realmCreated";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealmDestroyedParams {
    #[serde(rename = "realm")]
    pub realm: super::types::Realm,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RealmDestroyedMethod {
    #[serde(rename = "script.realmDestroyed")]
    RealmDestroyed,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealmDestroyed {
    pub method: RealmDestroyedMethod,
    pub params: RealmDestroyedParams,
}
impl RealmDestroyed {
    pub const IDENTIFIER: &'static str = "script.realmDestroyed";
    pub const DOMAIN_DIRECTION: &'static str = "local";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
group_enum ! (ScriptEvent { Message (Message) , RealmCreated (RealmCreated) , RealmDestroyed (RealmDestroyed) });
