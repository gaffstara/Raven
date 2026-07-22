---
title: "Closures in Rust"
language: "rust"
category: "rust"
topic: "closures"
difficulty: "intermediate"
keywords: ["closure","Fn","FnMut","FnOnce","capture"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Closures"
---

# Closures

Summary

Closures are anonymous functions that can capture variables from their environment. They can implement `Fn`, `FnMut`, or `FnOnce` depending on capture behavior.

Explanation

- Closures use parameter lists and an expression body, just like functions.
- Captures can be by reference, mutable reference, or value.
- Closures are useful for iterator adapters, callback APIs, and local computations.

Example

```rust
fn main() {
    let multiplier = 3;
    let multiply = |x| x * multiplier;
    println!("{}", multiply(5));
}
```

Notes

- Use closures to keep code concise.
- Be mindful of capture semantics when working with ownership and borrowing.

See also

- syntax/functions/functions.md
