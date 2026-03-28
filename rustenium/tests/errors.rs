use rustenium::error::bidi::*;

#[tokio::test]
async fn error_display_messages() {
    let cases: Vec<(Box<dyn std::error::Error>, &str)> = vec![
        (Box::new(ZeroBrowsingContextAtStartError), "No Browsing Context"),
        (Box::new(ContextIndexError {}), "Context does not exist"),
        (Box::new(InvalidPositionError), "valid position"),
        (Box::new(InputError::UnknownKey("Foo".into())), "Unknown key"),
        (Box::new(InputError::TouchAlreadyStarted), "already started"),
    ];

    for (err, expected_substr) in cases {
        let msg = format!("{}", err);
        assert!(msg.contains(expected_substr), "'{}' should contain '{}'", msg, expected_substr);
    }
}

#[tokio::test]
async fn context_index_error_converts_to_open_url_error() {
    let err: NavigateError = ContextIndexError {}.into();
    assert!(matches!(err, NavigateError::ContextIndexError(_)));
}

#[tokio::test]
async fn context_index_error_converts_to_find_nodes_error() {
    let err: FindNodesError = ContextIndexError {}.into();
    assert!(matches!(err, FindNodesError::ContextIndexError(_)));
}

#[tokio::test]
async fn screenshot_error_variants() {
    let path_err = ScreenshotError::InvalidPath("bad".into());
    assert!(format!("{}", path_err).contains("bad"));

    let b64_err = ScreenshotError::Base64DecodeError("decode fail".into());
    assert!(format!("{}", b64_err).contains("decode fail"));

    let write_err = ScreenshotError::FileWriteError("write fail".into());
    assert!(format!("{}", write_err).contains("write fail"));

    assert!(format!("{}", ScreenshotError::NoSharedId).contains("shared ID"));
    assert!(format!("{}", ScreenshotError::NoContext).contains("context"));
}
