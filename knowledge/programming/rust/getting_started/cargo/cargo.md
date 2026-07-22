---
title: "Cargo: Rust Package Management"
language: "rust"
category: "rust"
topic: "cargo"
difficulty: "beginner"
keywords: ["cargo","package","build","dependencies"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Cargo Book"
---

# Cargo

Summary

Cargo is Rust's package manager and build tool. It manages dependencies, builds packages, and integrates testing and documentation workflows.

Explanation

- `Cargo.toml` defines package metadata, dependencies, and profiles.
- `cargo build`, `cargo test`, and `cargo run` are the primary workflow commands.
- Cargo workspaces share dependencies among related crates.

Example

```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
```

Notes

- Use semantic version constraints and lock files for reproducible builds.

See also

- cargo/workspaces.md
