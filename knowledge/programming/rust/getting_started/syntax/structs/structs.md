---
title: "Structs in Rust"
language: "rust"
category: "rust"
topic: "structs"
difficulty: "beginner"
keywords: ["struct","data layout","fields","tuple struct"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Structs"
---

# Structs

Summary

Structs are Rust's primary way to define composite data. They group related fields into a named type.

Explanation

- Standard structs define named fields.
- Tuple structs are tuple-like with a named type.
- Unit-like structs have no fields and serve as markers.

Example

```rust
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };
    println!("({}, {})", p.x, p.y);
}
```

Best Practice

Use structs to model data with named fields. Keep them small and provide constructor functions when initialization is nontrivial.

See also

- syntax/enums/enums.md
