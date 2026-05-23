use rustenium_bidi_definitions::browsing_context::types::BrowsingContext;
use rustenium_cdp_definitions::browser_protocol::input::commands::{
    DispatchMouseEvent, DispatchMouseEventType,
};
use rustenium_cdp_definitions::browser_protocol::input::types::MouseButton as CdpMouseButton;
use rustenium_core::WebsocketConnectionTransport;
use rustenium_core::error::{CdpCommandResultError, CommandResultError};
use rustenium_core::session::CdpSession;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;

use crate::error::bidi::InputError;
use crate::error::cdp::MouseInputError;
use crate::input::mouse::{
    Mouse, MouseButton, MouseClickOptions, MouseMoveOptions, MouseOptions, MouseWheelOptions, Point,
};

const FLAG_LEFT: i64 = 1;
const FLAG_RIGHT: i64 = 1 << 1;
const FLAG_MIDDLE: i64 = 1 << 2;
const FLAG_BACK: i64 = 1 << 3;
const FLAG_FORWARD: i64 = 1 << 4;

struct CdpMouseState {
    position: Point,
    buttons: i64,
}

#[derive(Clone)]
pub struct CdpMouse {
    pub session: Arc<TokioMutex<CdpSession<WebsocketConnectionTransport>>>,
    /// Modifier bitmask (Alt=1, Ctrl=2, Meta=4, Shift=8) — shared with CdpKeyboard.
    pub modifiers: Arc<Mutex<i64>>,
    state: Arc<Mutex<CdpMouseState>>,
}

impl CdpMouse {
    pub fn new(
        session: Arc<TokioMutex<CdpSession<WebsocketConnectionTransport>>>,
        modifiers: Arc<Mutex<i64>>,
    ) -> Self {
        Self {
            session,
            modifiers,
            state: Arc::new(Mutex::new(CdpMouseState {
                position: Point { x: 0.0, y: 0.0 },
                buttons: 0,
            })),
        }
    }

    fn flag(button: MouseButton) -> i64 {
        match button {
            MouseButton::Left => FLAG_LEFT,
            MouseButton::Right => FLAG_RIGHT,
            MouseButton::Middle => FLAG_MIDDLE,
            MouseButton::Back => FLAG_BACK,
            MouseButton::Forward => FLAG_FORWARD,
        }
    }

    fn to_cdp(button: MouseButton) -> CdpMouseButton {
        match button {
            MouseButton::Left => CdpMouseButton::Left,
            MouseButton::Right => CdpMouseButton::Right,
            MouseButton::Middle => CdpMouseButton::Middle,
            MouseButton::Back => CdpMouseButton::Back,
            MouseButton::Forward => CdpMouseButton::Forward,
        }
    }

    fn active_from_flags(flags: i64) -> CdpMouseButton {
        if flags & FLAG_LEFT != 0 {
            CdpMouseButton::Left
        } else if flags & FLAG_RIGHT != 0 {
            CdpMouseButton::Right
        } else if flags & FLAG_MIDDLE != 0 {
            CdpMouseButton::Middle
        } else if flags & FLAG_BACK != 0 {
            CdpMouseButton::Back
        } else if flags & FLAG_FORWARD != 0 {
            CdpMouseButton::Forward
        } else {
            CdpMouseButton::None
        }
    }

    /// Move the mouse to `(x, y)` in `steps` interpolated steps.
    pub async fn move_to(&self, point: Point, steps: usize) -> Result<(), MouseInputError> {
        tracing::debug!(x = point.x, y = point.y, steps, "cdp mouse move_to start");
        let steps = steps.max(1);
        let modifiers = *self.modifiers.lock().unwrap();
        let (from_x, from_y, buttons) = {
            let s = self.state.lock().unwrap();
            (s.position.x, s.position.y, s.buttons)
        };

        for i in 1..=steps {
            let t = i as f64 / steps as f64;
            let cx = from_x + (point.x - from_x) * t;
            let cy = from_y + (point.y - from_y) * t;
            tracing::debug!(step = i, x = cx, y = cy, "cdp mouse move_to step");
            let cmd = DispatchMouseEvent::builder()
                .r#type(DispatchMouseEventType::MouseMoved)
                .x(cx)
                .y(cy)
                .button(Self::active_from_flags(buttons))
                .buttons(buttons)
                .modifiers(modifiers)
                .build()
                .unwrap();
            self.session.lock().await.send(cmd).await.map_err(|e| {
                MouseInputError::CommandError(CdpCommandResultError::SessionSendError(e))
            })?;
        }

        {
            let mut s = self.state.lock().unwrap();
            s.position.x = point.x;
            s.position.y = point.y;
        }
        tracing::debug!(x = point.x, y = point.y, "cdp mouse move_to done");
        Ok(())
    }

    /// Press a mouse button down.
    pub async fn down(
        &self,
        point: Option<Point>,
        button: MouseButton,
        click_count: i64,
    ) -> Result<(), MouseInputError> {
        tracing::debug!(button = ?button, "cdp mouse down start");
        let flag = Self::flag(button);
        let modifiers = *self.modifiers.lock().unwrap();
        let (x, y, buttons) = {
            let mut s = self.state.lock().unwrap();
            if s.buttons & flag != 0 {
                return Err(MouseInputError::ButtonAlreadyPressed(format!(
                    "{:?}",
                    button
                )));
            }
            s.buttons |= flag;
            let (x, y) = match point {
                Some(p) => (p.x, p.y),
                None => (s.position.x, s.position.y),
            };
            (x, y, s.buttons)
        };
        let cmd = DispatchMouseEvent::builder()
            .r#type(DispatchMouseEventType::MousePressed)
            .x(x)
            .y(y)
            .button(Self::to_cdp(button))
            .buttons(buttons)
            .modifiers(modifiers)
            .click_count(click_count)
            .build()
            .unwrap();
        self.session.lock().await.send(cmd).await.map_err(|e| {
            MouseInputError::CommandError(CdpCommandResultError::SessionSendError(e))
        })?;
        tracing::debug!(button = ?button, "cdp mouse down done");
        Ok(())
    }

    /// Release a mouse button.
    pub async fn up(
        &self,
        point: Option<Point>,
        button: MouseButton,
        click_count: i64,
    ) -> Result<(), MouseInputError> {
        tracing::debug!(button = ?button, "cdp mouse up start");
        let flag = Self::flag(button);
        let modifiers = *self.modifiers.lock().unwrap();
        let (x, y, buttons) = {
            let mut s = self.state.lock().unwrap();
            if s.buttons & flag == 0 {
                return Err(MouseInputError::ButtonNotPressed(format!("{:?}", button)));
            }
            s.buttons &= !flag;
            let (x, y) = match point {
                Some(p) => (p.x, p.y),
                None => (s.position.x, s.position.y),
            };
            (x, y, s.buttons)
        };

        let cmd = DispatchMouseEvent::builder()
            .r#type(DispatchMouseEventType::MouseReleased)
            .x(x)
            .y(y)
            .button(Self::to_cdp(button))
            .buttons(buttons)
            .modifiers(modifiers)
            .click_count(click_count)
            .build()
            .unwrap();
        self.session.lock().await.send(cmd).await.map_err(|e| {
            MouseInputError::CommandError(CdpCommandResultError::SessionSendError(e))
        })?;
        tracing::debug!(button = ?button, "cdp mouse up done");
        Ok(())
    }

    /// Move to `(x, y)` then click `count` times with optional delay between press and release.
    pub async fn click(
        &self,
        point: Option<Point>,
        options: MouseClickOptions,
    ) -> Result<(), MouseInputError> {
        tracing::debug!(
            x = point.map(|p| p.x),
            y = point.map(|p| p.y),
            count = options.count,
            "cdp mouse click start"
        );
        let button = options.button.unwrap_or(MouseButton::Left);
        let count = options.count.unwrap_or(1) as i64;
        let delay_ms = options.delay.unwrap_or(0);

        for i in 1..count {
            tracing::debug!(n = i, "cdp mouse click press");
            self.down(point, button, i).await?;
            self.up(point, button, i).await?;
        }

        self.down(point, button, count).await?;
        if delay_ms > 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
        }
        self.up(point, button, count).await?;
        tracing::debug!("cdp mouse click done");
        Ok(())
    }

    /// Dispatch a mouse wheel event at the current cursor position.
    pub async fn wheel(&self, options: MouseWheelOptions) -> Result<(), MouseInputError> {
        tracing::debug!(
            delta_x = options.delta_x,
            delta_y = options.delta_y,
            "cdp mouse wheel start"
        );
        let modifiers = *self.modifiers.lock().unwrap();
        let (x, y, buttons) = {
            let s = self.state.lock().unwrap();
            (s.position.x, s.position.y, s.buttons)
        };
        let cmd = DispatchMouseEvent::builder()
            .r#type(DispatchMouseEventType::MouseWheel)
            .x(x)
            .y(y)
            .button(CdpMouseButton::None)
            .buttons(buttons)
            .modifiers(modifiers)
            .delta_x(options.delta_x.unwrap_or(0) as f64)
            .delta_y(options.delta_y.unwrap_or(0) as f64)
            .build()
            .unwrap();
        self.session.lock().await.send(cmd).await.map_err(|e| {
            MouseInputError::CommandError(CdpCommandResultError::SessionSendError(e))
        })?;
        tracing::debug!("cdp mouse wheel done");
        Ok(())
    }

    /// Reset the mouse to the origin, releasing any held buttons.
    pub async fn reset(&self) -> Result<(), MouseInputError> {
        tracing::debug!("cdp mouse reset start");
        let held: Vec<MouseButton> = {
            let s = self.state.lock().unwrap();
            [
                (FLAG_LEFT, MouseButton::Left),
                (FLAG_RIGHT, MouseButton::Right),
                (FLAG_MIDDLE, MouseButton::Middle),
                (FLAG_BACK, MouseButton::Back),
                (FLAG_FORWARD, MouseButton::Forward),
            ]
            .iter()
            .filter(|(flag, _)| s.buttons & flag != 0)
            .map(|(_, btn)| *btn)
            .collect()
        };
        for btn in held {
            tracing::debug!(button = ?btn, "cdp mouse reset releasing button");
            self.up(None, btn, 1).await?;
        }
        let pos = self.position();
        if pos.x != 0.0 || pos.y != 0.0 {
            self.move_to(Point { x: 0.0, y: 0.0 }, 0).await?;
        }
        tracing::debug!("cdp mouse reset done");
        Ok(())
    }

    pub fn position(&self) -> Point {
        let s = self.state.lock().unwrap();
        s.position
    }
}

// ── Mouse trait impl ──────────────────────────────────────────────────────────

fn to_input_err(e: impl std::fmt::Display) -> InputError {
    InputError::CommandResultError(CommandResultError::InvalidResultTypeError(
        serde_json::Value::String(e.to_string()),
    ))
}

impl Mouse for CdpMouse {
    fn get_last_position(&self) -> Point {
        self.position()
    }

    fn set_last_position(&self, point: Point) {
        if let Ok(mut state) = self.state.try_lock() {
            state.position = point;
        }
    }

    async fn reset(&self, _context: &BrowsingContext) -> Result<(), InputError> {
        self.reset().await.map_err(to_input_err)
    }

    async fn move_to(
        &self,
        point: Point,
        _context: &BrowsingContext,
        options: MouseMoveOptions,
    ) -> Result<(), InputError> {
        let steps = options.steps.unwrap_or(1).max(1);
        self.move_to(point, steps)
            .await
            .map_err(to_input_err)
    }

    async fn down(
        &self,
        _context: &BrowsingContext,
        options: MouseOptions,
    ) -> Result<(), InputError> {
        let button = options.button.unwrap_or(MouseButton::Left);
        self.down(None, button, 1)
            .await
            .map_err(to_input_err)
    }

    async fn up(
        &self,
        _context: &BrowsingContext,
        options: MouseOptions,
    ) -> Result<(), InputError> {
        let button = options.button.unwrap_or(MouseButton::Left);
        self.up(None, button, 1).await.map_err(to_input_err)
    }

    async fn click(
        &self,
        point: Option<Point>,
        _context: &BrowsingContext,
        options: MouseClickOptions,
    ) -> Result<(), InputError> {
        self.click(point, options)
            .await
            .map_err(to_input_err)
    }

    async fn wheel(
        &self,
        _context: &BrowsingContext,
        options: MouseWheelOptions,
    ) -> Result<(), InputError> {
        self.wheel(options).await.map_err(to_input_err)
    }
}
