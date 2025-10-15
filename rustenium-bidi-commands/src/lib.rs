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
pub use input::commands::InputCommand;
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
    #[serde(rename = "EmptyParams")]
    pub empty_params: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    CommandResponse(CommandResponse),
    ErrorResponse(ErrorResponse),
    Event(Event),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResponse {
    #[serde(rename = "type")]
    pub r#type: SuccessEnum,
    #[serde(rename = "id")]
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
    pub id: Option<u64>,
    #[serde(rename = "error")]
    pub error: ErrorCode,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "stacktrace")]
    pub stacktrace: Option<String>,
    #[serde(flatten)]
    pub extensible: Extensible,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResultData {
    BrowserResult(BrowserResult),
    BrowsingContextResult(BrowsingContextResult),
    EmptyResult(EmptyResult),
    NetworkResult(NetworkResult),
    ScriptResult(ScriptResult),
    SessionResult(SessionResult),
    StorageResult(StorageResult),
    WebExtensionResult(WebExtensionResult),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResult {
    #[serde(rename = "EmptyResult")]
    pub empty_result: Extensible,
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
#[serde(untagged)]
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
    #[serde(rename = "unable to close browser")]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SuccessEnum {
    #[serde(rename = "success")]
    Success,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ErrorEnum {
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventEnum {
    #[serde(rename = "event")]
    Event,
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