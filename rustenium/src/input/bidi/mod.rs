pub mod keyboard;
pub mod mouse;
pub mod touchscreen;

/// Input device IDs used for BiDi actions
pub(crate) const MOUSE_ID: &str = "__rustenium_mouse";
pub(crate) const KEYBOARD_ID: &str = "__rustenium_keyboard";
pub(crate) const WHEEL_ID: &str = "__rustenium_wheel";
pub(crate) const FINGER_ID_PREFIX: &str = "__rustenium_finger";
