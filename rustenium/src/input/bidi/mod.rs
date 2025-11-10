mod keyboard;
mod mouse;
mod touchscreen;

/// Input device IDs used for BiDi actions
pub(crate) const MOUSE_ID: &str = "__rustenium_mouse";
pub(crate) const KEYBOARD_ID: &str = "__rustenium_keyboard";
pub(crate) const WHEEL_ID: &str = "__rustenium_wheel";
pub(crate) const FINGER_ID_PREFIX: &str = "__rustenium_finger";

/// Mouse button enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left = 0,
    Middle = 1,
    Right = 2,
    Back = 3,
    Forward = 4,
}

/// Point representing x, y coordinates
#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
