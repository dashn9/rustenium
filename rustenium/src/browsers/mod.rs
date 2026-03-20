mod chrome;
pub use chrome::chrome::{create_chrome_browser, ChromeBrowser, ChromeConfig};
pub use chrome::capabilities::{ChromeCapabilities, ChromeOptions, PerfLoggingPrefs};
pub use chrome::{
    BrowserScreenshotOptions, NavigateOptions, CreateContextOptions,
    FindNodesOptions, WaitForNodesOptions, OnRequestOptions, SubscribeEventsOptions,
    EvaluateScriptOptions, AddPreloadScriptOptions, EmulateTimezoneOptions, AuthenticateOptions,
};