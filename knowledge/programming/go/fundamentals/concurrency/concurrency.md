---
title: "Concurrency in Go"
language: "go"
category: "fundamentals"
topic: "concurrency"
tags: ["go","concurrency","goroutines"]
version: "1.0"
difficulty: "intermediate"
source: "local"
last_updated: "2026-07-22T00:00:00Z"
---

# Concurrency in Go

Summary

Go uses goroutines and channels to implement lightweight concurrency.

Explanation

- Launch goroutines with `go` keyword.
- Communicate using channels to avoid explicit locks when possible.

Example

```go
package main

import "fmt"

func worker(id int, ch chan int) {
    ch <- id
}

func main() {
    ch := make(chan int)
    go worker(1, ch)
    fmt.Println(<-ch)
}
```

Notes

- Prefer channel-based communication for clarity.
