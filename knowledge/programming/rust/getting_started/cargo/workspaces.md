---
title: "Cargo Workspaces"
language: "rust"
category: "rust"
topic: "cargo"
difficulty: "intermediate"
keywords: ["workspace","members","dependency"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Cargo Book, Workspaces"
---

# Cargo Workspaces

Summary

Workspaces let multiple crates share a single `Cargo.lock` and build configuration. They are ideal for monorepos and related projects.

Explanation

- A workspace root contains a `Cargo.toml` with a `[workspace]` section.
- Member crates are listed in `members = ["crate-a", "crate-b"]`.
- Workspaces reduce duplicate dependency downloads and simplify testing.

Example

```toml
[workspace]
members = ["core", "cli", "web"]
```

Best Practice

Keep workspace members focused and avoid overly large dependency sets in shared crates.

See also

- cargo/crates.md
