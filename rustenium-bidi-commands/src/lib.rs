use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub mod browser;
pub mod browsing_context;
pub mod emulation;
pub mod input;
pub mod log;
pub mod network;
pub mod script;
pub mod session;
pub mod storage;
pub mod web_extension;

pub use browser::commands::BrowserCommand;
pub use browser::commands::BrowserResult;
pub use browsing_context::commands::BrowsingContextCommand;
pub use browsing_context::commands::BrowsingContextResult;
pub use browsing_context::events::BrowsingContextEvent;
pub use emulation::commands::EmulationCommand;
pub use emulation::commands::EmulationResult;
pub use input::commands::InputCommand;
pub use input::commands::InputResult;
pub use input::events::InputEvent;
pub use log::events::LogEvent;
pub use network::commands::NetworkCommand;
pub use network::commands::NetworkResult;
pub use network::events::NetworkEvent;
pub use script::commands::ScriptCommand;
pub use script::commands::ScriptResult;
pub use script::events::ScriptEvent;
pub use session::commands::SessionCommand;
pub use session::commands::SessionResult;
pub use storage::commands::StorageCommand;
pub use storage::commands::StorageResult;
pub use web_extension::commands::WebExtensionCommand;
pub use web_extension::commands::WebExtensionResult;

use serde::Deserializer;

fn float_or_int_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    match value {
        serde_json::Value::Number(num) => {
            if let Some(i) = num.as_u64() {
                Ok(i)
            } else if let Some(f) = num.as_f64() {
                Ok(f as u64)
            } else {
                Err(serde::de::Error::custom("Invalid number"))
            }
        }
        _ => Err(serde::de::Error::custom("Expected a number")),
    }
}

fn option_float_or_int_to_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;

    if value.is_null() {
        return Ok(None);
    }

    match value {
        serde_json::Value::Number(num) => {
            if let Some(i) = num.as_u64() {
                Ok(Some(i))
            } else if let Some(f) = num.as_f64() {
                Ok(Some(f as u64))
            } else {
                Err(serde::de::Error::custom("Invalid number"))
            }
        }
        _ => Err(serde::de::Error::custom("Expected a number")),
    }
}

fn deserialize_empty_map<'de, D>(deserializer: D) -> Result<Extensible, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let map = Extensible::deserialize(deserializer)?;

    if map.is_empty() {
        Ok(map)
    } else {
        Err(serde::de::Error::custom("expected empty object"))
    }
}

pub type Extensible = HashMap<String, serde_json::Value>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    #[serde(rename = "id")]
    pub id: u64,
    #[serde(flatten)]
    pub command_data: CommandData,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandData {
    BrowserCommand(BrowserCommand),
    BrowsingContextCommand(BrowsingContextCommand),
    EmulationCommand(EmulationCommand),
    InputCommand(InputCommand),
    NetworkCommand(NetworkCommand),
    ScriptCommand(ScriptCommand),
    SessionCommand(SessionCommand),
    StorageCommand(StorageCommand),
    WebExtensionCommand(WebExtensionCommand),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyParams {
    #[serde(flatten, deserialize_with = "deserialize_empty_map")]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    ErrorResponse(ErrorResponse),
    CommandResponse(CommandResponse),
    Event(Event),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResponse {
    #[serde(rename = "type")]
    pub r#type: SuccessEnum,
    #[serde(rename = "id")]
    #[serde(deserialize_with = "float_or_int_to_u64")]
    pub id: u64,
    #[serde(rename = "result")]
    pub result: ResultData,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "type")]
    pub r#type: ErrorEnum,
    #[serde(rename = "id")]
    #[serde(deserialize_with = "option_float_or_int_to_u64")]
    pub id: Option<u64>,
    #[serde(rename = "error")]
    pub error: ErrorCode,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "stacktrace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacktrace: Option<String>,
    #[serde(flatten)]
    pub extensible: Extensible,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error[{}]: {} (ID: {}){}",
            self.error,
            self.message,
            self.id.map_or("None".to_string(), |id| id.to_string()),
            self.stacktrace.as_ref().map_or("".to_string(), |st| format!("\nStacktrace:\n{}", st))
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResultData {
    EmptyResult(EmptyResult),
    BrowserResult(BrowserResult),
    BrowsingContextResult(BrowsingContextResult),
    EmulationResult(EmulationResult),
    InputResult(InputResult),
    NetworkResult(NetworkResult),
    ScriptResult(ScriptResult),
    SessionResult(SessionResult),
    StorageResult(StorageResult),
    WebExtensionResult(WebExtensionResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResult {
    #[serde(flatten, deserialize_with = "deserialize_empty_map")]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub r#type: EventEnum,
    #[serde(flatten)]
    pub event_data: EventData,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventData {
    BrowsingContextEvent(BrowsingContextEvent),
    InputEvent(InputEvent),
    LogEvent(LogEvent),
    NetworkEvent(NetworkEvent),
    ScriptEvent(ScriptEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorCode {
    #[serde(rename = "invalid argument")]
    InvalidArgument,
    #[serde(rename = "invalid selector")]
    InvalidSelector,
    #[serde(rename = "invalid session id")]
    InvalidSessionId,
    #[serde(rename = "invalid web extension")]
    InvalidWebExtension,
    #[serde(rename = "move target out of bounds")]
    MoveTargetOutOfBounds,
    #[serde(rename = "no such alert")]
    NoSuchAlert,
    #[serde(rename = "no such network collector")]
    NoSuchNetworkCollector,
    #[serde(rename = "no such element")]
    NoSuchElement,
    #[serde(rename = "no such frame")]
    NoSuchFrame,
    #[serde(rename = "no such handle")]
    NoSuchHandle,
    #[serde(rename = "no such history entry")]
    NoSuchHistoryEntry,
    #[serde(rename = "no such intercept")]
    NoSuchIntercept,
    #[serde(rename = "no such network data")]
    NoSuchNetworkData,
    #[serde(rename = "no such node")]
    NoSuchNode,
    #[serde(rename = "no such request")]
    NoSuchRequest,
    #[serde(rename = "no such script")]
    NoSuchScript,
    #[serde(rename = "no such storage partition")]
    NoSuchStoragePartition,
    #[serde(rename = "no such user context")]
    NoSuchUserContext,
    #[serde(rename = "no such web extension")]
    NoSuchWebExtension,
    #[serde(rename = "session not created")]
    SessionNotCreated,
    #[serde(rename = "unable to capture screen")]
    UnableToCaptureScreen,
    #[serde(rename = "unable to close drivers")]
    UnableToCloseBrowser,
    #[serde(rename = "unable to set cookie")]
    UnableToSetCookie,
    #[serde(rename = "unable to set file input")]
    UnableToSetFileInput,
    #[serde(rename = "unavailable network data")]
    UnavailableNetworkData,
    #[serde(rename = "underspecified storage partition")]
    UnderspecifiedStoragePartition,
    #[serde(rename = "unknown command")]
    UnknownCommand,
    #[serde(rename = "unknown error")]
    UnknownError,
    #[serde(rename = "unsupported operation")]
    UnsupportedOperation,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::InvalidArgument => write!(f, "invalid argument"),
            ErrorCode::InvalidSelector => write!(f, "invalid selector"),
            ErrorCode::InvalidSessionId => write!(f, "invalid session id"),
            ErrorCode::InvalidWebExtension => write!(f, "invalid web extension"),
            ErrorCode::MoveTargetOutOfBounds => write!(f, "move target out of bounds"),
            ErrorCode::NoSuchAlert => write!(f, "no such alert"),
            ErrorCode::NoSuchNetworkCollector => write!(f, "no such network collector"),
            ErrorCode::NoSuchElement => write!(f, "no such element"),
            ErrorCode::NoSuchFrame => write!(f, "no such frame"),
            ErrorCode::NoSuchHandle => write!(f, "no such handle"),
            ErrorCode::NoSuchHistoryEntry => write!(f, "no such history entry"),
            ErrorCode::NoSuchIntercept => write!(f, "no such intercept"),
            ErrorCode::NoSuchNetworkData => write!(f, "no such network data"),
            ErrorCode::NoSuchNode => write!(f, "no such node"),
            ErrorCode::NoSuchRequest => write!(f, "no such request"),
            ErrorCode::NoSuchScript => write!(f, "no such script"),
            ErrorCode::NoSuchStoragePartition => write!(f, "no such storage partition"),
            ErrorCode::NoSuchUserContext => write!(f, "no such user context"),
            ErrorCode::NoSuchWebExtension => write!(f, "no such web extension"),
            ErrorCode::SessionNotCreated => write!(f, "session not created"),
            ErrorCode::UnableToCaptureScreen => write!(f, "unable to capture screen"),
            ErrorCode::UnableToCloseBrowser => write!(f, "unable to close drivers"),
            ErrorCode::UnableToSetCookie => write!(f, "unable to set cookie"),
            ErrorCode::UnableToSetFileInput => write!(f, "unable to set file input"),
            ErrorCode::UnavailableNetworkData => write!(f, "unavailable network data"),
            ErrorCode::UnderspecifiedStoragePartition => write!(f, "underspecified storage partition"),
            ErrorCode::UnknownCommand => write!(f, "unknown command"),
            ErrorCode::UnknownError => write!(f, "unknown error"),
            ErrorCode::UnsupportedOperation => write!(f, "unsupported operation"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SuccessEnum {
    #[serde(rename = "success")]
    Success,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorEnum {
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventEnum {
    #[serde(rename = "event")]
    Event,
}

