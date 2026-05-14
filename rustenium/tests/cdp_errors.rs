use rustenium::error::cdp::*;

#[test]
fn nodes_fetch_error_parse_contains_message() {
    let err = NodesFetchError::ParseError("bad tree".into());
    assert!(format!("{}", err).contains("parse"));
}

#[test]
fn mouse_input_error_button_already_pressed() {
    let err = MouseInputError::ButtonAlreadyPressed("left".into());
    let msg = format!("{}", err);
    assert!(msg.contains("already pressed"));
    assert!(msg.contains("left"));
}

#[test]
fn mouse_input_error_button_not_pressed() {
    let err = MouseInputError::ButtonNotPressed("right".into());
    let msg = format!("{}", err);
    assert!(msg.contains("not pressed"));
    assert!(msg.contains("right"));
}

#[test]
fn input_error_unknown_key() {
    let err = InputError::UnknownKey("SuperKey".into());
    let msg = format!("{}", err);
    assert!(msg.contains("Unknown key"));
    assert!(msg.contains("SuperKey"));
}

#[test]
fn locate_error_timeout_contains_selector() {
    let err = LocateError::Timeout("#foo".into());
    let msg = format!("{}", err);
    assert!(msg.contains("Timed out"));
    assert!(msg.contains("#foo"));
}

#[test]
fn locate_error_parse_contains_message() {
    let err = LocateError::ParseError("bad json".into());
    assert!(format!("{}", err).contains("bad json"));
}

#[test]
fn evaluate_script_error_parse_contains_message() {
    let err = EvaluateScriptError::ParseError("no result".into());
    assert!(format!("{}", err).contains("no result"));
}

#[test]
fn preload_script_error_parse_contains_message() {
    let err = PreloadScriptError::ParseError("bad source".into());
    assert!(format!("{}", err).contains("bad source"));
}

#[test]
fn screenshot_error_variants_display_message() {
    assert!(format!("{}", ScreenshotError::InvalidPath("bad".into())).contains("bad"));
    assert!(format!("{}", ScreenshotError::Base64DecodeError("decode".into())).contains("decode"));
    assert!(format!("{}", ScreenshotError::FileWriteError("write".into())).contains("write"));
    assert!(format!("{}", ScreenshotError::ParseError("parse".into())).contains("parse"));
}

#[test]
fn create_tab_error_wraps_create_target_error() {
    use rustenium_core::error::CdpCommandResultError;
    let inner = CreateTargetError::CommandResultError(
        CdpCommandResultError::InvalidResultTypeError(serde_json::Value::Null)
    );
    let err = CreateTabError::CreateTargetError(inner);
    assert!(matches!(err, CreateTabError::CreateTargetError(_)));
}
