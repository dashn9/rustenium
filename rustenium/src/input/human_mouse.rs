use crate::error::InputError;
use std::sync::Arc;
use rustenium_bidi_definitions::browsing_context::types::BrowsingContext;
use tokio::sync::Mutex;

use super::mouse::{Mouse, MouseMoveOptions, MouseClickOptions, MouseOptions, MouseWheelOptions, Point};

pub struct HumanMouse<M: Mouse> {
    mouse: M,
}

impl<M: Mouse> HumanMouse<M> {
    pub fn new(mouse: M) -> Self {
        Self { mouse }
    }

    pub fn get_last_position(&self) -> Arc<Mutex<Point>> {
        self.mouse.get_last_position()
    }

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

    fn add_jitter(value: f64) -> f64 {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hash, Hasher};

        let s = RandomState::new();
        let mut hasher = s.build_hasher();
        std::time::SystemTime::now().hash(&mut hasher);
        let random = (hasher.finish() % 1000) as f64 / 1000.0;

        value + (random - 0.5) * 3.0
    }

    fn calculate_steps(start: Point, end: Point) -> usize {
        let distance_x = end.x - start.x;
        let distance_y = end.y - start.y;
        let distance = (distance_x * distance_x + distance_y * distance_y).sqrt();
        ((distance / 4.0).ceil() as usize).max(5).min(100)
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
        options: MouseMoveOptions,
    ) -> Result<(), InputError> {
        let last_point = *self.mouse.get_last_position().lock().await;
        let to = Point { x: point.x.round(), y: point.y.round() };

        let base_steps = options.steps.unwrap_or_else(|| Self::calculate_steps(last_point, to));

        let distance_x = to.x - last_point.x;
        let distance_y = to.y - last_point.y;

        let cp1 = Point {
            x: last_point.x + distance_x * 0.25 + Self::add_jitter(0.0) * 10.0,
            y: last_point.y + distance_y * 0.25 - Self::add_jitter(0.0) * 10.0,
        };
        let cp2 = Point {
            x: last_point.x + distance_x * 0.75 - Self::add_jitter(0.0) * 10.0,
            y: last_point.y + distance_y * 0.75 + Self::add_jitter(0.0) * 10.0,
        };

        for i in 0..base_steps {
            let progress = (i as f64) / (base_steps as f64);
            let curve_point = Self::bezier_curve(progress, last_point, cp1, cp2, to);

            self.mouse.move_to(
                Point {
                    x: Self::add_jitter(curve_point.x).max(0.0),
                    y: Self::add_jitter(curve_point.y).max(0.0),
                },
                context,
                MouseMoveOptions { steps: Some(1), origin: options.origin.clone() },
            ).await?;

            tokio::time::sleep(tokio::time::Duration::from_millis(
                5 + (Self::add_jitter(0.0).abs() * 10.0) as u64
            )).await;
        }

        self.mouse.move_to(to, context, MouseMoveOptions { steps: Some(1), origin: options.origin }).await?;

        Ok(())
    }

    async fn down(&self, context: &BrowsingContext, options: MouseOptions) -> Result<(), InputError> {
        self.mouse.down(context, options).await
    }

    async fn up(&self, context: &BrowsingContext, options: MouseOptions) -> Result<(), InputError> {
        self.mouse.up(context, options).await
    }

    async fn click(
        &self,
        point: Option<Point>,
        context: &BrowsingContext,
        options: MouseClickOptions,
    ) -> Result<(), InputError> {
        if let Some(p) = point {
            self.mouse.move_to(p, context, MouseMoveOptions::default()).await?;
        }

        let click_delay = options.delay.unwrap_or(120 + (Self::add_jitter(0.0).abs() * 80.0) as u64);
        let count = options.count.unwrap_or(1);
        let button = options.button;

        for i in 0..count {
            self.mouse.down(context, MouseOptions { button }).await?;
            tokio::time::sleep(tokio::time::Duration::from_millis(click_delay)).await;
            self.mouse.up(context, MouseOptions { button }).await?;

            if i < count - 1 {
                tokio::time::sleep(tokio::time::Duration::from_millis(
                    150 + (Self::add_jitter(0.0).abs() * 100.0) as u64
                )).await;
            }
        }

        Ok(())
    }

    async fn wheel(&self, context: &BrowsingContext, options: MouseWheelOptions) -> Result<(), InputError> {
        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
        self.mouse.wheel(context, options).await
    }
}
