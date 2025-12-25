mod bidi;
mod mouse;

mod human_mouse;

pub use crate::input::{
    human_mouse::HumanMouse,
    bidi::{
        mouse::BidiMouse,
        keyboard::Keyboard,
        touchscreen::{Touchscreen, TouchHandle, TouchMoveOptions}
    },
    mouse::{Point, Mouse, MouseClickOptions}
};