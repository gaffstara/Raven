---
title: "Procedural Macros in Rust"
language: "rust"
category: "rust"
topic: "proc_macro"
difficulty: "advanced"
keywords: ["proc macro","derive","token stream","hygiene"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Procedural Macros"
---

# Procedural Macros

Summary

Procedural macros generate code at compile time by operating on token streams. They enable powerful custom derive and attribute-based DSLs.

Explanation

- Procedural macros are defined in a dedicated crate with `proc-macro = true`.
- There are derive macros, attribute macros, and function-like macros.
- The `syn` and `quote` crates are commonly used for parsing and emitting Rust code.

Example

```rust
#[proc_macro_derive(MyTrait)]
pub fn my_trait_derive(input: TokenStream) -> TokenStream {
    // parse and generate code here
}
```

Notes

- Keep procedural macro crates minimal and safe.
- Document generated behavior clearly.

See also

- macros/macros.md
