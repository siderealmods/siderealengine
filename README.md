## Note: This project is still work in progress.

![SiderealEngine Logo](./assets/logo.svg "SiderealEngine")

---

# SiderealEngine

A 2D / 3D graphics engine for games, arts and applications. Flexibly embeddable to your project.

## Features

### Embeddable

TODO

### Multiple Frameworks

Sidereal includes 2 options. These frameworks are useful to develop games and applications.

1. ECS Framework ([`sidereal-ecs`](https://github.com/siderealmods/ecs) crate)

   Designed for gamedev.

2. AdaptiveFramework ([`sidereal-adaptive`](https://github.com/siderealmods/adaptive) crate)

   Designed for application. Supports retained mode rendering.

- or you can use sidereal without these frameworks!

By implementing `sidereal-framework`'s traits, you can make a custom framework.

### Modules and Ecosystems

TODO

## Getting started

## Rust

### Installation

```bash
cargo add sidereal
```

### `main.rs`

```rs
use sidereal::prelude::*;

fn main() {
    Sidereal::new()
}
```

## Examples

TODO
