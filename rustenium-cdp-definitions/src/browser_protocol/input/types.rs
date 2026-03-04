use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TouchPoint {
    #[doc = "X coordinate of the event relative to the main frame's viewport in CSS pixels."]
    #[serde(rename = "x")]
    pub x: f64,
    #[doc = "Y coordinate of the event relative to the main frame's viewport in CSS pixels. 0 refers to\nthe top of the viewport and Y increases as it proceeds towards the bottom of the viewport."]
    #[serde(rename = "y")]
    pub y: f64,
    #[doc = "X radius of the touch area (default: 1.0)."]
    #[serde(rename = "radiusX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub radius_x: Option<f64>,
    #[doc = "Y radius of the touch area (default: 1.0)."]
    #[serde(rename = "radiusY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub radius_y: Option<f64>,
    #[doc = "Rotation angle (default: 0.0)."]
    #[serde(rename = "rotationAngle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub rotation_angle: Option<f64>,
    #[doc = "Force (default: 1.0)."]
    #[serde(rename = "force")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub force: Option<f64>,
    #[doc = "The normalized tangential pressure, which has a range of [-1,1] (default: 0)."]
    #[serde(rename = "tangentialPressure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tangential_pressure: Option<f64>,
    #[doc = "The plane angle between the Y-Z plane and the plane containing both the stylus axis and the Y axis, in degrees of the range [-90,90], a positive tiltX is to the right (default: 0)"]
    #[serde(rename = "tiltX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tilt_x: Option<f64>,
    #[doc = "The plane angle between the X-Z plane and the plane containing both the stylus axis and the X axis, in degrees of the range [-90,90], a positive tiltY is towards the user (default: 0)."]
    #[serde(rename = "tiltY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub tilt_y: Option<f64>,
    #[doc = "The clockwise rotation of a pen stylus around its own major axis, in degrees in the range [0,359] (default: 0)."]
    #[serde(rename = "twist")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub twist: Option<i64>,
    #[doc = "Identifier used to track touch sources between events, must be unique within an event."]
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub id: Option<f64>,
}
impl TouchPoint {
    pub fn new(x: impl Into<f64>, y: impl Into<f64>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            radius_x: None,
            radius_y: None,
            rotation_angle: None,
            force: None,
            tangential_pressure: None,
            tilt_x: None,
            tilt_y: None,
            twist: None,
            id: None,
        }
    }
}
impl TouchPoint {
    pub const IDENTIFIER: &'static str = "Input.TouchPoint";
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GestureSourceType {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "touch")]
    Touch,
    #[serde(rename = "mouse")]
    Mouse,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MouseButton {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "back")]
    Back,
    #[serde(rename = "forward")]
    Forward,
}
#[doc = "UTC time in seconds, counted from January 1, 1970.\n[TimeSinceEpoch](https://chromedevtools.github.io/devtools-protocol/tot/Input/#type-TimeSinceEpoch)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TimeSinceEpoch(f64);
impl TimeSinceEpoch {
    pub fn new(val: impl Into<f64>) -> Self {
        TimeSinceEpoch(val.into())
    }
    pub fn inner(&self) -> &f64 {
        &self.0
    }
}
impl TimeSinceEpoch {
    pub const IDENTIFIER: &'static str = "Input.TimeSinceEpoch";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DragDataItem {
    #[doc = "Mime type of the dragged data."]
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[doc = "Depending of the value of `mimeType`, it contains the dragged link,\ntext, HTML markup or any other data."]
    #[serde(rename = "data")]
    pub data: String,
    #[doc = "Title associated with a link. Only valid when `mimeType` == \"text/uri-list\"."]
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub title: Option<String>,
    #[doc = "Stores the base URL for the contained markup. Only valid when `mimeType`\n== \"text/html\"."]
    #[serde(rename = "baseURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub base_url: Option<String>,
}
impl DragDataItem {
    pub fn new(mime_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            mime_type: mime_type.into(),
            data: data.into(),
            title: None,
            base_url: None,
        }
    }
}
impl DragDataItem {
    pub const IDENTIFIER: &'static str = "Input.DragDataItem";
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DragData {
    #[serde(rename = "items")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DragDataItem>,
    #[doc = "List of filenames that should be included when dropping"]
    #[serde(rename = "files")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub files: Option<Vec<String>>,
    #[doc = "Bit field representing allowed drag operations. Copy = 1, Link = 2, Move = 16"]
    #[serde(rename = "dragOperationsMask")]
    pub drag_operations_mask: i64,
}
impl DragData {
    pub fn new(items: Vec<DragDataItem>, drag_operations_mask: impl Into<i64>) -> Self {
        Self {
            items,
            drag_operations_mask: drag_operations_mask.into(),
            files: None,
        }
    }
}
impl DragData {
    pub const IDENTIFIER: &'static str = "Input.DragData";
}
group_enum ! (Type { TouchPoint (TouchPoint) , GestureSourceType (GestureSourceType) , MouseButton (MouseButton) , TimeSinceEpoch (TimeSinceEpoch) , DragDataItem (DragDataItem) , DragData (DragData) });
