# Rustenium

The most robust, high-performance multi-protocol browser automation library for Rust.

Rustenium provides a powerful and ergonomic API for browser automation using the WebDriver BiDi and Chrome DevTools Protocol (CDP). It offers both low-level control and high-level abstractions for common automation tasks, with the ability to use BiDi and CDP independently or together.

## Features

- **Dual Protocol Support**: Full WebDriver BiDi and Chrome DevTools Protocol (CDP) support
- **Flexible Launch Modes**: Manage Chrome yourself, connect to an existing instance, or let chromedriver handle it
- **Optional Protocols**: Enable BiDi, CDP, or both — connect and disconnect at runtime
- **Chrome Support**: First-class support for Chrome/Chromium browsers
- **Auto-Download**: Automatically downloads Chrome and chromedriver if not present
- **Flexible Input Methods**:
  - **BidiMouse**: Direct, precise mouse movements for fast automation
  - **HumanMouse**: Realistic mouse movements with Bezier curves and jitter to mimic human behavior
  - **Keyboard**: Full keyboard support with modifier keys
  - **Touchscreen**: Multi-touch gesture support for mobile testing
- **CSS & XPath Selectors**: Convenient macros (`css!()`, `xpath!()`) for element location
- **Screenshot Capture**: Take screenshots of elements or entire pages
- **Network Interception**: Monitor and intercept network requests with BiDi
- **Event System**: Subscribe to browser events in real-time
- **Script Evaluation**: Execute JavaScript with preload script support
- **Timezone Emulation**: Emulate different timezones for testing
- **Device Emulation**: Emulate device metrics via CDP for responsive testing
- **Tab Management**: Create and manage browser tabs via CDP
- **Type-Safe API**: Leverages Rust's type system for compile-time safety

## Installation

Add Rustenium to your `Cargo.toml`:

```toml
[dependencies]
rustenium = { version = "1.0.1", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
```

## Launch Modes

Rustenium supports three ways to launch Chrome:

```rust
use rustenium::browsers::{ChromeConfig, ChromeLaunchMode};

// SpawnAndAttach (default) — Rustenium starts Chrome and attaches chromedriver
let config = ChromeConfig::default();

// DriverManaged — Chromedriver spawns and manages Chrome
let config = ChromeConfig {
    launch_mode: ChromeLaunchMode::DriverManaged,
    ..Default::default()
};

// Remote — Connect to an existing Chrome instance
let config = ChromeConfig {
    launch_mode: ChromeLaunchMode::Remote(9222),
    ..Default::default()
};
```

## Protocol Selection

BiDi is enabled by default. CDP is opt-in. You can use them independently or together,
though note that CDP can become buggy in the presence of an active BiDi connection.
Connecting BiDi after CDP does not affect the CDP setup.

```rust
use rustenium::browsers::ChromeConfig;

// BiDi only (default)
let config = ChromeConfig::default();

// CDP only
let config = ChromeConfig {
    enable_bidi: false,
    enable_cdp: true,
    ..Default::default()
};

// Both
let config = ChromeConfig {
    enable_bidi: true,
    enable_cdp: true,
    ..Default::default()
};
```

You can also connect protocols at runtime:

```rust
// Start with CDP only, then connect BiDi later
let mut browser = ChromeBrowser::new(ChromeConfig {
    enable_bidi: false,
    enable_cdp: true,
    ..Default::default()
}).await;

// ... do CDP work ...

browser.connect_bidi().await; // connects BiDi without affecting CDP
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

    // Close the browser
    browser.close().await?;

    Ok(())
}
```

## Examples

### Browser Setup and Navigation (BiDi)

```rust
use rustenium::browsers::{ChromeConfig, NavigateOptionsBuilder, create_chrome_browser};
use rustenium_bidi_definitions::browsing_context::types::ReadinessState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = ChromeConfig::default();

    config.capabilities.add_arg("--disable-gpu")
        .add_args(["--window-size=1920,1080"])
        .accept_insecure_certs(true);

    let mut browser = create_chrome_browser(Some(config)).await;

    // Navigate and wait for load
    browser.navigate_with_options("https://example.com",
        NavigateOptionsBuilder::default()
            .wait(ReadinessState::Interactive)
            .build()
    ).await?;

    browser.close().await?;
    Ok(())
}
```

### CDP Navigation and Tab Management

```rust
use rustenium::browsers::{ChromeConfig, ChromeBrowser};
use rustenium::browsers::cdp_browser::CdpBrowser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut browser = ChromeBrowser::new(ChromeConfig {
        enable_bidi: false,
        enable_cdp: true,
        ..Default::default()
    }).await;

    // Navigate via CDP
    browser.navigate("https://example.com").await?;

    // Create a new tab
    let tab = browser.create_tab("https://example.org").await?;

    // Emulate device metrics
    browser.emulate_device_metrics(375, 812, 3.0, true).await?;

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
    browser.navigate("https://example.com").await?;

    // Using CSS selectors
    let element = browser.find_node(css!("#my-id")).await?;
    let buttons = browser.find_nodes(css!(".btn-primary")).await?;

    // Using XPath
    let headers = browser.find_nodes(xpath!("//h1[@class='title']")).await?;

    // Wait for elements to appear
    let node = browser.wait_for_node(css!(".dynamic-content")).await?;

    browser.close().await?;
    Ok(())
}
```

### Mouse Input — Precise Movements

```rust
use rustenium::browsers::create_chrome_browser;
use rustenium::input::{MouseMoveOptions, MouseClickOptions, Point};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut browser = create_chrome_browser(None).await;
    let context_id = browser.get_active_context_id()?;

    // Instant, precise movements
    browser.mouse().move_to(Point { x: 100.0, y: 200.0 }, &context_id, MouseMoveOptions {
        steps: Some(5),
        ..Default::default()
    }).await?;

    browser.mouse().click(None, &context_id, MouseClickOptions::default()).await?;

    browser.close().await?;
    Ok(())
}
```

### Mouse Input — Human-Like Movements

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

    browser.close().await?;
    Ok(())
}
```

### Keyboard Input

```rust
use rustenium::browsers::create_chrome_browser;
use rustenium::input::KeyboardTypeOptions;
use rustenium_macros::css;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut browser = create_chrome_browser(None).await;
    let context_id = browser.get_active_context_id()?;

    browser.navigate("https://example.com").await?;

    let text = browser.wait_for_node(css!("#text")).await?.expect("No node exists");

    // Type text with delay between keystrokes
    browser.keyboard().type_text(
        &text.get_inner_text().await,
        &context_id,
        Some(KeyboardTypeOptions { delay: Some(36) }),
    ).await?;

    // Modifier key combinations (Ctrl+A)
    browser.keyboard().down("Control", &context_id).await?;
    browser.keyboard().press("a", &context_id, None).await?;
    browser.keyboard().up("Control", &context_id).await?;

    browser.close().await?;
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
    browser.on_request(|request| async move {
        println!("Request URL: {}", request.url());

        if request.url().contains("ads.example.com") {
            let _ = request.abort().await;
            return;
        }
        request.continue_().await;
    }).await?;

    // Add authentication handler
    browser.authenticate("username", "password").await?;

    browser.navigate("https://example.com").await?;

    browser.close().await?;
    Ok(())
}
```

### Script Evaluation & Preload Scripts

```rust
use rustenium::browsers::create_chrome_browser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut browser = create_chrome_browser(None).await;

    // Evaluate JavaScript
    let result = browser.evaluate_script(
        "document.title".to_string(),
        true,
    ).await?;

    // Add a preload script that runs on every page load
    let script_id = browser.add_preload_script(
        "() => { window.__injected = true; }".to_string(),
    ).await?;

    // Remove it later
    browser.remove_preload_script(script_id).await?;

    browser.close().await?;
    Ok(())
}
```

### Timezone Emulation

```rust
use rustenium::browsers::create_chrome_browser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut browser = create_chrome_browser(None).await;

    browser.emulate_timezone(Some("Asia/Tokyo".to_string())).await?;
    browser.navigate("https://example.com").await?;

    browser.close().await?;
    Ok(())
}
```

## Crate Structure

| Crate | Description |
|---|---|
| `rustenium` | Main library — browser impls, input devices, node interactions |
| `rustenium-core` | Protocol transport, sessions, connections, event system |
| `rustenium-bidi-definitions` | WebDriver BiDi protocol type definitions |
| `rustenium-cdp-definitions` | Chrome DevTools Protocol type definitions |
| `rustenium-macros` | Procedural macros (`css!`, `xpath!`) |
| `rustenium-generator` | Code generator for protocol definitions from specs |

## Browser Support

Currently supported browsers:
- Chrome/Chromium (stable, beta, dev, canary)

Support for Firefox and other browsers is planned for future releases.

## Requirements

- Rust 1.85 or later (2024 edition)
- Chrome/Chromium browser (auto-downloaded if not present)

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

- [HumanCursor](https://github.com/riflosnake/HumanCursor) — the human-like mouse trajectory algorithm (Bezier curves, easing, Gaussian distortion) is ported from this excellent Python library
- [Chromium Oxide](https://github.com/mattsse/chromium-oxide) — inspiration for the protocol definition code generator
- [The Ish Bot](https://github.com/dashn9/ish-adf-bot) — some bot automation inspiration was borrowed from this project

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
