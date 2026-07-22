---
title: "Borrowing in Rust"
language: "rust"
category: "rust"
topic: "ownership"
difficulty: "intermediate"
keywords: ["borrowing","reference","borrow checker","mutable borrow"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Borrowing"
---

# Borrowing

Summary

Borrowing lets functions and expressions use a value without taking ownership. Rust supports immutable (`&T`) and mutable (`&mut T`) borrows.

Explanation

- Immutable references allow read-only access and can be shared multiple times.
- Mutable references allow exclusive access and can only exist one at a time.
- The borrow checker enforces these rules to prevent data races and undefined behavior.

Example

```rust
fn main() {
    let mut text = String::from("hello");
    let a = &text;
    let b = &text;
    println!("{} and {}", a, b);

    let c = &mut text; // legal only after immutable borrows end
    c.push_str(" world");
    println!("{}", c);
}
```

Common Pitfalls

- You cannot have an immutable borrow and a mutable borrow at the same time.
- Mutable borrows are not allowed across loops or function calls that hold references.

Notes

- Borrowing is central to safe concurrency because it prevents aliasing violations.
- Use references for temporary access and ownership for long-term data storage.

See also

- ownership/lifetimes.md
