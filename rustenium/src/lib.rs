//! # Rustenium
//!
//! A modern, high-performance WebDriver BiDi automation library for Rust.
//!
//! ## Features
//!
//! - **WebDriver BiDi Protocol**: Native support for the next-generation WebDriver BiDi protocol
//! - **Chrome/Chromium Support**: Full Chrome and Chromium browser automation
//! - **High-Level API**: Intuitive, ergonomic API for browser automation tasks
//! - **Human-like Input**: Built-in support for realistic mouse movements and interactions
//! - **Touch Support**: Multi-touch gestures and touchscreen simulation
//! - **Network Interception**: Intercept and modify network requests
//! - **Screenshot Capture**: Element and viewport screenshot capabilities
//! - **Event Handling**: Subscribe to browser and page events
//! - **Type-Safe**: Strongly-typed API leveraging Rust's type system
//!
//! ## Quick Start
//!
//! ```no_run
//! use rustenium::browsers::{ChromeBrowser, ChromeConfig, create_chrome_browser};
//! use rustenium::css;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Option 1: Using ChromeBrowser::new
//!     let config = ChromeConfig {
//!         driver_executable_path: "chromedriver".to_string(),
//!         ..Default::default()
//!     };
//!     let mut browser = ChromeBrowser::new(config).await;
//!
//!     // Option 2: Using create_chrome_browser helper
//!     let mut browser = create_chrome_browser(None).await; // Uses default config
//!
//!     // Navigate to a page
//!     browser.open_url("https://example.com", None, None).await?;
//!
//!     // Find elements using CSS selectors
//!     let nodes = browser.find_nodes(css!("h1"), None, None, None, None).await?;
//!
//!     // Get text content
//!     if let Some(node) = nodes.first() {
//!         let text = node.get_inner_text().await;
//!         println!("Heading: {}", text);
//!     }
//!
//!     // Clean up
//!     browser.end_bidi_session().await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Main Components
//!
//! - [`browsers::ChromeBrowser`] - Main browser automation interface
//! - [`browsers::ChromeConfig`] - Browser configuration
//! - [`browsers::ChromeCapabilities`] - Browser capabilities builder
//! - [`browsers::create_chrome_browser()`] - Convenience function to create a browser
//! - [`nodes::ChromeNode`] - DOM element representation for Chrome
//! - [`input::BidiMouse`] - Direct, instant mouse movements for fast automation
//! - [`input::HumanMouse`] - Realistic mouse movements with Bezier curves and jitter
//! - [`input::Keyboard`] - Keyboard input for typing text and pressing keys
//! - [`input::Touchscreen`] - Multi-touch gesture support for mobile testing
//!
//! ## Macros
//!
//! Enable the `macros` feature for convenient selector macros:
//!
//! ```toml
//! [dependencies]
//! rustenium = { version = "0.1.1", features = ["macros"] }
//! ```
//!
//! Then use `css!()` and `xpath!()` macros for element selection.

mod drivers;
pub mod error;
pub mod browsers;
pub mod nodes;
pub mod input;


#[cfg(feature = "macros")]
pub use rustenium_macros::*;

pub use drivers::*;


#[cfg(test)]
mod tests {
    use super::*;
    
}
