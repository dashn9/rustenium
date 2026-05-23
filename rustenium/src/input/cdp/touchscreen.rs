use std::sync::Arc;
use tokio::sync::Mutex;

use rustenium_cdp_definitions::browser_protocol::input::commands::{
    DispatchTouchEvent, DispatchTouchEventMethod, DispatchTouchEventParams, DispatchTouchEventType,
};
use rustenium_cdp_definitions::browser_protocol::input::types::TouchPoint;
use rustenium_core::WebsocketConnectionTransport;
use rustenium_core::error::CdpCommandResultError;
use rustenium_core::session::CdpSession;
use tokio::sync::Mutex as TokioMutex;

use crate::error::cdp::InputError;
use crate::input::mouse::Point;

/// A single active touch point, returned by [`Touchscreen::touch_start`].
pub struct TouchHandle {
    session: Arc<TokioMutex<CdpSession<WebsocketConnectionTransport>>>,
    touchscreen: Arc<Touchscreen>,
    id: usize,
    position: Arc<Mutex<Point>>,
}

impl TouchHandle {
    fn new(
        session: Arc<TokioMutex<CdpSession<WebsocketConnectionTransport>>>,
        touchscreen: Arc<Touchscreen>,
        id: usize,
        x: f64,
        y: f64,
    ) -> Self {
        Self {
            session,
            touchscreen,
            id,
            position: Arc::new(Mutex::new(Point {
                x: x.round(),
                y: y.round(),
            })),
        }
    }

    fn make_point(x: f64, y: f64, id: usize) -> TouchPoint {
        let mut p = TouchPoint::new(x.round(), y.round());
        p.id = Some(id as f64);
        p.radius_x = Some(1.0);
        p.radius_y = Some(1.0);
        p.force = Some(1.0);
        p
    }

    /// Move this touch point to a new position.
    pub async fn move_to(&self, x: f64, y: f64) -> Result<(), InputError> {
        let new_pos = Point {
            x: x.round(),
            y: y.round(),
        };
        let cmd = DispatchTouchEvent {
            method: DispatchTouchEventMethod::DispatchTouchEvent,
            params: DispatchTouchEventParams::new(
                DispatchTouchEventType::TouchMove,
                vec![Self::make_point(new_pos.x, new_pos.y, self.id)],
            ),
        };
        self.session
            .lock()
            .await
            .send(cmd)
            .await
            .map_err(|e| InputError::CommandError(CdpCommandResultError::SessionSendError(e)))?;
        *self.position.lock().await = new_pos;
        Ok(())
    }

    /// Release this touch point.
    pub async fn end(&self) -> Result<(), InputError> {
        let cmd = DispatchTouchEvent {
            method: DispatchTouchEventMethod::DispatchTouchEvent,
            params: DispatchTouchEventParams::new(DispatchTouchEventType::TouchEnd, vec![]),
        };
        self.session
            .lock()
            .await
            .send(cmd)
            .await
            .map_err(|e| InputError::CommandError(CdpCommandResultError::SessionSendError(e)))?;
        self.touchscreen.remove_handle(self.id).await;
        Ok(())
    }
}

/// Simulates touch gestures via `Input.dispatchTouchEvent`.
#[derive(Clone)]
pub struct Touchscreen {
    session: Arc<TokioMutex<CdpSession<WebsocketConnectionTransport>>>,
    touches: Arc<Mutex<Vec<usize>>>,
    id_counter: Arc<Mutex<usize>>,
}

impl Touchscreen {
    pub fn new(session: Arc<TokioMutex<CdpSession<WebsocketConnectionTransport>>>) -> Self {
        Self {
            session,
            touches: Arc::new(Mutex::new(Vec::new())),
            id_counter: Arc::new(Mutex::new(0)),
        }
    }

    /// Start a new touch at `(x, y)` and return a handle to control it.
    pub async fn touch_start(self: &Arc<Self>, x: f64, y: f64) -> Result<TouchHandle, InputError> {
        let id = {
            let mut counter = self.id_counter.lock().await;
            let id = *counter;
            *counter += 1;
            id
        };

        let mut p = TouchPoint::new(x.round(), y.round());
        p.id = Some(id as f64);
        p.radius_x = Some(1.0);
        p.radius_y = Some(1.0);
        p.force = Some(1.0);

        let cmd = DispatchTouchEvent {
            method: DispatchTouchEventMethod::DispatchTouchEvent,
            params: DispatchTouchEventParams::new(DispatchTouchEventType::TouchStart, vec![p]),
        };
        self.session
            .lock()
            .await
            .send(cmd)
            .await
            .map_err(|e| InputError::CommandError(CdpCommandResultError::SessionSendError(e)))?;

        self.touches.lock().await.push(id);

        Ok(TouchHandle::new(
            self.session.clone(),
            self.clone(),
            id,
            x,
            y,
        ))
    }

    pub(crate) async fn remove_handle(&self, id: usize) {
        self.touches.lock().await.retain(|&t| t != id);
    }
}
