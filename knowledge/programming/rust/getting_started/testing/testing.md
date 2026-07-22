---
title: "Testing in Rust"
language: "rust"
category: "rust"
topic: "testing"
difficulty: "intermediate"
keywords: ["cargo test","unit test","integration test","assert"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Testing"
---

# Testing

Summary

Rust integrates testing through `cargo test`. It supports unit tests, integration tests, and documentation tests.

Explanation

- Annotate functions with `#[test]` to create unit tests.
- Use the `tests/` directory for integration tests.
- Documentation examples in `///` comments can be verified as doc tests.

Example

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn adds_two() {
        assert_eq!(2 + 2, 4);
    }
}
```

Best Practice

Write small focused tests and run them frequently. Use `cargo test -- --nocapture` to inspect output for failing cases.

See also

- cargo/cargo.md
