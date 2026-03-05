use serde::{Deserialize, Serialize};
#[doc = "Players will get an ID that is unique within the agent context.\n[PlayerId](https://chromedevtools.github.io/devtools-protocol/tot/Media/#type-PlayerId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct PlayerId(String);
impl PlayerId {
    pub fn new(val: impl Into<String>) -> Self {
        PlayerId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for PlayerId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<PlayerId> for String {
    fn from(el: PlayerId) -> String {
        el.0
    }
}
impl From<String> for PlayerId {
    fn from(expr: String) -> Self {
        PlayerId(expr)
    }
}
impl std::borrow::Borrow<str> for PlayerId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl PlayerId {
    pub const IDENTIFIER: &'static str = "Media.PlayerId";
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Timestamp(f64);
impl Timestamp {
    pub fn new(val: impl Into<f64>) -> Self {
        Timestamp(val.into())
    }
    pub fn inner(&self) -> &f64 {
        &self.0
    }
}
impl Timestamp {
    pub const IDENTIFIER: &'static str = "Media.Timestamp";
}
#[doc = "Have one type per entry in MediaLogRecord::Type\nCorresponds to kMessage\n[PlayerMessage](https://chromedevtools.github.io/devtools-protocol/tot/Media/#type-PlayerMessage)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerMessage {
    #[doc = "Keep in sync with MediaLogMessageLevel\nWe are currently keeping the message level 'error' separate from the\nPlayerError type because right now they represent different things,\nthis one being a DVLOG(ERROR) style log message that gets printed\nbased on what log level is selected in the UI, and the other is a\nrepresentation of a media::PipelineStatus object. Soon however we're\ngoing to be moving away from using PipelineStatus for errors and\nintroducing a new error type which should hopefully let us integrate\nthe error log level into the PlayerError type."]
    #[serde(rename = "level")]
    pub level: PlayerMessageLevel,
    #[serde(rename = "message")]
    pub message: String,
}
#[doc = "Keep in sync with MediaLogMessageLevel\nWe are currently keeping the message level 'error' separate from the\nPlayerError type because right now they represent different things,\nthis one being a DVLOG(ERROR) style log message that gets printed\nbased on what log level is selected in the UI, and the other is a\nrepresentation of a media::PipelineStatus object. Soon however we're\ngoing to be moving away from using PipelineStatus for errors and\nintroducing a new error type which should hopefully let us integrate\nthe error log level into the PlayerError type."]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerMessageLevel {
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
}
impl PlayerMessage {
    pub fn new(level: impl Into<PlayerMessageLevel>, message: impl Into<String>) -> Self {
        Self {
            level: level.into(),
            message: message.into(),
        }
    }
}
impl PlayerMessage {
    pub const IDENTIFIER: &'static str = "Media.PlayerMessage";
}
#[doc = "Corresponds to kMediaPropertyChange\n[PlayerProperty](https://chromedevtools.github.io/devtools-protocol/tot/Media/#type-PlayerProperty)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerProperty {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl PlayerProperty {
    pub fn new(name: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
impl PlayerProperty {
    pub const IDENTIFIER: &'static str = "Media.PlayerProperty";
}
#[doc = "Corresponds to kMediaEventTriggered\n[PlayerEvent](https://chromedevtools.github.io/devtools-protocol/tot/Media/#type-PlayerEvent)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerEvent {
    #[serde(rename = "timestamp")]
    pub timestamp: Timestamp,
    #[serde(rename = "value")]
    pub value: String,
}
impl PlayerEvent {
    pub fn new(timestamp: impl Into<Timestamp>, value: impl Into<String>) -> Self {
        Self {
            timestamp: timestamp.into(),
            value: value.into(),
        }
    }
}
impl PlayerEvent {
    pub const IDENTIFIER: &'static str = "Media.PlayerEvent";
}
#[doc = "Represents logged source line numbers reported in an error.\nNOTE: file and line are from chromium c++ implementation code, not js.\n[PlayerErrorSourceLocation](https://chromedevtools.github.io/devtools-protocol/tot/Media/#type-PlayerErrorSourceLocation)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerErrorSourceLocation {
    #[serde(rename = "file")]
    pub file: String,
    #[serde(rename = "line")]
    pub line: i64,
}
impl PlayerErrorSourceLocation {
    pub fn new(file: impl Into<String>, line: impl Into<i64>) -> Self {
        Self {
            file: file.into(),
            line: line.into(),
        }
    }
}
impl PlayerErrorSourceLocation {
    pub const IDENTIFIER: &'static str = "Media.PlayerErrorSourceLocation";
}
#[doc = "Corresponds to kMediaError\n[PlayerError](https://chromedevtools.github.io/devtools-protocol/tot/Media/#type-PlayerError)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerError {
    #[serde(rename = "errorType")]
    pub error_type: String,
    #[doc = "Code is the numeric enum entry for a specific set of error codes, such\nas PipelineStatusCodes in media/base/pipeline_status.h"]
    #[serde(rename = "code")]
    pub code: i64,
    #[doc = "A trace of where this error was caused / where it passed through."]
    #[serde(rename = "stack")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub stack: Vec<PlayerErrorSourceLocation>,
    #[doc = "Errors potentially have a root cause error, ie, a DecoderError might be\ncaused by an WindowsError"]
    #[serde(rename = "cause")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cause: Vec<PlayerError>,
    #[doc = "Extra data attached to an error, such as an HRESULT, Video Codec, etc."]
    #[serde(rename = "data")]
    pub data: serde_json::Value,
}
impl PlayerError {
    pub const IDENTIFIER: &'static str = "Media.PlayerError";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "playerId")]
    pub player_id: PlayerId,
    #[serde(rename = "domNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub dom_node_id: Option<super::super::dom::types::BackendNodeId>,
}
impl Player {
    pub fn new(player_id: impl Into<PlayerId>) -> Self {
        Self {
            player_id: player_id.into(),
            dom_node_id: None,
        }
    }
}
impl Player {
    pub const IDENTIFIER: &'static str = "Media.Player";
}
group_enum ! (MediaTypes { PlayerId (PlayerId) , Timestamp (Timestamp) , PlayerMessage (PlayerMessage) , PlayerProperty (PlayerProperty) , PlayerEvent (PlayerEvent) , PlayerErrorSourceLocation (PlayerErrorSourceLocation) , PlayerError (PlayerError) , Player (Player) });
