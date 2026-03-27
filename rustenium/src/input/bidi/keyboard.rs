use rustenium_bidi_definitions::browsing_context::types::BrowsingContext;
use rustenium_bidi_definitions::input::command_builders::PerformActionsBuilder;
use rustenium_bidi_definitions::input::type_builders::{
    KeySourceActionsBuilder, KeyDownActionBuilder, KeyUpActionBuilder, PauseActionBuilder,
};
use rustenium_bidi_definitions::input::types::{
    KeySourceActionsType, KeyDownActionType, KeyUpActionType, PauseActionType,
};
use rustenium_core::BidiSession;
use rustenium_core::transport::ConnectionTransport;
use crate::error::InputError;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::KEYBOARD_ID;

/// Options for keyboard key press operations.
#[derive(Debug, Clone, Default)]
pub struct KeyPressOptions {
    pub delay: Option<u64>,
}


#[derive(Default, Clone)]
pub struct KeyPressOptionsBuilder {
    delay: Option<u64>,
}

impl KeyPressOptionsBuilder {
    pub fn delay(mut self, v: u64) -> Self { self.delay = Some(v); self }
    pub fn build(self) -> KeyPressOptions { KeyPressOptions { delay: self.delay } }
}

/// Options for keyboard text typing operations.
#[derive(Debug, Clone, Default)]
pub struct KeyboardTypeOptions {
    pub delay: Option<u64>,
}


#[derive(Default, Clone)]
pub struct KeyboardTypeOptionsBuilder {
    delay: Option<u64>,
}

impl KeyboardTypeOptionsBuilder {
    pub fn delay(mut self, v: u64) -> Self { self.delay = Some(v); self }
    pub fn build(self) -> KeyboardTypeOptions { KeyboardTypeOptions { delay: self.delay } }
}

pub(crate) fn get_bidi_key_value(key: &str) -> Result<String, InputError> {
    let key = match key {
        "\r" | "\n" => "Enter",
        _ => key,
    };

    if key.chars().count() == 1 {
        return Ok(key.to_string());
    }

    let value = match key {
        "Cancel" => "\u{E001}",
        "Help" => "\u{E002}",
        "Backspace" => "\u{E003}",
        "Tab" => "\u{E004}",
        "Clear" => "\u{E005}",
        "Enter" => "\u{E007}",
        "Shift" | "ShiftLeft" => "\u{E008}",
        "Control" | "ControlLeft" => "\u{E009}",
        "Alt" | "AltLeft" => "\u{E00A}",
        "Pause" => "\u{E00B}",
        "Escape" => "\u{E00C}",
        "PageUp" => "\u{E00E}",
        "PageDown" => "\u{E00F}",
        "End" => "\u{E010}",
        "Home" => "\u{E011}",
        "ArrowLeft" => "\u{E012}",
        "ArrowUp" => "\u{E013}",
        "ArrowRight" => "\u{E014}",
        "ArrowDown" => "\u{E015}",
        "Insert" => "\u{E016}",
        "Delete" => "\u{E017}",
        "NumpadEqual" => "\u{E019}",
        "Numpad0" => "\u{E01A}",
        "Numpad1" => "\u{E01B}",
        "Numpad2" => "\u{E01C}",
        "Numpad3" => "\u{E01D}",
        "Numpad4" => "\u{E01E}",
        "Numpad5" => "\u{E01F}",
        "Numpad6" => "\u{E020}",
        "Numpad7" => "\u{E021}",
        "Numpad8" => "\u{E022}",
        "Numpad9" => "\u{E023}",
        "NumpadMultiply" => "\u{E024}",
        "NumpadAdd" => "\u{E025}",
        "NumpadSubtract" => "\u{E027}",
        "NumpadDecimal" => "\u{E028}",
        "NumpadDivide" => "\u{E029}",
        "F1" => "\u{E031}",
        "F2" => "\u{E032}",
        "F3" => "\u{E033}",
        "F4" => "\u{E034}",
        "F5" => "\u{E035}",
        "F6" => "\u{E036}",
        "F7" => "\u{E037}",
        "F8" => "\u{E038}",
        "F9" => "\u{E039}",
        "F10" => "\u{E03A}",
        "F11" => "\u{E03B}",
        "F12" => "\u{E03C}",
        "Meta" | "MetaLeft" => "\u{E03D}",
        "ShiftRight" => "\u{E050}",
        "ControlRight" => "\u{E051}",
        "AltRight" => "\u{E052}",
        "MetaRight" => "\u{E053}",
        "Digit0" => "0",
        "Digit1" => "1",
        "Digit2" => "2",
        "Digit3" => "3",
        "Digit4" => "4",
        "Digit5" => "5",
        "Digit6" => "6",
        "Digit7" => "7",
        "Digit8" => "8",
        "Digit9" => "9",
        "KeyA" => "a",
        "KeyB" => "b",
        "KeyC" => "c",
        "KeyD" => "d",
        "KeyE" => "e",
        "KeyF" => "f",
        "KeyG" => "g",
        "KeyH" => "h",
        "KeyI" => "i",
        "KeyJ" => "j",
        "KeyK" => "k",
        "KeyL" => "l",
        "KeyM" => "m",
        "KeyN" => "n",
        "KeyO" => "o",
        "KeyP" => "p",
        "KeyQ" => "q",
        "KeyR" => "r",
        "KeyS" => "s",
        "KeyT" => "t",
        "KeyU" => "u",
        "KeyV" => "v",
        "KeyW" => "w",
        "KeyX" => "x",
        "KeyY" => "y",
        "KeyZ" => "z",
        "Semicolon" => ";",
        "Equal" => "=",
        "Comma" => ",",
        "Minus" => "-",
        "Period" => ".",
        "Slash" => "/",
        "Backquote" => "`",
        "BracketLeft" => "[",
        "Backslash" => "\\",
        "BracketRight" => "]",
        "Quote" => "\"",
        _ => return Err(InputError::UnknownKey(key.to_string())),
    };

    Ok(value.to_string())
}

pub struct BidiKeyboard<OT: ConnectionTransport> {
    session: Arc<Mutex<BidiSession<OT>>>,
}

impl<OT: ConnectionTransport> BidiKeyboard<OT> {
    pub fn new(session: Arc<Mutex<BidiSession<OT>>>) -> Self {
        Self { session }
    }

    pub async fn down(&self, key: &str, context: &BrowsingContext) -> Result<(), InputError> {
        let key_value = get_bidi_key_value(key)?;

        let command = PerformActionsBuilder::default()
            .context(context.clone())
            .action(
                KeySourceActionsBuilder::default()
                    .r#type(KeySourceActionsType::Key)
                    .id(KEYBOARD_ID)
                    .action(KeyDownActionBuilder::default()
                        .r#type(KeyDownActionType::KeyDown)
                        .value(key_value)
                        .build().unwrap())
                    .build().unwrap()
            )
            .build().unwrap();

        self.session.lock().await.send(command).await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        Ok(())
    }

    pub async fn up(&self, key: &str, context: &BrowsingContext) -> Result<(), InputError> {
        let key_value = get_bidi_key_value(key)?;

        let command = PerformActionsBuilder::default()
            .context(context.clone())
            .action(
                KeySourceActionsBuilder::default()
                    .r#type(KeySourceActionsType::Key)
                    .id(KEYBOARD_ID)
                    .action(KeyUpActionBuilder::default()
                        .r#type(KeyUpActionType::KeyUp)
                        .value(key_value)
                        .build().unwrap())
                    .build().unwrap()
            )
            .build().unwrap();

        self.session.lock().await.send(command).await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        Ok(())
    }

    pub async fn press(
        &self,
        key: &str,
        context: &BrowsingContext,
        options: Option<KeyPressOptions>,
    ) -> Result<(), InputError> {
        let key_value = get_bidi_key_value(key)?;
        let options = options.unwrap_or_default();

        let mut key_actions = KeySourceActionsBuilder::default()
            .r#type(KeySourceActionsType::Key)
            .id(KEYBOARD_ID)
            .action(KeyDownActionBuilder::default()
                .r#type(KeyDownActionType::KeyDown)
                .value(key_value.clone())
                .build().unwrap());

        if let Some(delay) = options.delay {
            if delay > 0 {
                key_actions = key_actions.action(PauseActionBuilder::default()
                    .r#type(PauseActionType::Pause)
                    .duration(delay)
                    .build().unwrap());
            }
        }

        key_actions = key_actions.action(KeyUpActionBuilder::default()
            .r#type(KeyUpActionType::KeyUp)
            .value(key_value)
            .build().unwrap());

        let command = PerformActionsBuilder::default()
            .context(context.clone())
            .action(key_actions.build().unwrap())
            .build().unwrap();

        self.session.lock().await.send(command).await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        Ok(())
    }

    pub async fn type_text(
        &self,
        text: &str,
        context: &BrowsingContext,
        options: Option<KeyboardTypeOptions>,
    ) -> Result<(), InputError> {
        let options = options.unwrap_or_default();
        let delay = options.delay.unwrap_or(0);

        let mut key_actions = KeySourceActionsBuilder::default()
            .r#type(KeySourceActionsType::Key)
            .id(KEYBOARD_ID);

        for ch in text.chars() {
            let key_value = get_bidi_key_value(&ch.to_string())?;

            key_actions = key_actions.action(KeyDownActionBuilder::default()
                .r#type(KeyDownActionType::KeyDown)
                .value(key_value.clone())
                .build().unwrap());

            if delay > 0 {
                key_actions = key_actions.action(PauseActionBuilder::default()
                    .r#type(PauseActionType::Pause)
                    .duration(delay)
                    .build().unwrap());
            }

            key_actions = key_actions.action(KeyUpActionBuilder::default()
                .r#type(KeyUpActionType::KeyUp)
                .value(key_value)
                .build().unwrap());
        }

        let command = PerformActionsBuilder::default()
            .context(context.clone())
            .action(key_actions.build().unwrap())
            .build().unwrap();

        self.session.lock().await.send(command).await
            .map_err(|e| InputError::CommandResultError(rustenium_core::error::CommandResultError::SessionSendError(e)))?;
        Ok(())
    }
}

impl<OT: ConnectionTransport> crate::input::keyboard::Keyboard for BidiKeyboard<OT> {
    async fn down(&self, key: &str, context: &BrowsingContext) -> Result<(), InputError> {
        self.down(key, context).await
    }
    async fn up(&self, key: &str, context: &BrowsingContext) -> Result<(), InputError> {
        self.up(key, context).await
    }
    async fn press(&self, key: &str, context: &BrowsingContext, options: Option<KeyPressOptions>) -> Result<(), InputError> {
        self.press(key, context, options).await
    }
    async fn type_text(&self, text: &str, context: &BrowsingContext, options: Option<KeyboardTypeOptions>) -> Result<(), InputError> {
        self.type_text(text, context, options).await
    }
}

#[cfg(test)]
mod tests {
    use super::get_bidi_key_value;

    #[test]
    fn single_char_keys_pass_through() {
        for ch in ['a', 'Z', '5', '!', ' '] {
            let result = get_bidi_key_value(&ch.to_string()).unwrap();
            assert_eq!(result, ch.to_string());
        }
    }

    #[test]
    fn special_keys_map_correctly() {
        let cases = vec![
            ("Enter", "\u{E007}"),
            ("Tab", "\u{E004}"),
            ("Backspace", "\u{E003}"),
            ("Escape", "\u{E00C}"),
            ("ArrowUp", "\u{E013}"),
            ("ArrowDown", "\u{E015}"),
            ("ArrowLeft", "\u{E012}"),
            ("ArrowRight", "\u{E014}"),
            ("F1", "\u{E031}"),
            ("F12", "\u{E03C}"),
            ("Shift", "\u{E008}"),
            ("Control", "\u{E009}"),
            ("Alt", "\u{E00A}"),
            ("Meta", "\u{E03D}"),
            ("Delete", "\u{E017}"),
            ("Home", "\u{E011}"),
            ("End", "\u{E010}"),
            ("PageUp", "\u{E00E}"),
            ("PageDown", "\u{E00F}"),
        ];
        for (key, expected) in cases {
            assert_eq!(get_bidi_key_value(key).unwrap(), expected, "failed for key: {}", key);
        }
    }

    #[test]
    fn code_style_keys_map_to_chars() {
        let cases = vec![
            ("KeyA", "a"), ("KeyZ", "z"),
            ("Digit0", "0"), ("Digit9", "9"),
            ("Semicolon", ";"), ("Slash", "/"),
            ("BracketLeft", "["), ("BracketRight", "]"),
        ];
        for (key, expected) in cases {
            assert_eq!(get_bidi_key_value(key).unwrap(), expected, "failed for key: {}", key);
        }
    }

    #[test]
    fn newline_and_carriage_return_map_to_enter() {
        assert_eq!(get_bidi_key_value("\n").unwrap(), "\u{E007}");
        assert_eq!(get_bidi_key_value("\r").unwrap(), "\u{E007}");
    }

    #[test]
    fn unknown_key_returns_error() {
        let result = get_bidi_key_value("NonExistentKey");
        assert!(result.is_err());
    }

    #[test]
    fn left_right_modifier_variants() {
        assert_eq!(get_bidi_key_value("ShiftLeft").unwrap(), "\u{E008}");
        assert_eq!(get_bidi_key_value("ControlLeft").unwrap(), "\u{E009}");
        assert_eq!(get_bidi_key_value("AltLeft").unwrap(), "\u{E00A}");
        assert_eq!(get_bidi_key_value("MetaLeft").unwrap(), "\u{E03D}");
        assert_eq!(get_bidi_key_value("ShiftRight").unwrap(), "\u{E050}");
        assert_eq!(get_bidi_key_value("ControlRight").unwrap(), "\u{E051}");
        assert_eq!(get_bidi_key_value("AltRight").unwrap(), "\u{E052}");
        assert_eq!(get_bidi_key_value("MetaRight").unwrap(), "\u{E053}");
    }
}
