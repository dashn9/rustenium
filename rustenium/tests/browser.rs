use tokio::time::{Duration, sleep};
use rustenium::{Driver, ChromeDriver};
use rustenium::browsers::chrome::{create_chrome_browser, ChromeBrowserConfig};
use rustenium_core::session::SessionConnectionType;

#[tokio::test]
async fn open_browser() {
    let config = ChromeBrowserConfig {
        driver_path: "D:/Documents/m-workspace/rustenium/apps/drivers/chromedriver.exe".to_string(),
        host: None,
        port: None,
    };
    let mut browser = create_chrome_browser(config).await;
    sleep(Duration::from_secs(13)).await;
    drop(browser);
}