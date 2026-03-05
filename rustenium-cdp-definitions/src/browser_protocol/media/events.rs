use serde::{Deserialize, Serialize};
#[doc = "This can be called multiple times, and can be used to set / override /\nremove player properties. A null propValue indicates removal.\n[playerPropertiesChanged](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerPropertiesChanged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerPropertiesChanged {
    #[serde(rename = "playerId")]
    pub player_id: super::types::PlayerId,
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<super::types::PlayerProperty>,
}
impl PlayerPropertiesChanged {
    pub const IDENTIFIER: &'static str = "Media.playerPropertiesChanged";
}
#[doc = "Send events as a list, allowing them to be batched on the browser for less\ncongestion. If batched, events must ALWAYS be in chronological order.\n[playerEventsAdded](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerEventsAdded)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerEventsAdded {
    #[serde(rename = "playerId")]
    pub player_id: super::types::PlayerId,
    #[serde(rename = "events")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<super::types::PlayerEvent>,
}
impl PlayerEventsAdded {
    pub const IDENTIFIER: &'static str = "Media.playerEventsAdded";
}
#[doc = "Send a list of any messages that need to be delivered.\n[playerMessagesLogged](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerMessagesLogged)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerMessagesLogged {
    #[serde(rename = "playerId")]
    pub player_id: super::types::PlayerId,
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<super::types::PlayerMessage>,
}
impl PlayerMessagesLogged {
    pub const IDENTIFIER: &'static str = "Media.playerMessagesLogged";
}
#[doc = "Send a list of any errors that need to be delivered.\n[playerErrorsRaised](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerErrorsRaised)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerErrorsRaised {
    #[serde(rename = "playerId")]
    pub player_id: super::types::PlayerId,
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<super::types::PlayerError>,
}
impl PlayerErrorsRaised {
    pub const IDENTIFIER: &'static str = "Media.playerErrorsRaised";
}
#[doc = "Called whenever a player is created, or when a new agent joins and receives\na list of active players. If an agent is restored, it will receive one\nevent for each active player.\n[playerCreated](https://chromedevtools.github.io/devtools-protocol/tot/Media/#event-playerCreated)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerCreated {
    #[serde(rename = "player")]
    pub player: super::types::Player,
}
impl PlayerCreated {
    pub const IDENTIFIER: &'static str = "Media.playerCreated";
}
group_enum ! (MediaEvents { PlayerPropertiesChanged (PlayerPropertiesChanged) , PlayerEventsAdded (PlayerEventsAdded) , PlayerMessagesLogged (PlayerMessagesLogged) , PlayerErrorsRaised (PlayerErrorsRaised) , PlayerCreated (PlayerCreated) });
