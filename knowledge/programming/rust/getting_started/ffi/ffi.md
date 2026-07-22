---
title: "FFI in Rust"
language: "rust"
category: "rust"
topic: "ffi"
difficulty: "advanced"
keywords: ["ffi","extern","C","unsafe"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust FFI Omnibus"
---

# Foreign Function Interface (FFI)

Summary

Rust's FFI allows calling and exposing functions to C and other languages via `extern` blocks and `unsafe` boundaries.

Explanation

- Use `extern "C"` to define C-compatible functions and types.
- Raw pointers and `unsafe` are required to cross the language boundary.
- Verify ABI compatibility and memory ownership carefully.

Example

```rust
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Notes

- Use `#[repr(C)]` on structs passed across FFI boundaries.
- Avoid Rust-only types like `String` in public FFI APIs.

See also

- unsafe/unsafe.md
