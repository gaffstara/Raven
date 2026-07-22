---
title: "Traits in Rust"
language: "rust"
category: "rust"
topic: "traits"
difficulty: "intermediate"
keywords: ["trait","implementation","dynamic dispatch","bounds"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Traits"
---

# Traits

Summary

Traits define shared behavior in Rust. They declare methods that types can implement, enabling polymorphism and generic programming.

Explanation

- A trait is similar to an interface in other languages.
- Types implement traits using `impl Trait for Type`.
- Traits support static dispatch with generic bounds and dynamic dispatch with trait objects.

Example

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article { title: String, content: String }

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}: {}...", self.title, &self.content[..20])
    }
}
```

Best Practice

Use traits for behavior abstraction and to make code extensible. Avoid overly broad traits; keep them cohesive.

See also

- generics/generics.md
