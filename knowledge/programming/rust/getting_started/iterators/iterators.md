---
title: "Iterators in Rust"
language: "rust"
category: "rust"
topic: "iterators"
difficulty: "intermediate"
keywords: ["iterators","Iterator","collect","adaptors"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Iterators"
---

# Iterators

Summary

Iterators provide a lazy sequence abstraction over collections. They support adaptors like `map`, `filter`, and `collect`.

Explanation

- The `Iterator` trait requires the `next()` method.
- Iterator adaptors return new iterator values without consuming the underlying collection.
- Consumers like `collect` and `fold` drive iteration.

Example

```rust
fn main() {
    let values = vec![1, 2, 3];
    let doubled: Vec<i32> = values.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);
}
```

Best Practice

Use iterator chains to express transformations clearly. Avoid collecting too early when processing streams.

See also

- collections/collections.md
