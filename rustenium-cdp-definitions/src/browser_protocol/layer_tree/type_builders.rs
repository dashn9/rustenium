use super::types::*;
impl ScrollRect {
    pub fn builder() -> ScrollRectBuilder {
        <ScrollRectBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ScrollRectBuilder {
    rect: Option<crate::browser_protocol::dom::types::Rect>,
    r#type: Option<ScrollRectType>,
}
impl ScrollRectBuilder {
    pub fn rect(mut self, rect: impl Into<crate::browser_protocol::dom::types::Rect>) -> Self {
        self.rect = Some(rect.into());
        self
    }
    pub fn r#type(mut self, r#type: impl Into<ScrollRectType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn build(self) -> Result<ScrollRect, String> {
        Ok(ScrollRect {
            rect: self
                .rect
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(rect)))?,
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
        })
    }
}
impl StickyPositionConstraint {
    pub fn builder() -> StickyPositionConstraintBuilder {
        <StickyPositionConstraintBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct StickyPositionConstraintBuilder {
    sticky_box_rect: Option<crate::browser_protocol::dom::types::Rect>,
    containing_block_rect: Option<crate::browser_protocol::dom::types::Rect>,
    nearest_layer_shifting_sticky_box: Option<LayerId>,
    nearest_layer_shifting_containing_block: Option<LayerId>,
}
impl StickyPositionConstraintBuilder {
    pub fn sticky_box_rect(
        mut self,
        sticky_box_rect: impl Into<crate::browser_protocol::dom::types::Rect>,
    ) -> Self {
        self.sticky_box_rect = Some(sticky_box_rect.into());
        self
    }
    pub fn containing_block_rect(
        mut self,
        containing_block_rect: impl Into<crate::browser_protocol::dom::types::Rect>,
    ) -> Self {
        self.containing_block_rect = Some(containing_block_rect.into());
        self
    }
    pub fn nearest_layer_shifting_sticky_box(
        mut self,
        nearest_layer_shifting_sticky_box: impl Into<LayerId>,
    ) -> Self {
        self.nearest_layer_shifting_sticky_box = Some(nearest_layer_shifting_sticky_box.into());
        self
    }
    pub fn nearest_layer_shifting_containing_block(
        mut self,
        nearest_layer_shifting_containing_block: impl Into<LayerId>,
    ) -> Self {
        self.nearest_layer_shifting_containing_block =
            Some(nearest_layer_shifting_containing_block.into());
        self
    }
    pub fn build(self) -> Result<StickyPositionConstraint, String> {
        Ok(StickyPositionConstraint {
            sticky_box_rect: self.sticky_box_rect.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(sticky_box_rect))
            })?,
            containing_block_rect: self.containing_block_rect.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(containing_block_rect)
                )
            })?,
            nearest_layer_shifting_sticky_box: self.nearest_layer_shifting_sticky_box,
            nearest_layer_shifting_containing_block: self.nearest_layer_shifting_containing_block,
        })
    }
}
impl PictureTile {
    pub fn builder() -> PictureTileBuilder {
        <PictureTileBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PictureTileBuilder {
    x: Option<f64>,
    y: Option<f64>,
    picture: Option<crate::Binary>,
}
impl PictureTileBuilder {
    pub fn x(mut self, x: impl Into<f64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<f64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn picture(mut self, picture: impl Into<crate::Binary>) -> Self {
        self.picture = Some(picture.into());
        self
    }
    pub fn build(self) -> Result<PictureTile, String> {
        Ok(PictureTile {
            x: self
                .x
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
            y: self
                .y
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
            picture: self
                .picture
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(picture)))?,
        })
    }
}
impl Layer {
    pub fn builder() -> LayerBuilder {
        <LayerBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct LayerBuilder {
    layer_id: Option<LayerId>,
    parent_layer_id: Option<LayerId>,
    backend_node_id: Option<crate::browser_protocol::dom::types::BackendNodeId>,
    offset_x: Option<f64>,
    offset_y: Option<f64>,
    width: Option<f64>,
    height: Option<f64>,
    transform: Option<Vec<f64>>,
    anchor_x: Option<f64>,
    anchor_y: Option<f64>,
    anchor_z: Option<f64>,
    paint_count: Option<i64>,
    draws_content: Option<bool>,
    invisible: Option<bool>,
    scroll_rects: Option<Vec<ScrollRect>>,
    sticky_position_constraint: Option<StickyPositionConstraint>,
}
impl LayerBuilder {
    pub fn layer_id(mut self, layer_id: impl Into<LayerId>) -> Self {
        self.layer_id = Some(layer_id.into());
        self
    }
    pub fn parent_layer_id(mut self, parent_layer_id: impl Into<LayerId>) -> Self {
        self.parent_layer_id = Some(parent_layer_id.into());
        self
    }
    pub fn backend_node_id(
        mut self,
        backend_node_id: impl Into<crate::browser_protocol::dom::types::BackendNodeId>,
    ) -> Self {
        self.backend_node_id = Some(backend_node_id.into());
        self
    }
    pub fn offset_x(mut self, offset_x: impl Into<f64>) -> Self {
        self.offset_x = Some(offset_x.into());
        self
    }
    pub fn offset_y(mut self, offset_y: impl Into<f64>) -> Self {
        self.offset_y = Some(offset_y.into());
        self
    }
    pub fn width(mut self, width: impl Into<f64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn height(mut self, height: impl Into<f64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn transform(mut self, transform: impl Into<f64>) -> Self {
        let v = self.transform.get_or_insert(Vec::new());
        v.push(transform.into());
        self
    }
    pub fn transforms<I, S>(mut self, transforms: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<f64>,
    {
        let v = self.transform.get_or_insert(Vec::new());
        for val in transforms {
            v.push(val.into());
        }
        self
    }
    pub fn anchor_x(mut self, anchor_x: impl Into<f64>) -> Self {
        self.anchor_x = Some(anchor_x.into());
        self
    }
    pub fn anchor_y(mut self, anchor_y: impl Into<f64>) -> Self {
        self.anchor_y = Some(anchor_y.into());
        self
    }
    pub fn anchor_z(mut self, anchor_z: impl Into<f64>) -> Self {
        self.anchor_z = Some(anchor_z.into());
        self
    }
    pub fn paint_count(mut self, paint_count: impl Into<i64>) -> Self {
        self.paint_count = Some(paint_count.into());
        self
    }
    pub fn draws_content(mut self, draws_content: impl Into<bool>) -> Self {
        self.draws_content = Some(draws_content.into());
        self
    }
    pub fn invisible(mut self, invisible: impl Into<bool>) -> Self {
        self.invisible = Some(invisible.into());
        self
    }
    pub fn scroll_rect(mut self, scroll_rect: impl Into<ScrollRect>) -> Self {
        let v = self.scroll_rects.get_or_insert(Vec::new());
        v.push(scroll_rect.into());
        self
    }
    pub fn scroll_rects<I, S>(mut self, scroll_rects: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<ScrollRect>,
    {
        let v = self.scroll_rects.get_or_insert(Vec::new());
        for val in scroll_rects {
            v.push(val.into());
        }
        self
    }
    pub fn sticky_position_constraint(
        mut self,
        sticky_position_constraint: impl Into<StickyPositionConstraint>,
    ) -> Self {
        self.sticky_position_constraint = Some(sticky_position_constraint.into());
        self
    }
    pub fn build(self) -> Result<Layer, String> {
        Ok(Layer {
            layer_id: self
                .layer_id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(layer_id)))?,
            parent_layer_id: self.parent_layer_id,
            backend_node_id: self.backend_node_id,
            offset_x: self
                .offset_x
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(offset_x)))?,
            offset_y: self
                .offset_y
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(offset_y)))?,
            width: self
                .width
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(width)))?,
            height: self
                .height
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(height)))?,
            transform: self.transform,
            anchor_x: self.anchor_x,
            anchor_y: self.anchor_y,
            anchor_z: self.anchor_z,
            paint_count: self
                .paint_count
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(paint_count)))?,
            draws_content: self.draws_content.ok_or_else(|| {
                format!("Field `{}` is mandatory.", std::stringify!(draws_content))
            })?,
            invisible: self.invisible,
            scroll_rects: self.scroll_rects,
            sticky_position_constraint: self.sticky_position_constraint,
        })
    }
}
