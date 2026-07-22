---
title: "Ownership in Rust"
language: "rust"
category: "fundamentals"
topic: "ownership"
tags: ["rust","ownership"]
version: "1.0"
difficulty: "intermediate"
source: "local"
last_updated: "2026-07-22T00:00:00Z"
---

# Ownership in Rust

Summary

Ownership is Rust's central memory management model. It enforces a single owner for each value.

Explanation

- Each value has a single owner.
- When the owner goes out of scope, the value is dropped.
- Ownership can be transferred (moved) or borrowed.

Example

```rust
fn main() {
    let s = String::from("hello");
    let t = s; // move: s no longer valid
    println!("{}", t);
}
```

Notes

- Understand moves vs borrows to avoid compile errors.

References

- The Rust Programming Language (book)
