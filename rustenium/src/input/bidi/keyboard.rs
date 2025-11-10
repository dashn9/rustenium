use rustenium_bidi_commands::browsing_context::types::BrowsingContext;
use rustenium_bidi_commands::input::commands::{
    InputCommand, PerformActions, PerformActionsParameters, InputPerformActionsMethod,
};
use rustenium_bidi_commands::input::types::{
    KeySourceActions, KeySourceAction, KeyDownAction, KeyUpAction, PauseAction,
    SourceActions, KeyEnum, KeyDownEnum, KeyUpEnum, PauseEnum,
};
use rustenium_bidi_commands::CommandData;
use rustenium_core::Session;
use rustenium_core::transport::ConnectionTransport;
use crate::error::InputError;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::KEYBOARD_ID;

/// Options for keyboard key press
#[derive(Debug, Clone, Default)]
pub struct KeyPressOptions {
    /// Delay in milliseconds between keydown and keyup
    pub delay: Option<u64>,
}

/// Options for keyboard typing
#[derive(Debug, Clone, Default)]
pub struct KeyboardTypeOptions {
    /// Delay in milliseconds between each key press
    pub delay: Option<u64>,
}

/// Convert a key string to BiDi key value
fn get_bidi_key_value(key: &str) -> Result<String, InputError> {
    // Handle newline conversions
    let key = match key {
        "\r" | "\n" => "Enter",
        _ => key,
    };

    // If it's a single character, return it as-is
    if key.chars().count() == 1 {
        return Ok(key.to_string());
    }

    // Map special keys to their BiDi Unicode values
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

/// BiDi Keyboard implementation
pub struct Keyboard<OT: ConnectionTransport> {
    session: Arc<Mutex<Session<OT>>>,
}

impl<OT: ConnectionTransport> Keyboard<OT> {
    /// Create a new Keyboard instance
    pub fn new(session: Arc<Mutex<Session<OT>>>) -> Self {
        Self { session }
    }

    /// Press a key down
    pub async fn down(&self, key: &str, context: &BrowsingContext) -> Result<(), InputError> {
        let key_value = get_bidi_key_value(key)?;

        let command = InputCommand::PerformActions(PerformActions {
            method: InputPerformActionsMethod::InputPerformActions,
            params: PerformActionsParameters {
                context: context.clone(),
                actions: vec![SourceActions::KeySourceActions(KeySourceActions {
                    r#type: KeyEnum::Key,
                    id: KEYBOARD_ID.to_string(),
                    actions: vec![KeySourceAction::KeyDownAction(KeyDownAction {
                        r#type: KeyDownEnum::KeyDown,
                        value: key_value,
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

    /// Release a key
    pub async fn up(&self, key: &str, context: &BrowsingContext) -> Result<(), InputError> {
        let key_value = get_bidi_key_value(key)?;

        let command = InputCommand::PerformActions(PerformActions {
            method: InputPerformActionsMethod::InputPerformActions,
            params: PerformActionsParameters {
                context: context.clone(),
                actions: vec![SourceActions::KeySourceActions(KeySourceActions {
                    r#type: KeyEnum::Key,
                    id: KEYBOARD_ID.to_string(),
                    actions: vec![KeySourceAction::KeyUpAction(KeyUpAction {
                        r#type: KeyUpEnum::KeyUp,
                        value: key_value,
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

    /// Press and release a key
    pub async fn press(
        &self,
        key: &str,
        context: &BrowsingContext,
        options: Option<KeyPressOptions>,
    ) -> Result<(), InputError> {
        let key_value = get_bidi_key_value(key)?;
        let options = options.unwrap_or_default();

        let mut actions = vec![KeySourceAction::KeyDownAction(KeyDownAction {
            r#type: KeyDownEnum::KeyDown,
            value: key_value.clone(),
        })];

        if let Some(delay) = options.delay {
            if delay > 0 {
                actions.push(KeySourceAction::PauseAction(PauseAction {
                    r#type: PauseEnum::Pause,
                    duration: Some(delay),
                }));
            }
        }

        actions.push(KeySourceAction::KeyUpAction(KeyUpAction {
            r#type: KeyUpEnum::KeyUp,
            value: key_value,
        }));

        let command = InputCommand::PerformActions(PerformActions {
            method: InputPerformActionsMethod::InputPerformActions,
            params: PerformActionsParameters {
                context: context.clone(),
                actions: vec![SourceActions::KeySourceActions(KeySourceActions {
                    r#type: KeyEnum::Key,
                    id: KEYBOARD_ID.to_string(),
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

    /// Type a string of text
    pub async fn type_text(
        &self,
        text: &str,
        context: &BrowsingContext,
        options: Option<KeyboardTypeOptions>,
    ) -> Result<(), InputError> {
        let options = options.unwrap_or_default();
        let delay = options.delay.unwrap_or(0);

        // Convert text to individual characters (code points)
        let mut actions = Vec::new();
        for ch in text.chars() {
            let key_value = get_bidi_key_value(&ch.to_string())?;

            actions.push(KeySourceAction::KeyDownAction(KeyDownAction {
                r#type: KeyDownEnum::KeyDown,
                value: key_value.clone(),
            }));

            if delay > 0 {
                actions.push(KeySourceAction::PauseAction(PauseAction {
                    r#type: PauseEnum::Pause,
                    duration: Some(delay),
                }));
            }

            actions.push(KeySourceAction::KeyUpAction(KeyUpAction {
                r#type: KeyUpEnum::KeyUp,
                value: key_value,
            }));
        }

        let command = InputCommand::PerformActions(PerformActions {
            method: InputPerformActionsMethod::InputPerformActions,
            params: PerformActionsParameters {
                context: context.clone(),
                actions: vec![SourceActions::KeySourceActions(KeySourceActions {
                    r#type: KeyEnum::Key,
                    id: KEYBOARD_ID.to_string(),
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
}
