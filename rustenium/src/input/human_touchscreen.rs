use std::sync::Arc;
use rustenium_bidi_definitions::browsing_context::types::BrowsingContext;
use rustenium_core::transport::ConnectionTransport;
use rand::Rng;

use crate::error::bidi::InputError;
use super::bidi::touchscreen::Touchscreen;
use super::mouse::Point;
use super::touch::{Touch, SwipeOptions, ScrollOptions, Viewport};
use super::trajectory::{
    generate_trajectory, generate_durations, random_curve_params, weighted_pick,
};

// ── 9-zone grid ─────────────────────────────────────────────────────────────
//
//  +--------+--------+--------+
//  |  TL    |  TC    |  TR    |   top
//  +--------+--------+--------+
//  |  ML    |  MC    |  MR    |   middle
//  +--------+--------+--------+
//  |  BL    |  BC    |  BR    |   bottom    ← thumb rests here
//  +--------+--------+--------+

const ZONE_WEIGHTS: &[(usize, f64)] = &[
    (0, 0.02), // TL
    (1, 0.05), // TC
    (2, 0.02), // TR
    (3, 0.05), // ML
    (4, 0.15), // MC
    (5, 0.05), // MR
    (6, 0.07), // BL
    (7, 0.45), // BC
    (8, 0.07), // BR
];

fn zone_origin(rng: &mut impl Rng, viewport: &Viewport) -> Point {
    let zone = weighted_pick(rng, ZONE_WEIGHTS);
    let col = zone % 3;
    let row = zone / 3;
    let w3 = viewport.width / 3.0;
    let h3 = viewport.height / 3.0;
    let x_lo = col as f64 * w3;
    let y_lo = row as f64 * h3;
    Point {
        x: rng.random_range(x_lo..(x_lo + w3)),
        y: rng.random_range(y_lo..(y_lo + h3)),
    }
}

// ── HumanTouchscreen ────────────────────────────────────────────────────────

pub struct HumanTouchscreen<OT: ConnectionTransport> {
    touchscreen: Arc<Touchscreen<OT>>,
}

impl<OT: ConnectionTransport> HumanTouchscreen<OT> {
    pub fn new(touchscreen: Arc<Touchscreen<OT>>) -> Self {
        Self { touchscreen }
    }

    async fn swipe_internal(
        &self,
        from: Point,
        to: Point,
        duration_ms: u64,
        context: &BrowsingContext,
    ) -> Result<(), InputError> {
        let params = random_curve_params(from, to);
        let trajectory = generate_trajectory(from, to, &params);

        if trajectory.is_empty() {
            return Ok(());
        }

        let durations = generate_durations(
            trajectory.len(),
            duration_ms as f64 / 1000.0,
            (0.004, 0.025),
        );

        let first = trajectory[0];
        let handle = self.touchscreen
            .touch_start(first.x.max(0.0), first.y.max(0.0), context, None)
            .await?;

        for (i, pt) in trajectory.iter().enumerate().skip(1) {
            tokio::time::sleep(tokio::time::Duration::from_secs_f64(durations[i])).await;
            handle.move_to(pt.x.max(0.0), pt.y.max(0.0), context).await?;
        }

        let mut rng = rand::rng();
        let lift_delay = 10 + rng.random_range(0..30);
        tokio::time::sleep(tokio::time::Duration::from_millis(lift_delay)).await;

        handle.end(context).await?;
        Ok(())
    }
}

impl<OT: ConnectionTransport> Touch for HumanTouchscreen<OT> {
    async fn tap(
        &self,
        point: Point,
        context: &BrowsingContext,
    ) -> Result<(), InputError> {
        let handle = self.touchscreen.touch_start(point.x, point.y, context, None).await?;

        let mut rng = rand::rng();
        let hold = 50 + rng.random_range(0..60);
        tokio::time::sleep(tokio::time::Duration::from_millis(hold)).await;

        handle.end(context).await?;
        Ok(())
    }

    async fn swipe(
        &self,
        from: Point,
        to: Point,
        context: &BrowsingContext,
        options: SwipeOptions,
    ) -> Result<(), InputError> {
        let duration_ms = options.duration_ms.unwrap_or(600);
        self.swipe_internal(from, to, duration_ms, context).await
    }

    async fn scroll_to(
        &self,
        point: Point,
        viewport: &Viewport,
        context: &BrowsingContext,
        options: ScrollOptions,
    ) -> Result<(), InputError> {
        let duration_ms = options.duration_ms.unwrap_or(600);
        let mut rng = rand::rng();
        let origin = zone_origin(&mut rng, viewport);
        self.swipe_internal(origin, point, duration_ms, context).await
    }

    async fn long_press(
        &self,
        point: Point,
        hold_ms: u64,
        context: &BrowsingContext,
    ) -> Result<(), InputError> {
        let handle = self.touchscreen.touch_start(point.x, point.y, context, None).await?;

        let mut rng = rand::rng();
        let jitter = rng.random_range(0..50) as u64;
        tokio::time::sleep(tokio::time::Duration::from_millis(hold_ms + jitter)).await;

        handle.end(context).await?;
        Ok(())
    }
}
