use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RealmCreatedParams {}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RealmCreatedMethod {
    #[serde(rename = "script.realmCreated")]
    RealmCreated,
}
#[derive(Debug, Clone, PartialEq)]
pub struct RealmCreated {
    pub method: RealmCreatedMethod,
    pub params: RealmCreatedParams,
}
impl RealmCreated {
    pub const IDENTIFIER: &'static str = "script.realmCreated";
    pub const DOMAIN_DIRECTION: &'static str = "local";
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
#[derive(Debug, Clone, PartialEq)]
pub struct RealmDestroyed {
    pub method: RealmDestroyedMethod,
    pub params: RealmDestroyedParams,
}
impl RealmDestroyed {
    pub const IDENTIFIER: &'static str = "script.realmDestroyed";
    pub const DOMAIN_DIRECTION: &'static str = "local";
}
group_enum ! (ScriptEvents { RealmCreated (RealmCreated) , RealmDestroyed (RealmDestroyed) });
