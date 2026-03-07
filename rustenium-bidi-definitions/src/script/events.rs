use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealmCreatedParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RealmCreatedMethod {
    #[serde(rename = "script.realmCreated")]
    RealmCreated,
}
impl RealmCreatedMethod {
    pub const IDENTIFIER: &'static str = "script.realmCreated";
}
#[derive(Debug, Clone, PartialEq)]
pub struct RealmCreated {
    pub method: RealmCreatedMethod,
    pub params: RealmCreatedParams,
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
impl RealmDestroyedMethod {
    pub const IDENTIFIER: &'static str = "script.realmDestroyed";
}
#[derive(Debug, Clone, PartialEq)]
pub struct RealmDestroyed {
    pub method: RealmDestroyedMethod,
    pub params: RealmDestroyedParams,
}
group_enum ! (ScriptEvents { RealmCreated (RealmCreated) , RealmDestroyed (RealmDestroyed) });
