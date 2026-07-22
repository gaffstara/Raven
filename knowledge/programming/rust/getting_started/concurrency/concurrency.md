---
title: "Concurrency in Rust"
language: "rust"
category: "rust"
topic: "concurrency"
difficulty: "advanced"
keywords: ["concurrency","thread","Send","Sync","channel"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Concurrency"
---

# Concurrency

Summary

Rust makes concurrency safe with ownership and thread-safety traits. The type system prevents data races by enforcing `Send` and `Sync` constraints.

Explanation

- `std::thread::spawn` launches new threads.
- `Send` means a type can be transferred between threads.
- `Sync` means a shared reference is safe across threads.
- Channels allow message passing without shared mutable state.

Example

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        println!("hello from thread");
    });
    handle.join().unwrap();
}
```

Best Practice

Prefer message passing over shared mutable state. Use thread-safe containers like `Arc<Mutex<T>>` when shared ownership is required.

See also

- async/async.md
