---
title: "Overview of Ownership in Rust"
language: "rust"
category: "rust"
topic: "ownership"
difficulty: "intermediate"
keywords: ["ownership","move","borrow","lifetime"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Ownership, Rustonomicon"
---

# Ownership Overview

Summary

Ownership is Rust's core memory safety mechanism. Every value in Rust has a single owner, and the compiler enforces a clear set of rules that determine when values are moved, copied, borrowed, or dropped.

Explanation

- When a value is assigned or passed to a function, ownership may be transferred (moved) unless the type implements `Copy`.
- Rust tracks lifetime and borrowing relationships at compile time, so data races and invalid pointers are prevented without a garbage collector.
- The scope of ownership defines when values are dropped and resources are freed.

Definition

Ownership means a variable is responsible for cleaning up a value when it goes out of scope. The compiler enforces at most one mutable reference or many immutable references at any time.

Compiler Behavior

Rust uses the borrow checker to validate ownership rules. Move semantics become explicit when data is transferred, and the compiler rejects uses of values after they have been moved.

Example

```rust
fn main() {
    let a = String::from("hello");
    let b = a; // move: a is no longer valid
    println!("{}", b);
}
```

Notes

- `Copy` types are implicitly copied rather than moved.
- `Clone` provides explicit deep copies for non-`Copy` types.
- Ownership is fundamental to safe concurrency and predictable performance.

See also

- ownership/move.md
- ownership/borrowing.md
- ownership/lifetimes.md
