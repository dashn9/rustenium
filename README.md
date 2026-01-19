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
use rustenium::browsers::{ChromeBrowser, ChromeConfig, create_chrome_browser};
use rustenium::css;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a Chrome browser instance
    let config = ChromeConfig::default();
    let browser = create_chrome_browser(config, None).await?;

    // Navigate to a page
    browser.navigate("https://example.com").await?;

    // Find and interact with elements
    let search_box = browser.find_element(css!("input[type='search']")).await?;
    search_box.send_keys("Rustenium").await?;

    let submit_button = browser.find_element(css!("button[type='submit']")).await?;
    submit_button.click().await?;

    // Take a screenshot
    browser.screenshot(None, None, Some("screenshot.png")).await?;

    // Close the browser
    browser.close().await?;

    Ok(())
}
```

## Examples

### Browser Setup and Navigation

```rust
use rustenium::browsers::{ChromeConfig, create_chrome_browser, ChromeCapabilities};

// Create custom configuration
let mut config = ChromeConfig::default();
config.headless = true;

// Configure capabilities
let mut caps = ChromeCapabilities::default();
caps.add_arg("--disable-gpu")
    .add_arg("--window-size=1920,1080")
    .accept_insecure_certs(true);

// Create browser with custom config
let browser = create_chrome_browser(config, Some(caps)).await?;

// Navigate and wait for load
browser.navigate("https://example.com").await?;
```

### Finding Elements

```rust
use rustenium::css;
use rustenium::xpath;

// Using CSS selectors
let element = browser.find_element(css!("#my-id")).await?;
let buttons = browser.find_elements(css!(".btn-primary")).await?;

// Using XPath
let header = browser.find_element(xpath!("//h1[@class='title']")).await?;
```

### Mouse Input - Precise Movements

```rust
use rustenium::input::{BidiMouse, Point};

let mouse = BidiMouse::new(session);

// Instant, precise movements
mouse.move_to(Point { x: 100.0, y: 200.0 }, &context, None).await?;
mouse.click(None, &context, None).await?;

// Double-click
mouse.click(
    Some(Point { x: 300.0, y: 400.0 }),
    &context,
    Some(MouseClickOptions {
        count: Some(2),
        ..Default::default()
    })
).await?;
```

### Mouse Input - Human-Like Movements

```rust
use rustenium::input::{BidiMouse, HumanMouse, Point};

let bidi_mouse = BidiMouse::new(session);
let human_mouse = HumanMouse::new(bidi_mouse);

// Realistic movements with Bezier curves and jitter
human_mouse.move_to(Point { x: 500.0, y: 300.0 }, &context, None).await?;
human_mouse.click(None, &context, None).await?;

// Scroll with natural delays
human_mouse.wheel(&context, Some(MouseWheelOptions {
    delta_y: Some(100),
    ..Default::default()
})).await?;
```

### Keyboard Input

```rust
use rustenium::input::Keyboard;

let keyboard = Keyboard::new(session);

// Type text
keyboard.type_text("Hello, World!", &context, None).await?;

// Press special keys
keyboard.press("Enter", &context, None).await?;
keyboard.press("Tab", &context, None).await?;

// Modifier key combinations
keyboard.down("Control", &context).await?;
keyboard.press("a", &context, None).await?; // Ctrl+A
keyboard.up("Control", &context).await?;
```

### Multi-Touch Gestures

```rust
use rustenium::input::Touchscreen;
use std::sync::Arc;

let touchscreen = Arc::new(Touchscreen::new(session));

// Pinch gesture with two fingers
let touch1 = touchscreen.touch_start(100.0, 200.0, &context, None).await?;
let touch2 = touchscreen.touch_start(300.0, 200.0, &context, None).await?;

// Move fingers closer together
touch1.move_to(150.0, 200.0, &context).await?;
touch2.move_to(250.0, 200.0, &context).await?;

// Release touches
touch1.end(&context).await?;
touch2.end(&context).await?;
```

### Screenshots

```rust
use rustenium_bidi_commands::browsing_context::types::OriginUnion;

// Capture full viewport
browser.screenshot(None, None, Some("viewport.png")).await?;

// Capture specific element
let element = browser.find_element(css!("#content")).await?;
element.screenshot(None, None, Some("element.png")).await?;
```

### Network Interception

```rust
// Intercept and handle network requests
browser.on_request_bidi(|request| async move {
    println!("Request URL: {}", request.params.base_parameters.request.url);

    // Block requests to specific domains
    if request.params.base_parameters.request.url.contains("ads.example.com") {
        let _ = request.abort().await;
    }
}).await?;

// Add authentication handler
browser.add_authentication(|params| async move {
    // Return credentials for authentication challenges
    AuthenticationCredentials {
        username: "user".to_string(),
        password: "pass".to_string(),
    }
}).await?;
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
