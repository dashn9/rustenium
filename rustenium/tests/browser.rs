use tokio::time::{Duration, sleep};
use rustenium::browsers::{create_chrome_browser, ChromeConfig};
use rustenium::css;
use rustenium_bidi_commands::browsing_context::types::{CssEnum, CssLocator, Locator, ReadinessState};

#[tokio::test]
async fn open_browser() {
    let mut browser = create_chrome_browser(None).await;
    browser.open_url("https://linkedin.com", Some(ReadinessState::Complete), None).await.unwrap();
    let elements = browser.find_nodes(css!("body"), None, None, None, None).await.unwrap();
    for mut element in elements {
        browser.update_node_position(&mut element).await.unwrap();
    }
    sleep(Duration::from_secs(13)).await;
}