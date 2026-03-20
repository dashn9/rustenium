use std::future::Future;
use rustenium_bidi_definitions::browsing_context::types::BrowsingContext;
use crate::error::InputError;
use super::mouse::Point;

#[derive(Debug, Clone, Default)]
pub struct SwipeOptions {
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct Viewport {
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Default)]
pub struct ScrollOptions {
    pub duration_ms: Option<u64>,
}

pub trait Touch {
    fn tap(
        &self,
        point: Point,
        context: &BrowsingContext,
    ) -> impl Future<Output = Result<(), InputError>>;

    fn swipe(
        &self,
        from: Point,
        to: Point,
        context: &BrowsingContext,
        options: SwipeOptions,
    ) -> impl Future<Output = Result<(), InputError>>;

    /// Scroll to a point. Origin is auto-picked from the 9-zone grid.
    fn scroll_to(
        &self,
        point: Point,
        viewport: &Viewport,
        context: &BrowsingContext,
        options: ScrollOptions,
    ) -> impl Future<Output = Result<(), InputError>>;

    fn long_press(
        &self,
        point: Point,
        hold_ms: u64,
        context: &BrowsingContext,
    ) -> impl Future<Output = Result<(), InputError>>;
}
