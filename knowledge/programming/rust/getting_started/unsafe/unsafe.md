---
title: "Unsafe Rust"
language: "rust"
category: "rust"
topic: "unsafe"
difficulty: "advanced"
keywords: ["unsafe","raw pointer","undefined behavior","invariant"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rustonomicon, Unsafe Rust"
---

# Unsafe Rust

Summary

Unsafe Rust allows operations that the compiler cannot guarantee are safe, such as raw pointer dereferencing and calling foreign functions.

Explanation

- `unsafe` blocks and `unsafe fn` unlock operations like dereferencing raw pointers and implementing unsafe traits.
- The programmer is responsible for upholding invariants and avoiding undefined behavior.
- Keep unsafe code small, isolated, and carefully audited.

Example

```rust
unsafe fn get_value(ptr: *const i32) -> i32 {
    *ptr
}
```

Best Practice

Encapsulate unsafe code in safe abstractions. Document assumptions and invariants clearly.

See also

- ffi/ffi.md
