# Rustenium

A modern, high-performance WebDriver BiDi automation library for Rust.

Rustenium provides a powerful and ergonomic API for browser automation using the WebDriver BiDi protocol. It offers both low-level control and high-level abstractions for common automation tasks.

## Features

- **WebDriver BiDi Protocol**: Built on the modern BiDi protocol for bidirectional communication
- **Chrome Support**: First-class support for Chrome/Chromium browsers
- **Flexible Input Methods**:
  - **BidiMouse**: Direct, precise mouse movements for fast automation
  - **HumanMouse**: Realistic mouse movements with Bezier curves and jitter to mimic human behavior
  - **Keyboard**: Full keyboard support with modifier keys
  - **Touchscreen**: Multi-touch gesture support for mobile testing
- **CSS & XPath Selectors**: Convenient macros (`css!()`, `xpath!()`) for element location
- **Screenshot Capture**: Take screenshots of elements or entire pages
- **Network Interception**: Monitor and intercept network requests
- **Event System**: Subscribe to browser events in real-time
- **Type-Safe API**: Leverages Rust's type system for compile-time safety

## Installation

Add Rustenium to your `Cargo.toml`:

```toml
[dependencies]
rustenium = "0.1.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use rustenium::browsers::{create_chrome_browser};
use rustenium_macros::css;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Create a Chrome browser instance
  let mut browser = create_chrome_browser(None).await;

  // Navigate to a page
  browser.open_url("https://example.com", None, None).await;

  // Find and interact with elements
  let search_box = browser.find_node(css!("input[type='search']"), None, None, None, None).await.expect("failed to search for node").expect("No node found");
  search_box.screenshot(None, None, Some("./search_box_screenshot.png")).await?;

  let mut submit_button = browser.find_node(css!("button[type='submit']"), None, None, None, None).await.expect("failed to find node").expect("No node found");
  browser.click_on_node_bidi(&mut submit_button, None, None).await?;

  // Take a screenshot
  browser.screenshot(None, None, None, None, Some("screenshot.png")).await?;

  Ok(())
}
```

## Examples

### Browser Setup and Navigation

```rust
use rustenium::browsers::{ChromeConfig, create_chrome_browser};
use rustenium_bidi_commands::browsing_context::types::ReadinessState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

  // Create custom configuration
  let mut config = ChromeConfig::default();

  config.capabilities.add_arg("--disable-gpu")
          .add_args(["--window-size=1920,1080"])
          .accept_insecure_certs(true);

  // Create browser with custom config
  let mut browser = create_chrome_browser(Some(config)).await;

  // Navigate and wait for load
  browser.open_url("https://example.com", Some(ReadinessState::Interactive), None).await;
  Ok(())
}
```

### Finding Elements

```rust
use rustenium::browsers::{create_chrome_browser};
use rustenium_macros::css;
use rustenium_macros::xpath;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut browser = create_chrome_browser(None).await;

    // Using CSS selectors
    let element = browser.find_node(css!("#my-id"), None, None, None, None).await;
    let buttons = browser.find_nodes(css!(".btn-primary"), None, None, None, None).await;

    // Using XPath
    let header = browser.find_nodes(xpath!("//h1[@class='title']"), None, None, None, None).await;
    Ok(())
}
```

### Mouse Input - Precise Movements

```rust

use rustenium::browsers::{create_chrome_browser};
use rustenium::input::{MouseMoveOptions, Point};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut browser = create_chrome_browser(None).await;

    let context_id = browser.get_active_context_id().unwrap();

    // Instant, precise movements - Use this way only if you desire precise control
    browser.mouse().move_to(Point { x: 100.0, y: 200.0 }, &context_id, Some(MouseMoveOptions {
        steps: Some(5),
        origin: None,
    })).await;

    browser.mouse().click(None, &context_id, None).await?;
    Ok(())
}
```

### Mouse Input - Human-Like Movements

```rust
use rustenium::browsers::{create_chrome_browser};
use rustenium::input::{Mouse, Point};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut browser = create_chrome_browser(None).await;

    let context_id = browser.get_active_context_id().unwrap();

    // Realistic movements with Bezier curves and jitter
    browser.human_mouse().move_to(Point { x: 100.0, y: 200.0 }, &context_id, None).await?;

    browser.human_mouse().click(None, &context_id, None).await?;
    Ok(())
}
```

### Keyboard Input

```rust
use rustenium::browsers::{create_chrome_browser};
use rustenium::input::{KeyboardTypeOptions};
use rustenium::nodes::Node;
use rustenium_macros::css;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut  browser = create_chrome_browser(None).await;

  let context_id = browser.get_active_context_id().unwrap();

  browser.open_url("https://example.com", None, None);

  let mut text = browser.wait_for_node(css!("#text"), None, None, None).await.expect("Unable to search for node").expect("No node exists");

  browser.keyboard().down("Enter", &context_id).await.unwrap();
  sleep(Duration::from_secs(1)).await;
  browser.keyboard().up("Enter", &context_id).await.unwrap();

  browser.keyboard().type_text(&text.get_inner_text().await, &context_id, Some(KeyboardTypeOptions { delay: Some(3.6 as u64) })).await.unwrap();

  // Modifier key combinations
  browser.keyboard().down("Control", &context_id).await.unwrap();
  browser.keyboard().press("a", &context_id, None).await.unwrap(); // Ctrl+A
  browser.keyboard().up("Control", &context_id).await.unwrap();
}
```

### Network Interception

```rust
use rustenium::browsers::{create_chrome_browser};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut browser = create_chrome_browser(None).await;
  // Intercept and handle network requests
  browser.on_request_bidi(|request| async move {
    println!("Request URL: {}", request.url());

    // Block requests to specific domains
    if request.url().contains("ads.example.com") {
      let _ = request.abort().await;
      return;
    }
    request.continue_().await;
  }, None, None).await.unwrap();

  // Add authentication handler
  browser.authenticate(String::from("username"), String::from("password"), None, None).await?;

  Ok(())
}
```

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/rustenium).

## Browser Support

Currently supported browsers:
- Chrome/Chromium (stable, beta, dev, canary)

Support for Firefox and other browsers is planned for future releases.

## Requirements

- Rust 1.75 or later
- Chrome/Chromium browser installed (for Chrome automation)

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
