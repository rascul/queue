[![Build Status](https://travis-ci.org/rascul/queue.svg?branch=master)](https://travis-ci.org/rascul/queue)

A FIFO queue built around `Vec` with an optional capacity.

[Documentation](https://rascul.github.io/queue)

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
