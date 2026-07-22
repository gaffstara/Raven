---
title: "Strings in Rust"
language: "rust"
category: "rust"
topic: "strings"
difficulty: "intermediate"
keywords: ["String","&str","UTF-8","text"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Strings"
---

# Strings

Summary

Rust distinguishes owned strings (`String`) from string slices (`&str`). Strings are UTF-8 encoded and require careful handling when slicing or building text.

Explanation

- `String` owns heap-allocated UTF-8 data.
- `&str` borrows a string slice without owning the data.
- Use `to_string()` or `String::from` to create owned strings.

Example

```rust
fn main() {
    let hello = String::from("Hello");
    let slice: &str = &hello[0..5];
    println!("{} {}", hello, slice);
}
```

Common Pitfalls

- Slicing on byte offsets can panic if not aligned to UTF-8 code points.
- Use `chars()` or `graphemes` for Unicode-safe indexing.

See also

- slices/slices.md
