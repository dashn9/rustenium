use rand::Rng;
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
use crate::input::bidi::keyboard::{DelayRange, KeyPressOptions, KeyboardTypeOptions};
use crate::input::cdp::keymap::{KeyDefinition, key_definition};

/// Per-event description resolved from a `KeyDefinition` against current
/// modifier state. Mirrors the description object puppeteer's CDP keyboard
/// builds before each `Input.dispatchKeyEvent`.
struct KeyDescription {
    key: &'static str,
    code: &'static str,
    text: Option<&'static str>,
    key_code: Option<i64>,
    location: i64,
}
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

    /// Compute the per-event description (`key`, `code`, `text`, `key_code`,
    /// `location`) for `def` against the current modifier state. Mirrors
    /// puppeteer's `_keyDescriptionForString` exactly so that, for example,
    /// `press("J")` types `J` and `press("@")` types `@` regardless of whether
    /// Shift is currently held.
    fn describe(def: &KeyDefinition, modifiers: i64) -> KeyDescription {
        let shift_active = modifiers & 8 != 0;

        let key = match (shift_active, def.shift_key) {
            (true, Some(k)) => k,
            _ => def.key.unwrap_or(""),
        };

        let key_code = if shift_active && def.shift_key_code.is_some() {
            def.shift_key_code
        } else {
            def.key_code
        };

        let code = def.code.unwrap_or("");
        let location = def.location.unwrap_or(0);

        // Default text from a single-char key value, then explicit overrides.
        let mut text: Option<&str> = if key.chars().count() == 1 {
            Some(key)
        } else {
            None
        };
        if let Some(t) = def.text {
            text = Some(t);
        }
        if shift_active && let Some(t) = def.shift_text {
            text = Some(t);
        }
        // Any modifier other than Shift suppresses text (it's a shortcut, not typing).
        if modifiers & !8 != 0 {
            text = None;
        }

        KeyDescription {
            key,
            code,
            text,
            key_code,
            location,
        }
    }

    /// Press a key down. Uses the keymap for non-printable keys; falls back gracefully.
    pub async fn down(&self, key: &str) -> Result<(), InputError> {
        tracing::debug!(key, "keyboard down start");
        let def = key_definition(key).ok_or_else(|| InputError::UnknownKey(key.to_string()))?;

        // Description is computed against the *current* modifiers, before this
        // keydown updates them — so pressing Shift itself emits a Shift keydown
        // without the shift bit influencing its own description.
        let modifiers_before = *self.modifiers.lock().unwrap();
        let desc = Self::describe(&def, modifiers_before);

        let auto_repeat = self.pressed_keys.lock().unwrap().contains(desc.code);
        self.pressed_keys
            .lock()
            .unwrap()
            .insert(desc.code.to_string());

        // Update modifier bitmask if this key is a modifier.
        *self.modifiers.lock().unwrap() |= Self::modifier_bit(desc.key);
        let modifiers_now = *self.modifiers.lock().unwrap();

        let event_type = if desc.text.is_some() {
            DispatchKeyEventType::KeyDown
        } else {
            DispatchKeyEventType::RawKeyDown
        };

        let mut builder = DispatchKeyEvent::builder()
            .r#type(event_type)
            .modifiers(modifiers_now)
            .code(desc.code)
            .key(desc.key)
            .auto_repeat(auto_repeat)
            .is_keypad(desc.location == 3);

        if let Some(kc) = desc.key_code {
            builder = builder.windows_virtual_key_code(kc);
        }
        if desc.location != 0 {
            builder = builder.location(desc.location);
        }
        if let Some(t) = desc.text {
            builder = builder.text(t).unmodified_text(t);
        }

        let cmd = builder.build().unwrap();
        self.session
            .lock()
            .await
            .send(cmd)
            .await
            .map_err(|e| InputError::CommandError(CdpCommandResultError::SessionSendError(e)))?;
        tracing::debug!(key, "keyboard down done");
        Ok(())
    }

    /// Release a key.
    pub async fn up(&self, key: &str) -> Result<(), InputError> {
        tracing::debug!(key, "keyboard up start");
        let def = key_definition(key).ok_or_else(|| InputError::UnknownKey(key.to_string()))?;

        let modifiers_before = *self.modifiers.lock().unwrap();
        let desc = Self::describe(&def, modifiers_before);

        *self.modifiers.lock().unwrap() &= !Self::modifier_bit(desc.key);
        self.pressed_keys.lock().unwrap().remove(desc.code);
        let modifiers_now = *self.modifiers.lock().unwrap();

        let mut builder = DispatchKeyEvent::builder()
            .r#type(DispatchKeyEventType::KeyUp)
            .modifiers(modifiers_now)
            .code(desc.code)
            .key(desc.key);

        if let Some(kc) = desc.key_code {
            builder = builder.windows_virtual_key_code(kc);
        }
        if desc.location != 0 {
            builder = builder.location(desc.location);
        }

        let cmd = builder.build().unwrap();
        self.session
            .lock()
            .await
            .send(cmd)
            .await
            .map_err(|e| InputError::CommandError(CdpCommandResultError::SessionSendError(e)))?;
        tracing::debug!(key, "keyboard up done");
        Ok(())
    }

    /// Press and release a key (optionally holding for `delay_ms` between down and up).
    pub async fn press(&self, key: &str, delay_ms: u64) -> Result<(), InputError> {
        tracing::debug!(key, "keyboard press start");
        self.down(key).await?;
        if delay_ms > 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
        }
        self.up(key).await?;
        tracing::debug!(key, "keyboard press done");
        Ok(())
    }

    /// Hold a key down for `hold_for` milliseconds, firing autoRepeat keydown events
    /// at random intervals sampled from `step_range`, then release.
    /// The first repeat is delayed 3–5× longer than normal to mirror OS initial-repeat latency.
    pub async fn hold_press(
        &self,
        key: &str,
        hold_for: u64,
        step_range: DelayRange,
    ) -> Result<(), InputError> {
        self.down(key).await?;
        let initial = {
            let mut rng = rand::rng();
            rng.random_range(step_range.min..=step_range.max) * rng.random_range(3u64..=5u64)
        };
        let mut elapsed = initial.min(hold_for);
        tokio::time::sleep(tokio::time::Duration::from_millis(elapsed)).await;
        while elapsed < hold_for {
            self.down(key).await?;
            let step = {
                let mut rng = rand::rng();
                rng.random_range(step_range.min..=step_range.max)
            };
            let sleep = step.min(hold_for - elapsed);
            tokio::time::sleep(tokio::time::Duration::from_millis(sleep)).await;
            elapsed += sleep;
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
        tracing::debug!(text, "keyboard type_text start");
        for ch in text.chars() {
            let s = ch.to_string();
            tracing::debug!(key = s, "keyboard type_text key");
            if key_definition(&s).is_some() {
                self.press(&s, delay_ms).await?;
            } else {
                if delay_ms > 0 {
                    tokio::time::sleep(tokio::time::Duration::from_millis(delay_ms)).await;
                }
                self.send_character(&s).await?;
            }
        }
        tracing::debug!(text, "keyboard type_text done");
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
