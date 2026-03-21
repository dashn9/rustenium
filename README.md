# Rustenium

The most robust, high-performance WebDriver BiDi automation library for Rust.

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
rustenium = "1.0.0"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use rustenium::browsers::create_chrome_browser;
use rustenium_macros::css;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Create a Chrome browser instance
  let mut browser = create_chrome_browser(None).await;

  // Navigate to a page
  browser.navigate("https://example.com").await?;

  // Find and interact with elements
  let search_box = browser.find_node(css!("input[type='search']")).await?.expect("No node found");
  search_box.screenshot().await?;

  let mut submit_button = browser.find_node(css!("button[type='submit']")).await?.expect("No node found");
  submit_button.mouse_click().await?;

  // Take a screenshot
  browser.screenshot().await?;

  Ok(())
}
```

## Examples

### Browser Setup and Navigation

```rust
use rustenium::browsers::{ChromeConfig, NavigateOptions, create_chrome_browser};
use rustenium_bidi_definitions::browsing_context::types::ReadinessState;

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
  browser.navigate_with_options("https://example.com", NavigateOptions {
    wait: Some(ReadinessState::Interactive),
    ..Default::default()
  }).await?;

  Ok(())
}
```

### Finding Elements

```rust
use rustenium::browsers::create_chrome_browser;
use rustenium_macros::{css, xpath};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut browser = create_chrome_browser(None).await;

    // Using CSS selectors
    let element = browser.find_node(css!("#my-id")).await?;
    let buttons = browser.find_nodes(css!(".btn-primary")).await?;

    // Using XPath
    let headers = browser.find_nodes(xpath!("//h1[@class='title']")).await?;
    Ok(())
}
```

### Mouse Input - Precise Movements

```rust
use rustenium::browsers::create_chrome_browser;
use rustenium::input::{MouseMoveOptions, Point};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut browser = create_chrome_browser(None).await;

    let context_id = browser.get_active_context_id()?;

    // Instant, precise movements - Use this way only if you desire precise control
    browser.mouse().move_to(Point { x: 100.0, y: 200.0 }, &context_id, MouseMoveOptions {
        steps: Some(5),
        ..Default::default()
    }).await?;

    browser.mouse().click(None, &context_id, MouseClickOptions::default()).await?;
    Ok(())
}
```

### Mouse Input - Human-Like Movements

```rust
use rustenium::browsers::create_chrome_browser;
use rustenium::input::{MouseMoveOptions, MouseClickOptions, Point};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut browser = create_chrome_browser(None).await;

    let context_id = browser.get_active_context_id()?;

    // Realistic movements with Bezier curves and jitter
    browser.human_mouse().move_to(Point { x: 100.0, y: 200.0 }, &context_id, MouseMoveOptions::default()).await?;

    browser.human_mouse().click(None, &context_id, MouseClickOptions::default()).await?;
    Ok(())
}
```

### Keyboard Input

```rust
use rustenium::browsers::create_chrome_browser;
use rustenium::input::KeyboardTypeOptions;
use rustenium_macros::css;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut browser = create_chrome_browser(None).await;

  let context_id = browser.get_active_context_id()?;

  browser.navigate("https://example.com").await?;

  let text = browser.wait_for_node(css!("#text")).await?.expect("No node exists");

  browser.keyboard().down("Enter", &context_id).await?;
  sleep(Duration::from_secs(1)).await;
  browser.keyboard().up("Enter", &context_id).await?;

  browser.keyboard().type_text(&text.get_inner_text().await, &context_id, Some(KeyboardTypeOptions { delay: Some(36) })).await?;

  // Modifier key combinations
  browser.keyboard().down("Control", &context_id).await?;
  browser.keyboard().press("a", &context_id, None).await?; // Ctrl+A
  browser.keyboard().up("Control", &context_id).await?;

  Ok(())
}
```

### Network Interception

```rust
use rustenium::browsers::create_chrome_browser;

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
  }).await?;

  // Add authentication handler
  browser.authenticate("username", "password").await?;

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

- Rust 1.85 or later (2024 edition)
- Chrome/Chromium browser installed (for Chrome automation)

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

- [HumanCursor](https://github.com/riflosnake/HumanCursor) — the human-like mouse trajectory algorithm (Bezier curves, easing, Gaussian distortion) is ported from this excellent Python library
- [Chromium Oxide](https://github.com/mattsse/chromium-oxide) — inspiration for the protocol definition code generator
- [The Ish Bot](https://github.com/dashn9/ish-adf-bot) — some bot automation inspiration was borrowed from this project

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
