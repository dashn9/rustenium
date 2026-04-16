mod bidi;
pub mod cdp;
mod mouse;
mod keyboard;
mod trajectory;
mod touch;
mod human_mouse;
mod human_touchscreen;

pub use crate::input::{
    trajectory::{
        generate_trajectory, generate_durations, random_curve_params, CurveParams,
    },
    human_mouse::HumanMouse,
    human_touchscreen::HumanTouchscreen,
    touch::{Touch, SwipeOptions, ScrollOptions, Viewport, SwipeOptionsBuilder, ScrollOptionsBuilder},
    bidi::{
        mouse::BidiMouse,
        keyboard::{BidiKeyboard, DelayRange, KeyPressOptions, KeyboardTypeOptions, KeyPressOptionsBuilder, KeyboardTypeOptionsBuilder},
        touchscreen::{Touchscreen, TouchHandle, TouchMoveOptions, TouchMoveOptionsBuilder}
    },
    cdp::{CdpMouse, CdpKeyboard, CdpTouchscreen, CdpTouchHandle},
    keyboard::Keyboard,
    mouse::{Point, Mouse, MouseButton, MouseClickOptions, MouseMoveOptions, MouseOptions, MouseWheelOptions,
        MouseClickOptionsBuilder, MouseMoveOptionsBuilder, MouseOptionsBuilder, MouseWheelOptionsBuilder}
};
