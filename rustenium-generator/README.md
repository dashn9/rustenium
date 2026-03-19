# rustenium-generator

Code generator that parses protocol definition files and outputs typed Rust modules.

## Supported Frontends

- **CDP** (Chrome DevTools Protocol) — parses `.pdl` files
- **BiDi** (WebDriver BiDi) — parses `.cddl` files

## Usage

Set the output directories via environment variables, then run:

```sh
CDP_OUT_DIR=../rustenium-cdp-definitions/src BIDI_OUT_DIR=../rustenium-bidi-definitions/src cargo run
```

The generator reads PDL files from `raw/pdl/` and CDDL files from `raw/cddl/`, writing Rust source to the specified output directories.

## Generated Structure

```
<out_dir>/
  lib.rs              # Top-level: Binary type, Command/Type/Event group enums, CommandResult trait
  macros.rs            # group_enum! and impl_from! macros
  <protocol>/
    mod.rs             # Protocol-level group enums (e.g. BrowserProtocolCommands)
    <module>/
      mod.rs
      types.rs         # Type definitions, newtypes, sub-enums
      commands.rs      # Command params, method enums, definition structs
      results.rs       # Command return types
      events.rs        # Event structs
      command_builders.rs
      type_builders.rs
```

## Key Conventions

- **Group enums** aggregate items per module/protocol with `From`/`TryFrom` impls:
  - Module: `AccessibilityTypes`, `AccessibilityCommands`, `AccessibilityEvents`
  - Protocol: `BrowserProtocolTypes`, `BrowserProtocolCommands`, `BrowserProtocolEvents`
  - Top: `Type`, `Command`, `Event`

- **Transitive `From`** — leaf types convert directly to any ancestor group enum without chaining (via `impl_from!` macro).

- **`CommandResult` trait** — every command implements `CommandResult` with an associated `Result` type and a `result_from_value(Value)` deserializer. Commands without explicit returns get an empty result struct. The `Result` type is bounded by `DeserializeOwned + Debug`.
  ```rust
  // Deserialize a command's result directly:
  let result = Evaluate::result_from_value(json_value)?;
  ```

- **Cross-module refs** resolve to qualified paths (e.g. `super::super::runtime::types::ScriptId`).

- **Serde** — all generated types derive `Serialize`/`Deserialize` with accurate `rename`, `skip_serializing_if`, and `default` attributes.

- Empty files are omitted. `Default` is only derived when all fields are optional.

## Assumptions

- Input PDL files follow the Chromium PDL format.
- Input CDDL files follow the WebDriver BiDi specification format.
- `rustfmt` (2024 edition) is available on `PATH` for formatting output.
- The output directory will be created if it doesn't exist; existing files are overwritten.
