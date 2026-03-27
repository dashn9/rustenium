pub mod chrome;
mod bidi_browser;

pub use bidi_browser::BidiBrowser;
pub use chrome::chrome::{create_chrome_browser, ChromeBrowser, ChromeConfig};
pub use chrome::capabilities::{ChromeCapabilities, ChromeOptions, PerfLoggingPrefs};
pub use chrome::{
    BrowserScreenshotOptionsBuilder, NavigateOptionsBuilder, CreateContextOptionsBuilder,
    FindNodesOptionsBuilder, WaitForNodesOptionsBuilder, OnRequestOptionsBuilder,
    SubscribeEventsOptionsBuilder, EvaluateScriptOptionsBuilder, AddPreloadScriptOptionsBuilder,
    EmulateTimezoneOptionsBuilder, AuthenticateOptionsBuilder,
};