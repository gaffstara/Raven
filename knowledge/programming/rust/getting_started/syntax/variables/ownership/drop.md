---
title: "Drop and Resource Cleanup"
language: "rust"
category: "rust"
topic: "ownership"
difficulty: "intermediate"
keywords: ["drop","destructor","ownership","resource cleanup"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Drop"
---

# Drop and Resource Cleanup

Summary

Rust automatically drops values when they go out of scope. The `Drop` trait allows custom cleanup logic, such as closing files or freeing memory.

Explanation

- When a value leaves its scope, Rust calls its `drop` implementation.
- The `Drop` trait provides a `drop(&mut self)` method for cleanup.
- Types with `Drop` cannot implement `Copy`.

Example

```rust
struct FileGuard;

impl Drop for FileGuard {
    fn drop(&mut self) {
        println!("closing file guard");
    }
}

fn main() {
    let guard = FileGuard;
    println!("guard created");
} // guard dropped here
```

Compiler Behavior

The compiler inserts calls to `drop` at scope exits. When ownership is moved, only the final owner drops the value.

Common Errors

- Calling `std::mem::drop()` too early invalidates the value afterward.
- Implementing `Drop` for types that should be `Copy` can introduce unexpected move behavior.

See also

- ownership/move.md
- ownership/copy.md
