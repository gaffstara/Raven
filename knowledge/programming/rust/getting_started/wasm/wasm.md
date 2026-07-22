---
title: "WebAssembly with Rust"
language: "rust"
category: "rust"
topic: "wasm"
difficulty: "advanced"
keywords: ["wasm","wasm-bindgen","target","webassembly"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust and WebAssembly Book"
---

# Rust and WebAssembly

Summary

Rust can compile to WebAssembly for safe, high-performance code in the browser and on the server.

Explanation

- Install the `wasm32-unknown-unknown` target with rustup.
- Use tools like `wasm-bindgen` to interoperate with JavaScript.
- Keep the public interface minimal and avoid unsupported standard library features.

Example

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

Notes

- WebAssembly modules are sandboxed and use linear memory.
- Use `wee_alloc` or `wasm-bindgen` for smaller binaries.

See also

- compiler/compiler.md
