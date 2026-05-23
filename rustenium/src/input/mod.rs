mod bidi;
pub mod cdp;
mod human_mouse;
mod human_touchscreen;
mod keyboard;
mod mouse;
mod touch;
mod trajectory;

pub use crate::input::{
    bidi::{
        keyboard::{
            BidiKeyboard, DelayRange, KeyPressOptions, KeyPressOptionsBuilder, KeyboardTypeOptions,
            KeyboardTypeOptionsBuilder,
        },
        mouse::BidiMouse,
        touchscreen::{TouchHandle, TouchMoveOptions, TouchMoveOptionsBuilder, Touchscreen},
    },
    cdp::{CdpKeyboard, CdpMouse, CdpTouchHandle, CdpTouchscreen},
    human_mouse::HumanMouse,
    human_touchscreen::HumanTouchscreen,
    keyboard::Keyboard,
    mouse::{
        Mouse, MouseButton, MouseClickOptions, MouseClickOptionsBuilder, MouseMoveOptions,
        MouseMoveOptionsBuilder, MouseOptions, MouseOptionsBuilder, MouseWheelOptions,
        MouseWheelOptionsBuilder, Point,
    },
    touch::{
        ScrollOptions, ScrollOptionsBuilder, SwipeOptions, SwipeOptionsBuilder, Touch, Viewport,
    },
    trajectory::{CurveParams, generate_durations, generate_trajectory, random_curve_params},
};
