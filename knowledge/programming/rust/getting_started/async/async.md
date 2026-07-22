---
title: "Async/Await in Rust"
language: "rust"
category: "rust"
topic: "async"
difficulty: "advanced"
keywords: ["async","await","future","runtime"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Async"
---

# Async/Await

Summary

Async/await enables asynchronous programming in Rust. `async fn` returns a `Future`, and `await` polls it to completion.

Explanation

- The Rust language provides the async syntax and the `Future` trait.
- An executor such as `tokio` or `async-std` runs asynchronous tasks.
- Async tasks are non-blocking and useful for I/O-bound workloads.

Example

```rust
use futures::executor::block_on;

async fn fetch() -> String {
    "data".to_string()
}

fn main() {
    let data = block_on(fetch());
    println!("{}", data);
}
```

Notes

- Async code still uses ownership and borrow semantics.
- Avoid blocking operations inside async functions.

See also

- tokio/tokio.md
