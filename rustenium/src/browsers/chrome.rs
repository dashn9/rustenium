use rustenium_bidi_commands::browsing_context::commands::NavigateResult;
use rustenium_bidi_commands::browsing_context::types::{BrowsingContext, ReadinessState};
use crate::chrome::{ChromeConfig, ChromeDriver};
use crate::error::OpenUrlError;

pub struct ChromeBrowser {
    config: ChromeConfig,
    driver: ChromeDriver,
}

impl ChromeBrowser {
    pub async fn new(config: ChromeConfig) -> ChromeBrowser {
        Self {
            driver: create_chrome_driver(config.clone()).await,
            config,
        }
    }

    pub async fn open_url(
        &mut self,
        url: &str,
        wait: Option<ReadinessState>,
        context_id: Option<BrowsingContext>,
    ) -> Result<NavigateResult, OpenUrlError> {
        self.driver.open_url(url.to_string(), wait, context_id).await
    }
}


pub async fn create_chrome_browser(config: ChromeConfig) -> ChromeBrowser {
    let chrome_browser = ChromeBrowser::new(config).await;
    chrome_browser
}

async fn create_chrome_driver<'a>(config: ChromeConfig) -> ChromeDriver {
    let chrome_driver = ChromeDriver::new(config).await;
    chrome_driver
}
