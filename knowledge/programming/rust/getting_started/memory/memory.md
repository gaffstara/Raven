---
title: "Memory Management in Rust"
language: "rust"
category: "rust"
topic: "memory"
difficulty: "intermediate"
keywords: ["memory","heap","stack","allocation"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Memory"
---

# Memory Management

Summary

Rust manages memory with ownership and borrowing. Stack allocation is deterministic, and heap allocation occurs when data is owned by `String`, `Vec`, or smart pointers.

Explanation

- Stack memory stores fixed-size values local to a function.
- Heap memory stores dynamically sized data owned by heap-backed containers.
- The compiler inserts cleanup code automatically when values leave scope.

Notes

- Avoid holding long-lived heap data when not necessary.
- Use `Box<T>` for recursive types and heap allocation of large values.

See also

- smart_pointers/smart_pointers.md
