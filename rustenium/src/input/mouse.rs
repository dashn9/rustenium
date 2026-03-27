use std::future::Future;
use std::sync::Arc;
use rustenium_bidi_definitions::browsing_context::types::BrowsingContext;
use tokio::sync::Mutex;
use crate::error::InputError;

#[derive(Debug, Clone, Default)]
pub struct MouseMoveOptions {
    pub steps: Option<usize>,
    pub origin: Option<rustenium_bidi_definitions::input::types::Origin>,
}


#[derive(Default, Clone)]
pub struct MouseMoveOptionsBuilder {
    steps: Option<usize>,
    origin: Option<rustenium_bidi_definitions::input::types::Origin>,
}

impl MouseMoveOptionsBuilder {
    pub fn steps(mut self, v: usize) -> Self { self.steps = Some(v); self }
    pub fn origin(mut self, v: rustenium_bidi_definitions::input::types::Origin) -> Self { self.origin = Some(v); self }
    pub fn build(self) -> MouseMoveOptions {
        MouseMoveOptions { steps: self.steps, origin: self.origin }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MouseClickOptions {
    pub button: Option<MouseButton>,
    pub count: Option<u64>,
    pub delay: Option<u64>,
    pub origin: Option<rustenium_bidi_definitions::input::types::Origin>,
}


#[derive(Default, Clone)]
pub struct MouseClickOptionsBuilder {
    button: Option<MouseButton>,
    count: Option<u64>,
    delay: Option<u64>,
    origin: Option<rustenium_bidi_definitions::input::types::Origin>,
}

impl MouseClickOptionsBuilder {
    pub fn button(mut self, v: MouseButton) -> Self { self.button = Some(v); self }
    pub fn count(mut self, v: u64) -> Self { self.count = Some(v); self }
    pub fn delay(mut self, v: u64) -> Self { self.delay = Some(v); self }
    pub fn origin(mut self, v: rustenium_bidi_definitions::input::types::Origin) -> Self { self.origin = Some(v); self }
    pub fn build(self) -> MouseClickOptions {
        MouseClickOptions { button: self.button, count: self.count, delay: self.delay, origin: self.origin }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MouseOptions {
    pub button: Option<MouseButton>,
}


#[derive(Default, Clone)]
pub struct MouseOptionsBuilder {
    button: Option<MouseButton>,
}

impl MouseOptionsBuilder {
    pub fn button(mut self, v: MouseButton) -> Self { self.button = Some(v); self }
    pub fn build(self) -> MouseOptions {
        MouseOptions { button: self.button }
    }
}

#[derive(Debug, Clone, Default)]
pub struct MouseWheelOptions {
    pub delta_x: Option<i64>,
    pub delta_y: Option<i64>,
}


#[derive(Default, Clone)]
pub struct MouseWheelOptionsBuilder {
    delta_x: Option<i64>,
    delta_y: Option<i64>,
}

impl MouseWheelOptionsBuilder {
    pub fn delta_x(mut self, v: i64) -> Self { self.delta_x = Some(v); self }
    pub fn delta_y(mut self, v: i64) -> Self { self.delta_y = Some(v); self }
    pub fn build(self) -> MouseWheelOptions {
        MouseWheelOptions { delta_x: self.delta_x, delta_y: self.delta_y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left = 0,
    Middle = 1,
    Right = 2,
    Back = 3,
    Forward = 4,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub trait Mouse {
    fn get_last_position(&self) -> Arc<Mutex<Point>>;
    fn set_last_position(&self, point: Point);

    fn reset(&self, context: &BrowsingContext) -> impl Future<Output = Result<(), InputError>>;

    fn move_to(
        &self,
        point: Point,
        context: &BrowsingContext,
        options: MouseMoveOptions,
    ) -> impl Future<Output = Result<(), InputError>>;

    fn down(
        &self,
        context: &BrowsingContext,
        options: MouseOptions,
    ) -> impl Future<Output = Result<(), InputError>>;

    fn up(
        &self,
        context: &BrowsingContext,
        options: MouseOptions,
    ) -> impl Future<Output = Result<(), InputError>>;

    fn click(
        &self,
        point: Option<Point>,
        context: &BrowsingContext,
        options: MouseClickOptions,
    ) -> impl Future<Output = Result<(), InputError>>;

    fn wheel(
        &self,
        context: &BrowsingContext,
        options: MouseWheelOptions,
    ) -> impl Future<Output = Result<(), InputError>>;
}
