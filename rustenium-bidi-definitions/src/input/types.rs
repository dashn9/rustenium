use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElementOrigin {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "element")]
    pub element: crate::script::types::SharedReference,
}
impl ElementOrigin {
    pub fn new(
        r#type: impl Into<String>,
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
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceActions {
    NoneSourceActions(NoneSourceActions),
    KeySourceActions(KeySourceActions),
    PointerSourceActions(PointerSourceActions),
    WheelSourceActions(WheelSourceActions),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NoneSourceActions {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<NoneSourceAction>,
}
impl NoneSourceActions {
    pub fn new(
        r#type: impl Into<String>,
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
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeySourceActions {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<KeySourceAction>,
}
impl KeySourceActions {
    pub fn new(
        r#type: impl Into<String>,
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
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeySourceAction {
    PauseAction(PauseAction),
    KeyDownAction(KeyDownAction),
    KeyUpAction(KeyUpAction),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointerSourceActions {
    #[serde(rename = "type")]
    pub r#type: String,
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
impl PointerSourceActions {
    pub fn new(
        r#type: impl Into<String>,
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
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointerSourceAction {
    PauseAction(PauseAction),
    PointerDownAction(PointerDownAction),
    PointerUpAction(PointerUpAction),
    PointerMoveAction(PointerMoveAction),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelSourceActions {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<WheelSourceAction>,
}
impl WheelSourceActions {
    pub fn new(
        r#type: impl Into<String>,
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
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WheelSourceAction {
    PauseAction(PauseAction),
    WheelScrollAction(WheelScrollAction),
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PauseAction {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub duration: Option<u64>,
}
impl PauseAction {
    pub fn new(r#type: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            duration: None,
        }
    }
}
impl<T: Into<String>> From<T> for PauseAction {
    fn from(url: T) -> Self {
        PauseAction::new(url)
    }
}
impl PauseAction {
    pub const IDENTIFIER: &'static str = "input.PauseAction";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyDownAction {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl KeyDownAction {
    pub fn new(r#type: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl KeyDownAction {
    pub const IDENTIFIER: &'static str = "input.KeyDownAction";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KeyUpAction {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "value")]
    pub value: String,
}
impl KeyUpAction {
    pub fn new(r#type: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            value: value.into(),
        }
    }
}
impl KeyUpAction {
    pub const IDENTIFIER: &'static str = "input.KeyUpAction";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointerUpAction {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "button")]
    pub button: u64,
}
impl PointerUpAction {
    pub fn new(r#type: impl Into<String>, button: impl Into<u64>) -> Self {
        Self {
            r#type: r#type.into(),
            button: button.into(),
        }
    }
}
impl PointerUpAction {
    pub const IDENTIFIER: &'static str = "input.PointerUpAction";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointerDownAction {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "button")]
    pub button: u64,
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_down_action_width")]
    pub width: Option<u64>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_down_action_height")]
    pub height: Option<u64>,
    #[serde(rename = "pressure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_down_action_pressure")]
    pub pressure: Option<f64>,
    #[serde(rename = "tangentialPressure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_down_action_tangential_pressure")]
    pub tangential_pressure: Option<f64>,
    #[serde(rename = "twist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_down_action_twist")]
    pub twist: Option<u64>,
    #[serde(rename = "altitudeAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_down_action_altitude_angle")]
    pub altitude_angle: Option<f64>,
    #[serde(rename = "azimuthAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_down_action_azimuth_angle")]
    pub azimuth_angle: Option<f64>,
}
fn default_pointer_down_action_width() -> Option<u64> {
    Some(1u64)
}
fn default_pointer_down_action_height() -> Option<u64> {
    Some(1u64)
}
fn default_pointer_down_action_pressure() -> Option<f64> {
    Some(0f64)
}
fn default_pointer_down_action_tangential_pressure() -> Option<f64> {
    Some(0f64)
}
fn default_pointer_down_action_twist() -> Option<u64> {
    Some(0u64)
}
fn default_pointer_down_action_altitude_angle() -> Option<f64> {
    Some(0f64)
}
fn default_pointer_down_action_azimuth_angle() -> Option<f64> {
    Some(0f64)
}
impl PointerDownAction {
    pub fn new(r#type: impl Into<String>, button: impl Into<u64>) -> Self {
        Self {
            r#type: r#type.into(),
            button: button.into(),
            width: None,
            height: None,
            pressure: None,
            tangential_pressure: None,
            twist: None,
            altitude_angle: None,
            azimuth_angle: None,
        }
    }
}
impl PointerDownAction {
    pub const IDENTIFIER: &'static str = "input.PointerDownAction";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointerMoveAction {
    #[serde(rename = "type")]
    pub r#type: String,
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
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_move_action_width")]
    pub width: Option<u64>,
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_move_action_height")]
    pub height: Option<u64>,
    #[serde(rename = "pressure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_move_action_pressure")]
    pub pressure: Option<f64>,
    #[serde(rename = "tangentialPressure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_move_action_tangential_pressure")]
    pub tangential_pressure: Option<f64>,
    #[serde(rename = "twist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_move_action_twist")]
    pub twist: Option<u64>,
    #[serde(rename = "altitudeAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_move_action_altitude_angle")]
    pub altitude_angle: Option<f64>,
    #[serde(rename = "azimuthAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default = "default_pointer_move_action_azimuth_angle")]
    pub azimuth_angle: Option<f64>,
}
fn default_pointer_move_action_width() -> Option<u64> {
    Some(1u64)
}
fn default_pointer_move_action_height() -> Option<u64> {
    Some(1u64)
}
fn default_pointer_move_action_pressure() -> Option<f64> {
    Some(0f64)
}
fn default_pointer_move_action_tangential_pressure() -> Option<f64> {
    Some(0f64)
}
fn default_pointer_move_action_twist() -> Option<u64> {
    Some(0u64)
}
fn default_pointer_move_action_altitude_angle() -> Option<f64> {
    Some(0f64)
}
fn default_pointer_move_action_azimuth_angle() -> Option<f64> {
    Some(0f64)
}
impl PointerMoveAction {
    pub fn new(r#type: impl Into<String>, x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Self {
            r#type: r#type.into(),
            x: x.into(),
            y: y.into(),
            duration: None,
            origin: None,
            width: None,
            height: None,
            pressure: None,
            tangential_pressure: None,
            twist: None,
            altitude_angle: None,
            azimuth_angle: None,
        }
    }
}
impl PointerMoveAction {
    pub const IDENTIFIER: &'static str = "input.PointerMoveAction";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WheelScrollAction {
    #[serde(rename = "type")]
    pub r#type: String,
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
impl WheelScrollAction {
    pub const IDENTIFIER: &'static str = "input.WheelScrollAction";
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
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Origin {
    #[serde(rename = "viewport")]
    Viewport,
    #[serde(rename = "pointer")]
    Pointer,
}
group_enum ! (InputTypes { ElementOrigin (ElementOrigin) , SourceActions (SourceActions) , NoneSourceActions (NoneSourceActions) , NoneSourceAction (NoneSourceAction) , KeySourceActions (KeySourceActions) , KeySourceAction (KeySourceAction) , PointerSourceActions (PointerSourceActions) , PointerType (PointerType) , PointerParameters (PointerParameters) , PointerSourceAction (PointerSourceAction) , WheelSourceActions (WheelSourceActions) , WheelSourceAction (WheelSourceAction) , PauseAction (PauseAction) , KeyDownAction (KeyDownAction) , KeyUpAction (KeyUpAction) , PointerUpAction (PointerUpAction) , PointerDownAction (PointerDownAction) , PointerMoveAction (PointerMoveAction) , WheelScrollAction (WheelScrollAction) , PointerCommonProperties (PointerCommonProperties) , Origin (Origin) });
