# sketchybar-rs
[![ci](https://github.com/johnallen3d/sketchybar-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/johnallen3d/sketchybar-rs/actions/workflows/ci.yml)

Send messages to [SketchyBar](https://github.com/FelixKratz/SketchyBar) from Rust! This library crate embeds the [SketchyBarHelper](https://github.com/FelixKratz/SketchyBarHelper) `sketchybar.h`.

## Usage

Add this to your `Cargo.toml`:
```toml
[dependencies]
sketchybar = "0.2"
```

Then call the `message` function:

```rust
extern crate sketchybar_rs;

fn main() {
    let _ = sketchybar_rs::message("--query bar", None);
}
```

More practically, update a widget on a secondary bar named `bottombar`:

```rust
extern crate sketchybar_rs;

fn main() {
    let _ = sketchybar_rs::message("--set weather label=42°F", Some("bottombar"));
}
```

## Why?

For fun. I'm learning Rust and [writing](https://github.com/johnallen3d/conditions) some [crates](https://github.com/johnallen3d/mp-cli) to execute as scripts for my widgets for practice.
