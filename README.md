[![Build Status](https://travis-ci.org/rascul/queue.svg?branch=master)](https://travis-ci.org/rascul/queue)

[![Documentation](https://docs.rs/queue/badge.svg)](https://docs.rs/queue)

A FIFO queue built around `Vec` with an optional capacity.

[Documentation](https://docs.rs/queue)

```rust
use queue::Queue;

let mut q = Queue::new();

q.queue("hello").unwrap();
q.queue("out").unwrap();
q.queue("there!").unwrap();

while let Some(item) = q.dequeue() {
    println!("{}", item);
}
```
