---
title: "Copy and Clone in Rust"
language: "rust"
category: "rust"
topic: "ownership"
difficulty: "intermediate"
keywords: ["copy","clone","ownership","semantics"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Copy and Clone"
---

# Copy and Clone

Summary

`Copy` is a marker trait for implicit bitwise copy semantics. `Clone` is an explicit trait for deep cloning. Types implementing `Copy` can be duplicated by simple assignment.

Explanation

- Primitive scalar types like `i32`, `bool`, and `char` implement `Copy`.
- Compound types composed solely of `Copy` fields also implement `Copy`.
- Types that allocate on the heap, such as `String` and `Vec<T>`, do not implement `Copy` by default.
- Use `clone()` to explicitly duplicate non-`Copy` values.

Example

```rust
fn main() {
    let x = 42;
    let y = x; // copy: x remains usable
    println!("x={} y={}", x, y);

    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy
    println!("{} {}", s1, s2);
}
```

Best Practice

Prefer `Copy` types for small, simple values because moves are free. Reserve `Clone` for cases where duplicating heap-owned data is necessary.

Notes

- `Clone` can be implemented manually for custom deep-copy behavior.
- Derive `Copy` and `Clone` when semantically appropriate.

See also

- ownership/move.md
- ownership/drop.md
