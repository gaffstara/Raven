---
title: "Enums in Rust"
language: "rust"
category: "rust"
topic: "enums"
difficulty: "intermediate"
keywords: ["enum","variant","pattern matching","algebraic data type"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Enums"
---

# Enums

Summary

Enums define a type by enumerating possible variants. Rust enums support data attached to each variant, making them powerful algebraic data types.

Explanation

- Each enum variant can be unit-like, tuple-like, or struct-like.
- Enums pair naturally with `match` to handle each variant explicitly.
- `Option<T>` and `Result<T, E>` are the most important standard enums for optional values and error handling.

Example

```rust
enum Operation {
    Add,
    Multiply,
    Power(i32),
}

fn apply(op: Operation, a: i32, b: i32) -> i32 {
    match op {
        Operation::Add => a + b,
        Operation::Multiply => a * b,
        Operation::Power(exp) => a.pow(exp as u32),
    }
}
```

Best Practice

Use enums to represent finite choices and error conditions. Encapsulate state in variants rather than scattering boolean flags.

See also

- syntax/pattern_matching/pattern_matching.md
