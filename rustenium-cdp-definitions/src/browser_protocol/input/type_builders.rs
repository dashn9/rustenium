use super::types::*;
impl TouchPoint {
    pub fn builder() -> TouchPointBuilder {
        TouchPointBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct TouchPointBuilder {
    x: Option<f64>,
    y: Option<f64>,
    radius_x: Option<f64>,
    radius_y: Option<f64>,
    rotation_angle: Option<f64>,
    force: Option<f64>,
    tangential_pressure: Option<f64>,
    tilt_x: Option<f64>,
    tilt_y: Option<f64>,
    twist: Option<i64>,
    id: Option<f64>,
}
impl TouchPointBuilder {
    pub fn x(mut self, x: impl Into<f64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<f64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn radius_x(mut self, radius_x: impl Into<f64>) -> Self {
        self.radius_x = Some(radius_x.into());
        self
    }
    pub fn radius_y(mut self, radius_y: impl Into<f64>) -> Self {
        self.radius_y = Some(radius_y.into());
        self
    }
    pub fn rotation_angle(mut self, rotation_angle: impl Into<f64>) -> Self {
        self.rotation_angle = Some(rotation_angle.into());
        self
    }
    pub fn force(mut self, force: impl Into<f64>) -> Self {
        self.force = Some(force.into());
        self
    }
    pub fn tangential_pressure(mut self, tangential_pressure: impl Into<f64>) -> Self {
        self.tangential_pressure = Some(tangential_pressure.into());
        self
    }
    pub fn tilt_x(mut self, tilt_x: impl Into<f64>) -> Self {
        self.tilt_x = Some(tilt_x.into());
        self
    }
    pub fn tilt_y(mut self, tilt_y: impl Into<f64>) -> Self {
        self.tilt_y = Some(tilt_y.into());
        self
    }
    pub fn twist(mut self, twist: impl Into<i64>) -> Self {
        self.twist = Some(twist.into());
        self
    }
    pub fn id(mut self, id: impl Into<f64>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn build(self) -> Result<TouchPoint, String> {
        Ok(TouchPoint {
            x: self
                .x
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
            y: self
                .y
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
            radius_x: self.radius_x,
            radius_y: self.radius_y,
            rotation_angle: self.rotation_angle,
            force: self.force,
            tangential_pressure: self.tangential_pressure,
            tilt_x: self.tilt_x,
            tilt_y: self.tilt_y,
            twist: self.twist,
            id: self.id,
        })
    }
}
impl DragDataItem {
    pub fn builder() -> DragDataItemBuilder {
        DragDataItemBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DragDataItemBuilder {
    mime_type: Option<String>,
    data: Option<String>,
    title: Option<String>,
    base_url: Option<String>,
}
impl DragDataItemBuilder {
    pub fn mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.mime_type = Some(mime_type.into());
        self
    }
    pub fn data(mut self, data: impl Into<String>) -> Self {
        self.data = Some(data.into());
        self
    }
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = Some(base_url.into());
        self
    }
    pub fn build(self) -> Result<DragDataItem, String> {
        Ok(DragDataItem {
            mime_type: self
                .mime_type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(mime_type)))?,
            data: self
                .data
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(data)))?,
            title: self.title,
            base_url: self.base_url,
        })
    }
}
impl DragData {
    pub fn builder() -> DragDataBuilder {
        DragDataBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DragDataBuilder {
    items: Option<Vec<DragDataItem>>,
    files: Option<Vec<String>>,
    drag_operations_mask: Option<i64>,
}
impl DragDataBuilder {
    pub fn item(mut self, item: impl Into<DragDataItem>) -> Self {
        let v = self.items.get_or_insert(Vec::new());
        v.push(item.into());
        self
    }
    pub fn items<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<DragDataItem>,
    {
        let v = self.items.get_or_insert(Vec::new());
        for val in items {
            v.push(val.into());
        }
        self
    }
    pub fn file(mut self, file: impl Into<String>) -> Self {
        let v = self.files.get_or_insert(Vec::new());
        v.push(file.into());
        self
    }
    pub fn files<I, S>(mut self, files: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.files.get_or_insert(Vec::new());
        for val in files {
            v.push(val.into());
        }
        self
    }
    pub fn drag_operations_mask(mut self, drag_operations_mask: impl Into<i64>) -> Self {
        self.drag_operations_mask = Some(drag_operations_mask.into());
        self
    }
    pub fn build(self) -> Result<DragData, String> {
        Ok(DragData {
            items: self
                .items
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(items)))?,
            files: self.files,
            drag_operations_mask: self.drag_operations_mask.ok_or_else(|| {
                format!(
                    "Field `{}` is mandatory.",
                    std::stringify!(drag_operations_mask)
                )
            })?,
        })
    }
}
