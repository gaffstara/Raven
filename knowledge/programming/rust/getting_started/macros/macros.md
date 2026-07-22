---
title: "Macros in Rust"
language: "rust"
category: "rust"
topic: "macros"
difficulty: "advanced"
keywords: ["macro_rules","procedural macro","hygiene","derive"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Macros"
---

# Macros

Summary

Macros provide metaprogramming capabilities. `macro_rules!` defines declarative macros, while procedural macros generate code from Rust syntax.

Explanation

- Declarative macros match token patterns and expand expressions.
- Procedural macros operate on token streams and can implement custom derives.
- Macro hygiene prevents accidental variable capture.

Example

```rust
macro_rules! say_hello {
    () => {
        println!("Hello from macro!");
    };
}

fn main() {
    say_hello!();
}
```

Notes

- Use macros sparingly; prefer functions and traits for most abstractions.
- Document macro behavior clearly and test expansions.

See also

- proc_macro/proc_macro.md
