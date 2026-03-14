pub mod chrome;
pub mod capabilities;

pub use chrome::{
    BrowserScreenshotOptions, NavigateOptions, CreateContextOptions,
    FindNodesOptions, WaitForNodesOptions, OnRequestOptions, SubscribeEventsOptions,
    EvaluateScriptOptions, AddPreloadScriptOptions, EmulateTimezoneOptions, AuthenticateOptions,
};

