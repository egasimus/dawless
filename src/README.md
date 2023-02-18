# Dawless

## Device support crates

Each crate is responsible for a brand of devices.
The file layout is as follows:

```
README.md      - documentation entry point
Cargo.toml     - crate manifest
_brand.rs      - crate entry point
_brand-cli.rs  - CLI entry point
_brand-tui.rs  - TUI entry point
device1.rs     - device 1 data model
device1-cli.rs - device 1 CLI commands
device1-tui.rs - device 1 TUI components
device2.rs     - device 2 data model
device2-cli.rs - ...
device2-tui.rs . ...
...
```
