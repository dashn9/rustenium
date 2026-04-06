use crate::error::bidi::InputError;
use std::sync::Arc;
use rustenium_bidi_definitions::browsing_context::types::BrowsingContext;
use tokio::sync::Mutex;
use rand::Rng;

use super::mouse::{Mouse, MouseMoveOptions, MouseClickOptions, MouseOptions, MouseWheelOptions, Point};
use super::trajectory::{random_curve_params, generate_trajectory};

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
        let from = *self.mouse.get_last_position().lock().await;
        let to = Point { x: point.x.round(), y: point.y.round() };

        let params = random_curve_params(from, to);
        let trajectory = generate_trajectory(from, to, &params);

        for pt in &trajectory {
            self.mouse.move_to(
                Point { x: pt.x.max(0.0), y: pt.y.max(0.0) },
                context,
                MouseMoveOptions { steps: Some(1), origin: options.origin.clone() },
            ).await?;
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
            self.move_to(p, context, MouseMoveOptions::default()).await?;
        }

        let mut rng = rand::rng();
        let click_delay = options.delay.unwrap_or(80 + rng.random_range(0..80));
        let count = options.count.unwrap_or(1);
        let button = options.button;

        for i in 0..count {
            self.mouse.down(context, MouseOptions { button }).await?;
            tokio::time::sleep(tokio::time::Duration::from_millis(click_delay)).await;
            self.mouse.up(context, MouseOptions { button }).await?;

            if i < count - 1 {
                let pause = 100 + rng.random_range(0..100);
                tokio::time::sleep(tokio::time::Duration::from_millis(pause)).await;
            }
        }

        Ok(())
    }

    async fn wheel(&self, context: &BrowsingContext, options: MouseWheelOptions) -> Result<(), InputError> {
        self.mouse.wheel(context, options).await
    }
}

impl<M: Mouse> HumanMouse<M> {
    /// Scrolls vertically by `y_distance` pixels.
    ///
    /// Positive values scroll down; negative values scroll up.
    /// `x_distance` is reserved for future horizontal scroll support and is currently ignored.
    ///
    /// The total distance is split into ease-out steps with ±15 % per-step noise,
    /// followed by a correction step to reach the exact target.
    pub async fn scroll(
        &self,
        y_distance: i32,
        _x_distance: i32,
        context: &BrowsingContext,
    ) -> Result<(), InputError> {
        if y_distance == 0 { return Ok(()); }

        let total = y_distance.unsigned_abs() as f64;
        let sign = y_distance.signum() as i64;
        let steps = ((total / 40.0) as usize).max(3).min(20);

        let ease = |t: f64| -> f64 {
            if t >= 1.0 { 1.0 } else { 1.0 - f64::powf(2.0, -10.0 * t) }
        };

        let (noises, delays) = {
            let mut rng = rand::rng();
            let noises: Vec<f64> = (0..steps).map(|_| 1.0 + rng.random_range(-0.15_f64..0.15_f64)).collect();
            let delays: Vec<u64> = (0..steps - 1).map(|_| rng.random_range(12_u64..45_u64)).collect();
            (noises, delays)
        };

        let mut accumulated = 0.0_f64;
        for i in 0..steps {
            let t0 = i as f64 / steps as f64;
            let t1 = (i + 1) as f64 / steps as f64;
            let ideal = (ease(t1) - ease(t0)) * total;
            let step = (ideal * noises[i]).max(1.0).round() as i64 * sign;
            accumulated += step as f64;

            self.mouse.wheel(context, MouseWheelOptions { delta_x: Some(0), delta_y: Some(step) }).await?;

            if i < steps - 1 {
                tokio::time::sleep(tokio::time::Duration::from_millis(delays[i])).await;
            }
        }

        // Correction step if accumulated scroll drifted from target
        let correction = (y_distance as f64 - accumulated).round() as i64;
        if correction != 0 {
            self.mouse.wheel(context, MouseWheelOptions { delta_x: Some(0), delta_y: Some(correction) }).await?;
        }

        Ok(())
    }
}
