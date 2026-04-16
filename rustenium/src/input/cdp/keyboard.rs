use rustenium_cdp_definitions::browser_protocol::input::commands::{
    DispatchKeyEvent, DispatchKeyEventType, InsertText,
};
use rustenium_core::error::CdpCommandResultError;
use rustenium_core::session::CdpSession;
use rustenium_core::transport::ConnectionTransport;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TokioMutex;

use crate::error::bidi::InputError as BidiInputError;
use crate::error::cdp::InputError;
use crate::input::Keyboard;
use crate::input::bidi::keyboard::{KeyPressOptions, KeyboardTypeOptions};
use crate::input::cdp::keymap::key_definition;
use rustenium_bidi_definitions::browsing_context::types::BrowsingContext;
use rustenium_core::error::CommandResultError;

#[derive(Clone)]
pub struct CdpKeyboard<OT: ConnectionTransport> {
    session: Arc<TokioMutex<CdpSession<OT>>>,
    /// Modifier bitmask (Alt=1, Ctrl=2, Meta=4, Shift=8) — shared with CdpMouse.
    pub modifiers: Arc<Mutex<i64>>,
    /// Tracks which key `code` values are currently held down (for autoRepeat detection).
    pressed_keys: Arc<Mutex<HashSet<String>>>,
}

impl<OT: ConnectionTransport> CdpKeyboard<OT> {
    pub fn new(session: Arc<TokioMutex<CdpSession<OT>>>, modifiers: Arc<Mutex<i64>>) -> Self {
        Self {
            session,
            modifiers,
            pressed_keys: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    fn modifier_bit(key: &str) -> i64 {
        match key {
            "Alt" => 1,
            "Control" => 2,
            "Meta" => 4,
            "Shift" => 8,
            _ => 0,
        }
    }

    /// Press a key down. Uses the keymap for non-printable keys; falls back gracefully.
    pub async fn down(&self, key: &str) -> Result<(), InputError> {
        let def = key_definition(key).ok_or_else(|| InputError::UnknownKey(key.to_string()))?;

        let auto_repeat = self.pressed_keys.lock().unwrap().contains(def.code);
        self.pressed_keys
            .lock()
            .unwrap()
            .insert(def.code.to_string());

        let shift_active = *self.modifiers.lock().unwrap() & 8 != 0;
        let effective_key = if shift_active {
            def.shift_key.unwrap_or(def.key)
        } else {
            def.key
        };
        *self.modifiers.lock().unwrap() |= Self::modifier_bit(effective_key);

        // Resolve text: single-char key → use as text, unless non-shift modifiers are held
        let text = if shift_active {
            def.shift_text.or(def.text)
        } else {
            def.text
        };
        let has_non_shift_modifier = *self.modifiers.lock().unwrap() & !8 != 0;
        let text = if has_non_shift_modifier { None } else { text };

        let event_type = if text.is_some() {
            DispatchKeyEventType::KeyDown
        } else {
            DispatchKeyEventType::RawKeyDown
        };

        let modifiers = *self.modifiers.lock().unwrap();
        let mut builder = DispatchKeyEvent::builder()
            .r#type(event_type)
            .modifiers(modifiers)
            .code(def.code)
            .key(effective_key)
            .auto_repeat(auto_repeat)
            .is_keypad(def.location.unwrap_or(0) == 3);

        if let Some(kc) = def.key_code {
            builder = builder.windows_virtual_key_code(kc);
        }
        if let Some(loc) = def.location {
            builder = builder.location(loc);
        }
        if let Some(t) = text {
            builder = builder.text(t).unmodified_text(t);
        }

        let cmd = builder.build().unwrap();
        self.session
            .lock()
            .await
            .send(cmd)
            .await
            .map_err(|e| InputError::CommandError(CdpCommandResultError::SessionSendError(e)))?;
        Ok(())
    }

    /// Release a key.
    pub async fn up(&self, key: &str) -> Result<(), InputError> {
        let def = key_definition(key).ok_or_else(|| InputError::UnknownKey(key.to_string()))?;

        let shift_active = *self.modifiers.lock().unwrap() & 8 != 0;
        let effective_key = if shift_active {
            def.shift_key.unwrap_or(def.key)
        } else {
            def.key
        };
        *self.modifiers.lock().unwrap() &= !Self::modifier_bit(effective_key);
        self.pressed_keys.lock().unwrap().remove(def.code);

        let modifiers = *self.modifiers.lock().unwrap();
        let mut builder = DispatchKeyEvent::builder()
            .r#type(DispatchKeyEventType::KeyUp)
            .modifiers(modifiers)
            .code(def.code)
            .key(effective_key);

        if let Some(kc) = def.key_code {
            builder = builder.windows_virtual_key_code(kc);
        }
        if let Some(loc) = def.location {
            builder = builder.location(loc);
        }

        let cmd = builder.build().unwrap();
        self.session
            .lock()
            .await
            .send(cmd)
            .await
            .map_err(|e| InputError::CommandError(CdpCommandResultError::SessionSendError(e)))?;
        Ok(())
    }

    /// Press and release a key (optionally holding for `delay_ms` between down and up).
    pub async fn press(&self, key: &str, delay_ms: u64) -> Result<(), InputError> {
        self.down(key).await?;
        if delay_ms > 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
        }
        self.up(key).await
    }

    /// Send a raw character via `Input.insertText` (bypasses key definitions).
    pub async fn send_character(&self, ch: &str) -> Result<(), InputError> {
        let cmd = InsertText::builder().text(ch).build().unwrap();
        self.session
            .lock()
            .await
            .send(cmd)
            .await
            .map_err(|e| InputError::CommandError(CdpCommandResultError::SessionSendError(e)))?;
        Ok(())
    }

    /// Type `text`, using `press()` for known keys and `send_character()` for others.
    /// `delay_ms` is inserted between each character.
    pub async fn type_text(&self, text: &str, delay_ms: u64) -> Result<(), InputError> {
        for ch in text.chars() {
            let s = ch.to_string();
            if key_definition(&s).is_some() {
                self.press(&s, delay_ms).await?;
            } else {
                if delay_ms > 0 {
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                }
                self.send_character(&s).await?;
            }
        }
        Ok(())
    }
}

fn to_bidi_input_error(error: InputError) -> BidiInputError {
    match error {
        InputError::UnknownKey(key) => BidiInputError::UnknownKey(key),
        InputError::CommandError(error) => {
            BidiInputError::CommandResultError(CommandResultError::InvalidResultTypeError(
                serde_json::Value::String(error.to_string()),
            ))
        }
    }
}

impl<OT: ConnectionTransport> Keyboard for CdpKeyboard<OT> {
    async fn down(&self, key: &str, _context: &BrowsingContext) -> Result<(), BidiInputError> {
        Self::down(self, key).await.map_err(to_bidi_input_error)
    }

    async fn up(&self, key: &str, _context: &BrowsingContext) -> Result<(), BidiInputError> {
        Self::up(self, key).await.map_err(to_bidi_input_error)
    }

    async fn press(
        &self,
        key: &str,
        _context: &BrowsingContext,
        options: Option<KeyPressOptions>,
    ) -> Result<(), BidiInputError> {
        let delay_ms = options
            .and_then(|options| options.delay)
            .map(|delay| delay.max)
            .unwrap_or(0);
        Self::press(self, key, delay_ms)
            .await
            .map_err(to_bidi_input_error)
    }

    async fn type_text(
        &self,
        text: &str,
        _context: &BrowsingContext,
        options: Option<KeyboardTypeOptions>,
    ) -> Result<(), BidiInputError> {
        let delay_ms = options
            .and_then(|options| options.delay)
            .map(|delay| delay.max)
            .unwrap_or(0);
        Self::type_text(self, text, delay_ms)
            .await
            .map_err(to_bidi_input_error)
    }
}
