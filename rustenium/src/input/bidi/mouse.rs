use rustenium_bidi_commands::browsing_context::types::BrowsingContext;
use rustenium_bidi_commands::input::commands::{
    InputCommand, PerformActions, PerformActionsParameters, ReleaseActions,
    ReleaseActionsParameters, InputPerformActionsMethod, InputReleaseActionsMethod,
};
use rustenium_bidi_commands::input::types::{
    PointerSourceActions, PointerSourceAction, PointerDownAction, PointerUpAction,
    PointerMoveAction, WheelSourceActions, WheelSourceAction, WheelScrollAction,
    SourceActions, PointerEnum, WheelEnum, PointerDownEnum, PointerUpEnum,
    PointerMoveEnum, PauseEnum, ScrollEnum, PointerCommonProperties, PauseAction,
};
use rustenium_bidi_commands::CommandData;
use rustenium_core::Session;
use rustenium_core::transport::ConnectionTransport;
use crate::error::InputError;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::MOUSE_ID;
use super::WHEEL_ID;
use crate::input::mouse::{MouseTrait, MouseMoveOptions, MouseClickOptions, MouseOptions, MouseWheelOptions, MouseButton, Point};

/// BiDi Mouse implementation - direct, precise movements
pub struct Mouse<OT: ConnectionTransport> {
    session: Arc<Mutex<Session<OT>>>,
    last_move_point: Arc<Mutex<Point>>,
}

impl<OT: ConnectionTransport> Mouse<OT> {
    /// Create a new Mouse instance
    pub fn new(session: Arc<Mutex<Session<OT>>>) -> Self {
        Self {
            session,
            last_move_point: Arc::new(Mutex::new(Point::default())),
        }
    }

    /// Reset the mouse state
    pub async fn reset(&self, context: &BrowsingContext) -> Result<(), InputError> {
        let mut last_point = self.last_move_point.lock().await;
        *last_point = Point::default();

        let command = InputCommand::ReleaseActions(ReleaseActions {
            method: InputReleaseActionsMethod::InputReleaseActions,
            params: ReleaseActionsParameters {
                context: context.clone(),
            },
        });

        let mut session = self.session.lock().await;
        session.send(CommandData::InputCommand(command))
            .await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        Ok(())
    }

    /// Move the mouse to a position
    pub async fn move_to(
        &self,
        x: f64,
        y: f64,
        context: &BrowsingContext,
        options: Option<MouseMoveOptions>,
    ) -> Result<(), InputError> {
        let options = options.unwrap_or_default();
        let last_point = *self.last_move_point.lock().await;
        let to = Point {
            x: x.round(),
            y: y.round(),
        };

        let mut actions = Vec::new();
        let steps = options.steps.unwrap_or(0);

        // Generate intermediate steps
        for i in 0..steps {
            let progress = (i as f64) / (steps as f64);
            actions.push(PointerSourceAction::PointerMoveAction(PointerMoveAction {
                r#type: PointerMoveEnum::PointerMove,
                x: last_point.x + (to.x - last_point.x) * progress,
                y: last_point.y + (to.y - last_point.y) * progress,
                duration: None,
                origin: options.origin.clone(),
                pointer_common_properties: PointerCommonProperties {
                    width: None,
                    height: None,
                    pressure: None,
                    tangential_pressure: None,
                    twist: None,
                    altitude_angle: None,
                    azimuth_angle: None,
                },
            }));
        }

        // Final move to exact position
        actions.push(PointerSourceAction::PointerMoveAction(PointerMoveAction {
            r#type: PointerMoveEnum::PointerMove,
            x: to.x,
            y: to.y,
            duration: None,
            origin: options.origin.clone(),
            pointer_common_properties: PointerCommonProperties {
                width: None,
                height: None,
                pressure: None,
                tangential_pressure: None,
                twist: None,
                altitude_angle: None,
                azimuth_angle: None,
            },
        }));

        let command = InputCommand::PerformActions(PerformActions {
            method: InputPerformActionsMethod::InputPerformActions,
            params: PerformActionsParameters {
                context: context.clone(),
                actions: vec![SourceActions::PointerSourceActions(PointerSourceActions {
                    r#type: PointerEnum::Pointer,
                    id: MOUSE_ID.to_string(),
                    parameters: None,
                    actions,
                })],
            },
        });

        // Update last position
        *self.last_move_point.lock().await = to;

        let mut session = self.session.lock().await;
        session.send(CommandData::InputCommand(command))
            .await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        Ok(())
    }

    /// Press a mouse button down
    pub async fn down(
        &self,
        context: &BrowsingContext,
        options: Option<MouseOptions>,
    ) -> Result<(), InputError> {
        let options = options.unwrap_or_default();
        let button = options.button.unwrap_or(MouseButton::Left) as u64;

        let command = InputCommand::PerformActions(PerformActions {
            method: InputPerformActionsMethod::InputPerformActions,
            params: PerformActionsParameters {
                context: context.clone(),
                actions: vec![SourceActions::PointerSourceActions(PointerSourceActions {
                    r#type: PointerEnum::Pointer,
                    id: MOUSE_ID.to_string(),
                    parameters: None,
                    actions: vec![PointerSourceAction::PointerDownAction(PointerDownAction {
                        r#type: PointerDownEnum::PointerDown,
                        button,
                        pointer_common_properties: PointerCommonProperties {
                            width: None,
                            height: None,
                            pressure: None,
                            tangential_pressure: None,
                            twist: None,
                            altitude_angle: None,
                            azimuth_angle: None,
                        },
                    })],
                })],
            },
        });

        let mut session = self.session.lock().await;
        session.send(CommandData::InputCommand(command))
            .await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        Ok(())
    }

    /// Release a mouse button
    pub async fn up(
        &self,
        context: &BrowsingContext,
        options: Option<MouseOptions>,
    ) -> Result<(), InputError> {
        let options = options.unwrap_or_default();
        let button = options.button.unwrap_or(MouseButton::Left) as u64;

        let command = InputCommand::PerformActions(PerformActions {
            method: InputPerformActionsMethod::InputPerformActions,
            params: PerformActionsParameters {
                context: context.clone(),
                actions: vec![SourceActions::PointerSourceActions(PointerSourceActions {
                    r#type: PointerEnum::Pointer,
                    id: MOUSE_ID.to_string(),
                    parameters: None,
                    actions: vec![PointerSourceAction::PointerUpAction(PointerUpAction {
                        r#type: PointerUpEnum::PointerUp,
                        button,
                    })],
                })],
            },
        });

        let mut session = self.session.lock().await;
        session.send(CommandData::InputCommand(command))
            .await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        Ok(())
    }

    /// Click at a position
    pub async fn click(
        &self,
        x: f64,
        y: f64,
        context: &BrowsingContext,
        options: Option<MouseClickOptions>,
    ) -> Result<(), InputError> {
        let options = options.unwrap_or_default();
        let button = options.button.unwrap_or(MouseButton::Left) as u64;
        let count = options.count.unwrap_or(1);

        let mut actions = vec![PointerSourceAction::PointerMoveAction(PointerMoveAction {
            r#type: PointerMoveEnum::PointerMove,
            x: x.round(),
            y: y.round(),
            duration: None,
            origin: options.origin.clone(),
            pointer_common_properties: PointerCommonProperties {
                width: None,
                height: None,
                pressure: None,
                tangential_pressure: None,
                twist: None,
                altitude_angle: None,
                azimuth_angle: None,
            },
        })];

        let pointer_down = PointerSourceAction::PointerDownAction(PointerDownAction {
            r#type: PointerDownEnum::PointerDown,
            button,
            pointer_common_properties: PointerCommonProperties {
                width: None,
                height: None,
                pressure: None,
                tangential_pressure: None,
                twist: None,
                altitude_angle: None,
                azimuth_angle: None,
            },
        });

        let pointer_up = PointerSourceAction::PointerUpAction(PointerUpAction {
            r#type: PointerUpEnum::PointerUp,
            button,
        });

        // Add multiple clicks if count > 1
        for i in 1..count {
            actions.push(pointer_down.clone());
            actions.push(pointer_up.clone());
        }

        // Final click
        actions.push(pointer_down);

        if let Some(delay) = options.delay {
            if delay > 0 {
                actions.push(PointerSourceAction::PauseAction(PauseAction {
                    r#type: PauseEnum::Pause,
                    duration: Some(delay),
                }));
            }
        }

        actions.push(pointer_up);

        let command = InputCommand::PerformActions(PerformActions {
            method: InputPerformActionsMethod::InputPerformActions,
            params: PerformActionsParameters {
                context: context.clone(),
                actions: vec![SourceActions::PointerSourceActions(PointerSourceActions {
                    r#type: PointerEnum::Pointer,
                    id: MOUSE_ID.to_string(),
                    parameters: None,
                    actions,
                })],
            },
        });

        let mut session = self.session.lock().await;
        session.send(CommandData::InputCommand(command))
            .await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        Ok(())
    }

    /// Scroll the mouse wheel
    pub async fn wheel(
        &self,
        context: &BrowsingContext,
        options: Option<MouseWheelOptions>,
    ) -> Result<(), InputError> {
        let options = options.unwrap_or_default();
        let last_point = *self.last_move_point.lock().await;

        let command = InputCommand::PerformActions(PerformActions {
            method: InputPerformActionsMethod::InputPerformActions,
            params: PerformActionsParameters {
                context: context.clone(),
                actions: vec![SourceActions::WheelSourceActions(WheelSourceActions {
                    r#type: WheelEnum::Wheel,
                    id: WHEEL_ID.to_string(),
                    actions: vec![WheelSourceAction::WheelScrollAction(WheelScrollAction {
                        r#type: ScrollEnum::Scroll,
                        x: last_point.x as i64,
                        y: last_point.y as i64,
                        delta_x: options.delta_x.unwrap_or(0),
                        delta_y: options.delta_y.unwrap_or(0),
                        duration: None,
                        origin: None,
                    })],
                })],
            },
        });

        let mut session = self.session.lock().await;
        session.send(CommandData::InputCommand(command))
            .await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        Ok(())
    }
}

impl<OT: ConnectionTransport> MouseTrait for Mouse<OT> {
    async fn reset(&self, context: &BrowsingContext) -> Result<(), InputError> {
        Self::reset(self, context).await
    }

    async fn move_to(
        &self,
        x: f64,
        y: f64,
        context: &BrowsingContext,
        options: Option<MouseMoveOptions>,
    ) -> Result<(), InputError> {
        Self::move_to(self, x, y, context, options).await
    }

    async fn down(
        &self,
        context: &BrowsingContext,
        options: Option<MouseOptions>,
    ) -> Result<(), InputError> {
        Self::down(self, context, options).await
    }

    async fn up(
        &self,
        context: &BrowsingContext,
        options: Option<MouseOptions>,
    ) -> Result<(), InputError> {
        Self::up(self, context, options).await
    }

    async fn click(
        &self,
        x: f64,
        y: f64,
        context: &BrowsingContext,
        options: Option<MouseClickOptions>,
    ) -> Result<(), InputError> {
        Self::click(self, x, y, context, options).await
    }

    async fn wheel(
        &self,
        context: &BrowsingContext,
        options: Option<MouseWheelOptions>,
    ) -> Result<(), InputError> {
        Self::wheel(self, context, options).await
    }
}
