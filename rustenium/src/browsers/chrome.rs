use rustenium_bidi_commands::browsing_context::commands::{LocateNodes, LocateNodesResult, NavigateResult};
use rustenium_bidi_commands::browsing_context::types::{BrowsingContext, Locator, ReadinessState};
use rustenium_bidi_commands::{CommandData, ResultData};
use rustenium_bidi_commands::script::types::{SerializationOptions, SharedReference};
use rustenium_core::error::SessionSendError;
use crate::chrome::{ChromeConfig, ChromeDriver};
use crate::error::{EvaluateResultError, FindNodesError, OpenUrlError};
use crate::nodes::chrome::ChromeNode;
use crate::nodes::Node;

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

    pub async fn find_nodes(
        &mut self,
        locator: Locator,
        context_id: Option<BrowsingContext>,
        max_node_count: Option<u64>,
        serialization_options: Option<SerializationOptions>,
        start_nodes: Option<Vec<SharedReference>>,
    ) -> Result<Vec<ChromeNode>, FindNodesError> {
        self.driver.find_nodes(locator, context_id, max_node_count, serialization_options, start_nodes).await
    }

    pub async fn update_node_position(&mut self, node: &mut ChromeNode) -> Result<bool, EvaluateResultError> {
        self.driver.update_node_position_bidi(node).await
    }
    pub async fn send_bidi_command(&mut self, command: CommandData) -> Result<ResultData, SessionSendError>  {
        self.driver.send_bidi_command(command).await
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
