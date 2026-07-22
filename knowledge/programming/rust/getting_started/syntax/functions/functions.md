---
title: "Functions in Rust"
language: "rust"
category: "rust"
topic: "functions"
difficulty: "beginner"
keywords: ["function","parameters","return","scope"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Functions"
---

# Functions

Summary

Functions encapsulate behavior in Rust. They accept arguments, return values, and form the basic building blocks of Rust programs.

Explanation

- A function signature begins with `fn`, followed by the name, parameter list, and optional return type.
- Rust uses expression syntax, so the last expression in a function can be returned without `return`.
- Functions can accept references to avoid moving ownership.

Example

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    let result = add(2, 3);
    println!("{}", result);
}
```

Notes

- Keep function bodies short and focused.
- Prefer explicit argument and return types for public APIs.
- Use `impl Trait` for flexible return types where appropriate.

See also

- closures/closures.md
