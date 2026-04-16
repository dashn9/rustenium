/// A single key definition from the US keyboard layout.
/// Maps a human-readable key name (e.g. `"Enter"`, `"ArrowLeft"`) to the
/// parameters required by `Input.dispatchKeyEvent`.
#[derive(Debug, Clone)]
pub struct KeyDefinition {
    /// Virtual key code sent as `windowsVirtualKeyCode`.
    pub key_code: Option<i64>,
    /// The `key` field (e.g. `"Enter"`, `"a"`).
    pub key: &'static str,
    /// The `code` field (e.g. `"Enter"`, `"KeyA"`).
    pub code: &'static str,
    /// `text` field for keyDown (printable single-char keys).
    pub text: Option<&'static str>,
    /// `key` when Shift is held.
    pub shift_key: Option<&'static str>,
    /// `text` when Shift is held.
    pub shift_text: Option<&'static str>,
    /// 0 = standard, 1 = left, 2 = right, 3 = numpad.
    pub location: Option<i64>,
}

/// Look up a key by its `key` value or `code` value.
/// Returns `None` for unmapped keys (caller should fall back to `Input.insertText`).
pub fn key_definition(key: &str) -> Option<&'static KeyDefinition> {
    US_KEYBOARD_LAYOUT
        .iter()
        .find(|def| def.key == key || def.code == key)
}

/// US keyboard layout.
///
/// # TODO
/// Populate this with the full layout from puppeteer's
/// `packages/puppeteer-core/src/common/USKeyboardLayout.ts`.
/// Every entry maps a key name / code to the CDP `DispatchKeyEvent` parameters.
pub static US_KEYBOARD_LAYOUT: &[KeyDefinition] = &[
    // ── Navigation ──────────────────────────────────────────────────────────
    // TODO: fill in full layout
];
