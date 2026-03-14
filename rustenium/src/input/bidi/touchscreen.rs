use rustenium_bidi_definitions::browsing_context::types::BrowsingContext;
use rustenium_bidi_definitions::input::command_builders::PerformActionsBuilder;
use rustenium_bidi_definitions::input::type_builders::{
    PointerSourceActionsBuilder, PointerDownActionBuilder, PointerUpActionBuilder,
    PointerMoveActionBuilder, PointerCommonPropertiesBuilder, PointerParametersBuilder,
};
use rustenium_bidi_definitions::input::types::{
    PointerSourceActionsType, PointerDownActionType, PointerUpActionType,
    PointerMoveActionType, PointerCommonProperties, PointerType, Origin,
};
use rustenium_core::BidiSession;
use rustenium_core::transport::ConnectionTransport;
use crate::error::InputError;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::input::mouse::Point;
use super::{FINGER_ID_PREFIX};

/// Options for touch movement operations.
#[derive(Debug, Clone, Default)]
pub struct TouchMoveOptions {
    /// Origin for the touch movement.
    pub origin: Option<Origin>,
}

/// Handle representing a single touch point for multi-touch gestures.
///
/// Each `TouchHandle` represents one finger/touch point. Create multiple handles
/// to simulate multi-touch gestures like pinch, zoom, or multi-finger swipes.
pub struct TouchHandle<OT: ConnectionTransport> {
    session: Arc<Mutex<BidiSession<OT>>>,
    touchscreen: Arc<Touchscreen<OT>>,
    id: usize,
    bidi_id: String,
    position: Arc<Mutex<Point>>,
    started: Arc<Mutex<bool>>,
    properties: PointerCommonProperties,
}

impl<OT: ConnectionTransport> TouchHandle<OT> {
    pub(crate) fn new(
        session: Arc<Mutex<BidiSession<OT>>>,
        touchscreen: Arc<Touchscreen<OT>>,
        id: usize,
        x: f64,
        y: f64,
    ) -> Self {
        let properties = PointerCommonPropertiesBuilder::default()
            .width(1u64)
            .height(1u64)
            .pressure(0.5f64)
            .altitude_angle(std::f64::consts::PI / 2.0)
            .build();

        Self {
            session,
            touchscreen,
            id,
            bidi_id: format!("{}_{}", FINGER_ID_PREFIX, id),
            position: Arc::new(Mutex::new(Point { x: x.round(), y: y.round() })),
            started: Arc::new(Mutex::new(false)),
            properties,
        }
    }

    /// Start the touch at the handle's initial position.
    ///
    /// This performs a touch down event at the coordinates specified when the handle was created.
    /// Must be called before `move_to` or `end`.
    ///
    /// # Errors
    /// Returns `InputError::TouchAlreadyStarted` if this handle has already been started.
    pub async fn start(
        &self,
        context: &BrowsingContext,
        options: Option<TouchMoveOptions>,
    ) -> Result<(), InputError> {
        let mut started = self.started.lock().await;
        if *started {
            return Err(InputError::TouchAlreadyStarted);
        }

        let options = options.unwrap_or_default();
        let position = *self.position.lock().await;

        let mut move_action = PointerMoveActionBuilder::default()
            .r#type(PointerMoveActionType::PointerMove)
            .x(position.x)
            .y(position.y)
            .pointer_common_properties(PointerCommonPropertiesBuilder::default().build());

        if let Some(origin) = options.origin {
            move_action = move_action.origin(origin);
        }

        let command = PerformActionsBuilder::default()
            .context(context.clone())
            .action(
                PointerSourceActionsBuilder::default()
                    .r#type(PointerSourceActionsType::Pointer)
                    .id(self.bidi_id.clone())
                    .parameters(PointerParametersBuilder::default().pointer_type(PointerType::Touch).build())
                    .action(move_action.build().unwrap())
                    .action(PointerDownActionBuilder::default()
                        .r#type(PointerDownActionType::PointerDown)
                        .button(0u64)
                        .pointer_common_properties(self.properties.clone())
                        .build().unwrap())
                    .build().unwrap()
            )
            .build().unwrap();

        let mut session = self.session.lock().await;
        session.send(command)
            .await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        *started = true;
        Ok(())
    }

    /// Move the touch to a new position.
    ///
    /// Simulates dragging the touch point from its current position to the new coordinates.
    /// The touch must be started first with `start()`.
    ///
    /// # Arguments
    /// * `x` - Target X coordinate in pixels
    /// * `y` - Target Y coordinate in pixels
    /// * `context` - The browsing context to perform the touch in
    pub async fn move_to(
        &self,
        x: f64,
        y: f64,
        context: &BrowsingContext,
    ) -> Result<(), InputError> {
        let new_position = Point { x: x.round(), y: y.round() };

        let command = PerformActionsBuilder::default()
            .context(context.clone())
            .action(
                PointerSourceActionsBuilder::default()
                    .r#type(PointerSourceActionsType::Pointer)
                    .id(self.bidi_id.clone())
                    .parameters(PointerParametersBuilder::default().pointer_type(PointerType::Touch).build())
                    .action(PointerMoveActionBuilder::default()
                        .r#type(PointerMoveActionType::PointerMove)
                        .x(new_position.x)
                        .y(new_position.y)
                        .pointer_common_properties(self.properties.clone())
                        .build().unwrap())
                    .build().unwrap()
            )
            .build().unwrap();

        *self.position.lock().await = new_position;

        let mut session = self.session.lock().await;
        session.send(command)
            .await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        Ok(())
    }

    /// End the touch by releasing it.
    ///
    /// This performs a touch up event, completing the touch gesture. After calling this method,
    /// the handle is automatically removed from the touchscreen and cannot be reused.
    pub async fn end(&self, context: &BrowsingContext) -> Result<(), InputError> {
        let command = PerformActionsBuilder::default()
            .context(context.clone())
            .action(
                PointerSourceActionsBuilder::default()
                    .r#type(PointerSourceActionsType::Pointer)
                    .id(self.bidi_id.clone())
                    .parameters(PointerParametersBuilder::default().pointer_type(PointerType::Touch).build())
                    .action(PointerUpActionBuilder::default()
                        .r#type(PointerUpActionType::PointerUp)
                        .button(0u64)
                        .build().unwrap())
                    .build().unwrap()
            )
            .build().unwrap();

        let mut session = self.session.lock().await;
        session.send(command)
            .await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;

        // Remove this handle from the touchscreen
        self.touchscreen.remove_handle(self.id).await;
        Ok(())
    }
}

/// BiDi Touchscreen implementation for simulating multi-touch gestures.
///
/// `Touchscreen` manages multiple touch points simultaneously, allowing you to simulate
/// complex multi-touch gestures like pinch-to-zoom, multi-finger swipes, and other
/// touch interactions.
///
/// # Examples
///
/// ```no_run
/// # use rustenium::input::{Touchscreen, TouchMoveOptions};
/// # use rustenium_bidi_commands::browsing_context::types::BrowsingContext;
/// # use std::sync::Arc;
/// # use tokio::sync::Mutex;
/// # use rustenium_core::Session;
/// # async fn example(session: Arc<Mutex<Session<rustenium_core::transport::WebsocketConnectionTransport>>>, context: BrowsingContext) -> Result<(), Box<dyn std::error::Error>> {
/// let touchscreen = Arc::new(Touchscreen::new(session));
///
/// // Simulate a pinch gesture with two fingers
/// let touch1 = touchscreen.touch_start(100.0, 200.0, &context, None).await?;
/// let touch2 = touchscreen.touch_start(300.0, 200.0, &context, None).await?;
///
/// // Move fingers closer together
/// touch1.move_to(150.0, 200.0, &context).await?;
/// touch2.move_to(250.0, 200.0, &context).await?;
///
/// // Release both touches
/// touch1.end(&context).await?;
/// touch2.end(&context).await?;
/// # Ok(())
/// # }
/// ```
pub struct Touchscreen<OT: ConnectionTransport> {
    session: Arc<Mutex<BidiSession<OT>>>,
    touches: Arc<Mutex<Vec<usize>>>,
    id_counter: Arc<Mutex<usize>>,
}

impl<OT: ConnectionTransport> Touchscreen<OT> {
    /// Creates a new Touchscreen instance.
    pub fn new(session: Arc<Mutex<BidiSession<OT>>>) -> Self {
        Self {
            session,
            touches: Arc::new(Mutex::new(Vec::new())),
            id_counter: Arc::new(Mutex::new(0)),
        }
    }

    /// Start a new touch at the given position and return a handle to control it.
    ///
    /// Creates a new touch point and immediately performs a touch down event at the specified
    /// coordinates. Returns a `TouchHandle` that can be used to move or end the touch.
    ///
    /// # Arguments
    /// * `x` - X coordinate in pixels where the touch starts
    /// * `y` - Y coordinate in pixels where the touch starts
    /// * `context` - The browsing context to perform the touch in
    /// * `options` - Optional touch movement options
    ///
    /// # Returns
    /// A `TouchHandle` that represents this touch point and can be used to control its movement.
    pub async fn touch_start(
        self: &Arc<Self>,
        x: f64,
        y: f64,
        context: &BrowsingContext,
        options: Option<TouchMoveOptions>,
    ) -> Result<TouchHandle<OT>, InputError> {
        let mut counter = self.id_counter.lock().await;
        let id = *counter;
        *counter += 1;
        drop(counter);

        let touch = TouchHandle::new(
            self.session.clone(),
            self.clone(),
            id,
            x,
            y,
        );

        touch.start(context, options).await?;

        let mut touches = self.touches.lock().await;
        touches.push(id);

        Ok(touch)
    }

    /// Remove a touch handle (called internally by TouchHandle::end)
    pub(crate) async fn remove_handle(&self, id: usize) {
        let mut touches = self.touches.lock().await;
        touches.retain(|&touch_id| touch_id != id);
    }
}

impl<OT: ConnectionTransport> Clone for Touchscreen<OT> {
    fn clone(&self) -> Self {
        Self {
            session: self.session.clone(),
            touches: self.touches.clone(),
            id_counter: self.id_counter.clone(),
        }
    }
}
