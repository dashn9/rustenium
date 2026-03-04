use super::commands::*;
impl DispatchDragEvent {
    pub fn builder() -> DispatchDragEventBuilder {
        DispatchDragEventBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DispatchDragEventBuilder {
    r#type: Option<DispatchDragEventType>,
    x: Option<f64>,
    y: Option<f64>,
    data: Option<super::types::DragData>,
    modifiers: Option<i64>,
}
impl DispatchDragEventBuilder {
    pub fn r#type(mut self, r#type: impl Into<DispatchDragEventType>) -> Self {
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
    pub fn data(mut self, data: impl Into<super::types::DragData>) -> Self {
        self.data = Some(data.into());
        self
    }
    pub fn modifiers(mut self, modifiers: impl Into<i64>) -> Self {
        self.modifiers = Some(modifiers.into());
        self
    }
    pub fn build(self) -> Result<DispatchDragEvent, String> {
        Ok(DispatchDragEvent {
            method: DispatchDragEventMethod::DispatchDragEvent,
            params: DispatchDragEventParams {
                r#type: self
                    .r#type
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
                x: self
                    .x
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
                y: self
                    .y
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
                data: self
                    .data
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(data)))?,
                modifiers: self.modifiers,
            },
        })
    }
}
impl DispatchKeyEvent {
    pub fn builder() -> DispatchKeyEventBuilder {
        DispatchKeyEventBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DispatchKeyEventBuilder {
    r#type: Option<DispatchKeyEventType>,
    modifiers: Option<i64>,
    timestamp: Option<super::types::TimeSinceEpoch>,
    text: Option<String>,
    unmodified_text: Option<String>,
    key_identifier: Option<String>,
    code: Option<String>,
    key: Option<String>,
    windows_virtual_key_code: Option<i64>,
    native_virtual_key_code: Option<i64>,
    auto_repeat: Option<bool>,
    is_keypad: Option<bool>,
    is_system_key: Option<bool>,
    location: Option<i64>,
    commands: Option<Vec<String>>,
}
impl DispatchKeyEventBuilder {
    pub fn r#type(mut self, r#type: impl Into<DispatchKeyEventType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn modifiers(mut self, modifiers: impl Into<i64>) -> Self {
        self.modifiers = Some(modifiers.into());
        self
    }
    pub fn timestamp(mut self, timestamp: impl Into<super::types::TimeSinceEpoch>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn unmodified_text(mut self, unmodified_text: impl Into<String>) -> Self {
        self.unmodified_text = Some(unmodified_text.into());
        self
    }
    pub fn key_identifier(mut self, key_identifier: impl Into<String>) -> Self {
        self.key_identifier = Some(key_identifier.into());
        self
    }
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.key = Some(key.into());
        self
    }
    pub fn windows_virtual_key_code(mut self, windows_virtual_key_code: impl Into<i64>) -> Self {
        self.windows_virtual_key_code = Some(windows_virtual_key_code.into());
        self
    }
    pub fn native_virtual_key_code(mut self, native_virtual_key_code: impl Into<i64>) -> Self {
        self.native_virtual_key_code = Some(native_virtual_key_code.into());
        self
    }
    pub fn auto_repeat(mut self, auto_repeat: impl Into<bool>) -> Self {
        self.auto_repeat = Some(auto_repeat.into());
        self
    }
    pub fn is_keypad(mut self, is_keypad: impl Into<bool>) -> Self {
        self.is_keypad = Some(is_keypad.into());
        self
    }
    pub fn is_system_key(mut self, is_system_key: impl Into<bool>) -> Self {
        self.is_system_key = Some(is_system_key.into());
        self
    }
    pub fn location(mut self, location: impl Into<i64>) -> Self {
        self.location = Some(location.into());
        self
    }
    pub fn command(mut self, command: impl Into<String>) -> Self {
        let v = self.commands.get_or_insert(Vec::new());
        v.push(command.into());
        self
    }
    pub fn commands<I, S>(mut self, commands: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let v = self.commands.get_or_insert(Vec::new());
        for val in commands {
            v.push(val.into());
        }
        self
    }
    pub fn build(self) -> Result<DispatchKeyEvent, String> {
        Ok(DispatchKeyEvent {
            method: DispatchKeyEventMethod::DispatchKeyEvent,
            params: DispatchKeyEventParams {
                r#type: self
                    .r#type
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
                modifiers: self.modifiers,
                timestamp: self.timestamp,
                text: self.text,
                unmodified_text: self.unmodified_text,
                key_identifier: self.key_identifier,
                code: self.code,
                key: self.key,
                windows_virtual_key_code: self.windows_virtual_key_code,
                native_virtual_key_code: self.native_virtual_key_code,
                auto_repeat: self.auto_repeat,
                is_keypad: self.is_keypad,
                is_system_key: self.is_system_key,
                location: self.location,
                commands: self.commands,
            },
        })
    }
}
impl InsertText {
    pub fn builder() -> InsertTextBuilder {
        InsertTextBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct InsertTextBuilder {
    text: Option<String>,
}
impl InsertTextBuilder {
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn build(self) -> Result<InsertText, String> {
        Ok(InsertText {
            method: InsertTextMethod::InsertText,
            params: InsertTextParams {
                text: self
                    .text
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
            },
        })
    }
}
impl ImeSetComposition {
    pub fn builder() -> ImeSetCompositionBuilder {
        ImeSetCompositionBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct ImeSetCompositionBuilder {
    text: Option<String>,
    selection_start: Option<i64>,
    selection_end: Option<i64>,
    replacement_start: Option<i64>,
    replacement_end: Option<i64>,
}
impl ImeSetCompositionBuilder {
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    pub fn selection_start(mut self, selection_start: impl Into<i64>) -> Self {
        self.selection_start = Some(selection_start.into());
        self
    }
    pub fn selection_end(mut self, selection_end: impl Into<i64>) -> Self {
        self.selection_end = Some(selection_end.into());
        self
    }
    pub fn replacement_start(mut self, replacement_start: impl Into<i64>) -> Self {
        self.replacement_start = Some(replacement_start.into());
        self
    }
    pub fn replacement_end(mut self, replacement_end: impl Into<i64>) -> Self {
        self.replacement_end = Some(replacement_end.into());
        self
    }
    pub fn build(self) -> Result<ImeSetComposition, String> {
        Ok(ImeSetComposition {
            method: ImeSetCompositionMethod::ImeSetComposition,
            params: ImeSetCompositionParams {
                text: self
                    .text
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(text)))?,
                selection_start: self.selection_start.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(selection_start))
                })?,
                selection_end: self.selection_end.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(selection_end))
                })?,
                replacement_start: self.replacement_start,
                replacement_end: self.replacement_end,
            },
        })
    }
}
impl DispatchMouseEvent {
    pub fn builder() -> DispatchMouseEventBuilder {
        DispatchMouseEventBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DispatchMouseEventBuilder {
    r#type: Option<DispatchMouseEventType>,
    x: Option<f64>,
    y: Option<f64>,
    modifiers: Option<i64>,
    timestamp: Option<super::types::TimeSinceEpoch>,
    button: Option<super::types::MouseButton>,
    buttons: Option<i64>,
    click_count: Option<i64>,
    force: Option<f64>,
    tangential_pressure: Option<f64>,
    tilt_x: Option<f64>,
    tilt_y: Option<f64>,
    twist: Option<i64>,
    delta_x: Option<f64>,
    delta_y: Option<f64>,
    pointer_type: Option<DispatchMouseEventPointerType>,
}
impl DispatchMouseEventBuilder {
    pub fn r#type(mut self, r#type: impl Into<DispatchMouseEventType>) -> Self {
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
    pub fn modifiers(mut self, modifiers: impl Into<i64>) -> Self {
        self.modifiers = Some(modifiers.into());
        self
    }
    pub fn timestamp(mut self, timestamp: impl Into<super::types::TimeSinceEpoch>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
    pub fn button(mut self, button: impl Into<super::types::MouseButton>) -> Self {
        self.button = Some(button.into());
        self
    }
    pub fn buttons(mut self, buttons: impl Into<i64>) -> Self {
        self.buttons = Some(buttons.into());
        self
    }
    pub fn click_count(mut self, click_count: impl Into<i64>) -> Self {
        self.click_count = Some(click_count.into());
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
    pub fn delta_x(mut self, delta_x: impl Into<f64>) -> Self {
        self.delta_x = Some(delta_x.into());
        self
    }
    pub fn delta_y(mut self, delta_y: impl Into<f64>) -> Self {
        self.delta_y = Some(delta_y.into());
        self
    }
    pub fn pointer_type(mut self, pointer_type: impl Into<DispatchMouseEventPointerType>) -> Self {
        self.pointer_type = Some(pointer_type.into());
        self
    }
    pub fn build(self) -> Result<DispatchMouseEvent, String> {
        Ok(DispatchMouseEvent {
            method: DispatchMouseEventMethod::DispatchMouseEvent,
            params: DispatchMouseEventParams {
                r#type: self
                    .r#type
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
                x: self
                    .x
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
                y: self
                    .y
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
                modifiers: self.modifiers,
                timestamp: self.timestamp,
                button: self.button,
                buttons: self.buttons,
                click_count: self.click_count,
                force: self.force,
                tangential_pressure: self.tangential_pressure,
                tilt_x: self.tilt_x,
                tilt_y: self.tilt_y,
                twist: self.twist,
                delta_x: self.delta_x,
                delta_y: self.delta_y,
                pointer_type: self.pointer_type,
            },
        })
    }
}
impl DispatchTouchEvent {
    pub fn builder() -> DispatchTouchEventBuilder {
        DispatchTouchEventBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct DispatchTouchEventBuilder {
    r#type: Option<DispatchTouchEventType>,
    touch_points: Option<Vec<super::types::TouchPoint>>,
    modifiers: Option<i64>,
    timestamp: Option<super::types::TimeSinceEpoch>,
}
impl DispatchTouchEventBuilder {
    pub fn r#type(mut self, r#type: impl Into<DispatchTouchEventType>) -> Self {
        self.r#type = Some(r#type.into());
        self
    }
    pub fn touch_point(mut self, touch_point: impl Into<super::types::TouchPoint>) -> Self {
        let v = self.touch_points.get_or_insert(Vec::new());
        v.push(touch_point.into());
        self
    }
    pub fn touch_points<I, S>(mut self, touch_points: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<super::types::TouchPoint>,
    {
        let v = self.touch_points.get_or_insert(Vec::new());
        for val in touch_points {
            v.push(val.into());
        }
        self
    }
    pub fn modifiers(mut self, modifiers: impl Into<i64>) -> Self {
        self.modifiers = Some(modifiers.into());
        self
    }
    pub fn timestamp(mut self, timestamp: impl Into<super::types::TimeSinceEpoch>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
    pub fn build(self) -> Result<DispatchTouchEvent, String> {
        Ok(DispatchTouchEvent {
            method: DispatchTouchEventMethod::DispatchTouchEvent,
            params: DispatchTouchEventParams {
                r#type: self
                    .r#type
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
                touch_points: self.touch_points.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(touch_points))
                })?,
                modifiers: self.modifiers,
                timestamp: self.timestamp,
            },
        })
    }
}
impl EmulateTouchFromMouseEvent {
    pub fn builder() -> EmulateTouchFromMouseEventBuilder {
        EmulateTouchFromMouseEventBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct EmulateTouchFromMouseEventBuilder {
    r#type: Option<EmulateTouchFromMouseEventType>,
    x: Option<i64>,
    y: Option<i64>,
    button: Option<super::types::MouseButton>,
    timestamp: Option<super::types::TimeSinceEpoch>,
    delta_x: Option<f64>,
    delta_y: Option<f64>,
    modifiers: Option<i64>,
    click_count: Option<i64>,
}
impl EmulateTouchFromMouseEventBuilder {
    pub fn r#type(mut self, r#type: impl Into<EmulateTouchFromMouseEventType>) -> Self {
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
    pub fn button(mut self, button: impl Into<super::types::MouseButton>) -> Self {
        self.button = Some(button.into());
        self
    }
    pub fn timestamp(mut self, timestamp: impl Into<super::types::TimeSinceEpoch>) -> Self {
        self.timestamp = Some(timestamp.into());
        self
    }
    pub fn delta_x(mut self, delta_x: impl Into<f64>) -> Self {
        self.delta_x = Some(delta_x.into());
        self
    }
    pub fn delta_y(mut self, delta_y: impl Into<f64>) -> Self {
        self.delta_y = Some(delta_y.into());
        self
    }
    pub fn modifiers(mut self, modifiers: impl Into<i64>) -> Self {
        self.modifiers = Some(modifiers.into());
        self
    }
    pub fn click_count(mut self, click_count: impl Into<i64>) -> Self {
        self.click_count = Some(click_count.into());
        self
    }
    pub fn build(self) -> Result<EmulateTouchFromMouseEvent, String> {
        Ok(EmulateTouchFromMouseEvent {
            method: EmulateTouchFromMouseEventMethod::EmulateTouchFromMouseEvent,
            params: EmulateTouchFromMouseEventParams {
                r#type: self
                    .r#type
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(r#type)))?,
                x: self
                    .x
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
                y: self
                    .y
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
                button: self
                    .button
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(button)))?,
                timestamp: self.timestamp,
                delta_x: self.delta_x,
                delta_y: self.delta_y,
                modifiers: self.modifiers,
                click_count: self.click_count,
            },
        })
    }
}
impl SetIgnoreInputEvents {
    pub fn builder() -> SetIgnoreInputEventsBuilder {
        SetIgnoreInputEventsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetIgnoreInputEventsBuilder {
    ignore: Option<bool>,
}
impl SetIgnoreInputEventsBuilder {
    pub fn ignore(mut self, ignore: impl Into<bool>) -> Self {
        self.ignore = Some(ignore.into());
        self
    }
    pub fn build(self) -> Result<SetIgnoreInputEvents, String> {
        Ok(SetIgnoreInputEvents {
            method: SetIgnoreInputEventsMethod::SetIgnoreInputEvents,
            params: SetIgnoreInputEventsParams {
                ignore: self
                    .ignore
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(ignore)))?,
            },
        })
    }
}
impl SetInterceptDrags {
    pub fn builder() -> SetInterceptDragsBuilder {
        SetInterceptDragsBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SetInterceptDragsBuilder {
    enabled: Option<bool>,
}
impl SetInterceptDragsBuilder {
    pub fn enabled(mut self, enabled: impl Into<bool>) -> Self {
        self.enabled = Some(enabled.into());
        self
    }
    pub fn build(self) -> Result<SetInterceptDrags, String> {
        Ok(SetInterceptDrags {
            method: SetInterceptDragsMethod::SetInterceptDrags,
            params: SetInterceptDragsParams {
                enabled: self
                    .enabled
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(enabled)))?,
            },
        })
    }
}
impl SynthesizePinchGesture {
    pub fn builder() -> SynthesizePinchGestureBuilder {
        SynthesizePinchGestureBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SynthesizePinchGestureBuilder {
    x: Option<f64>,
    y: Option<f64>,
    scale_factor: Option<f64>,
    relative_speed: Option<i64>,
    gesture_source_type: Option<super::types::GestureSourceType>,
}
impl SynthesizePinchGestureBuilder {
    pub fn x(mut self, x: impl Into<f64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<f64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn scale_factor(mut self, scale_factor: impl Into<f64>) -> Self {
        self.scale_factor = Some(scale_factor.into());
        self
    }
    pub fn relative_speed(mut self, relative_speed: impl Into<i64>) -> Self {
        self.relative_speed = Some(relative_speed.into());
        self
    }
    pub fn gesture_source_type(
        mut self,
        gesture_source_type: impl Into<super::types::GestureSourceType>,
    ) -> Self {
        self.gesture_source_type = Some(gesture_source_type.into());
        self
    }
    pub fn build(self) -> Result<SynthesizePinchGesture, String> {
        Ok(SynthesizePinchGesture {
            method: SynthesizePinchGestureMethod::SynthesizePinchGesture,
            params: SynthesizePinchGestureParams {
                x: self
                    .x
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
                y: self
                    .y
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
                scale_factor: self.scale_factor.ok_or_else(|| {
                    format!("Field `{}` is mandatory.", std::stringify!(scale_factor))
                })?,
                relative_speed: self.relative_speed,
                gesture_source_type: self.gesture_source_type,
            },
        })
    }
}
impl SynthesizeScrollGesture {
    pub fn builder() -> SynthesizeScrollGestureBuilder {
        SynthesizeScrollGestureBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SynthesizeScrollGestureBuilder {
    x: Option<f64>,
    y: Option<f64>,
    x_distance: Option<f64>,
    y_distance: Option<f64>,
    x_overscroll: Option<f64>,
    y_overscroll: Option<f64>,
    prevent_fling: Option<bool>,
    speed: Option<i64>,
    gesture_source_type: Option<super::types::GestureSourceType>,
    repeat_count: Option<i64>,
    repeat_delay_ms: Option<i64>,
    interaction_marker_name: Option<String>,
}
impl SynthesizeScrollGestureBuilder {
    pub fn x(mut self, x: impl Into<f64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<f64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn x_distance(mut self, x_distance: impl Into<f64>) -> Self {
        self.x_distance = Some(x_distance.into());
        self
    }
    pub fn y_distance(mut self, y_distance: impl Into<f64>) -> Self {
        self.y_distance = Some(y_distance.into());
        self
    }
    pub fn x_overscroll(mut self, x_overscroll: impl Into<f64>) -> Self {
        self.x_overscroll = Some(x_overscroll.into());
        self
    }
    pub fn y_overscroll(mut self, y_overscroll: impl Into<f64>) -> Self {
        self.y_overscroll = Some(y_overscroll.into());
        self
    }
    pub fn prevent_fling(mut self, prevent_fling: impl Into<bool>) -> Self {
        self.prevent_fling = Some(prevent_fling.into());
        self
    }
    pub fn speed(mut self, speed: impl Into<i64>) -> Self {
        self.speed = Some(speed.into());
        self
    }
    pub fn gesture_source_type(
        mut self,
        gesture_source_type: impl Into<super::types::GestureSourceType>,
    ) -> Self {
        self.gesture_source_type = Some(gesture_source_type.into());
        self
    }
    pub fn repeat_count(mut self, repeat_count: impl Into<i64>) -> Self {
        self.repeat_count = Some(repeat_count.into());
        self
    }
    pub fn repeat_delay_ms(mut self, repeat_delay_ms: impl Into<i64>) -> Self {
        self.repeat_delay_ms = Some(repeat_delay_ms.into());
        self
    }
    pub fn interaction_marker_name(mut self, interaction_marker_name: impl Into<String>) -> Self {
        self.interaction_marker_name = Some(interaction_marker_name.into());
        self
    }
    pub fn build(self) -> Result<SynthesizeScrollGesture, String> {
        Ok(SynthesizeScrollGesture {
            method: SynthesizeScrollGestureMethod::SynthesizeScrollGesture,
            params: SynthesizeScrollGestureParams {
                x: self
                    .x
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
                y: self
                    .y
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
                x_distance: self.x_distance,
                y_distance: self.y_distance,
                x_overscroll: self.x_overscroll,
                y_overscroll: self.y_overscroll,
                prevent_fling: self.prevent_fling,
                speed: self.speed,
                gesture_source_type: self.gesture_source_type,
                repeat_count: self.repeat_count,
                repeat_delay_ms: self.repeat_delay_ms,
                interaction_marker_name: self.interaction_marker_name,
            },
        })
    }
}
impl SynthesizeTapGesture {
    pub fn builder() -> SynthesizeTapGestureBuilder {
        SynthesizeTapGestureBuilder::default()
    }
}
#[derive(Default, Clone)]
pub struct SynthesizeTapGestureBuilder {
    x: Option<f64>,
    y: Option<f64>,
    duration: Option<i64>,
    tap_count: Option<i64>,
    gesture_source_type: Option<super::types::GestureSourceType>,
}
impl SynthesizeTapGestureBuilder {
    pub fn x(mut self, x: impl Into<f64>) -> Self {
        self.x = Some(x.into());
        self
    }
    pub fn y(mut self, y: impl Into<f64>) -> Self {
        self.y = Some(y.into());
        self
    }
    pub fn duration(mut self, duration: impl Into<i64>) -> Self {
        self.duration = Some(duration.into());
        self
    }
    pub fn tap_count(mut self, tap_count: impl Into<i64>) -> Self {
        self.tap_count = Some(tap_count.into());
        self
    }
    pub fn gesture_source_type(
        mut self,
        gesture_source_type: impl Into<super::types::GestureSourceType>,
    ) -> Self {
        self.gesture_source_type = Some(gesture_source_type.into());
        self
    }
    pub fn build(self) -> Result<SynthesizeTapGesture, String> {
        Ok(SynthesizeTapGesture {
            method: SynthesizeTapGestureMethod::SynthesizeTapGesture,
            params: SynthesizeTapGestureParams {
                x: self
                    .x
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(x)))?,
                y: self
                    .y
                    .ok_or_else(|| format!("Field `{}` is mandatory.", std::stringify!(y)))?,
                duration: self.duration,
                tap_count: self.tap_count,
                gesture_source_type: self.gesture_source_type,
            },
        })
    }
}
