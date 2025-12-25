use std::future::Future;
use std::sync::Arc;
use tokio::sync::Mutex;
use rustenium_bidi_commands::browsing_context::types::BrowsingContext;
use crate::error::InputError;

/// Options for mouse movement operations.
#[derive(Debug, Clone, Default)]
pub struct MouseMoveOptions {
    /// Number of intermediate steps for smooth movement (used by BidiMouse).
    pub steps: Option<usize>,
    /// Origin point for relative movements.
    pub origin: Option<rustenium_bidi_commands::input::types::Origin>,
}

/// Options for mouse click operations.
#[derive(Debug, Clone, Default)]
pub struct MouseClickOptions {
    /// Mouse button to click (default: left button).
    pub button: Option<MouseButton>,
    /// Number of clicks to perform (default: 1, use 2 for double-click).
    pub count: Option<u64>,
    /// Delay in milliseconds between mousedown and mouseup events.
    pub delay: Option<u64>,
    /// Origin point for the click.
    pub origin: Option<rustenium_bidi_commands::input::types::Origin>,
}

/// Options for mouse button press/release operations.
#[derive(Debug, Clone, Default)]
pub struct MouseOptions {
    /// Mouse button to press or release.
    pub button: Option<MouseButton>,
}

/// Options for mouse wheel scrolling.
#[derive(Debug, Clone, Default)]
pub struct MouseWheelOptions {
    /// Horizontal scroll delta (positive = right, negative = left).
    pub delta_x: Option<i64>,
    /// Vertical scroll delta (positive = down, negative = up).
    pub delta_y: Option<i64>,
}

/// Mouse button identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    /// Primary button (usually left).
    Left = 0,
    /// Middle button (wheel).
    Middle = 1,
    /// Secondary button (usually right).
    Right = 2,
    /// Back button (browser back).
    Back = 3,
    /// Forward button (browser forward).
    Forward = 4,
}

/// A point in 2D space with x and y coordinates.
#[derive(Debug, Clone, Copy, Default)]
pub struct Point {
    /// X coordinate in pixels.
    pub x: f64,
    /// Y coordinate in pixels.
    pub y: f64,
}

/// Trait defining mouse input behavior.
///
/// Implementations include:
/// - [`BidiMouse`](crate::input::BidiMouse) - Direct, precise movements
/// - [`HumanMouse`](crate::input::HumanMouse) - Realistic, human-like movements with curves and jitter
pub trait Mouse {
    /// Get the last mouse position
    fn get_last_position(&self) -> Arc<Mutex<Point>>;

    /// Set the last mouse position
    fn set_last_position(&self, point: Point);

    /// Reset the mouse state
    fn reset(&self, context: &BrowsingContext) -> impl Future<Output = Result<(), InputError>>;

    /// Move the mouse to a position
    fn move_to(
        &self,
        point: Point,
        context: &BrowsingContext,
        options: Option<MouseMoveOptions>,
    ) -> impl Future<Output = Result<(), InputError>>;

    /// Press a mouse button down
    fn down(
        &self,
        context: &BrowsingContext,
        options: Option<MouseOptions>,
    ) -> impl Future<Output = Result<(), InputError>>;

    /// Release a mouse button
    fn up(
        &self,
        context: &BrowsingContext,
        options: Option<MouseOptions>,
    ) -> impl Future<Output = Result<(), InputError>>;

    /// Click at a position (or last position if point is None)
    fn click(
        &self,
        point: Option<Point>,
        context: &BrowsingContext,
        options: Option<MouseClickOptions>,
    ) -> impl Future<Output = Result<(), InputError>>;

    /// Scroll the mouse wheel
    fn wheel(
        &self,
        context: &BrowsingContext,
        options: Option<MouseWheelOptions>,
    ) -> impl Future<Output = Result<(), InputError>>;
}
