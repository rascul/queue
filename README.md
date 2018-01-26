[![Crate](https://img.shields.io/crates/v/queue.svg)](https://crates.io/crates/queue)
[![Documentation](https://docs.rs/queue/badge.svg)](https://docs.rs/queue)
[![Build Status](https://travis-ci.org/rascul/queue.svg?branch=master)](https://travis-ci.org/rascul/queue)

A FIFO queue built around `Vec` with an optional capacity.

This project is hosted at [Gitlab](https://gitlab.com/rascul/queue) and the
repo is mirrored to [Github](https://github.com/rascul/queue). Pull requests,
issues, etc. will be at [Gitlab](https://gitlab.com/rascul/queue).

Documentation is located at <https://rascul.gitlab.io/queue> for master
and at at [Docs.rs](https://docs.rs/queue) for published releases.

A crate is made available at [Crates.io](https://crates.io/crates/queue).

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
