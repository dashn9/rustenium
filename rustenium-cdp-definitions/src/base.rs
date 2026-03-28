use serde::{Deserialize, Serialize};

use crate::Command;

#[derive(Debug, Clone, Serialize)]
pub struct CommandMessage {
    #[serde(rename = "id")]
    pub id: u64,
    #[serde(flatten)]
    pub command_data: Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResponse {
    pub id: u64,
    pub result: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    pub code: i64,
    pub message: String,
}

impl std::fmt::Display for ErrorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CDP Error[{}]: {}", self.code, self.message)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<u64>,
    pub error: ErrorInfo,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.id {
            Some(id) => write!(f, "Error for command {}: {}", id, self.error),
            None => write!(f, "Error: {}", self.error),
        }
    }
}

/// CDP events arrive as `{"method": "Domain.event", "params": {...}}` with no
/// outer envelope distinguishing them from commands/errors (unlike BiDi which
/// wraps events in a typed structure). `EventResponse` captures this raw shape
/// so we can route by method name and then deserialize into concrete event
/// types on demand via `TryInto`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventResponse {
    pub method: String,
    pub params: serde_json::Value,
}

impl EventResponse {
    /// Attempt to convert this raw event into a concrete CDP event type.
    ///
    /// The concrete type (e.g. `TargetCreated`) is expected to deserialize from
    /// the full `{"method": "...", "params": {...}}` shape — not just the params.
    pub fn try_into_event<T: serde::de::DeserializeOwned>(self) -> Result<T, serde_json::Error> {
        let value = serde_json::json!({
            "method": self.method,
            "params": self.params,
        });
        serde_json::from_value(value)
    }

    pub fn identifier(&self) -> &str {
        &self.method
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Message {
    ErrorResponse(ErrorResponse),
    CommandResponse(CommandResponse),
    Event(EventResponse),
}
