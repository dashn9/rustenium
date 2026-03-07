use serde::{Deserialize, Serialize};
#[doc = "This can be called multiple times, and can be used to set / override /\nremove player properties. A null propValue indicates removal.\n[playerPropertiesChanged](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerPropertiesChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerPropertiesChangedParams {
    #[serde(rename = "playerId")]
    pub player_id: super::types::PlayerId,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<super::types::PlayerProperty>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerPropertiesChangedMethod {
    #[serde(rename = "Media.playerPropertiesChanged")]
    PlayerPropertiesChanged,
}
impl PlayerPropertiesChangedMethod {
    pub const IDENTIFIER: &'static str = "Media.playerPropertiesChanged";
}
#[doc = "This can be called multiple times, and can be used to set / override /\nremove player properties. A null propValue indicates removal.\n[playerPropertiesChanged](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerPropertiesChanged)"]
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerPropertiesChanged {
    pub method: PlayerPropertiesChangedMethod,
    pub params: PlayerPropertiesChangedParams,
}
#[doc = "Send events as a list, allowing them to be batched on the browser for less\ncongestion. If batched, events must ALWAYS be in chronological order.\n[playerEventsAdded](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerEventsAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerEventsAddedParams {
    #[serde(rename = "playerId")]
    pub player_id: super::types::PlayerId,
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<super::types::PlayerEvent>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerEventsAddedMethod {
    #[serde(rename = "Media.playerEventsAdded")]
    PlayerEventsAdded,
}
impl PlayerEventsAddedMethod {
    pub const IDENTIFIER: &'static str = "Media.playerEventsAdded";
}
#[doc = "Send events as a list, allowing them to be batched on the browser for less\ncongestion. If batched, events must ALWAYS be in chronological order.\n[playerEventsAdded](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerEventsAdded)"]
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerEventsAdded {
    pub method: PlayerEventsAddedMethod,
    pub params: PlayerEventsAddedParams,
}
#[doc = "Send a list of any messages that need to be delivered.\n[playerMessagesLogged](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerMessagesLogged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerMessagesLoggedParams {
    #[serde(rename = "playerId")]
    pub player_id: super::types::PlayerId,
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<super::types::PlayerMessage>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerMessagesLoggedMethod {
    #[serde(rename = "Media.playerMessagesLogged")]
    PlayerMessagesLogged,
}
impl PlayerMessagesLoggedMethod {
    pub const IDENTIFIER: &'static str = "Media.playerMessagesLogged";
}
#[doc = "Send a list of any messages that need to be delivered.\n[playerMessagesLogged](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerMessagesLogged)"]
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerMessagesLogged {
    pub method: PlayerMessagesLoggedMethod,
    pub params: PlayerMessagesLoggedParams,
}
#[doc = "Send a list of any errors that need to be delivered.\n[playerErrorsRaised](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerErrorsRaised)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerErrorsRaisedParams {
    #[serde(rename = "playerId")]
    pub player_id: super::types::PlayerId,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<super::types::PlayerError>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerErrorsRaisedMethod {
    #[serde(rename = "Media.playerErrorsRaised")]
    PlayerErrorsRaised,
}
impl PlayerErrorsRaisedMethod {
    pub const IDENTIFIER: &'static str = "Media.playerErrorsRaised";
}
#[doc = "Send a list of any errors that need to be delivered.\n[playerErrorsRaised](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerErrorsRaised)"]
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerErrorsRaised {
    pub method: PlayerErrorsRaisedMethod,
    pub params: PlayerErrorsRaisedParams,
}
#[doc = "Called whenever a player is created, or when a new agent joins and receives\na list of active players. If an agent is restored, it will receive one\nevent for each active player.\n[playerCreated](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerCreatedParams {
    #[serde(rename = "player")]
    pub player: super::types::Player,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlayerCreatedMethod {
    #[serde(rename = "Media.playerCreated")]
    PlayerCreated,
}
impl PlayerCreatedMethod {
    pub const IDENTIFIER: &'static str = "Media.playerCreated";
}
#[doc = "Called whenever a player is created, or when a new agent joins and receives\na list of active players. If an agent is restored, it will receive one\nevent for each active player.\n[playerCreated](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerCreated)"]
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerCreated {
    pub method: PlayerCreatedMethod,
    pub params: PlayerCreatedParams,
}
group_enum ! (MediaEvents { PlayerPropertiesChanged (PlayerPropertiesChanged) , PlayerEventsAdded (PlayerEventsAdded) , PlayerMessagesLogged (PlayerMessagesLogged) , PlayerErrorsRaised (PlayerErrorsRaised) , PlayerCreated (PlayerCreated) });
