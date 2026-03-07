use super::types::*;
impl PlayerMessage {
    pub fn builder() -> PlayerMessageBuilder {
        <PlayerMessageBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PlayerMessageBuilder {
    level: Option<PlayerMessageLevel>,
    message: Option<String>,
}
impl PlayerMessageBuilder {
    pub fn level(mut self, level: impl Into<PlayerMessageLevel>) -> Self {
        self.level = Some(level.into());
        self
    }
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
    pub fn build(self) -> Result<PlayerMessage, String> {
        Ok(PlayerMessage {
            level: self
                .level
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(level)))?,
            message: self
                .message
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(message)))?,
        })
    }
}
impl PlayerProperty {
    pub fn builder() -> PlayerPropertyBuilder {
        <PlayerPropertyBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PlayerPropertyBuilder {
    name: Option<String>,
    value: Option<String>,
}
impl PlayerPropertyBuilder {
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<PlayerProperty, String> {
        Ok(PlayerProperty {
            name: self
                .name
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(name)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl PlayerEvent {
    pub fn builder() -> PlayerEventBuilder {
        <PlayerEventBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PlayerEventBuilder {
    timestamp: Option<Timestamp>,
    value: Option<String>,
}
impl PlayerEventBuilder {
    pub fn timestamp(mut self, timestamp: impl Into<Timestamp>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<PlayerEvent, String> {
        Ok(PlayerEvent {
            timestamp: self
                .timestamp
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(timestamp)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl PlayerErrorSourceLocation {
    pub fn builder() -> PlayerErrorSourceLocationBuilder {
        <PlayerErrorSourceLocationBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PlayerErrorSourceLocationBuilder {
    file: Option<String>,
    line: Option<i64>,
}
impl PlayerErrorSourceLocationBuilder {
    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }
    pub fn line(mut self, line: impl Into<i64>) -> Self {
        self.line = Some(line.into());
        self
    }
    pub fn build(self) -> Result<PlayerErrorSourceLocation, String> {
        Ok(PlayerErrorSourceLocation {
            file: self
                .file
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(file)))?,
            line: self
                .line
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(line)))?,
        })
    }
}
impl PlayerError {
    pub fn builder() -> PlayerErrorBuilder {
        <PlayerErrorBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PlayerErrorBuilder {
    error_type: Option<String>,
    code: Option<i64>,
    stack: Option<Vec<PlayerErrorSourceLocation>>,
    cause: Option<Vec<PlayerError>>,
    data: Option<serde_json::Value>,
}
impl PlayerErrorBuilder {
    pub fn error_type(mut self, error_type: impl Into<String>) -> Self {
        self.error_type = Some(error_type.into());
        self
    }
    pub fn code(mut self, code: impl Into<i64>) -> Self {
        self.code = Some(code.into());
        self
    }
    pub fn stack(mut self, stack: impl Into<PlayerErrorSourceLocation>) -> Self {
        let v = self.stack.get_or_insert(Vec::new());
        v.push(stack.into());
        self
    }
    pub fn stacks<I, S>(mut self, stacks: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<PlayerErrorSourceLocation>,
    {
        let v = self.stack.get_or_insert(Vec::new());
        for val in stacks {
            v.push(val.into());
        }
        self
    }
    pub fn cause(mut self, cause: impl Into<PlayerError>) -> Self {
        let v = self.cause.get_or_insert(Vec::new());
        v.push(cause.into());
        self
    }
    pub fn causes<I, S>(mut self, causes: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<PlayerError>,
    {
        let v = self.cause.get_or_insert(Vec::new());
        for val in causes {
            v.push(val.into());
        }
        self
    }
    pub fn data(mut self, data: impl Into<serde_json::Value>) -> Self {
        self.data = Some(data.into());
        self
    }
    pub fn build(self) -> Result<PlayerError, String> {
        Ok(PlayerError {
            error_type: self
                .error_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(error_type)))?,
            code: self
                .code
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(code)))?,
            stack: self
                .stack
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(stack)))?,
            cause: self
                .cause
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(cause)))?,
            data: self
                .data
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(data)))?,
        })
    }
}
impl Player {
    pub fn builder() -> PlayerBuilder {
        <PlayerBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PlayerBuilder {
    player_id: Option<PlayerId>,
    dom_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
}
impl PlayerBuilder {
    pub fn player_id(mut self, player_id: impl Into<PlayerId>) -> Self {
        self.player_id = Some(player_id.into());
        self
    }
    pub fn dom_node_id(
        mut self,
        dom_node_id: impl Into<crate::browser_protocol::dom::types::BackendNodeId>,
    ) -> Self {
        self.dom_node_id = Some(dom_node_id.into());
        self
    }
    pub fn build(self) -> Result<Player, String> {
        Ok(Player {
            player_id: self
                .player_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(player_id)))?,
            dom_node_id: self.dom_node_id,
        })
    }
}
