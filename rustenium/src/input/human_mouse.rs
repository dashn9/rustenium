use crate::error::bidi::InputError;
use rustenium_bidi_definitions::browsing_context::types::BrowsingContext;
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

    pub fn get_last_position(&self) -> Point {
        self.mouse.get_last_position()
    }
}

impl<M: Mouse> Mouse for HumanMouse<M> {
    fn get_last_position(&self) -> Point {
        self.mouse.get_last_position()
    }

    fn set_last_position(&self, point: Point) {
        self.mouse.set_last_position(point)
    }

    async fn reset(&self, context: &BrowsingContext) -> Result<(), InputError> {
        tracing::info!("mouse reset");
        let result = self.mouse.reset(context).await;
        tracing::info!("mouse reset done");
        result
    }

    async fn move_to(
        &self,
        point: Point,
        context: &BrowsingContext,
        options: MouseMoveOptions,
    ) -> Result<(), InputError> {
        let from = self.mouse.get_last_position();
        let to = Point { x: point.x.round(), y: point.y.round() };

        tracing::info!(from_x = from.x, from_y = from.y, to_x = to.x, to_y = to.y, "mouse move_to start");

        let dist = ((to.x - from.x).powi(2) + (to.y - from.y).powi(2)).sqrt();
        if dist < 1.0 {
            let result = self.mouse.move_to(to, context, options).await;
            tracing::info!("mouse move_to done");
            return result;
        }

        let params = random_curve_params(from, to);
        let traj = generate_trajectory(from, to, &params);

        for (i, pt) in traj.points.iter().enumerate() {
            tracing::debug!(x = pt.x, y = pt.y, step = i, "mouse move_to step");
            self.mouse.move_to(
                Point { x: pt.x.max(0.0), y: pt.y.max(0.0) },
                context,
                MouseMoveOptions { steps: Some(1), origin: options.origin.clone() },
            ).await?;

            if i < traj.step_delays_ms.len() {
                tokio::time::sleep(tokio::time::Duration::from_millis(traj.step_delays_ms[i])).await;
            }
        }

        self.mouse.move_to(to, context, MouseMoveOptions { steps: Some(1), origin: options.origin }).await?;

        tracing::info!(x = to.x, y = to.y, "mouse move_to done");
        Ok(())
    }

    async fn down(&self, context: &BrowsingContext, options: MouseOptions) -> Result<(), InputError> {
        tracing::info!(button = ?options.button, "mouse down");
        let result = self.mouse.down(context, options).await;
        tracing::info!("mouse down done");
        result
    }

    async fn up(&self, context: &BrowsingContext, options: MouseOptions) -> Result<(), InputError> {
        tracing::info!(button = ?options.button, "mouse up");
        let result = self.mouse.up(context, options).await;
        tracing::info!("mouse up done");
        result
    }

    async fn click(
        &self,
        point: Option<Point>,
        context: &BrowsingContext,
        options: MouseClickOptions,
    ) -> Result<(), InputError> {
        let count = options.count.unwrap_or(1);
        tracing::info!(x = point.map(|p| p.x), y = point.map(|p| p.y), count, button = ?options.button, "mouse click start");

        if let Some(p) = point {
            self.move_to(p, context, MouseMoveOptions::default()).await?;
        }

        let button = options.button;
        let (click_delay, pauses) = {
            let mut rng = rand::rng();
            let delay = options.delay.unwrap_or(80 + rng.random_range(0..80));
            let pauses: Vec<u64> = (0..count.saturating_sub(1)).map(|_| 100 + rng.random_range(0..100)).collect();
            (delay, pauses)
        };

        for i in 0..count {
            tracing::debug!(n = i + 1, count, "mouse click press");
            self.mouse.down(context, MouseOptions { button }).await?;
            tokio::time::sleep(tokio::time::Duration::from_millis(click_delay)).await;
            self.mouse.up(context, MouseOptions { button }).await?;

            if i < count - 1 {
                tokio::time::sleep(tokio::time::Duration::from_millis(pauses[i as usize])).await;
            }
        }

        tracing::info!("mouse click done");
        Ok(())
    }

    async fn wheel(&self, context: &BrowsingContext, options: MouseWheelOptions) -> Result<(), InputError> {
        tracing::info!(delta_x = options.delta_x, delta_y = options.delta_y, "mouse wheel start");
        let result = self.mouse.wheel(context, options).await;
        tracing::info!("mouse wheel done");
        result
    }
}

impl<M: Mouse> HumanMouse<M> {
    pub async fn scroll(
        &self,
        y_distance: i32,
        _x_distance: i32,
        context: &BrowsingContext,
    ) -> Result<(), InputError> {
        if y_distance == 0 { return Ok(()); }

        tracing::info!(y_distance, "mouse scroll start");

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

            tracing::debug!(step = i, delta_y = step, "mouse scroll step");
            self.mouse.wheel(context, MouseWheelOptions { delta_x: Some(0), delta_y: Some(step) }).await?;

            if i < steps - 1 {
                tokio::time::sleep(tokio::time::Duration::from_millis(delays[i])).await;
            }
        }

        let correction = (y_distance as f64 - accumulated).round() as i64;
        if correction != 0 {
            tracing::debug!(correction, "mouse scroll correction");
            self.mouse.wheel(context, MouseWheelOptions { delta_x: Some(0), delta_y: Some(correction) }).await?;
        }

        tracing::info!(y_distance, "mouse scroll done");
        Ok(())
    }
}
