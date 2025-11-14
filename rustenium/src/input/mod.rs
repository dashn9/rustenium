mod bidi;
mod mouse;

mod human_mouse;

pub use crate::input::{human_mouse::HumanMouse, bidi::{mouse::BidiMouse, keyboard::Keyboard}, mouse::{Point, Mouse, MouseClickOptions}};