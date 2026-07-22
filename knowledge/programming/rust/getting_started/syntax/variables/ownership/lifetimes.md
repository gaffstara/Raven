---
title: "Lifetimes in Rust"
language: "rust"
category: "rust"
topic: "ownership"
difficulty: "advanced"
keywords: ["lifetime","borrow checker","references","scope"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Lifetimes"
---

# Lifetimes

Summary

Lifetimes describe the scope during which a reference is valid. Rust requires explicit lifetime annotations when the compiler cannot infer relationships between references.

Explanation

- Every reference has a lifetime that must not outlive the value it points to.
- Lifetime syntax uses apostrophes, for example `<'a>`, to parameterize references.
- Lifetime elision rules simplify common cases by inferring lifetimes automatically.

Example

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    let x = String::from("short");
    let y = String::from("longer");
    let result = longest(&x, &y);
    println!("{}", result);
}
```

Compiler Behavior

The borrow checker compares lifetimes across function parameters, return values, and local references. When lifetimes are incompatible, the compiler emits descriptive errors.

Best Practice

- Prefer lifetime elision when possible.
- Use explicit lifetimes when references are returned or stored in structs.
- Keep reference scopes short and avoid unnecessary borrowing.

See also

- ownership/borrowing.md
