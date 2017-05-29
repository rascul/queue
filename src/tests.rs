use super::*;

#[test]
fn queue() {
	let mut q = Queue::new();
	assert_eq!(q.queue("hello"), Ok(1));
}

#[test]
fn dequeue() {
	let mut q = Queue::new();
	q.queue("hello").unwrap();
	assert_eq!(q.dequeue(), Some("hello"));
}

#[test]
fn no_capacity() {
	let q: Queue<u8> = Queue::new();
	assert_eq!(q.capacity(), None);
}

#[test]
fn some_capacity() {
	let q: Queue<u8> = Queue::with_capacity(12);
	assert_eq!(q.capacity(), Some(12));
}

#[test]
fn queue_full_capacity() {
	let mut q: Queue<u8> = Queue::with_capacity(0);
	assert_eq!(q.queue(3), Err(()));
}

#[test]
fn peek_at_empty() {
	let q: Queue<u8> = Queue::with_capacity(1);
	assert_eq!(q.peek(), None);
}

#[test]
fn peek_at_something() {
	let mut q = Queue::new();
	q.queue(1).unwrap();
	assert_eq!(q.peek(), Some(1));
}

#[test]
fn force_queue() {
	let mut q: Queue<u8> = Queue::with_capacity(1);
	q.queue(1).unwrap();
	let _ = q.force_queue(2);
	assert_eq!(q.peek(), Some(2));
}

#[test]
fn no_len() {
	let q: Queue<u8> = Queue::new();
	assert_eq!(q.len(), 0);
}

#[test]
fn some_len() {
	let mut q = Queue::new();
	q.queue(1).unwrap();
	q.queue(2).unwrap();
	assert_eq!(q.len(), 2);
}
