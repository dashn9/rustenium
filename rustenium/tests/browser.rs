use tokio::time::{Duration, sleep};
use rustenium::browsers::chrome::{create_chrome_browser};
use rustenium::chrome::ChromeConfig;

#[tokio::test]
async fn open_browser() {
    let config = ChromeConfig {
        driver_executable_path: "D:/Documents/m-workspace/rustenium/apps/drivers/chromedriver.exe".to_string(),
        host: None,
        port: None,
    };
    let mut browser = create_chrome_browser(config).await;
    browser.open_url("https://linkedin.com", None, None).await.unwrap();
    sleep(Duration::from_secs(13)).await;
}