use rustenium_bidi_commands::browsing_context::types::BrowsingContext;
use crate::error::InputError;

/// Options for mouse movement
#[derive(Debug, Clone, Default)]
pub struct MouseMoveOptions {
    /// Number of intermediate steps for the move
    pub steps: Option<usize>,
    /// Origin for the movement
    pub origin: Option<rustenium_bidi_commands::input::types::Origin>,
}

/// Options for mouse click
#[derive(Debug, Clone, Default)]
pub struct MouseClickOptions {
    /// Mouse button to click
    pub button: Option<MouseButton>,
    /// Number of clicks
    pub count: Option<u64>,
    /// Delay in milliseconds between mousedown and mouseup
    pub delay: Option<u64>,
    /// Origin for the click
    pub origin: Option<rustenium_bidi_commands::input::types::Origin>,
}

/// Options for mouse button actions
#[derive(Debug, Clone, Default)]
pub struct MouseOptions {
    /// Mouse button
    pub button: Option<MouseButton>,
}

/// Options for mouse wheel
#[derive(Debug, Clone, Default)]
pub struct MouseWheelOptions {
    /// Delta X for horizontal scroll
    pub delta_x: Option<i64>,
    /// Delta Y for vertical scroll
    pub delta_y: Option<i64>,
}

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

/// Trait for mouse input behavior
pub trait Mouse {
    /// Reset the mouse state
    async fn reset(&self, context: &BrowsingContext) -> Result<(), InputError>;

    /// Move the mouse to a position
    async fn move_to(
        &self,
        x: f64,
        y: f64,
        context: &BrowsingContext,
        options: Option<MouseMoveOptions>,
    ) -> Result<(), InputError>;

    /// Press a mouse button down
    async fn down(
        &self,
        context: &BrowsingContext,
        options: Option<MouseOptions>,
    ) -> Result<(), InputError>;

    /// Release a mouse button
    async fn up(
        &self,
        context: &BrowsingContext,
        options: Option<MouseOptions>,
    ) -> Result<(), InputError>;

    /// Click at a position
    async fn click(
        &self,
        x: f64,
        y: f64,
        context: &BrowsingContext,
        options: Option<MouseClickOptions>,
    ) -> Result<(), InputError>;

    /// Scroll the mouse wheel
    async fn wheel(
        &self,
        context: &BrowsingContext,
        options: Option<MouseWheelOptions>,
    ) -> Result<(), InputError>;
}
