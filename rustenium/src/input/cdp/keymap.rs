/// US keyboard layout, mirroring Puppeteer's `USKeyboardLayout`.
///
/// Each input string maps to exactly one definition. The keyboard handler is
/// responsible for resolving `key`/`text`/`key_code` against the current
/// modifier state — see `CdpKeyboard::down`.
#[derive(Debug, Clone, Copy)]
pub struct KeyDefinition {
    /// Windows virtual key code sent as `windowsVirtualKeyCode`.
    pub key_code: Option<i64>,
    /// Windows VK when Shift is held (numpad keys swap codes under Shift).
    pub shift_key_code: Option<i64>,
    /// `KeyboardEvent.key` value (e.g. `"Enter"`, `"a"`, `"ArrowLeft"`).
    pub key: Option<&'static str>,
    /// `KeyboardEvent.key` value when Shift is held.
    pub shift_key: Option<&'static str>,
    /// `KeyboardEvent.code` value (e.g. `"KeyA"`, `"Digit0"`).
    pub code: Option<&'static str>,
    /// Explicit text inserted on KeyDown (e.g. `"\r"` for Enter).
    pub text: Option<&'static str>,
    /// Explicit text when Shift is held.
    pub shift_text: Option<&'static str>,
    /// 0=standard, 1=left, 2=right, 3=numpad, 4=mobile.
    pub location: Option<i64>,
}

const DEFAULT: KeyDefinition = KeyDefinition {
    key_code: None,
    shift_key_code: None,
    key: None,
    shift_key: None,
    code: None,
    text: None,
    shift_text: None,
    location: None,
};

macro_rules! kd {
    ($($field:ident: $val:expr),* $(,)?) => {
        KeyDefinition { $($field: Some($val),)* ..DEFAULT }
    };
}

/// Look up a key definition by its input string. Returns `None` if unknown.
///
/// Accepts any of the puppeteer `KeyInput` strings: `code` values (`"KeyA"`,
/// `"Digit0"`), `key` values (`"Enter"`, `"ArrowLeft"`), or single characters
/// (`"a"`, `"A"`, `"!"`, `" "`).
pub fn key_definition(input: &str) -> Option<KeyDefinition> {
    Some(match input {
        "0" => kd!(key_code: 48, key: "0", code: "Digit0"),
        "1" => kd!(key_code: 49, key: "1", code: "Digit1"),
        "2" => kd!(key_code: 50, key: "2", code: "Digit2"),
        "3" => kd!(key_code: 51, key: "3", code: "Digit3"),
        "4" => kd!(key_code: 52, key: "4", code: "Digit4"),
        "5" => kd!(key_code: 53, key: "5", code: "Digit5"),
        "6" => kd!(key_code: 54, key: "6", code: "Digit6"),
        "7" => kd!(key_code: 55, key: "7", code: "Digit7"),
        "8" => kd!(key_code: 56, key: "8", code: "Digit8"),
        "9" => kd!(key_code: 57, key: "9", code: "Digit9"),
        "Power" => kd!(key: "Power", code: "Power"),
        "Eject" => kd!(key: "Eject", code: "Eject"),
        "Abort" => kd!(key_code: 3, code: "Abort", key: "Cancel"),
        "Help" => kd!(key_code: 6, code: "Help", key: "Help"),
        "Backspace" => kd!(key_code: 8, code: "Backspace", key: "Backspace"),
        "Tab" => kd!(key_code: 9, code: "Tab", key: "Tab"),
        "Numpad5" => kd!(key_code: 12, shift_key_code: 101, key: "Clear", code: "Numpad5", shift_key: "5", location: 3),
        "NumpadEnter" => kd!(key_code: 13, code: "NumpadEnter", key: "Enter", text: "\r", location: 3),
        "Enter" | "\r" | "\n" => kd!(key_code: 13, code: "Enter", key: "Enter", text: "\r"),
        "ShiftLeft" => kd!(key_code: 16, code: "ShiftLeft", key: "Shift", location: 1),
        "ShiftRight" => kd!(key_code: 16, code: "ShiftRight", key: "Shift", location: 2),
        "ControlLeft" => kd!(key_code: 17, code: "ControlLeft", key: "Control", location: 1),
        "ControlRight" => kd!(key_code: 17, code: "ControlRight", key: "Control", location: 2),
        "AltLeft" => kd!(key_code: 18, code: "AltLeft", key: "Alt", location: 1),
        "AltRight" => kd!(key_code: 18, code: "AltRight", key: "Alt", location: 2),
        "Pause" => kd!(key_code: 19, code: "Pause", key: "Pause"),
        "CapsLock" => kd!(key_code: 20, code: "CapsLock", key: "CapsLock"),
        "Escape" => kd!(key_code: 27, code: "Escape", key: "Escape"),
        "Convert" => kd!(key_code: 28, code: "Convert", key: "Convert"),
        "NonConvert" => kd!(key_code: 29, code: "NonConvert", key: "NonConvert"),
        "Space" => kd!(key_code: 32, code: "Space", key: " "),
        "Numpad9" => kd!(key_code: 33, shift_key_code: 105, key: "PageUp", code: "Numpad9", shift_key: "9", location: 3),
        "PageUp" => kd!(key_code: 33, code: "PageUp", key: "PageUp"),
        "Numpad3" => kd!(key_code: 34, shift_key_code: 99, key: "PageDown", code: "Numpad3", shift_key: "3", location: 3),
        "PageDown" => kd!(key_code: 34, code: "PageDown", key: "PageDown"),
        "End" => kd!(key_code: 35, code: "End", key: "End"),
        "Numpad1" => kd!(key_code: 35, shift_key_code: 97, key: "End", code: "Numpad1", shift_key: "1", location: 3),
        "Home" => kd!(key_code: 36, code: "Home", key: "Home"),
        "Numpad7" => kd!(key_code: 36, shift_key_code: 103, key: "Home", code: "Numpad7", shift_key: "7", location: 3),
        "ArrowLeft" => kd!(key_code: 37, code: "ArrowLeft", key: "ArrowLeft"),
        "Numpad4" => kd!(key_code: 37, shift_key_code: 100, key: "ArrowLeft", code: "Numpad4", shift_key: "4", location: 3),
        "Numpad8" => kd!(key_code: 38, shift_key_code: 104, key: "ArrowUp", code: "Numpad8", shift_key: "8", location: 3),
        "ArrowUp" => kd!(key_code: 38, code: "ArrowUp", key: "ArrowUp"),
        "ArrowRight" => kd!(key_code: 39, code: "ArrowRight", key: "ArrowRight"),
        "Numpad6" => kd!(key_code: 39, shift_key_code: 102, key: "ArrowRight", code: "Numpad6", shift_key: "6", location: 3),
        "Numpad2" => kd!(key_code: 40, shift_key_code: 98, key: "ArrowDown", code: "Numpad2", shift_key: "2", location: 3),
        "ArrowDown" => kd!(key_code: 40, code: "ArrowDown", key: "ArrowDown"),
        "Select" => kd!(key_code: 41, code: "Select", key: "Select"),
        "Open" => kd!(key_code: 43, code: "Open", key: "Execute"),
        "PrintScreen" => kd!(key_code: 44, code: "PrintScreen", key: "PrintScreen"),
        "Insert" => kd!(key_code: 45, code: "Insert", key: "Insert"),
        "Numpad0" => kd!(key_code: 45, shift_key_code: 96, key: "Insert", code: "Numpad0", shift_key: "0", location: 3),
        "Delete" => kd!(key_code: 46, code: "Delete", key: "Delete"),
        "NumpadDecimal" => kd!(key_code: 46, shift_key_code: 110, code: "NumpadDecimal", key: "\0", shift_key: ".", location: 3),
        "Digit0" => kd!(key_code: 48, code: "Digit0", shift_key: ")", key: "0"),
        "Digit1" => kd!(key_code: 49, code: "Digit1", shift_key: "!", key: "1"),
        "Digit2" => kd!(key_code: 50, code: "Digit2", shift_key: "@", key: "2"),
        "Digit3" => kd!(key_code: 51, code: "Digit3", shift_key: "#", key: "3"),
        "Digit4" => kd!(key_code: 52, code: "Digit4", shift_key: "$", key: "4"),
        "Digit5" => kd!(key_code: 53, code: "Digit5", shift_key: "%", key: "5"),
        "Digit6" => kd!(key_code: 54, code: "Digit6", shift_key: "^", key: "6"),
        "Digit7" => kd!(key_code: 55, code: "Digit7", shift_key: "&", key: "7"),
        "Digit8" => kd!(key_code: 56, code: "Digit8", shift_key: "*", key: "8"),
        "Digit9" => kd!(key_code: 57, code: "Digit9", shift_key: "(", key: "9"),
        "KeyA" => kd!(key_code: 65, code: "KeyA", shift_key: "A", key: "a"),
        "KeyB" => kd!(key_code: 66, code: "KeyB", shift_key: "B", key: "b"),
        "KeyC" => kd!(key_code: 67, code: "KeyC", shift_key: "C", key: "c"),
        "KeyD" => kd!(key_code: 68, code: "KeyD", shift_key: "D", key: "d"),
        "KeyE" => kd!(key_code: 69, code: "KeyE", shift_key: "E", key: "e"),
        "KeyF" => kd!(key_code: 70, code: "KeyF", shift_key: "F", key: "f"),
        "KeyG" => kd!(key_code: 71, code: "KeyG", shift_key: "G", key: "g"),
        "KeyH" => kd!(key_code: 72, code: "KeyH", shift_key: "H", key: "h"),
        "KeyI" => kd!(key_code: 73, code: "KeyI", shift_key: "I", key: "i"),
        "KeyJ" => kd!(key_code: 74, code: "KeyJ", shift_key: "J", key: "j"),
        "KeyK" => kd!(key_code: 75, code: "KeyK", shift_key: "K", key: "k"),
        "KeyL" => kd!(key_code: 76, code: "KeyL", shift_key: "L", key: "l"),
        "KeyM" => kd!(key_code: 77, code: "KeyM", shift_key: "M", key: "m"),
        "KeyN" => kd!(key_code: 78, code: "KeyN", shift_key: "N", key: "n"),
        "KeyO" => kd!(key_code: 79, code: "KeyO", shift_key: "O", key: "o"),
        "KeyP" => kd!(key_code: 80, code: "KeyP", shift_key: "P", key: "p"),
        "KeyQ" => kd!(key_code: 81, code: "KeyQ", shift_key: "Q", key: "q"),
        "KeyR" => kd!(key_code: 82, code: "KeyR", shift_key: "R", key: "r"),
        "KeyS" => kd!(key_code: 83, code: "KeyS", shift_key: "S", key: "s"),
        "KeyT" => kd!(key_code: 84, code: "KeyT", shift_key: "T", key: "t"),
        "KeyU" => kd!(key_code: 85, code: "KeyU", shift_key: "U", key: "u"),
        "KeyV" => kd!(key_code: 86, code: "KeyV", shift_key: "V", key: "v"),
        "KeyW" => kd!(key_code: 87, code: "KeyW", shift_key: "W", key: "w"),
        "KeyX" => kd!(key_code: 88, code: "KeyX", shift_key: "X", key: "x"),
        "KeyY" => kd!(key_code: 89, code: "KeyY", shift_key: "Y", key: "y"),
        "KeyZ" => kd!(key_code: 90, code: "KeyZ", shift_key: "Z", key: "z"),
        "MetaLeft" => kd!(key_code: 91, code: "MetaLeft", key: "Meta", location: 1),
        "MetaRight" => kd!(key_code: 92, code: "MetaRight", key: "Meta", location: 2),
        "ContextMenu" => kd!(key_code: 93, code: "ContextMenu", key: "ContextMenu"),
        "NumpadMultiply" => kd!(key_code: 106, code: "NumpadMultiply", key: "*", location: 3),
        "NumpadAdd" => kd!(key_code: 107, code: "NumpadAdd", key: "+", location: 3),
        "NumpadSubtract" => kd!(key_code: 109, code: "NumpadSubtract", key: "-", location: 3),
        "NumpadDivide" => kd!(key_code: 111, code: "NumpadDivide", key: "/", location: 3),
        "F1" => kd!(key_code: 112, code: "F1", key: "F1"),
        "F2" => kd!(key_code: 113, code: "F2", key: "F2"),
        "F3" => kd!(key_code: 114, code: "F3", key: "F3"),
        "F4" => kd!(key_code: 115, code: "F4", key: "F4"),
        "F5" => kd!(key_code: 116, code: "F5", key: "F5"),
        "F6" => kd!(key_code: 117, code: "F6", key: "F6"),
        "F7" => kd!(key_code: 118, code: "F7", key: "F7"),
        "F8" => kd!(key_code: 119, code: "F8", key: "F8"),
        "F9" => kd!(key_code: 120, code: "F9", key: "F9"),
        "F10" => kd!(key_code: 121, code: "F10", key: "F10"),
        "F11" => kd!(key_code: 122, code: "F11", key: "F11"),
        "F12" => kd!(key_code: 123, code: "F12", key: "F12"),
        "F13" => kd!(key_code: 124, code: "F13", key: "F13"),
        "F14" => kd!(key_code: 125, code: "F14", key: "F14"),
        "F15" => kd!(key_code: 126, code: "F15", key: "F15"),
        "F16" => kd!(key_code: 127, code: "F16", key: "F16"),
        "F17" => kd!(key_code: 128, code: "F17", key: "F17"),
        "F18" => kd!(key_code: 129, code: "F18", key: "F18"),
        "F19" => kd!(key_code: 130, code: "F19", key: "F19"),
        "F20" => kd!(key_code: 131, code: "F20", key: "F20"),
        "F21" => kd!(key_code: 132, code: "F21", key: "F21"),
        "F22" => kd!(key_code: 133, code: "F22", key: "F22"),
        "F23" => kd!(key_code: 134, code: "F23", key: "F23"),
        "F24" => kd!(key_code: 135, code: "F24", key: "F24"),
        "NumLock" => kd!(key_code: 144, code: "NumLock", key: "NumLock"),
        "ScrollLock" => kd!(key_code: 145, code: "ScrollLock", key: "ScrollLock"),
        "AudioVolumeMute" => kd!(key_code: 173, code: "AudioVolumeMute", key: "AudioVolumeMute"),
        "AudioVolumeDown" => kd!(key_code: 174, code: "AudioVolumeDown", key: "AudioVolumeDown"),
        "AudioVolumeUp" => kd!(key_code: 175, code: "AudioVolumeUp", key: "AudioVolumeUp"),
        "MediaTrackNext" => kd!(key_code: 176, code: "MediaTrackNext", key: "MediaTrackNext"),
        "MediaTrackPrevious" => kd!(key_code: 177, code: "MediaTrackPrevious", key: "MediaTrackPrevious"),
        "MediaStop" => kd!(key_code: 178, code: "MediaStop", key: "MediaStop"),
        "MediaPlayPause" => kd!(key_code: 179, code: "MediaPlayPause", key: "MediaPlayPause"),
        "Semicolon" => kd!(key_code: 186, code: "Semicolon", shift_key: ":", key: ";"),
        "Equal" => kd!(key_code: 187, code: "Equal", shift_key: "+", key: "="),
        "NumpadEqual" => kd!(key_code: 187, code: "NumpadEqual", key: "=", location: 3),
        "Comma" => kd!(key_code: 188, code: "Comma", shift_key: "<", key: ","),
        "Minus" => kd!(key_code: 189, code: "Minus", shift_key: "_", key: "-"),
        "Period" => kd!(key_code: 190, code: "Period", shift_key: ">", key: "."),
        "Slash" => kd!(key_code: 191, code: "Slash", shift_key: "?", key: "/"),
        "Backquote" => kd!(key_code: 192, code: "Backquote", shift_key: "~", key: "`"),
        "BracketLeft" => kd!(key_code: 219, code: "BracketLeft", shift_key: "{", key: "["),
        "Backslash" => kd!(key_code: 220, code: "Backslash", shift_key: "|", key: "\\"),
        "BracketRight" => kd!(key_code: 221, code: "BracketRight", shift_key: "}", key: "]"),
        "Quote" => kd!(key_code: 222, code: "Quote", shift_key: "\"", key: "'"),
        "AltGraph" => kd!(key_code: 225, code: "AltGraph", key: "AltGraph"),
        "Props" => kd!(key_code: 247, code: "Props", key: "CrSel"),
        "Cancel" => kd!(key_code: 3, key: "Cancel", code: "Abort"),
        "Clear" => kd!(key_code: 12, key: "Clear", code: "Numpad5", location: 3),
        "Shift" => kd!(key_code: 16, key: "Shift", code: "ShiftLeft", location: 1),
        "Control" => kd!(key_code: 17, key: "Control", code: "ControlLeft", location: 1),
        "Alt" => kd!(key_code: 18, key: "Alt", code: "AltLeft", location: 1),
        "Accept" => kd!(key_code: 30, key: "Accept"),
        "ModeChange" => kd!(key_code: 31, key: "ModeChange"),
        " " => kd!(key_code: 32, key: " ", code: "Space"),
        "Print" => kd!(key_code: 42, key: "Print"),
        "Execute" => kd!(key_code: 43, key: "Execute", code: "Open"),
        "\0" => kd!(key_code: 46, key: "\0", code: "NumpadDecimal", location: 3),
        "a" => kd!(key_code: 65, key: "a", code: "KeyA"),
        "b" => kd!(key_code: 66, key: "b", code: "KeyB"),
        "c" => kd!(key_code: 67, key: "c", code: "KeyC"),
        "d" => kd!(key_code: 68, key: "d", code: "KeyD"),
        "e" => kd!(key_code: 69, key: "e", code: "KeyE"),
        "f" => kd!(key_code: 70, key: "f", code: "KeyF"),
        "g" => kd!(key_code: 71, key: "g", code: "KeyG"),
        "h" => kd!(key_code: 72, key: "h", code: "KeyH"),
        "i" => kd!(key_code: 73, key: "i", code: "KeyI"),
        "j" => kd!(key_code: 74, key: "j", code: "KeyJ"),
        "k" => kd!(key_code: 75, key: "k", code: "KeyK"),
        "l" => kd!(key_code: 76, key: "l", code: "KeyL"),
        "m" => kd!(key_code: 77, key: "m", code: "KeyM"),
        "n" => kd!(key_code: 78, key: "n", code: "KeyN"),
        "o" => kd!(key_code: 79, key: "o", code: "KeyO"),
        "p" => kd!(key_code: 80, key: "p", code: "KeyP"),
        "q" => kd!(key_code: 81, key: "q", code: "KeyQ"),
        "r" => kd!(key_code: 82, key: "r", code: "KeyR"),
        "s" => kd!(key_code: 83, key: "s", code: "KeyS"),
        "t" => kd!(key_code: 84, key: "t", code: "KeyT"),
        "u" => kd!(key_code: 85, key: "u", code: "KeyU"),
        "v" => kd!(key_code: 86, key: "v", code: "KeyV"),
        "w" => kd!(key_code: 87, key: "w", code: "KeyW"),
        "x" => kd!(key_code: 88, key: "x", code: "KeyX"),
        "y" => kd!(key_code: 89, key: "y", code: "KeyY"),
        "z" => kd!(key_code: 90, key: "z", code: "KeyZ"),
        "Meta" => kd!(key_code: 91, key: "Meta", code: "MetaLeft", location: 1),
        "*" => kd!(key_code: 106, key: "*", code: "NumpadMultiply", location: 3),
        "+" => kd!(key_code: 107, key: "+", code: "NumpadAdd", location: 3),
        "-" => kd!(key_code: 109, key: "-", code: "NumpadSubtract", location: 3),
        "/" => kd!(key_code: 111, key: "/", code: "NumpadDivide", location: 3),
        ";" => kd!(key_code: 186, key: ";", code: "Semicolon"),
        "=" => kd!(key_code: 187, key: "=", code: "Equal"),
        "," => kd!(key_code: 188, key: ",", code: "Comma"),
        "." => kd!(key_code: 190, key: ".", code: "Period"),
        "`" => kd!(key_code: 192, key: "`", code: "Backquote"),
        "[" => kd!(key_code: 219, key: "[", code: "BracketLeft"),
        "\\" => kd!(key_code: 220, key: "\\", code: "Backslash"),
        "]" => kd!(key_code: 221, key: "]", code: "BracketRight"),
        "'" => kd!(key_code: 222, key: "'", code: "Quote"),
        "Attn" => kd!(key_code: 246, key: "Attn"),
        "CrSel" => kd!(key_code: 247, key: "CrSel", code: "Props"),
        "ExSel" => kd!(key_code: 248, key: "ExSel"),
        "EraseEof" => kd!(key_code: 249, key: "EraseEof"),
        "Play" => kd!(key_code: 250, key: "Play"),
        "ZoomOut" => kd!(key_code: 251, key: "ZoomOut"),
        ")" => kd!(key_code: 48, key: ")", code: "Digit0"),
        "!" => kd!(key_code: 49, key: "!", code: "Digit1"),
        "@" => kd!(key_code: 50, key: "@", code: "Digit2"),
        "#" => kd!(key_code: 51, key: "#", code: "Digit3"),
        "$" => kd!(key_code: 52, key: "$", code: "Digit4"),
        "%" => kd!(key_code: 53, key: "%", code: "Digit5"),
        "^" => kd!(key_code: 54, key: "^", code: "Digit6"),
        "&" => kd!(key_code: 55, key: "&", code: "Digit7"),
        "(" => kd!(key_code: 57, key: "(", code: "Digit9"),
        "A" => kd!(key_code: 65, key: "A", code: "KeyA"),
        "B" => kd!(key_code: 66, key: "B", code: "KeyB"),
        "C" => kd!(key_code: 67, key: "C", code: "KeyC"),
        "D" => kd!(key_code: 68, key: "D", code: "KeyD"),
        "E" => kd!(key_code: 69, key: "E", code: "KeyE"),
        "F" => kd!(key_code: 70, key: "F", code: "KeyF"),
        "G" => kd!(key_code: 71, key: "G", code: "KeyG"),
        "H" => kd!(key_code: 72, key: "H", code: "KeyH"),
        "I" => kd!(key_code: 73, key: "I", code: "KeyI"),
        "J" => kd!(key_code: 74, key: "J", code: "KeyJ"),
        "K" => kd!(key_code: 75, key: "K", code: "KeyK"),
        "L" => kd!(key_code: 76, key: "L", code: "KeyL"),
        "M" => kd!(key_code: 77, key: "M", code: "KeyM"),
        "N" => kd!(key_code: 78, key: "N", code: "KeyN"),
        "O" => kd!(key_code: 79, key: "O", code: "KeyO"),
        "P" => kd!(key_code: 80, key: "P", code: "KeyP"),
        "Q" => kd!(key_code: 81, key: "Q", code: "KeyQ"),
        "R" => kd!(key_code: 82, key: "R", code: "KeyR"),
        "S" => kd!(key_code: 83, key: "S", code: "KeyS"),
        "T" => kd!(key_code: 84, key: "T", code: "KeyT"),
        "U" => kd!(key_code: 85, key: "U", code: "KeyU"),
        "V" => kd!(key_code: 86, key: "V", code: "KeyV"),
        "W" => kd!(key_code: 87, key: "W", code: "KeyW"),
        "X" => kd!(key_code: 88, key: "X", code: "KeyX"),
        "Y" => kd!(key_code: 89, key: "Y", code: "KeyY"),
        "Z" => kd!(key_code: 90, key: "Z", code: "KeyZ"),
        ":" => kd!(key_code: 186, key: ":", code: "Semicolon"),
        "<" => kd!(key_code: 188, key: "<", code: "Comma"),
        "_" => kd!(key_code: 189, key: "_", code: "Minus"),
        ">" => kd!(key_code: 190, key: ">", code: "Period"),
        "?" => kd!(key_code: 191, key: "?", code: "Slash"),
        "~" => kd!(key_code: 192, key: "~", code: "Backquote"),
        "{" => kd!(key_code: 219, key: "{", code: "BracketLeft"),
        "|" => kd!(key_code: 220, key: "|", code: "Backslash"),
        "}" => kd!(key_code: 221, key: "}", code: "BracketRight"),
        "\"" => kd!(key_code: 222, key: "\"", code: "Quote"),
        "SoftLeft" => kd!(key: "SoftLeft", code: "SoftLeft", location: 4),
        "SoftRight" => kd!(key: "SoftRight", code: "SoftRight", location: 4),
        "Camera" => kd!(key_code: 44, key: "Camera", code: "Camera", location: 4),
        "Call" => kd!(key: "Call", code: "Call", location: 4),
        "EndCall" => kd!(key_code: 95, key: "EndCall", code: "EndCall", location: 4),
        "VolumeDown" => kd!(key_code: 182, key: "VolumeDown", code: "VolumeDown", location: 4),
        "VolumeUp" => kd!(key_code: 183, key: "VolumeUp", code: "VolumeUp", location: 4),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uppercase_letter_resolves_to_uppercase() {
        let def = key_definition("J").unwrap();
        assert_eq!(def.key, Some("J"));
        assert_eq!(def.code, Some("KeyJ"));
        assert_eq!(def.key_code, Some(74));
        assert!(def.shift_key.is_none());
        assert!(def.text.is_none());
    }

    #[test]
    fn lowercase_letter_resolves_to_lowercase() {
        let def = key_definition("j").unwrap();
        assert_eq!(def.key, Some("j"));
        assert_eq!(def.code, Some("KeyJ"));
        assert_eq!(def.key_code, Some(74));
        assert!(def.shift_key.is_none());
    }

    #[test]
    fn code_letter_carries_shift_variant() {
        let def = key_definition("KeyJ").unwrap();
        assert_eq!(def.key, Some("j"));
        assert_eq!(def.shift_key, Some("J"));
        assert_eq!(def.code, Some("KeyJ"));
    }

    #[test]
    fn shifted_symbol_resolves_directly() {
        let def = key_definition("@").unwrap();
        assert_eq!(def.key, Some("@"));
        assert_eq!(def.code, Some("Digit2"));
        assert_eq!(def.key_code, Some(50));
        assert!(def.shift_key.is_none());
    }

    #[test]
    fn enter_aliases_share_definition() {
        for k in ["Enter", "\r", "\n"] {
            let def = key_definition(k).unwrap();
            assert_eq!(def.key, Some("Enter"));
            assert_eq!(def.text, Some("\r"));
            assert_eq!(def.key_code, Some(13));
        }
    }

    #[test]
    fn numpad_carries_shift_key_code() {
        let def = key_definition("Numpad5").unwrap();
        assert_eq!(def.key_code, Some(12));
        assert_eq!(def.shift_key_code, Some(101));
        assert_eq!(def.shift_key, Some("5"));
        assert_eq!(def.location, Some(3));
    }

    #[test]
    fn modifier_left_right_have_locations() {
        assert_eq!(key_definition("ShiftLeft").unwrap().location, Some(1));
        assert_eq!(key_definition("ShiftRight").unwrap().location, Some(2));
        assert_eq!(key_definition("ControlLeft").unwrap().location, Some(1));
        assert_eq!(key_definition("ControlRight").unwrap().location, Some(2));
    }

    #[test]
    fn unknown_key_returns_none() {
        assert!(key_definition("NotARealKey").is_none());
    }
}
