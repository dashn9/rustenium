use tokio::time::{Duration, sleep};
use rustenium::browsers::{create_chrome_browser, ChromeConfig};
use rustenium_bidi_commands::browsing_context::types::ReadinessState;
use rustenium_macros::css;

#[tokio::test]
async fn open_browser() {
    let mut browser = create_chrome_browser(None).await;
    browser.open_url("https://linkedin.com", Some(ReadinessState::Complete), None).await.unwrap();
    let elements = browser.find_nodes(css!("body"), None, None, None, None).await.unwrap();
    sleep(Duration::from_secs(13)).await;
}

#[tokio::test]
async fn test_auto_attach_mode() {
    let mut config = ChromeConfig::default();
    config.driver_executable_path = "chromedriver".to_string();
    config.remote_debugging_port = Some(0); // Auto mode

    let mut browser = create_chrome_browser(Some(config)).await;
    browser.open_url("https://example.com",
                    Some(ReadinessState::Complete),
                    None).await.unwrap();

    let nodes = browser.find_nodes(css!("body"), None, None, None, None)
                       .await.unwrap();
    assert!(!nodes.is_empty());
}

#[tokio::test]
#[ignore] // Manual test - requires Chrome running with --remote-debugging-port=9222
async fn test_manual_attach_mode() {
    let mut config = ChromeConfig::default();
    config.driver_executable_path = "chromedriver".to_string();
    config.remote_debugging_port = Some(9222); // Connect to existing Chrome on port 9222

    let mut browser = create_chrome_browser(Some(config)).await;
    browser.open_url("https://example.com",
                    Some(ReadinessState::Complete),
                    None).await.unwrap();

    let nodes = browser.find_nodes(css!("body"), None, None, None, None)
                       .await.unwrap();
    assert!(!nodes.is_empty());
}