pub mod chrome;
mod bidi_browser;
pub mod cdp_browser;

pub use bidi_browser::BidiBrowser;
pub use chrome::browser::{create_chrome_browser, ChromeBrowser, ChromeConfig, ChromeLaunchMode};
pub use chrome::tab::ChromeTab;
pub use chrome::capabilities::{ChromeCapabilities, ChromeOptions, PerfLoggingPrefs};
pub use bidi_browser::{
    BrowserScreenshotOptionsBuilder, NavigateOptionsBuilder, CreateContextOptionsBuilder,
    FindNodesOptionsBuilder, WaitForNodesOptionsBuilder, OnRequestOptionsBuilder,
    SubscribeEventsOptionsBuilder, EvaluateScriptOptionsBuilder, AddPreloadScriptOptionsBuilder,
    EmulateTimezoneOptionsBuilder, AuthenticateOptionsBuilder,
};