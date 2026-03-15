use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct ClientWindow(String);
impl ClientWindow {
    pub fn new(val: impl Into<String>) -> Self {
        ClientWindow(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for ClientWindow {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<ClientWindow> for String {
    fn from(el: ClientWindow) -> String {
        el.0
    }
}
impl From<String> for ClientWindow {
    fn from(expr: String) -> Self {
        ClientWindow(expr)
    }
}
impl ClientWindow {
    pub const IDENTIFIER: &'static str = "browser.ClientWindow";
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientWindowInfo {
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "clientWindow")]
    pub client_window: ClientWindow,
    #[serde(rename = "height")]
    pub height: u64,
    #[serde(rename = "state")]
    pub state: ClientWindowInfoState,
    #[serde(rename = "width")]
    pub width: u64,
    #[serde(rename = "x")]
    pub x: i64,
    #[serde(rename = "y")]
    pub y: i64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClientWindowInfoState {
    #[serde(rename = "fullscreen")]
    Fullscreen,
    #[serde(rename = "maximized")]
    Maximized,
    #[serde(rename = "minimized")]
    Minimized,
    #[serde(rename = "normal")]
    Normal,
}
impl ClientWindowInfo {
    pub const IDENTIFIER: &'static str = "browser.ClientWindowInfo";
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct UserContext(String);
impl UserContext {
    pub fn new(val: impl Into<String>) -> Self {
        UserContext(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for UserContext {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<UserContext> for String {
    fn from(el: UserContext) -> String {
        el.0
    }
}
impl From<String> for UserContext {
    fn from(expr: String) -> Self {
        UserContext(expr)
    }
}
impl UserContext {
    pub const IDENTIFIER: &'static str = "browser.UserContext";
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserContextInfo {
    #[serde(rename = "userContext")]
    pub user_context: UserContext,
}
impl UserContextInfo {
    pub fn new(user_context: impl Into<UserContext>) -> Self {
        Self {
            user_context: user_context.into(),
        }
    }
}
impl UserContextInfo {
    pub const IDENTIFIER: &'static str = "browser.UserContextInfo";
    pub const DOMAIN_DIRECTION: &'static str = "all";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientWindowNamedStateClientWindowRectStateUnion {
    ClientWindowNamedState(ClientWindowNamedState),
    ClientWindowRectState(ClientWindowRectState),
}
impl From<ClientWindowNamedState> for ClientWindowNamedStateClientWindowRectStateUnion {
    fn from(v: ClientWindowNamedState) -> Self {
        ClientWindowNamedStateClientWindowRectStateUnion::ClientWindowNamedState(v)
    }
}
impl TryFrom<ClientWindowNamedStateClientWindowRectStateUnion> for ClientWindowNamedState {
    type Error = ClientWindowNamedStateClientWindowRectStateUnion;
    fn try_from(e: ClientWindowNamedStateClientWindowRectStateUnion) -> Result<Self, Self::Error> {
        match e {
            ClientWindowNamedStateClientWindowRectStateUnion::ClientWindowNamedState(inner) => {
                Ok(inner)
            }
            other => Err(other),
        }
    }
}
impl From<ClientWindowRectState> for ClientWindowNamedStateClientWindowRectStateUnion {
    fn from(v: ClientWindowRectState) -> Self {
        ClientWindowNamedStateClientWindowRectStateUnion::ClientWindowRectState(v)
    }
}
impl TryFrom<ClientWindowNamedStateClientWindowRectStateUnion> for ClientWindowRectState {
    type Error = ClientWindowNamedStateClientWindowRectStateUnion;
    fn try_from(e: ClientWindowNamedStateClientWindowRectStateUnion) -> Result<Self, Self::Error> {
        match e {
            ClientWindowNamedStateClientWindowRectStateUnion::ClientWindowRectState(inner) => {
                Ok(inner)
            }
            other => Err(other),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientWindowNamedState {
    #[serde(rename = "state")]
    pub state: ClientWindowNamedStateState,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClientWindowNamedStateState {
    #[serde(rename = "fullscreen")]
    Fullscreen,
    #[serde(rename = "maximized")]
    Maximized,
    #[serde(rename = "minimized")]
    Minimized,
}
impl ClientWindowNamedState {
    pub fn new(state: impl Into<ClientWindowNamedStateState>) -> Self {
        Self {
            state: state.into(),
        }
    }
}
impl ClientWindowNamedState {
    pub const IDENTIFIER: &'static str = "browser.ClientWindowNamedState";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClientWindowRectState {
    #[serde(rename = "state")]
    pub state: ClientWindowRectStateState,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub width: Option<u64>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub height: Option<u64>,
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub x: Option<i64>,
    #[serde(rename = "y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub y: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClientWindowRectStateState {
    #[serde(rename = "normal")]
    Normal,
}
impl ClientWindowRectState {
    pub fn new(state: impl Into<ClientWindowRectStateState>) -> Self {
        Self {
            state: state.into(),
            width: None,
            height: None,
            x: None,
            y: None,
        }
    }
}
impl ClientWindowRectState {
    pub const IDENTIFIER: &'static str = "browser.ClientWindowRectState";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadBehavior {
    DownloadBehaviorAllowed(DownloadBehaviorAllowed),
    DownloadBehaviorDenied(DownloadBehaviorDenied),
}
impl From<DownloadBehaviorAllowed> for DownloadBehavior {
    fn from(v: DownloadBehaviorAllowed) -> Self {
        DownloadBehavior::DownloadBehaviorAllowed(v)
    }
}
impl TryFrom<DownloadBehavior> for DownloadBehaviorAllowed {
    type Error = DownloadBehavior;
    fn try_from(e: DownloadBehavior) -> Result<Self, Self::Error> {
        match e {
            DownloadBehavior::DownloadBehaviorAllowed(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<DownloadBehaviorDenied> for DownloadBehavior {
    fn from(v: DownloadBehaviorDenied) -> Self {
        DownloadBehavior::DownloadBehaviorDenied(v)
    }
}
impl TryFrom<DownloadBehavior> for DownloadBehaviorDenied {
    type Error = DownloadBehavior;
    fn try_from(e: DownloadBehavior) -> Result<Self, Self::Error> {
        match e {
            DownloadBehavior::DownloadBehaviorDenied(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadBehaviorAllowed {
    #[serde(rename = "type")]
    pub r#type: DownloadBehaviorAllowedType,
    #[serde(rename = "destinationFolder")]
    pub destination_folder: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DownloadBehaviorAllowedType {
    #[serde(rename = "allowed")]
    Allowed,
}
impl DownloadBehaviorAllowed {
    pub fn new(
        r#type: impl Into<DownloadBehaviorAllowedType>,
        destination_folder: impl Into<String>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            destination_folder: destination_folder.into(),
        }
    }
}
impl DownloadBehaviorAllowed {
    pub const IDENTIFIER: &'static str = "browser.DownloadBehaviorAllowed";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownloadBehaviorDenied {
    #[serde(rename = "type")]
    pub r#type: DownloadBehaviorDeniedType,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DownloadBehaviorDeniedType {
    #[serde(rename = "denied")]
    Denied,
}
impl DownloadBehaviorDenied {
    pub fn new(r#type: impl Into<DownloadBehaviorDeniedType>) -> Self {
        Self {
            r#type: r#type.into(),
        }
    }
}
impl DownloadBehaviorDenied {
    pub const IDENTIFIER: &'static str = "browser.DownloadBehaviorDenied";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
