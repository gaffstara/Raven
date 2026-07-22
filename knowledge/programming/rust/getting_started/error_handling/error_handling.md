---
title: "Error Handling in Rust"
language: "rust"
category: "rust"
topic: "error handling"
difficulty: "intermediate"
keywords: ["Result","Option","?","error handling"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Error Handling"
---

# Error Handling

Summary

Rust uses `Result` and `Option` to represent recoverable and optional values. The `?` operator propagates errors cleanly through call chains.

Explanation

- `Result<T, E>` is the standard error type for recoverable failure.
- `Option<T>` expresses a value that may or may not be present.
- `?` returns early if a `Result` is `Err`.

Example

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
```

Best Practice

Define error types with `thiserror` or `anyhow` for application-level error handling. Keep error messages clear and preserve context.

See also

- result types in the standard library
