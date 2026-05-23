mod bidi_browser;
pub mod cdp_browser;
pub mod chrome;
pub mod firefox;

pub use bidi_browser::BidiBrowser;
pub use bidi_browser::{
    AddPreloadScriptOptionsBuilder, AuthenticateOptionsBuilder, BrowserScreenshotOptionsBuilder,
    CreateContextOptionsBuilder, EmulateTimezoneOptionsBuilder, EvaluateScriptOptionsBuilder,
    FindNodesOptionsBuilder, NavigateOptions, NavigateOptionsBuilder, OnRequestOptionsBuilder,
    SubscribeEventsOptionsBuilder, WaitForNodesOptionsBuilder,
};
pub use chrome::browser::{ChromeBrowser, ChromeConfig, ChromeLaunchMode, chrome};
pub use chrome::capabilities::{ChromeCapabilities, ChromeOptions, PerfLoggingPrefs};
pub use chrome::tab::ChromeTab;
pub use firefox::browser::{FirefoxBrowser, FirefoxConfig, FirefoxLaunchMode, firefox};
pub use firefox::capabilities::{FirefoxCapabilities, FirefoxOptions};
