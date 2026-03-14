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
use crate::error::InputError;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::MOUSE_ID;
use super::WHEEL_ID;
use crate::input::mouse::{Mouse, MouseMoveOptions, MouseClickOptions, MouseOptions, MouseWheelOptions, MouseButton, Point};

pub struct BidiMouse<OT: ConnectionTransport> {
    session: Arc<Mutex<BidiSession<OT>>>,
    last_move_point: Arc<Mutex<Point>>,
}

impl<OT: ConnectionTransport> BidiMouse<OT> {
    pub fn new(session: Arc<Mutex<BidiSession<OT>>>) -> Self {
        Self {
            session,
            last_move_point: Arc::new(Mutex::new(Point::default())),
        }
    }

    pub async fn reset(&self, context: &BrowsingContext) -> Result<(), InputError> {
        *self.last_move_point.lock().await = Point::default();

        let command = ReleaseActionsBuilder::default()
            .context(context.to_owned())
            .build().unwrap();

        self.session.lock().await.send(command).await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        Ok(())
    }

    pub async fn move_to(
        &self,
        point: Point,
        context: &BrowsingContext,
        options: MouseMoveOptions,
    ) -> Result<(), InputError> {
        let last_point = *self.last_move_point.lock().await;
        let to = Point { x: point.x.round(), y: point.y.round() };

        let steps = options.steps.unwrap_or(0);
        let empty_props = PointerCommonPropertiesBuilder::default().build();

        let mut pointer_actions = PointerSourceActionsBuilder::default()
            .r#type(PointerSourceActionsType::Pointer)
            .id(MOUSE_ID);

        for i in 0..steps {
            let progress = (i as f64) / (steps as f64);
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

        *self.last_move_point.lock().await = to;

        self.session.lock().await.send(command).await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        Ok(())
    }

    pub async fn down(
        &self,
        context: &BrowsingContext,
        options: MouseOptions,
    ) -> Result<(), InputError> {
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
        Ok(())
    }

    pub async fn up(
        &self,
        context: &BrowsingContext,
        options: MouseOptions,
    ) -> Result<(), InputError> {
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
        Ok(())
    }

    pub async fn click(
        &self,
        point: Option<Point>,
        context: &BrowsingContext,
        options: MouseClickOptions,
    ) -> Result<(), InputError> {
        let button = options.button.unwrap_or(MouseButton::Left) as u64;
        let count = options.count.unwrap_or(1);

        let click_point = match point {
            Some(p) => p,
            None => *self.last_move_point.lock().await,
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
        Ok(())
    }

    pub async fn wheel(
        &self,
        context: &BrowsingContext,
        options: MouseWheelOptions,
    ) -> Result<(), InputError> {
        let last_point = *self.last_move_point.lock().await;

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
        Ok(())
    }
}

impl<OT: ConnectionTransport> Mouse for BidiMouse<OT> {
    fn get_last_position(&self) -> Arc<Mutex<Point>> {
        self.last_move_point.clone()
    }

    fn set_last_position(&self, point: Point) {
        if let Ok(mut last_point) = self.last_move_point.try_lock() {
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
