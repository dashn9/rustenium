use rustenium::error::node::*;

#[test]
fn node_mouse_error_button_already_pressed() {
    let err = NodeMouseError::ButtonAlreadyPressed("left".into());
    let msg = format!("{}", err);
    assert!(msg.contains("already pressed"));
    assert!(msg.contains("left"));
}

#[test]
fn node_mouse_error_button_not_pressed() {
    let err = NodeMouseError::ButtonNotPressed("right".into());
    let msg = format!("{}", err);
    assert!(msg.contains("not pressed"));
    assert!(msg.contains("right"));
}

#[test]
fn node_mouse_error_driver_includes_reason() {
    let err = NodeMouseError::Driver("transport closed".into());
    let msg = format!("{}", err);
    assert!(msg.contains("transport closed"));
    assert!(msg.contains("driver"));
}

#[test]
fn node_mouse_error_invalid_position() {
    let err = NodeMouseError::InvalidPosition;
    let msg = format!("{}", err);
    assert!(msg.contains("valid position"));
}

#[test]
fn node_input_error_unknown_key() {
    let err = NodeInputError::UnknownKey("HyperKey".into());
    let msg = format!("{}", err);
    assert!(msg.contains("Unknown key"));
    assert!(msg.contains("HyperKey"));
}

#[test]
fn node_input_error_touch_already_started() {
    let err = NodeInputError::TouchAlreadyStarted;
    assert!(format!("{}", err).contains("already started"));
}

#[test]
fn node_screenshot_error_variants() {
    let invalid = NodeScreenshotError::InvalidPath("bad/path".into());
    assert!(format!("{}", invalid).contains("bad/path"));

    let b64 = NodeScreenshotError::Base64DecodeError("decode".into());
    assert!(format!("{}", b64).contains("decode"));

    let write = NodeScreenshotError::FileWriteError("write".into());
    assert!(format!("{}", write).contains("write"));
}

#[test]
fn bidi_node_screenshot_no_shared_id() {
    let err = BidiNodeScreenshotError::NoSharedId;
    assert!(format!("{}", err).contains("shared ID"));
}

#[test]
fn bidi_node_screenshot_no_context() {
    let err = BidiNodeScreenshotError::NoContext;
    assert!(format!("{}", err).contains("context"));
}

#[test]
fn cdp_node_screenshot_no_position() {
    let err = CdpNodeScreenshotError::NoPosition;
    assert!(format!("{}", err).contains("position"));
}

#[test]
fn bidi_node_action_no_shared_id() {
    let err = BidiNodeActionError::NoSharedId;
    assert!(format!("{}", err).contains("shared ID"));
}

#[test]
fn cdp_node_action_missing_object_id() {
    let err = CdpNodeActionError::MissingObjectId;
    assert!(format!("{}", err).contains("object id"));
}

#[test]
fn cdp_node_action_decode_contains_detail() {
    let err = CdpNodeActionError::Decode("malformed".into());
    assert!(format!("{}", err).contains("malformed"));
}

#[test]
fn bidi_node_mouse_touch_already_started() {
    let err = BidiNodeMouseError::TouchAlreadyStarted;
    assert!(format!("{}", err).contains("already started"));
}

// ── Conversions ───────────────────────────────────────────────────────────────

#[test]
fn bidi_input_error_unknown_key_converts_to_node_input_error() {
    use rustenium::error::bidi::InputError;
    let err: NodeInputError = InputError::UnknownKey("K".into()).into();
    assert!(matches!(err, NodeInputError::UnknownKey(ref k) if k == "K"));
}

#[test]
fn bidi_input_error_touch_converts_to_node_input_error() {
    use rustenium::error::bidi::InputError;
    let err: NodeInputError = InputError::TouchAlreadyStarted.into();
    assert!(matches!(err, NodeInputError::TouchAlreadyStarted));
}

#[test]
fn bidi_screenshot_error_invalid_path_converts() {
    use rustenium::error::bidi::ScreenshotError;
    let err: NodeScreenshotError = ScreenshotError::InvalidPath("p".into()).into();
    assert!(matches!(err, NodeScreenshotError::InvalidPath(ref p) if p == "p"));
}

#[test]
fn bidi_screenshot_error_base64_converts() {
    use rustenium::error::bidi::ScreenshotError;
    let err: NodeScreenshotError = ScreenshotError::Base64DecodeError("d".into()).into();
    assert!(matches!(err, NodeScreenshotError::Base64DecodeError(ref d) if d == "d"));
}

#[test]
fn bidi_screenshot_error_file_write_converts() {
    use rustenium::error::bidi::ScreenshotError;
    let err: NodeScreenshotError = ScreenshotError::FileWriteError("w".into()).into();
    assert!(matches!(err, NodeScreenshotError::FileWriteError(ref w) if w == "w"));
}

#[test]
fn bidi_screenshot_error_no_shared_id_converts() {
    use rustenium::error::bidi::ScreenshotError;
    let err: NodeScreenshotError = ScreenshotError::NoSharedId.into();
    assert!(matches!(
        err,
        NodeScreenshotError::Bidi(BidiNodeScreenshotError::NoSharedId)
    ));
}

#[test]
fn bidi_screenshot_error_no_context_converts() {
    use rustenium::error::bidi::ScreenshotError;
    let err: NodeScreenshotError = ScreenshotError::NoContext.into();
    assert!(matches!(
        err,
        NodeScreenshotError::Bidi(BidiNodeScreenshotError::NoContext)
    ));
}
