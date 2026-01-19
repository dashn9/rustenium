# rustenium-macros

Convenience macros for Rustenium element selection.

This crate provides ergonomic macros for creating element locators:

- `css!()` - Create CSS selector locators
- `xpath!()` - Create XPath selector locators

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
rustenium = { version = "0.1.1", features = ["macros"] }
```

Then use in your code:

```rust
use rustenium::css;
use rustenium::xpath;

// CSS selector
let button = browser.find_element(css!("button.submit")).await?;

// XPath selector
let header = browser.find_element(xpath!("//h1[@class='title']")).await?;
```

## Part of Rustenium

This is an optional component of the [Rustenium](https://github.com/dashn9/rustenium) project.
See the main [Rustenium documentation](https://docs.rs/rustenium) for complete usage examples.

## License

MIT
