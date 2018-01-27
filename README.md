[![Crate](https://img.shields.io/crates/v/queue.svg)](https://crates.io/crates/queue)
[![Documentation](https://docs.rs/queue/badge.svg)](https://docs.rs/queue)
[![Pipeline Status](https://gitlab.com/rascul/queue/badges/master/pipeline.svg)](https://gitlab.com/rascul/queue/commits/master)
[![Coverage](https://gitlab.com/rascul/queue/badges/master/coverage.svg?job=cov)](https://rascul.gitlab.io/queue/cov/index.html)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://gitlab.com/rascul/queue/blob/master/LICENSE)

A FIFO queue built around [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
with an optional capacity.

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
