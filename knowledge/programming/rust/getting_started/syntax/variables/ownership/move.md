---
title: "Move Semantics in Rust"
language: "rust"
category: "rust"
topic: "ownership"
difficulty: "intermediate"
keywords: ["move","ownership","transfer","borrow checker"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Move Semantics"
---

# Move Semantics

Summary

A move transfers ownership of a value from one variable to another without copying data. After a move, the original variable cannot be used unless the value is `Copy`.

Explanation

- Non-`Copy` types such as `String` or `Vec<T>` are moved by default when assigned or passed to a function.
- Move semantics avoid hidden allocations and allow zero-cost transfer of ownership.
- The compiler marks the source as invalid after the move, producing a compile-time error if it is used again.

Example

```rust
fn main() {
    let a = String::from("Rust");
    let b = a; // move occurs here
    // println!("{}", a); // compile error: use of moved value
    println!("{}", b);
}
```

Compiler Behavior

The borrow checker tracks the ownership state at each point in the code. When ownership moves, the source variable becomes invalid and cannot be used again.

Common Patterns

- Returning a value from a function moves ownership to the caller.
- Passing a value by reference avoids moving it.
- Use `clone()` only when you need a deep copy.

See also

- ownership/copy.md
- ownership/borrowing.md
*