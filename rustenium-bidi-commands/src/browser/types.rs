// Generated types for module

use serde::{Serialize, Deserialize};

pub type UserContext = String;


pub type ClientWindow = String;


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientWindowNamedStatestateUnion {
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
    pub state: ClientWindowNamedStatestateUnion,
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
#[serde(untagged)]
pub enum AllowedEnum {
    #[serde(rename = "allowed")]
    Allowed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadBehaviorAllowed {
    #[serde(rename = "type")]
    pub r#type: AllowedEnum,
    #[serde(rename = "destinationFolder")]
    pub destination_folder: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeniedEnum {
    #[serde(rename = "denied")]
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadBehaviorDenied {
    #[serde(rename = "type")]
    pub r#type: DeniedEnum,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadBehavior {
    DownloadBehaviorAllowed(DownloadBehaviorAllowed),
    DownloadBehaviorDenied(DownloadBehaviorDenied),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContextInfo {
    #[serde(rename = "userContext")]
    pub user_context: UserContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientWindowInfostateUnion {
    #[serde(rename = "fullscreen")]
    Fullscreen,
    #[serde(rename = "maximized")]
    Maximized,
    #[serde(rename = "minimized")]
    Minimized,
    #[serde(rename = "normal")]
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientWindowInfo {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "clientWindow")]
    pub client_window: ClientWindow,
    #[serde(rename = "height")]
    pub height: u64,
    #[serde(rename = "state")]
    pub state: ClientWindowInfostateUnion,
    #[serde(rename = "width")]
    pub width: u64,
    #[serde(rename = "x")]
    pub x: i64,
    #[serde(rename = "y")]
    pub y: i64,
}

