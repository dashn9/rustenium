use rustenium::browsers::{
    BrowserScreenshotOptionsBuilder, NavigateOptionsBuilder,
};
use rustenium_bidi_definitions::browsing_context::types::ReadinessState;

// ── NavigateOptions ───────────────────────────────────────────────────────────

#[test]
fn navigate_options_default_empty() {
    let opts = NavigateOptionsBuilder::default().build();
    assert!(opts.wait.is_none());
    assert!(opts.context_id.is_none());
}

#[test]
fn navigate_options_builder_sets_wait_complete() {
    let opts = NavigateOptionsBuilder::default()
        .wait(ReadinessState::Complete)
        .build();
    assert!(matches!(opts.wait, Some(ReadinessState::Complete)));
}

#[test]
fn navigate_options_builder_sets_wait_interactive() {
    let opts = NavigateOptionsBuilder::default()
        .wait(ReadinessState::Interactive)
        .build();
    assert!(matches!(opts.wait, Some(ReadinessState::Interactive)));
}

#[test]
fn navigate_options_builder_sets_context_id() {
    let opts = NavigateOptionsBuilder::default()
        .context_id("context-abc".to_string())
        .build();
    assert!(opts.context_id.is_some());
}

// ── BrowserScreenshotOptions ──────────────────────────────────────────────────

#[test]
fn screenshot_options_default_empty() {
    let opts = BrowserScreenshotOptionsBuilder::default().build();
    assert!(opts.context_id.is_none());
    assert!(opts.origin.is_none());
    assert!(opts.format.is_none());
    assert!(opts.clip.is_none());
    assert!(opts.save_path.is_none());
}

#[test]
fn screenshot_options_builder_sets_save_path() {
    let opts = BrowserScreenshotOptionsBuilder::default()
        .save_path("/tmp/shot.png")
        .build();
    assert_eq!(opts.save_path.as_deref(), Some("/tmp/shot.png"));
}

#[test]
fn screenshot_options_builder_sets_context_id() {
    let opts = BrowserScreenshotOptionsBuilder::default()
        .context_id("ctx-xyz".to_string())
        .build();
    assert!(opts.context_id.is_some());
}

#[test]
fn screenshot_options_builder_chains_multiple_fields() {
    let opts = BrowserScreenshotOptionsBuilder::default()
        .context_id("ctx".to_string())
        .save_path("/tmp/out.png")
        .build();
    assert!(opts.context_id.is_some());
    assert_eq!(opts.save_path.as_deref(), Some("/tmp/out.png"));
}
