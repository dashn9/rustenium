mod bidi;
mod mouse;
pub mod trajectory;
mod touch;
mod human_mouse;
mod human_touchscreen;

pub use crate::input::{
    trajectory::{
        bezier_point, bezier_curve, binomial, generate_trajectory, generate_durations,
        random_curve_params, gauss, weighted_pick, CurveParams, EasingFn, EASING_FUNCTIONS, linear,
    },
    human_mouse::HumanMouse,
    human_touchscreen::HumanTouchscreen,
    touch::{Touch, SwipeOptions, ScrollOptions, Viewport},
    bidi::{
        mouse::BidiMouse,
        keyboard::{Keyboard, KeyboardTypeOptions, KeyPressOptions},
        touchscreen::{Touchscreen, TouchHandle, TouchMoveOptions}
    },
    mouse::{Point, Mouse, MouseButton, MouseClickOptions, MouseMoveOptions, MouseOptions, MouseWheelOptions}
};
