use rustenium_bidi_commands::browsing_context::types::BrowsingContext;
use crate::error::InputError;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::mouse::{Mouse, MouseMoveOptions, MouseClickOptions, MouseOptions, MouseWheelOptions, Point};

/// Human-like mouse implementation with realistic movements and natural behavior.
///
/// `HumanMouse` wraps another mouse implementation (typically [`BidiMouse`](crate::input::BidiMouse))
/// and adds human-like characteristics to mouse movements:
///
/// - **Bezier curves**: Movements follow natural curved paths instead of straight lines
/// - **Random jitter**: Small random variations simulate human imprecision
/// - **Variable speed**: Movement speed varies naturally along the path
/// - **Realistic delays**: Pauses between actions mimic human timing
///
/// This makes automation harder to detect as it closely mimics real user behavior.
///
/// # Examples
///
/// ```no_run
/// # use rustenium::input::{HumanMouse, BidiMouse, Point};
/// # use rustenium_bidi_commands::browsing_context::types::BrowsingContext;
/// # use std::sync::Arc;
/// # use tokio::sync::Mutex;
/// # use rustenium_core::Session;
/// # async fn example(session: Arc<Mutex<Session<rustenium_core::transport::WebsocketConnectionTransport>>>, context: BrowsingContext) -> Result<(), Box<dyn std::error::Error>> {
/// let bidi_mouse = BidiMouse::new(session);
/// let human_mouse = HumanMouse::new(bidi_mouse);
///
/// // Move with natural curve and jitter
/// human_mouse.move_to(Point { x: 500.0, y: 300.0 }, &context, None).await?;
///
/// // Click with realistic delays
/// human_mouse.click(Some(Point { x: 500.0, y: 300.0 }), &context, None).await?;
/// # Ok(())
/// # }
/// ```
pub struct HumanMouse<M: Mouse> {
    mouse: M,
}

impl<M: Mouse> HumanMouse<M> {
    /// Creates a new HumanMouse wrapping the given mouse implementation.
    pub fn new(mouse: M) -> Self {
        Self {
            mouse,
        }
    }

    /// Get the last mouse position from the underlying mouse
    pub fn get_last_position(&self) -> Arc<Mutex<Point>> {
        self.mouse.get_last_position()
    }

    /// Generate a bezier curve point for natural mouse movement
    fn bezier_curve(t: f64, p0: Point, p1: Point, p2: Point, p3: Point) -> Point {
        let one_minus_t = 1.0 - t;
        let one_minus_t_sq = one_minus_t * one_minus_t;
        let one_minus_t_cu = one_minus_t_sq * one_minus_t;
        let t_sq = t * t;
        let t_cu = t_sq * t;

        Point {
            x: one_minus_t_cu * p0.x
                + 3.0 * one_minus_t_sq * t * p1.x
                + 3.0 * one_minus_t * t_sq * p2.x
                + t_cu * p3.x,
            y: one_minus_t_cu * p0.y
                + 3.0 * one_minus_t_sq * t * p1.y
                + 3.0 * one_minus_t * t_sq * p2.y
                + t_cu * p3.y,
        }
    }

    /// Add small random jitter to coordinates
    fn add_jitter(value: f64) -> f64 {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};

        let s = RandomState::new();
        let mut hasher = s.build_hasher();
        std::time::SystemTime::now().hash(&mut hasher);
        let random = (hasher.finish() % 1000) as f64 / 1000.0;

        value + (random - 0.5) * 3.0 // +/- 1.5 pixels
    }

    /// Calculate optimal number of steps based on start and end points
    /// Aims for ~3-5 pixels per step for natural movement
    fn calculate_steps(start: Point, end: Point) -> usize {
        let distance_x = end.x - start.x;
        let distance_y = end.y - start.y;
        let distance = (distance_x * distance_x + distance_y * distance_y).sqrt();

        let calculated = (distance / 4.0).ceil() as usize;
        // Clamp between 5 and 100 steps
        calculated.max(5).min(100)
    }
}

impl<M: Mouse> Mouse for HumanMouse<M> {
    fn get_last_position(&self) -> Arc<Mutex<Point>> {
        self.mouse.get_last_position()
    }

    fn set_last_position(&self, point: Point) {
        self.mouse.set_last_position(point)
    }

    async fn reset(&self, context: &BrowsingContext) -> Result<(), InputError> {
        self.mouse.reset(context).await
    }

    async fn move_to(
        &self,
        point: Point,
        context: &BrowsingContext,
        options: Option<MouseMoveOptions>,
    ) -> Result<(), InputError> {
            let options = options.unwrap_or_default();
            let last_point = *self.mouse.get_last_position().lock().await;
            let to = Point {
                x: point.x.round(),
                y: point.y.round(),
            };

            // Use provided steps or calculate based on distance
            let base_steps = options.steps.unwrap_or_else(|| Self::calculate_steps(last_point, to));

            // Calculate control points for bezier curve
            let distance_x = to.x - last_point.x;
            let distance_y = to.y - last_point.y;

            // Add curvature to the path
            let cp1 = Point {
                x: last_point.x + distance_x * 0.25 + Self::add_jitter(0.0) * 10.0,
                y: last_point.y + distance_y * 0.25 - Self::add_jitter(0.0) * 10.0,
            };

            let cp2 = Point {
                x: last_point.x + distance_x * 0.75 - Self::add_jitter(0.0) * 10.0,
                y: last_point.y + distance_y * 0.75 + Self::add_jitter(0.0) * 10.0,
            };

            // Generate curved path with micro-adjustments using the underlying mouse
            for i in 0..base_steps {
                let progress = (i as f64) / (base_steps as f64);
                let curve_point = Self::bezier_curve(progress, last_point, cp1, cp2, to);

                // Add slight jitter to each point for human-like imprecision
                let jittered_x = Self::add_jitter(curve_point.x).max(0.0);
                let jittered_y = Self::add_jitter(curve_point.y).max(0.0);

                // Use the underlying mouse to actually move (with 1 step for direct movement)
                self.mouse.move_to(Point { x: jittered_x, y: jittered_y }, context, Some(MouseMoveOptions {
                    steps: Some(1),
                    origin: options.origin.clone(),
                })).await?;

                // Small delay between steps (5-15ms)
                tokio::time::sleep(tokio::time::Duration::from_millis(
                    5 + (Self::add_jitter(0.0).abs() * 10.0) as u64
                )).await;
            }

            // Final move to exact position
            self.mouse.move_to(to, context, Some(MouseMoveOptions {
                steps: Some(1),
                origin: options.origin.clone(),
            })).await?;

        Ok(())
    }

    async fn down(
        &self,
        context: &BrowsingContext,
        options: Option<MouseOptions>,
    ) -> Result<(), InputError> {
        self.mouse.down(context, options).await
    }

    async fn up(
        &self,
        context: &BrowsingContext,
        options: Option<MouseOptions>,
    ) -> Result<(), InputError> {
        self.mouse.up(context, options).await
    }

    async fn click(
        &self,
        point: Option<Point>,
        context: &BrowsingContext,
        options: Option<MouseClickOptions>,
    ) -> Result<(), InputError> {
            // Move to point if provided
            if let Some(p) = point {
                self.mouse.move_to(p, context, None).await?;
            }

            let options = options.unwrap_or_default();

            // Human-like click delay (100-200ms default)
            let click_delay = options.delay.unwrap_or(120 + (Self::add_jitter(0.0).abs() * 80.0) as u64);

            let count = options.count.unwrap_or(1);
            let button = options.button;

            // Perform clicks with delays
            for i in 0..count {
                self.mouse.down(context, Some(MouseOptions { button })).await?;

                tokio::time::sleep(tokio::time::Duration::from_millis(click_delay)).await;

                self.mouse.up(context, Some(MouseOptions { button })).await?;

                // Add delay between multiple clicks
                if i < count - 1 {
                    tokio::time::sleep(tokio::time::Duration::from_millis(
                        150 + (Self::add_jitter(0.0).abs() * 100.0) as u64
                    )).await;
                }
        }

        Ok(())
    }

    async fn wheel(
        &self,
        context: &BrowsingContext,
        options: Option<MouseWheelOptions>,
    ) -> Result<(), InputError> {
        // Add a slight delay for human-like scrolling
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        self.mouse.wheel(context, options).await
    }
}
