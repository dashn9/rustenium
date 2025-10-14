// Generated types for module

use serde::{Serialize, Deserialize};
use crate::browsing_context::types::BrowsingContext;
use crate::script::types::SharedReference;
use serde_valid::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NoneEnum {
    #[serde(rename = "none")]
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PauseEnum {
    #[serde(rename = "pause")]
    Pause,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PauseAction {
    #[serde(rename = "type")]
    pub r#type: PauseEnum,
    #[serde(rename = "duration")]
    pub duration: Option<u64>,
}

pub type NoneSourceAction = PauseAction;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoneSourceActions {
    #[serde(rename = "type")]
    pub r#type: NoneEnum,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "actions")]
    pub actions: Vec<NoneSourceAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyEnum {
    #[serde(rename = "key")]
    Key,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyDownEnum {
    #[serde(rename = "keyDown")]
    KeyDown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyDownAction {
    #[serde(rename = "type")]
    pub r#type: KeyDownEnum,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeyUpEnum {
    #[serde(rename = "keyUp")]
    KeyUp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyUpAction {
    #[serde(rename = "type")]
    pub r#type: KeyUpEnum,
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum KeySourceAction {
    PauseAction(PauseAction),
    KeyDownAction(KeyDownAction),
    KeyUpAction(KeyUpAction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeySourceActions {
    #[serde(rename = "type")]
    pub r#type: KeyEnum,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "actions")]
    pub actions: Vec<KeySourceAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointerEnum {
    #[serde(rename = "pointer")]
    Pointer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointerType {
    #[serde(rename = "mouse")]
    Mouse,
    #[serde(rename = "pen")]
    Pen,
    #[serde(rename = "touch")]
    Touch,
}

fn pointer_parameters_default_pointer_type() -> PointerType {
    PointerType::Mouse
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointerParameters {
    #[serde(rename = "pointerType")]
    #[serde(default = "pointer_parameters_default_pointer_type")]
    pub pointer_type: PointerType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointerDownEnum {
    #[serde(rename = "pointerDown")]
    PointerDown,
}

fn pointer_common_properties_default_width() -> u64 {
    1
}

fn pointer_common_properties_default_height() -> u64 {
    1
}

fn pointer_common_properties_default_pressure() -> f64 {
    0.0
}

fn pointer_common_properties_default_tangential_pressure() -> f64 {
    0.0
}

fn pointer_common_properties_default_twist() -> i64 {
    0
}

fn pointer_common_properties_default_altitude_angle() -> f64 {
    0.0
}

fn pointer_common_properties_default_azimuth_angle() -> f64 {
    0.0
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PointerCommonProperties {
    #[serde(rename = "width")]
    #[serde(default = "pointer_common_properties_default_width")]
    pub width: u64,
    #[serde(rename = "height")]
    #[serde(default = "pointer_common_properties_default_height")]
    pub height: u64,
    #[serde(rename = "pressure")]
    #[serde(default = "pointer_common_properties_default_pressure")]
    pub pressure: f64,
    #[serde(rename = "tangentialPressure")]
    #[serde(default = "pointer_common_properties_default_tangential_pressure")]
    pub tangential_pressure: f64,
    #[serde(rename = "twist")]
    #[validate(minimum = 0)]
    #[validate(maximum = 359)]
    #[serde(default = "pointer_common_properties_default_twist")]
    pub twist: i64,
    #[serde(rename = "altitudeAngle")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 1.5707963267948966)]
    #[serde(default = "pointer_common_properties_default_altitude_angle")]
    pub altitude_angle: f64,
    #[serde(rename = "azimuthAngle")]
    #[validate(minimum = 0.0)]
    #[validate(maximum = 6.283185307179586)]
    #[serde(default = "pointer_common_properties_default_azimuth_angle")]
    pub azimuth_angle: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointerDownAction {
    #[serde(rename = "type")]
    pub r#type: PointerDownEnum,
    #[serde(rename = "button")]
    pub button: u64,
    #[serde(flatten)]
    pub pointer_common_properties: PointerCommonProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointerUpEnum {
    #[serde(rename = "pointerUp")]
    PointerUp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointerUpAction {
    #[serde(rename = "type")]
    pub r#type: PointerUpEnum,
    #[serde(rename = "button")]
    pub button: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointerMoveEnum {
    #[serde(rename = "pointerMove")]
    PointerMove,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ElementEnum {
    #[serde(rename = "element")]
    Element,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementOrigin {
    #[serde(rename = "type")]
    pub r#type: ElementEnum,
    #[serde(rename = "element")]
    pub element: SharedReference,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Origin {
    #[serde(rename = "viewport")]
    Viewport,
    #[serde(rename = "pointer")]
    Pointer,
    ElementOrigin(ElementOrigin),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointerMoveAction {
    #[serde(rename = "type")]
    pub r#type: PointerMoveEnum,
    #[serde(rename = "x")]
    pub x: f64,
    #[serde(rename = "y")]
    pub y: f64,
    #[serde(rename = "duration")]
    pub duration: Option<u64>,
    #[serde(rename = "origin")]
    pub origin: Option<Origin>,
    #[serde(flatten)]
    pub pointer_common_properties: PointerCommonProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PointerSourceAction {
    PauseAction(PauseAction),
    PointerDownAction(PointerDownAction),
    PointerUpAction(PointerUpAction),
    PointerMoveAction(PointerMoveAction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointerSourceActions {
    #[serde(rename = "type")]
    pub r#type: PointerEnum,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "parameters")]
    pub parameters: Option<PointerParameters>,
    #[serde(rename = "actions")]
    pub actions: Vec<PointerSourceAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WheelEnum {
    #[serde(rename = "wheel")]
    Wheel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScrollEnum {
    #[serde(rename = "scroll")]
    Scroll,
}

fn wheel_scroll_action_default_origin() -> Origin {
    Origin::Viewport
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelScrollAction {
    #[serde(rename = "type")]
    pub r#type: ScrollEnum,
    #[serde(rename = "x")]
    pub x: i64,
    #[serde(rename = "y")]
    pub y: i64,
    #[serde(rename = "deltaX")]
    pub delta_x: i64,
    #[serde(rename = "deltaY")]
    pub delta_y: i64,
    #[serde(rename = "duration")]
    pub duration: Option<u64>,
    #[serde(rename = "origin")]
    #[serde(default = "wheel_scroll_action_default_origin")]
    pub origin: Origin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WheelSourceAction {
    PauseAction(PauseAction),
    WheelScrollAction(WheelScrollAction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WheelSourceActions {
    #[serde(rename = "type")]
    pub r#type: WheelEnum,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "actions")]
    pub actions: Vec<WheelSourceAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceActions {
    NoneSourceActions(NoneSourceActions),
    KeySourceActions(KeySourceActions),
    PointerSourceActions(PointerSourceActions),
    WheelSourceActions(WheelSourceActions),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDialogInfo {
    #[serde(rename = "context")]
    pub context: BrowsingContext,
    #[serde(rename = "element")]
    pub element: Option<SharedReference>,
    #[serde(rename = "multiple")]
    pub multiple: bool,
}

