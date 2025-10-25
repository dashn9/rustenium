use crate::chrome::ChromeDriver;

impl
pub struct ChromeBrowser {
    config: ChromeConfig,
    driver: ChromeDriver,
}

impl ChromeBrowser {
    pub fn new(config: ChromeConfig) -> ChromeBrowser {
        let driver_path = config.driver_path.clone();
        Self {
            driver: create_chrome_driver(&config),
            config,
        }
    }

    pub async fn open_url(&self, url: &str) {

    }
}


pub async fn create_chrome_browser(config: ChromeConfig) -> ChromeBrowser {
    let mut chrome_browser = ChromeBrowser::new(config);
    chrome_browser
}
fn create_chrome_driver<'a>(config: &ChromeConfig) -> ChromeDriver {
    let mut chrome_driver = ChromeDriver::new(config);
    chrome_driver
}
