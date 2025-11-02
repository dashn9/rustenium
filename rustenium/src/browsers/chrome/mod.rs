pub mod chrome;
pub mod capabilities;

pub use chrome::{ChromeBrowser, ChromeConfig, create_chrome_browser};
pub use capabilities::{ChromeCapabilities, PerfLoggingPrefs};