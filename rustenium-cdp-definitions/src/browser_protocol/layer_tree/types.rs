use serde::{Deserialize, Serialize};
#[doc = "Unique Layer identifier.\n[LayerId](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#type-LayerId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct LayerId(String);
impl LayerId {
    pub fn new(val: impl Into<String>) -> Self {
        LayerId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for LayerId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<LayerId> for String {
    fn from(el: LayerId) -> String {
        el.0
    }
}
impl From<String> for LayerId {
    fn from(expr: String) -> Self {
        LayerId(expr)
    }
}
impl std::borrow::Borrow<str> for LayerId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl LayerId {
    pub const IDENTIFIER: &'static str = "LayerTree.LayerId";
}
#[doc = "Unique snapshot identifier.\n[SnapshotId](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#type-SnapshotId)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, Eq, Hash)]
pub struct SnapshotId(String);
impl SnapshotId {
    pub fn new(val: impl Into<String>) -> Self {
        SnapshotId(val.into())
    }
    pub fn inner(&self) -> &String {
        &self.0
    }
}
impl AsRef<str> for SnapshotId {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
impl From<SnapshotId> for String {
    fn from(el: SnapshotId) -> String {
        el.0
    }
}
impl From<String> for SnapshotId {
    fn from(expr: String) -> Self {
        SnapshotId(expr)
    }
}
impl std::borrow::Borrow<str> for SnapshotId {
    fn borrow(&self) -> &str {
        &self.0
    }
}
impl SnapshotId {
    pub const IDENTIFIER: &'static str = "LayerTree.SnapshotId";
}
#[doc = "Rectangle where scrolling happens on the main thread.\n[ScrollRect](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#type-ScrollRect)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrollRect {
    #[doc = "Rectangle itself."]
    #[serde(rename = "rect")]
    pub rect: super::super::dom::types::Rect,
    #[doc = "Reason for rectangle to force scrolling on the main thread"]
    #[serde(rename = "type")]
    pub r#type: ScrollRectType,
}
#[doc = "Reason for rectangle to force scrolling on the main thread"]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ScrollRectType {
    #[serde(rename = "RepaintsOnScroll")]
    RepaintsOnScroll,
    #[serde(rename = "TouchEventHandler")]
    TouchEventHandler,
    #[serde(rename = "WheelEventHandler")]
    WheelEventHandler,
}
impl ScrollRect {
    pub fn new(
        rect: impl Into<super::super::dom::types::Rect>,
        r#type: impl Into<ScrollRectType>,
    ) -> Self {
        Self {
            rect: rect.into(),
            r#type: r#type.into(),
        }
    }
}
impl ScrollRect {
    pub const IDENTIFIER: &'static str = "LayerTree.ScrollRect";
}
#[doc = "Sticky position constraints.\n[StickyPositionConstraint](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#type-StickyPositionConstraint)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StickyPositionConstraint {
    #[doc = "Layout rectangle of the sticky element before being shifted"]
    #[serde(rename = "stickyBoxRect")]
    pub sticky_box_rect: super::super::dom::types::Rect,
    #[doc = "Layout rectangle of the containing block of the sticky element"]
    #[serde(rename = "containingBlockRect")]
    pub containing_block_rect: super::super::dom::types::Rect,
    #[doc = "The nearest sticky layer that shifts the sticky box"]
    #[serde(rename = "nearestLayerShiftingStickyBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub nearest_layer_shifting_sticky_box: Option<LayerId>,
    #[doc = "The nearest sticky layer that shifts the containing block"]
    #[serde(rename = "nearestLayerShiftingContainingBlock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub nearest_layer_shifting_containing_block: Option<LayerId>,
}
impl StickyPositionConstraint {
    pub fn new(
        sticky_box_rect: impl Into<super::super::dom::types::Rect>,
        containing_block_rect: impl Into<super::super::dom::types::Rect>,
    ) -> Self {
        Self {
            sticky_box_rect: sticky_box_rect.into(),
            containing_block_rect: containing_block_rect.into(),
            nearest_layer_shifting_sticky_box: None,
            nearest_layer_shifting_containing_block: None,
        }
    }
}
impl StickyPositionConstraint {
    pub const IDENTIFIER: &'static str = "LayerTree.StickyPositionConstraint";
}
#[doc = "Serialized fragment of layer picture along with its offset within the layer.\n[PictureTile](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#type-PictureTile)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PictureTile {
    #[doc = "Offset from owning layer left boundary"]
    #[serde(rename = "x")]
    pub x: f64,
    #[doc = "Offset from owning layer top boundary"]
    #[serde(rename = "y")]
    pub y: f64,
    #[doc = "Base64-encoded snapshot data."]
    #[serde(rename = "picture")]
    pub picture: super::super::super::Binary,
}
impl PictureTile {
    pub fn new(
        x: impl Into<f64>,
        y: impl Into<f64>,
        picture: impl Into<super::super::super::Binary>,
    ) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            picture: picture.into(),
        }
    }
}
impl PictureTile {
    pub const IDENTIFIER: &'static str = "LayerTree.PictureTile";
}
#[doc = "Information about a compositing layer.\n[Layer](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#type-Layer)"]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    #[doc = "The unique id for this layer."]
    #[serde(rename = "layerId")]
    pub layer_id: LayerId,
    #[doc = "The id of parent (not present for root)."]
    #[serde(rename = "parentLayerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub parent_layer_id: Option<LayerId>,
    #[doc = "The backend id for the node associated with this layer."]
    #[serde(rename = "backendNodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub backend_node_id: Option<super::super::dom::types::BackendNodeId>,
    #[doc = "Offset from parent layer, X coordinate."]
    #[serde(rename = "offsetX")]
    pub offset_x: f64,
    #[doc = "Offset from parent layer, Y coordinate."]
    #[serde(rename = "offsetY")]
    pub offset_y: f64,
    #[doc = "Layer width."]
    #[serde(rename = "width")]
    pub width: f64,
    #[doc = "Layer height."]
    #[serde(rename = "height")]
    pub height: f64,
    #[doc = "Transformation matrix for layer, default is identity matrix"]
    #[serde(rename = "transform")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub transform: Option<Vec<f64>>,
    #[doc = "Transform anchor point X, absent if no transform specified"]
    #[serde(rename = "anchorX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub anchor_x: Option<f64>,
    #[doc = "Transform anchor point Y, absent if no transform specified"]
    #[serde(rename = "anchorY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub anchor_y: Option<f64>,
    #[doc = "Transform anchor point Z, absent if no transform specified"]
    #[serde(rename = "anchorZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub anchor_z: Option<f64>,
    #[doc = "Indicates how many time this layer has painted."]
    #[serde(rename = "paintCount")]
    pub paint_count: i64,
    #[doc = "Indicates whether this layer hosts any content, rather than being used for\ntransform/scrolling purposes only."]
    #[serde(rename = "drawsContent")]
    pub draws_content: bool,
    #[doc = "Set if layer is not visible."]
    #[serde(rename = "invisible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub invisible: Option<bool>,
    #[doc = "Rectangles scrolling on main thread only."]
    #[serde(rename = "scrollRects")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub scroll_rects: Option<Vec<ScrollRect>>,
    #[doc = "Sticky position constraint information"]
    #[serde(rename = "stickyPositionConstraint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub sticky_position_constraint: Option<StickyPositionConstraint>,
}
impl Layer {
    pub const IDENTIFIER: &'static str = "LayerTree.Layer";
}
#[doc = "Array of timings, one per paint step.\n[PaintProfile](https://chromedevtools.github.io/devtools-protocol/tot/LayerTree/#type-PaintProfile)"]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PaintProfile(Vec<f64>);
impl PaintProfile {
    pub fn new(val: impl Into<Vec<f64>>) -> Self {
        PaintProfile(val.into())
    }
    pub fn inner(&self) -> &Vec<f64> {
        &self.0
    }
}
impl PaintProfile {
    pub const IDENTIFIER: &'static str = "LayerTree.PaintProfile";
}
group_enum ! (LayerTreeTypes { LayerId (LayerId) , SnapshotId (SnapshotId) , ScrollRect (ScrollRect) , StickyPositionConstraint (StickyPositionConstraint) , PictureTile (PictureTile) , Layer (Layer) , PaintProfile (PaintProfile) });
