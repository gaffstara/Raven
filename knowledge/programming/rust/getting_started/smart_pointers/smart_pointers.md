---
title: "Smart Pointers in Rust"
language: "rust"
category: "rust"
topic: "smart pointers"
difficulty: "advanced"
keywords: ["Box","Rc","Arc","RefCell","smart pointer"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Smart Pointers"
---

# Smart Pointers

Summary

Smart pointers like `Box`, `Rc`, and `Arc` provide ownership semantics for heap allocation and shared references. `RefCell` enables interior mutability at runtime.

Explanation

- `Box<T>` owns heap-allocated data with a single owner.
- `Rc<T>` provides shared ownership in single-threaded contexts.
- `Arc<T>` provides shared ownership across threads.
- `RefCell<T>` enforces borrow rules at runtime.

Example

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(String::from("shared"));
    let a = Rc::clone(&data);
    let b = Rc::clone(&data);
    println!("{} {}", a, b);
}
```

Notes

- Prefer `Box` for simple heap allocation.
- Use `Rc` and `Arc` only when shared ownership is required.

See also

- ownership/borrowing.md
