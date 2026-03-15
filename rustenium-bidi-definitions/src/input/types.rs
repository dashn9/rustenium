use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElementOrigin {
    #[serde(rename = "type")]
    pub r#type: ElementOriginType,
    #[serde(rename = "element")]
    pub element: crate::script::types::SharedReference,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementOriginType {
    #[serde(rename = "element")]
    Element,
}
impl ElementOrigin {
    pub fn new(
        r#type: impl Into<ElementOriginType>,
        element: impl Into<crate::script::types::SharedReference>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            element: element.into(),
        }
    }
}
impl ElementOrigin {
    pub const IDENTIFIER: &'static str = "input.ElementOrigin";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceActions {
    NoneSourceActions(NoneSourceActions),
    KeySourceActions(KeySourceActions),
    PointerSourceActions(PointerSourceActions),
    WheelSourceActions(WheelSourceActions),
}
impl From<NoneSourceActions> for SourceActions {
    fn from(v: NoneSourceActions) -> Self {
        SourceActions::NoneSourceActions(v)
    }
}
impl TryFrom<SourceActions> for NoneSourceActions {
    type Error = SourceActions;
    fn try_from(e: SourceActions) -> Result<Self, Self::Error> {
        match e {
            SourceActions::NoneSourceActions(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<KeySourceActions> for SourceActions {
    fn from(v: KeySourceActions) -> Self {
        SourceActions::KeySourceActions(v)
    }
}
impl TryFrom<SourceActions> for KeySourceActions {
    type Error = SourceActions;
    fn try_from(e: SourceActions) -> Result<Self, Self::Error> {
        match e {
            SourceActions::KeySourceActions(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<PointerSourceActions> for SourceActions {
    fn from(v: PointerSourceActions) -> Self {
        SourceActions::PointerSourceActions(v)
    }
}
impl TryFrom<SourceActions> for PointerSourceActions {
    type Error = SourceActions;
    fn try_from(e: SourceActions) -> Result<Self, Self::Error> {
        match e {
            SourceActions::PointerSourceActions(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<WheelSourceActions> for SourceActions {
    fn from(v: WheelSourceActions) -> Self {
        SourceActions::WheelSourceActions(v)
    }
}
impl TryFrom<SourceActions> for WheelSourceActions {
    type Error = SourceActions;
    fn try_from(e: SourceActions) -> Result<Self, Self::Error> {
        match e {
            SourceActions::WheelSourceActions(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoneSourceActions {
    #[serde(rename = "type")]
    pub r#type: NoneSourceActionsType,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<NoneSourceAction>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NoneSourceActionsType {
    #[serde(rename = "none")]
    None,
}
impl NoneSourceActions {
    pub fn new(
        r#type: impl Into<NoneSourceActionsType>,
        id: impl Into<String>,
        actions: Vec<NoneSourceAction>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            id: id.into(),
            actions,
        }
    }
}
impl NoneSourceActions {
    pub const IDENTIFIER: &'static str = "input.NoneSourceActions";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoneSourceAction(PauseAction);
impl NoneSourceAction {
    pub fn new(val: impl Into<PauseAction>) -> Self {
        NoneSourceAction(val.into())
    }
    pub fn inner(&self) -> &PauseAction {
        &self.0
    }
}
impl NoneSourceAction {
    pub const IDENTIFIER: &'static str = "input.NoneSourceAction";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeySourceActions {
    #[serde(rename = "type")]
    pub r#type: KeySourceActionsType,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<KeySourceAction>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeySourceActionsType {
    #[serde(rename = "key")]
    Key,
}
impl KeySourceActions {
    pub fn new(
        r#type: impl Into<KeySourceActionsType>,
        id: impl Into<String>,
        actions: Vec<KeySourceAction>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            id: id.into(),
            actions,
        }
    }
}
impl KeySourceActions {
    pub const IDENTIFIER: &'static str = "input.KeySourceActions";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeySourceAction {
    PauseAction(PauseAction),
    KeyDownAction(KeyDownAction),
    KeyUpAction(KeyUpAction),
}
impl From<PauseAction> for KeySourceAction {
    fn from(v: PauseAction) -> Self {
        KeySourceAction::PauseAction(v)
    }
}
impl TryFrom<KeySourceAction> for PauseAction {
    type Error = KeySourceAction;
    fn try_from(e: KeySourceAction) -> Result<Self, Self::Error> {
        match e {
            KeySourceAction::PauseAction(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<KeyDownAction> for KeySourceAction {
    fn from(v: KeyDownAction) -> Self {
        KeySourceAction::KeyDownAction(v)
    }
}
impl TryFrom<KeySourceAction> for KeyDownAction {
    type Error = KeySourceAction;
    fn try_from(e: KeySourceAction) -> Result<Self, Self::Error> {
        match e {
            KeySourceAction::KeyDownAction(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<KeyUpAction> for KeySourceAction {
    fn from(v: KeyUpAction) -> Self {
        KeySourceAction::KeyUpAction(v)
    }
}
impl TryFrom<KeySourceAction> for KeyUpAction {
    type Error = KeySourceAction;
    fn try_from(e: KeySourceAction) -> Result<Self, Self::Error> {
        match e {
            KeySourceAction::KeyUpAction(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointerSourceActions {
    #[serde(rename = "type")]
    pub r#type: PointerSourceActionsType,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parameters: Option<PointerParameters>,
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<PointerSourceAction>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PointerSourceActionsType {
    #[serde(rename = "pointer")]
    Pointer,
}
impl PointerSourceActions {
    pub fn new(
        r#type: impl Into<PointerSourceActionsType>,
        id: impl Into<String>,
        actions: Vec<PointerSourceAction>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            id: id.into(),
            actions,
            parameters: None,
        }
    }
}
impl PointerSourceActions {
    pub const IDENTIFIER: &'static str = "input.PointerSourceActions";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PointerType {
    #[serde(rename = "mouse")]
    Mouse,
    #[serde(rename = "pen")]
    Pen,
    #[serde(rename = "touch")]
    Touch,
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PointerParameters {
    #[serde(rename = "pointerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub pointer_type: Option<PointerType>,
}
impl PointerParameters {
    pub const IDENTIFIER: &'static str = "input.PointerParameters";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointerSourceAction {
    PauseAction(PauseAction),
    PointerDownAction(PointerDownAction),
    PointerUpAction(PointerUpAction),
    PointerMoveAction(PointerMoveAction),
}
impl From<PauseAction> for PointerSourceAction {
    fn from(v: PauseAction) -> Self {
        PointerSourceAction::PauseAction(v)
    }
}
impl TryFrom<PointerSourceAction> for PauseAction {
    type Error = PointerSourceAction;
    fn try_from(e: PointerSourceAction) -> Result<Self, Self::Error> {
        match e {
            PointerSourceAction::PauseAction(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<PointerDownAction> for PointerSourceAction {
    fn from(v: PointerDownAction) -> Self {
        PointerSourceAction::PointerDownAction(v)
    }
}
impl TryFrom<PointerSourceAction> for PointerDownAction {
    type Error = PointerSourceAction;
    fn try_from(e: PointerSourceAction) -> Result<Self, Self::Error> {
        match e {
            PointerSourceAction::PointerDownAction(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<PointerUpAction> for PointerSourceAction {
    fn from(v: PointerUpAction) -> Self {
        PointerSourceAction::PointerUpAction(v)
    }
}
impl TryFrom<PointerSourceAction> for PointerUpAction {
    type Error = PointerSourceAction;
    fn try_from(e: PointerSourceAction) -> Result<Self, Self::Error> {
        match e {
            PointerSourceAction::PointerUpAction(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<PointerMoveAction> for PointerSourceAction {
    fn from(v: PointerMoveAction) -> Self {
        PointerSourceAction::PointerMoveAction(v)
    }
}
impl TryFrom<PointerSourceAction> for PointerMoveAction {
    type Error = PointerSourceAction;
    fn try_from(e: PointerSourceAction) -> Result<Self, Self::Error> {
        match e {
            PointerSourceAction::PointerMoveAction(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelSourceActions {
    #[serde(rename = "type")]
    pub r#type: WheelSourceActionsType,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<WheelSourceAction>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WheelSourceActionsType {
    #[serde(rename = "wheel")]
    Wheel,
}
impl WheelSourceActions {
    pub fn new(
        r#type: impl Into<WheelSourceActionsType>,
        id: impl Into<String>,
        actions: Vec<WheelSourceAction>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            id: id.into(),
            actions,
        }
    }
}
impl WheelSourceActions {
    pub const IDENTIFIER: &'static str = "input.WheelSourceActions";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WheelSourceAction {
    PauseAction(PauseAction),
    WheelScrollAction(WheelScrollAction),
}
impl From<PauseAction> for WheelSourceAction {
    fn from(v: PauseAction) -> Self {
        WheelSourceAction::PauseAction(v)
    }
}
impl TryFrom<WheelSourceAction> for PauseAction {
    type Error = WheelSourceAction;
    fn try_from(e: WheelSourceAction) -> Result<Self, Self::Error> {
        match e {
            WheelSourceAction::PauseAction(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
impl From<WheelScrollAction> for WheelSourceAction {
    fn from(v: WheelScrollAction) -> Self {
        WheelSourceAction::WheelScrollAction(v)
    }
}
impl TryFrom<WheelSourceAction> for WheelScrollAction {
    type Error = WheelSourceAction;
    fn try_from(e: WheelSourceAction) -> Result<Self, Self::Error> {
        match e {
            WheelSourceAction::WheelScrollAction(inner) => Ok(inner),
            other => Err(other),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PauseAction {
    #[serde(rename = "type")]
    pub r#type: PauseActionType,
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub duration: Option<u64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PauseActionType {
    #[serde(rename = "pause")]
    Pause,
}
impl PauseAction {
    pub fn new(r#type: impl Into<PauseActionType>) -> Self {
        Self {
            r#type: r#type.into(),
            duration: None,
        }
    }
}
impl PauseAction {
    pub const IDENTIFIER: &'static str = "input.PauseAction";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyDownAction {
    #[serde(rename = "type")]
    pub r#type: KeyDownActionType,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyDownActionType {
    #[serde(rename = "keyDown")]
    KeyDown,
}
impl KeyDownAction {
    pub fn new(r#type: impl Into<KeyDownActionType>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl KeyDownAction {
    pub const IDENTIFIER: &'static str = "input.KeyDownAction";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyUpAction {
    #[serde(rename = "type")]
    pub r#type: KeyUpActionType,
    #[serde(rename = "value")]
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyUpActionType {
    #[serde(rename = "keyUp")]
    KeyUp,
}
impl KeyUpAction {
    pub fn new(r#type: impl Into<KeyUpActionType>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl KeyUpAction {
    pub const IDENTIFIER: &'static str = "input.KeyUpAction";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointerUpAction {
    #[serde(rename = "type")]
    pub r#type: PointerUpActionType,
    #[serde(rename = "button")]
    pub button: u64,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PointerUpActionType {
    #[serde(rename = "pointerUp")]
    PointerUp,
}
impl PointerUpAction {
    pub fn new(r#type: impl Into<PointerUpActionType>, button: impl Into<u64>) -> Self {
        Self {
            r#type: r#type.into(),
            button: button.into(),
        }
    }
}
impl PointerUpAction {
    pub const IDENTIFIER: &'static str = "input.PointerUpAction";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointerDownAction {
    #[serde(rename = "type")]
    pub r#type: PointerDownActionType,
    #[serde(rename = "button")]
    pub button: u64,
    #[serde(flatten)]
    #[serde(default)]
    pub pointer_common_properties: PointerCommonProperties,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PointerDownActionType {
    #[serde(rename = "pointerDown")]
    PointerDown,
}
impl PointerDownAction {
    pub fn new(
        r#type: impl Into<PointerDownActionType>,
        button: impl Into<u64>,
        pointer_common_properties: impl Into<PointerCommonProperties>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            button: button.into(),
            pointer_common_properties: pointer_common_properties.into(),
        }
    }
}
impl PointerDownAction {
    pub const IDENTIFIER: &'static str = "input.PointerDownAction";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointerMoveAction {
    #[serde(rename = "type")]
    pub r#type: PointerMoveActionType,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "y")]
    pub y: f64,
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub duration: Option<u64>,
    #[serde(rename = "origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub origin: Option<Origin>,
    #[serde(flatten)]
    #[serde(default)]
    pub pointer_common_properties: PointerCommonProperties,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PointerMoveActionType {
    #[serde(rename = "pointerMove")]
    PointerMove,
}
impl PointerMoveAction {
    pub fn new(
        r#type: impl Into<PointerMoveActionType>,
        x: impl Into<f64>,
        y: impl Into<f64>,
        pointer_common_properties: impl Into<PointerCommonProperties>,
    ) -> Self {
        Self {
            r#type: r#type.into(),
            x: x.into(),
            y: y.into(),
            pointer_common_properties: pointer_common_properties.into(),
            duration: None,
            origin: None,
        }
    }
}
impl PointerMoveAction {
    pub const IDENTIFIER: &'static str = "input.PointerMoveAction";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelScrollAction {
    #[serde(rename = "type")]
    pub r#type: WheelScrollActionType,
    #[serde(rename = "x")]
    pub x: i64,
    #[serde(rename = "y")]
    pub y: i64,
    #[serde(rename = "deltaX")]
    pub delta_x: i64,
    #[serde(rename = "deltaY")]
    pub delta_y: i64,
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub duration: Option<u64>,
    #[serde(rename = "origin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub origin: Option<Origin>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WheelScrollActionType {
    #[serde(rename = "scroll")]
    Scroll,
}
impl WheelScrollAction {
    pub const IDENTIFIER: &'static str = "input.WheelScrollAction";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PointerCommonProperties {
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_common_properties_width")]
    pub width: Option<u64>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_common_properties_height")]
    pub height: Option<u64>,
    #[serde(rename = "pressure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_common_properties_pressure")]
    pub pressure: Option<f64>,
    #[serde(rename = "tangentialPressure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_common_properties_tangential_pressure")]
    pub tangential_pressure: Option<f64>,
    #[serde(rename = "twist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_common_properties_twist")]
    pub twist: Option<u64>,
    #[serde(rename = "altitudeAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_common_properties_altitude_angle")]
    pub altitude_angle: Option<f64>,
    #[serde(rename = "azimuthAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_common_properties_azimuth_angle")]
    pub azimuth_angle: Option<f64>,
}
fn default_pointer_common_properties_width() -> Option<u64> {
    Some(1u64)
}
fn default_pointer_common_properties_height() -> Option<u64> {
    Some(1u64)
}
fn default_pointer_common_properties_pressure() -> Option<f64> {
    Some(0f64)
}
fn default_pointer_common_properties_tangential_pressure() -> Option<f64> {
    Some(0f64)
}
fn default_pointer_common_properties_twist() -> Option<u64> {
    Some(0u64)
}
fn default_pointer_common_properties_altitude_angle() -> Option<f64> {
    Some(0f64)
}
fn default_pointer_common_properties_azimuth_angle() -> Option<f64> {
    Some(0f64)
}
impl PointerCommonProperties {
    pub const IDENTIFIER: &'static str = "input.PointerCommonProperties";
    pub const DOMAIN_DIRECTION: &'static str = "remote";
    pub fn identifier(&self) -> &'static str {
        Self::IDENTIFIER
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Origin {
    #[serde(rename = "viewport")]
    Viewport,
    #[serde(rename = "pointer")]
    Pointer,
}
