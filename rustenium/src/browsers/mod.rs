pub mod chrome;
pub mod firefox;
mod bidi_browser;
pub mod cdp_browser;

pub use bidi_browser::BidiBrowser;
pub use chrome::browser::{chrome, ChromeBrowser, ChromeConfig, ChromeLaunchMode};
pub use chrome::tab::ChromeTab;
pub use chrome::capabilities::{ChromeCapabilities, ChromeOptions, PerfLoggingPrefs};
pub use firefox::browser::{firefox, FirefoxBrowser, FirefoxConfig, FirefoxLaunchMode};
pub use firefox::capabilities::{FirefoxCapabilities, FirefoxOptions};
pub use bidi_browser::{
    NavigateOptions, BrowserScreenshotOptionsBuilder, NavigateOptionsBuilder, CreateContextOptionsBuilder,
    FindNodesOptionsBuilder, WaitForNodesOptionsBuilder, OnRequestOptionsBuilder,
    SubscribeEventsOptionsBuilder, EvaluateScriptOptionsBuilder, AddPreloadScriptOptionsBuilder,
    EmulateTimezoneOptionsBuilder, AuthenticateOptionsBuilder,
};