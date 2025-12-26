# rustenium-bidi-commands

WebDriver BiDi command types and data structures for Rustenium.

This crate provides strongly-typed representations of all WebDriver BiDi protocol commands, events, and data structures, including:

- Browsing context commands and events
- Network interception types
- Script evaluation structures
- Input action definitions
- Session management types
- Storage and cookie types
- Web extension types

All types are serializable/deserializable via Serde and validated using serde_valid.

## Usage

This crate is typically used through the main `rustenium` crate, which provides a higher-level API.

For direct usage, see the [documentation](https://docs.rs/rustenium-bidi-commands).

## Part of Rustenium

This is an internal component of the [Rustenium](https://github.com/dashn9/rustenium) project.
See the main [Rustenium documentation](https://docs.rs/rustenium) for usage examples.

## License

MIT
