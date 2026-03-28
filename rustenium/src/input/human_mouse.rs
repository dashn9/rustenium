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
