---
title: "Slices in Rust"
language: "rust"
category: "rust"
topic: "slices"
difficulty: "intermediate"
keywords: ["slice","borrow","array","vector"]
version: "1.0"
rust_edition: "2021"
last_reviewed: "2026-07-22"
references: "The Rust Programming Language, Slices"
---

# Slices

Summary

Slices provide a borrowed view into a contiguous sequence such as arrays or vectors. They preserve ownership rules while enabling windowed access.

Explanation

- `&[T]` is a borrowed slice of elements.
- `&str` is a slice of UTF-8 text.
- Slices are fat pointers containing length information.

Example

```rust
fn first_two(items: &[i32]) {
    println!("{:?}", &items[..2]);
}

fn main() {
    let data = vec![1, 2, 3];
    first_two(&data);
}
```

Notes

- Use slices for read-only access without copying.
- Lifetime annotations often appear with slices in function signatures.

See also

- ownership/borrowing.md
