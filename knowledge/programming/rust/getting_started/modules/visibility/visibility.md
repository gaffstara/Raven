---
title: "Visibility and Modules in Rust"
language: "rust"
category: "rust"
topic: "modules"
difficulty: "intermediate"
keywords: ["module","visibility","pub","crate"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Modules"
---

# Visibility and Modules

Summary

Rust modules organize code and control visibility with the `pub` keyword. Private items are visible only within their module.

Explanation

- `mod` defines a module. `pub` makes items visible outside the current module.
- Use `pub(crate)` to expose items within the crate only.
- Nested modules can re-export items with `pub use`.

Example

```rust
mod network {
    pub fn connect() {}
    fn internal_id() {}
}

fn main() {
    network::connect();
}
```

Notes

- Keep module boundaries clear and avoid overly broad public APIs.
- Re-export common items at the crate root for cleaner consumer imports.

See also

- cargo/cargo.md
