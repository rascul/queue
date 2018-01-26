use super::*;

#[test]
fn from_vec() {
	let q = Queue::from(vec![1, 2, 3]);
	assert_eq!(&vec![1, 2, 3], q.vec());
}

#[test]
fn from_empty_vec() {
	let v: Vec<i64> = Vec::new();
	let q = Queue::from(v.clone());
	assert_eq!(&v, q.vec());
}

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
fn dequeue_empty() {
	let mut q: Queue<i64> = Queue::new();
	assert_eq!(q.dequeue(), None);
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
fn vec_empty() {
	let q: Queue<i64> = Queue::new();
	let v: Vec<i64> = Vec::new();
	assert_eq!(&v, q.vec());
}

#[test]
fn vec_some() {
	let mut q = Queue::new();
	q.queue(1).unwrap();
	q.queue(2).unwrap();
	q.queue(3).unwrap();
	let v = vec![1, 2, 3];
	assert_eq!(&v, q.vec());
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
fn force_queue_with_capacity() {
	let mut q: Queue<u8> = Queue::with_capacity(1);
	q.queue(1).unwrap();
	let _ = q.force_queue(2);
	assert_eq!(q.peek(), Some(2));
}

#[test]
fn force_queue_with_capacity_check_size() {
	let mut q = Queue::with_capacity(1);
	q.queue(1).unwrap();
	assert_eq!(1, q.force_queue(2));
}

#[test]
fn force_queue_no_capacity() {
	let mut q = Queue::new();
	q.queue(1).unwrap();
	assert_eq!(2, q.force_queue(2));
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

#[test]
fn is_empty_empty() {
	let q: Queue<i64> = Queue::new();
	assert_eq!(q.is_empty(), true);
}

#[test]
fn is_empty_some() {
	let mut q = Queue::new();
	q.queue(1).unwrap();
	assert_eq!(q.is_empty(), false);
}
