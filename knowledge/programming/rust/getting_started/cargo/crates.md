---
title: "Crates and Dependencies"
language: "rust"
category: "rust"
topic: "cargo"
difficulty: "intermediate"
keywords: ["crate","dependency","semver","crates.io"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Cargo Book, Crates.io"
---

# Crates and Dependencies

Summary

A crate is the compilation unit in Rust. Dependencies come from `crates.io`, git repositories, or local paths.

Explanation

- Add dependencies under `[dependencies]` in `Cargo.toml`.
- Semantic versioning controls compatible upgrades.
- Use `cargo update` and `cargo check` to manage dependency updates.

Example

```toml
[dependencies]
reqwest = "0.11"
serde = { version = "1.0", features = ["derive"] }
```

Notes

- Prefer minimal dependency sets to reduce build times and security surface.

See also

- cargo/cargo.md
