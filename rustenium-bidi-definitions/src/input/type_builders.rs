use super::types::*;
impl ElementOrigin {
    pub fn builder() -> ElementOriginBuilder {
        <ElementOriginBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct ElementOriginBuilder {
    r#type: Option<String>,
    element: Option<crate::script::types::SharedReference>,
}
impl ElementOriginBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn element(mut self, element: impl Into<crate::script::types::SharedReference>) -> Self {
        self.element = Some(element.into());
        self
    }
    pub fn build(self) -> Result<ElementOrigin, String> {
        Ok(ElementOrigin {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            element: self
                .element
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(element)))?,
        })
    }
}
impl NoneSourceActions {
    pub fn builder() -> NoneSourceActionsBuilder {
        <NoneSourceActionsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct NoneSourceActionsBuilder {
    r#type: Option<String>,
    id: Option<String>,
    actions: Option<Vec<NoneSourceAction>>,
}
impl NoneSourceActionsBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn action(mut self, action: impl Into<NoneSourceAction>) -> Self {
        let v = self.actions.get_or_insert(Vec::new());
        v.push(action.into());
        self
    }
    pub fn actions<I, S>(mut self, actions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<NoneSourceAction>,
    {
        let v = self.actions.get_or_insert(Vec::new());
        for val in actions {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<NoneSourceActions, String> {
        Ok(NoneSourceActions {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            actions: self
                .actions
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(actions)))?,
        })
    }
}
impl KeySourceActions {
    pub fn builder() -> KeySourceActionsBuilder {
        <KeySourceActionsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct KeySourceActionsBuilder {
    r#type: Option<String>,
    id: Option<String>,
    actions: Option<Vec<KeySourceAction>>,
}
impl KeySourceActionsBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn action(mut self, action: impl Into<KeySourceAction>) -> Self {
        let v = self.actions.get_or_insert(Vec::new());
        v.push(action.into());
        self
    }
    pub fn actions<I, S>(mut self, actions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<KeySourceAction>,
    {
        let v = self.actions.get_or_insert(Vec::new());
        for val in actions {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<KeySourceActions, String> {
        Ok(KeySourceActions {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            actions: self
                .actions
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(actions)))?,
        })
    }
}
impl PointerSourceActions {
    pub fn builder() -> PointerSourceActionsBuilder {
        <PointerSourceActionsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PointerSourceActionsBuilder {
    r#type: Option<String>,
    id: Option<String>,
    parameters: Option<PointerParameters>,
    actions: Option<Vec<PointerSourceAction>>,
}
impl PointerSourceActionsBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn parameters(mut self, parameters: impl Into<PointerParameters>) -> Self {
        self.parameters = Some(parameters.into());
        self
    }
    pub fn action(mut self, action: impl Into<PointerSourceAction>) -> Self {
        let v = self.actions.get_or_insert(Vec::new());
        v.push(action.into());
        self
    }
    pub fn actions<I, S>(mut self, actions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<PointerSourceAction>,
    {
        let v = self.actions.get_or_insert(Vec::new());
        for val in actions {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<PointerSourceActions, String> {
        Ok(PointerSourceActions {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            parameters: self.parameters,
            actions: self
                .actions
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(actions)))?,
        })
    }
}
impl PointerParameters {
    pub fn builder() -> PointerParametersBuilder {
        <PointerParametersBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PointerParametersBuilder {
    pointer_type: Option<PointerType>,
}
impl PointerParametersBuilder {
    pub fn pointer_type(mut self, pointer_type: impl Into<PointerType>) -> Self {
        self.pointer_type = Some(pointer_type.into());
        self
    }
    pub fn build(self) -> PointerParameters {
        PointerParameters {
            pointer_type: self.pointer_type,
        }
    }
}
impl WheelSourceActions {
    pub fn builder() -> WheelSourceActionsBuilder {
        <WheelSourceActionsBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct WheelSourceActionsBuilder {
    r#type: Option<String>,
    id: Option<String>,
    actions: Option<Vec<WheelSourceAction>>,
}
impl WheelSourceActionsBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }
    pub fn action(mut self, action: impl Into<WheelSourceAction>) -> Self {
        let v = self.actions.get_or_insert(Vec::new());
        v.push(action.into());
        self
    }
    pub fn actions<I, S>(mut self, actions: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<WheelSourceAction>,
    {
        let v = self.actions.get_or_insert(Vec::new());
        for val in actions {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<WheelSourceActions, String> {
        Ok(WheelSourceActions {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            id: self
                .id
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(id)))?,
            actions: self
                .actions
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(actions)))?,
        })
    }
}
impl PauseAction {
    pub fn builder() -> PauseActionBuilder {
        <PauseActionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PauseActionBuilder {
    r#type: Option<String>,
    duration: Option<u64>,
}
impl PauseActionBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn duration(mut self, duration: impl Into<u64>) -> Self {
        self.duration = Some(duration.into());
        self
    }
    pub fn build(self) -> Result<PauseAction, String> {
        Ok(PauseAction {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            duration: self.duration,
        })
    }
}
impl KeyDownAction {
    pub fn builder() -> KeyDownActionBuilder {
        <KeyDownActionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct KeyDownActionBuilder {
    r#type: Option<String>,
    value: Option<String>,
}
impl KeyDownActionBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<KeyDownAction, String> {
        Ok(KeyDownAction {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl KeyUpAction {
    pub fn builder() -> KeyUpActionBuilder {
        <KeyUpActionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct KeyUpActionBuilder {
    r#type: Option<String>,
    value: Option<String>,
}
impl KeyUpActionBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }
    pub fn build(self) -> Result<KeyUpAction, String> {
        Ok(KeyUpAction {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            value: self
                .value
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(value)))?,
        })
    }
}
impl PointerUpAction {
    pub fn builder() -> PointerUpActionBuilder {
        <PointerUpActionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PointerUpActionBuilder {
    r#type: Option<String>,
    button: Option<u64>,
}
impl PointerUpActionBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn button(mut self, button: impl Into<u64>) -> Self {
        self.button = Some(button.into());
        self
    }
    pub fn build(self) -> Result<PointerUpAction, String> {
        Ok(PointerUpAction {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            button: self
                .button
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(button)))?,
        })
    }
}
impl PointerDownAction {
    pub fn builder() -> PointerDownActionBuilder {
        <PointerDownActionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PointerDownActionBuilder {
    r#type: Option<String>,
    button: Option<u64>,
    width: Option<u64>,
    height: Option<u64>,
    pressure: Option<f64>,
    tangential_pressure: Option<f64>,
    twist: Option<u64>,
    altitude_angle: Option<f64>,
    azimuth_angle: Option<f64>,
}
impl PointerDownActionBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn button(mut self, button: impl Into<u64>) -> Self {
        self.button = Some(button.into());
        self
    }
    pub fn width(mut self, width: impl Into<u64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn height(mut self, height: impl Into<u64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn pressure(mut self, pressure: impl Into<f64>) -> Self {
        self.pressure = Some(pressure.into());
        self
    }
    pub fn tangential_pressure(mut self, tangential_pressure: impl Into<f64>) -> Self {
        self.tangential_pressure = Some(tangential_pressure.into());
        self
    }
    pub fn twist(mut self, twist: impl Into<u64>) -> Self {
        self.twist = Some(twist.into());
        self
    }
    pub fn altitude_angle(mut self, altitude_angle: impl Into<f64>) -> Self {
        self.altitude_angle = Some(altitude_angle.into());
        self
    }
    pub fn azimuth_angle(mut self, azimuth_angle: impl Into<f64>) -> Self {
        self.azimuth_angle = Some(azimuth_angle.into());
        self
    }
    pub fn build(self) -> Result<PointerDownAction, String> {
        Ok(PointerDownAction {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            button: self
                .button
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(button)))?,
            width: self.width,
            height: self.height,
            pressure: self.pressure,
            tangential_pressure: self.tangential_pressure,
            twist: self.twist,
            altitude_angle: self.altitude_angle,
            azimuth_angle: self.azimuth_angle,
        })
    }
}
impl PointerMoveAction {
    pub fn builder() -> PointerMoveActionBuilder {
        <PointerMoveActionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PointerMoveActionBuilder {
    r#type: Option<String>,
    x: Option<f64>,
    y: Option<f64>,
    duration: Option<u64>,
    origin: Option<Origin>,
    width: Option<u64>,
    height: Option<u64>,
    pressure: Option<f64>,
    tangential_pressure: Option<f64>,
    twist: Option<u64>,
    altitude_angle: Option<f64>,
    azimuth_angle: Option<f64>,
}
impl PointerMoveActionBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn x(mut self, x: impl Into<f64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<f64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn duration(mut self, duration: impl Into<u64>) -> Self {
        self.duration = Some(duration.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<Origin>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn width(mut self, width: impl Into<u64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn height(mut self, height: impl Into<u64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn pressure(mut self, pressure: impl Into<f64>) -> Self {
        self.pressure = Some(pressure.into());
        self
    }
    pub fn tangential_pressure(mut self, tangential_pressure: impl Into<f64>) -> Self {
        self.tangential_pressure = Some(tangential_pressure.into());
        self
    }
    pub fn twist(mut self, twist: impl Into<u64>) -> Self {
        self.twist = Some(twist.into());
        self
    }
    pub fn altitude_angle(mut self, altitude_angle: impl Into<f64>) -> Self {
        self.altitude_angle = Some(altitude_angle.into());
        self
    }
    pub fn azimuth_angle(mut self, azimuth_angle: impl Into<f64>) -> Self {
        self.azimuth_angle = Some(azimuth_angle.into());
        self
    }
    pub fn build(self) -> Result<PointerMoveAction, String> {
        Ok(PointerMoveAction {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            x: self
                .x
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
            y: self
                .y
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
            duration: self.duration,
            origin: self.origin,
            width: self.width,
            height: self.height,
            pressure: self.pressure,
            tangential_pressure: self.tangential_pressure,
            twist: self.twist,
            altitude_angle: self.altitude_angle,
            azimuth_angle: self.azimuth_angle,
        })
    }
}
impl WheelScrollAction {
    pub fn builder() -> WheelScrollActionBuilder {
        <WheelScrollActionBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct WheelScrollActionBuilder {
    r#type: Option<String>,
    x: Option<i64>,
    y: Option<i64>,
    delta_x: Option<i64>,
    delta_y: Option<i64>,
    duration: Option<u64>,
    origin: Option<Origin>,
}
impl WheelScrollActionBuilder {
    pub fn r#type(mut self, r#type: impl Into<String>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn x(mut self, x: impl Into<i64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<i64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn delta_x(mut self, delta_x: impl Into<i64>) -> Self {
        self.delta_x = Some(delta_x.into());
        self
    }
    pub fn delta_y(mut self, delta_y: impl Into<i64>) -> Self {
        self.delta_y = Some(delta_y.into());
        self
    }
    pub fn duration(mut self, duration: impl Into<u64>) -> Self {
        self.duration = Some(duration.into());
        self
    }
    pub fn origin(mut self, origin: impl Into<Origin>) -> Self {
        self.origin = Some(origin.into());
        self
    }
    pub fn build(self) -> Result<WheelScrollAction, String> {
        Ok(WheelScrollAction {
            r#type: self
                .r#type
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
            x: self
                .x
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
            y: self
                .y
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
            delta_x: self
                .delta_x
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(delta_x)))?,
            delta_y: self
                .delta_y
                .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(delta_y)))?,
            duration: self.duration,
            origin: self.origin,
        })
    }
}
impl PointerCommonProperties {
    pub fn builder() -> PointerCommonPropertiesBuilder {
        <PointerCommonPropertiesBuilder as Default>::default()
    }
}
#[derive(Default, Clone)]
pub struct PointerCommonPropertiesBuilder {
    width: Option<u64>,
    height: Option<u64>,
    pressure: Option<f64>,
    tangential_pressure: Option<f64>,
    twist: Option<u64>,
    altitude_angle: Option<f64>,
    azimuth_angle: Option<f64>,
}
impl PointerCommonPropertiesBuilder {
    pub fn width(mut self, width: impl Into<u64>) -> Self {
        self.width = Some(width.into());
        self
    }
    pub fn height(mut self, height: impl Into<u64>) -> Self {
        self.height = Some(height.into());
        self
    }
    pub fn pressure(mut self, pressure: impl Into<f64>) -> Self {
        self.pressure = Some(pressure.into());
        self
    }
    pub fn tangential_pressure(mut self, tangential_pressure: impl Into<f64>) -> Self {
        self.tangential_pressure = Some(tangential_pressure.into());
        self
    }
    pub fn twist(mut self, twist: impl Into<u64>) -> Self {
        self.twist = Some(twist.into());
        self
    }
    pub fn altitude_angle(mut self, altitude_angle: impl Into<f64>) -> Self {
        self.altitude_angle = Some(altitude_angle.into());
        self
    }
    pub fn azimuth_angle(mut self, azimuth_angle: impl Into<f64>) -> Self {
        self.azimuth_angle = Some(azimuth_angle.into());
        self
    }
    pub fn build(self) -> PointerCommonProperties {
        PointerCommonProperties {
            width: self.width,
            height: self.height,
            pressure: self.pressure,
            tangential_pressure: self.tangential_pressure,
            twist: self.twist,
            altitude_angle: self.altitude_angle,
            azimuth_angle: self.azimuth_angle,
        }
    }
}
