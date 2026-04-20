use rustenium_bidi_definitions::browsing_context::types::BrowsingContext;
use rustenium_bidi_definitions::input::command_builders::{PerformActionsBuilder, ReleaseActionsBuilder};
use rustenium_bidi_definitions::input::type_builders::{
    PointerSourceActionsBuilder, PointerDownActionBuilder, PointerUpActionBuilder,
    PointerMoveActionBuilder, PointerCommonPropertiesBuilder, WheelSourceActionsBuilder,
    WheelScrollActionBuilder, PauseActionBuilder,
};
use rustenium_bidi_definitions::input::types::{
    PointerSourceActionsType, PointerDownActionType, PointerUpActionType, PointerMoveActionType,
    WheelSourceActionsType, WheelScrollActionType, PauseActionType,
};
use rustenium_core::BidiSession;
use rustenium_core::transport::ConnectionTransport;
use crate::error::bidi::InputError;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;

use super::MOUSE_ID;
use super::WHEEL_ID;
use crate::input::mouse::{Mouse, MouseMoveOptions, MouseClickOptions, MouseOptions, MouseWheelOptions, MouseButton, Point};

pub struct BidiMouse<OT: ConnectionTransport> {
    session: Arc<TokioMutex<BidiSession<OT>>>,
    last_position: Arc<Mutex<Point>>,
}

impl<OT: ConnectionTransport> BidiMouse<OT> {
    pub fn new(session: Arc<TokioMutex<BidiSession<OT>>>) -> Self {
        Self {
            session,
            last_position: Arc::new(Mutex::new(Point::default())),
        }
    }

    pub async fn reset(&self, context: &BrowsingContext) -> Result<(), InputError> {
        tracing::debug!("bidi mouse reset start");
        *self.last_position.try_lock().unwrap() = Point::default();

        let command = ReleaseActionsBuilder::default()
            .context(context.to_owned())
            .build().unwrap();

        self.session.lock().await.send(command).await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        tracing::debug!("bidi mouse reset done");
        Ok(())
    }

    pub async fn move_to(
        &self,
        point: Point,
        context: &BrowsingContext,
        options: MouseMoveOptions,
    ) -> Result<(), InputError> {
        tracing::debug!(x = point.x, y = point.y, "bidi mouse move_to start");
        let last_point = *self.last_position.try_lock().unwrap();
        let to = Point { x: point.x.round(), y: point.y.round() };

        let steps = options.steps.unwrap_or(0);
        let empty_props = PointerCommonPropertiesBuilder::default().build();

        let mut pointer_actions = PointerSourceActionsBuilder::default()
            .r#type(PointerSourceActionsType::Pointer)
            .id(MOUSE_ID);

        for i in 0..steps {
            let progress = (i as f64) / (steps as f64);
            tracing::debug!(step = i, progress, "bidi mouse move_to step");
            pointer_actions = pointer_actions.action(
                PointerMoveActionBuilder::default()
                    .r#type(PointerMoveActionType::PointerMove)
                    .x(last_point.x + (to.x - last_point.x) * progress)
                    .y(last_point.y + (to.y - last_point.y) * progress)
                    .pointer_common_properties(empty_props.clone())
                    .build().unwrap()
            );
        }

        let mut final_move = PointerMoveActionBuilder::default()
            .r#type(PointerMoveActionType::PointerMove)
            .x(to.x)
            .y(to.y)
            .pointer_common_properties(empty_props);

        if let Some(origin) = options.origin {
            final_move = final_move.origin(origin);
        }

        pointer_actions = pointer_actions.action(final_move.build().unwrap());

        let command = PerformActionsBuilder::default()
            .context(context.to_owned())
            .action(pointer_actions.build().unwrap())
            .build().unwrap();

        *self.last_position.try_lock().unwrap() = to;

        self.session.lock().await.send(command).await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        tracing::debug!(x = to.x, y = to.y, "bidi mouse move_to done");
        Ok(())
    }

    pub async fn down(
        &self,
        context: &BrowsingContext,
        options: MouseOptions,
    ) -> Result<(), InputError> {
        tracing::debug!(button = ?options.button, "bidi mouse down start");
        let button = options.button.unwrap_or(MouseButton::Left) as u64;

        let command = PerformActionsBuilder::default()
            .context(context.to_owned())
            .action(
                PointerSourceActionsBuilder::default()
                    .r#type(PointerSourceActionsType::Pointer)
                    .id(MOUSE_ID)
                    .action(PointerDownActionBuilder::default()
                        .r#type(PointerDownActionType::PointerDown)
                        .button(button)
                        .pointer_common_properties(PointerCommonPropertiesBuilder::default().build())
                        .build().unwrap())
                    .build().unwrap()
            )
            .build().unwrap();

        self.session.lock().await.send(command).await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        tracing::debug!("bidi mouse down done");
        Ok(())
    }

    pub async fn up(
        &self,
        context: &BrowsingContext,
        options: MouseOptions,
    ) -> Result<(), InputError> {
        tracing::debug!(button = ?options.button, "bidi mouse up start");
        let button = options.button.unwrap_or(MouseButton::Left) as u64;

        let command = PerformActionsBuilder::default()
            .context(context.to_owned())
            .action(
                PointerSourceActionsBuilder::default()
                    .r#type(PointerSourceActionsType::Pointer)
                    .id(MOUSE_ID)
                    .action(PointerUpActionBuilder::default()
                        .r#type(PointerUpActionType::PointerUp)
                        .button(button)
                        .build().unwrap())
                    .build().unwrap()
            )
            .build().unwrap();

        self.session.lock().await.send(command).await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        tracing::debug!("bidi mouse up done");
        Ok(())
    }

    pub async fn click(
        &self,
        point: Option<Point>,
        context: &BrowsingContext,
        options: MouseClickOptions,
    ) -> Result<(), InputError> {
        tracing::debug!(x = point.map(|p| p.x), y = point.map(|p| p.y), count = options.count, "bidi mouse click start");
        let button = options.button.unwrap_or(MouseButton::Left) as u64;
        let count = options.count.unwrap_or(1);

        let click_point = match point {
            Some(p) => p,
            None => *self.last_position.try_lock().unwrap(),
        };

        let pointer_down = PointerDownActionBuilder::default()
            .r#type(PointerDownActionType::PointerDown)
            .button(button)
            .pointer_common_properties(PointerCommonPropertiesBuilder::default().build())
            .build().unwrap();

        let pointer_up = PointerUpActionBuilder::default()
            .r#type(PointerUpActionType::PointerUp)
            .button(button)
            .build().unwrap();

        let mut pointer_actions = PointerSourceActionsBuilder::default()
            .r#type(PointerSourceActionsType::Pointer)
            .id(MOUSE_ID)
            .action(PointerMoveActionBuilder::default()
                .r#type(PointerMoveActionType::PointerMove)
                .x(click_point.x.round())
                .y(click_point.y.round())
                .pointer_common_properties(PointerCommonPropertiesBuilder::default().build())
                .build().unwrap());

        for _ in 1..count {
            pointer_actions = pointer_actions
                .action(pointer_down.clone())
                .action(pointer_up.clone());
        }

        pointer_actions = pointer_actions.action(pointer_down);

        if let Some(delay) = options.delay {
            if delay > 0 {
                pointer_actions = pointer_actions.action(PauseActionBuilder::default()
                    .r#type(PauseActionType::Pause)
                    .duration(delay)
                    .build().unwrap());
            }
        }

        pointer_actions = pointer_actions.action(pointer_up);

        let command = PerformActionsBuilder::default()
            .context(context.to_owned())
            .action(pointer_actions.build().unwrap())
            .build().unwrap();

        self.session.lock().await.send(command).await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        tracing::debug!("bidi mouse click done");
        Ok(())
    }

    pub async fn wheel(
        &self,
        context: &BrowsingContext,
        options: MouseWheelOptions,
    ) -> Result<(), InputError> {
        tracing::debug!(delta_x = options.delta_x, delta_y = options.delta_y, "bidi mouse wheel start");
        let last_point = *self.last_position.try_lock().unwrap();

        let command = PerformActionsBuilder::default()
            .context(context.to_owned())
            .action(
                WheelSourceActionsBuilder::default()
                    .r#type(WheelSourceActionsType::Wheel)
                    .id(WHEEL_ID)
                    .action(WheelScrollActionBuilder::default()
                        .r#type(WheelScrollActionType::Scroll)
                        .x(last_point.x as i64)
                        .y(last_point.y as i64)
                        .delta_x(options.delta_x.unwrap_or(0))
                        .delta_y(options.delta_y.unwrap_or(0))
                        .build().unwrap())
                    .build().unwrap()
            )
            .build().unwrap();

        self.session.lock().await.send(command).await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        tracing::debug!("bidi mouse wheel done");
        Ok(())
    }
}

impl<OT: ConnectionTransport> Mouse for BidiMouse<OT> {
    fn get_last_position(&self) -> Point {
        *self.last_position.try_lock().unwrap()
    }

    fn set_last_position(&self, point: Point) {
        if let Ok(mut last_point) = self.last_position.try_lock() {
            *last_point = point;
        }
    }

    async fn reset(&self, context: &BrowsingContext) -> Result<(), InputError> {
        Self::reset(self, context).await
    }

    async fn move_to(&self, point: Point, context: &BrowsingContext, options: MouseMoveOptions) -> Result<(), InputError> {
        Self::move_to(self, point, context, options).await
    }

    async fn down(&self, context: &BrowsingContext, options: MouseOptions) -> Result<(), InputError> {
        Self::down(self, context, options).await
    }

    async fn up(&self, context: &BrowsingContext, options: MouseOptions) -> Result<(), InputError> {
        Self::up(self, context, options).await
    }

    async fn click(&self, point: Option<Point>, context: &BrowsingContext, options: MouseClickOptions) -> Result<(), InputError> {
        Self::click(self, point, context, options).await
    }

    async fn wheel(&self, context: &BrowsingContext, options: MouseWheelOptions) -> Result<(), InputError> {
        Self::wheel(self, context, options).await
    }
}
