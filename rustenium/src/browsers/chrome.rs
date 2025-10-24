use crate::ChromeDriver;
pub struct ChromeBrowserConfig {
    pub driver_path: String,
    pub host: Option<String>,
    pub port: Option<u16>,
}
pub struct ChromeBrowser<'a> {
    config: ChromeBrowserConfig,
    driver: ChromeDriver<'a>,
}

impl<'a> ChromeBrowser<'a> {
    pub fn new(config: ChromeBrowserConfig) -> ChromeBrowser<'a> {
        let driver_path = config.driver_path.clone();
        Self {
            config,
            driver: create_chrome_driver(driver_path),
        }
    }
    pub async fn launch(&'a mut self) -> () {
        let host = self.config.host.clone().unwrap_or(String::from("localhost"));
        self.driver.launch(Some(host), self.config.port).await;
    }
}

async fn create_chrome_browser<'a>(config: ChromeBrowserConfig) -> ChromeBrowser<'a> {
    let mut chrome_browser = ChromeBrowser::new(config);
    chrome_browser.launch().await;
    chrome_browser
}
fn create_chrome_driver<'a>(driver_path: String) -> ChromeDriver<'a> {
    let mut chrome_driver = ChromeDriver::default();
    chrome_driver.driver.exe_path = driver_path;
    chrome_driver
}
