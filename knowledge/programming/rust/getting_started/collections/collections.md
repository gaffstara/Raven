---
title: "Collections in Rust"
language: "rust"
category: "rust"
topic: "collections"
difficulty: "intermediate"
keywords: ["Vec","HashMap","BTreeMap","collection"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Collections"
---

# Collections

Summary

Rust provides a standard library of collections, including `Vec`, `HashMap`, and `String`. Each collection has distinct ownership and performance characteristics.

Explanation

- `Vec<T>` is the growable array type for owned values.
- `HashMap<K, V>` and `BTreeMap<K, V>` store key-value pairs with different ordering guarantees.
- `String` and `&str` handle UTF-8 text data.

Example

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("apple", 3);
    map.insert("banana", 2);
    println!("{:?}", map);
}
```

Notes

- Choose the smallest collection type that meets your needs.
- Reserve capacity before pushing if you know the expected size.

See also

- strings/strings.md
- slices/slices.md
