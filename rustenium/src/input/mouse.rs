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

#[derive(Debug, Clone, Default)]
pub struct MouseClickOptions {
    pub button: Option<MouseButton>,
    pub count: Option<u64>,
    pub delay: Option<u64>,
    pub origin: Option<rustenium_bidi_definitions::input::types::Origin>,
}

#[derive(Debug, Clone, Default)]
pub struct MouseOptions {
    pub button: Option<MouseButton>,
}

#[derive(Debug, Clone, Default)]
pub struct MouseWheelOptions {
    pub delta_x: Option<i64>,
    pub delta_y: Option<i64>,
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
