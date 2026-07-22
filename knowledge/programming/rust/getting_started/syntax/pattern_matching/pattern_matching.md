---
title: "Pattern Matching in Rust"
language: "rust"
category: "rust"
topic: "pattern matching"
difficulty: "intermediate"
keywords: ["match","pattern","enum","destructuring"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Pattern Matching"
---

# Pattern Matching

Summary

Pattern matching is a core Rust control flow mechanism. `match` expressions and `if let` bindings allow concise destructuring of enums, tuples, and structs.

Explanation

- `match` checks patterns exhaustively, ensuring every branch is handled.
- Patterns can destructure values, bind variables, and include guards.
- `if let` is a convenient shorthand for matching a single successful pattern.

Example

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Text(String),
}

fn main() {
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to {} {}", x, y),
        Message::Text(text) => println!("Text: {}", text),
    }
}
```

Notes

- Wildcards (`_`) match any value and are useful for ignored fields.
- Refutable patterns are allowed in `if let` and `while let`.
- Avoid overly complex patterns that reduce readability.

See also

- syntax/enums/enums.md
