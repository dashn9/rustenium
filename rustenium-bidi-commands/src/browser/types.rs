// Generated types for module

use serde::{Serialize, Deserialize};

pub type UserContext = String;


pub type ClientWindow = String;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StateUnion {
    #[serde(rename = "fullscreen")]
    Fullscreen,
    #[serde(rename = "maximized")]
    Maximized,
    #[serde(rename = "minimized")]
    Minimized,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientWindowNamedState {
    #[serde(rename = "state")]
    pub state: StateUnion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NormalEnum {
    #[serde(rename = "normal")]
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientWindowRectState {
    #[serde(rename = "state")]
    pub state: NormalEnum,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u64>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u64>,
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    #[serde(rename = "y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientWindowNamedStateClientWindowRectStateUnion {
    ClientWindowNamedState(ClientWindowNamedState),
    ClientWindowRectState(ClientWindowRectState),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContextInfo {
    #[serde(rename = "userContext")]
    pub user_context: UserContext,
}

