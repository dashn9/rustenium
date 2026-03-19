use tokio::time::{Duration, sleep};
use rustenium::browsers::{create_chrome_browser, ChromeConfig, NavigateOptions};
use rustenium_bidi_definitions::browsing_context::types::ReadinessState;
use rustenium_macros::css;

#[tokio::test]
async fn open_browser() {
    let mut browser = create_chrome_browser(None).await;
    browser.navigate_with_options("https://linkedin.com", NavigateOptions {
        wait: Some(ReadinessState::Complete),
        ..Default::default()
    }).await.unwrap();
    // let elements = browser.find_nodes(css!("body")).await.unwrap();
    sleep(Duration::from_secs(13)).await;
}

#[tokio::test]
async fn test_auto_attach_mode() {
    let mut config = ChromeConfig::default();
    config.driver_executable_path = "chromedriver".to_string();
    config.remote_debugging_port = Some(0); // Auto mode

    let mut browser = create_chrome_browser(Some(config)).await;
    browser.navigate_with_options("https://example.com", NavigateOptions {
        wait: Some(ReadinessState::Complete),
        ..Default::default()
    }).await.unwrap();

    let nodes = browser.find_nodes(css!("body")).await.unwrap();
    assert!(!nodes.is_empty());
}

#[tokio::test]
#[ignore] // Manual test - requires Chrome running with --remote-debugging-port=9222
async fn test_manual_attach_mode() {
    let mut config = ChromeConfig::default();
    config.driver_executable_path = "chromedriver".to_string();
    config.remote_debugging_port = Some(9222); // Connect to existing Chrome on port 9222

    let mut browser = create_chrome_browser(Some(config)).await;
    browser.navigate_with_options("https://example.com", NavigateOptions {
        wait: Some(ReadinessState::Complete),
        ..Default::default()
    }).await.unwrap();

    let nodes = browser.find_nodes(css!("body")).await.unwrap();
    assert!(!nodes.is_empty());
}
