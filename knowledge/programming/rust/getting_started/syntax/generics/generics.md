---
title: "Generics in Rust"
language: "rust"
category: "rust"
topic: "generics"
difficulty: "intermediate"
keywords: ["generic","type parameter","trait bound","monomorphization"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Generics"
---

# Generics

Summary

Generics allow functions, structs, enums, and traits to operate on many types while preserving type safety.

Explanation

- Use type parameters like `T` inside angle brackets.
- Apply trait bounds to constrain permissible types.
- Rust monomorphizes generic code at compile time, producing efficient specialized variants.

Example

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

Notes

- Prefer generic parameters when code logic is independent of concrete types.
- Use `where` clauses for readability in complex bounds.

See also

- traits/traits.md
