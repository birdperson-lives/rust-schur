use std::collections::BinaryHeap;
use std::sync::{Arc, Condvar, Mutex};

const QUEUE_CAP: usize = 134217728;

pub struct ThreadSafeBinaryHeap<T: Ord> {
	heap: Arc<Mutex<BinaryHeap<T>>>,
	size: Arc<Mutex<usize>>,
	empty_var: Arc<Condvar>,
	full_var: Arc<Condvar>,
}

impl<T: Ord> ThreadSafeBinaryHeap<T> {
	pub fn new() -> ThreadSafeBinaryHeap<T> {
		ThreadSafeBinaryHeap { heap: Arc::new(Mutex::new(BinaryHeap::new())), size: Arc::new(Mutex::new(0usize)), empty_var: Arc::new(Condvar::new()), full_var: Arc::new(Condvar::new()) }
	}

	#[allow(dead_code)]
	pub fn size(&self) -> usize {
		*self.size.lock().unwrap()
	}

	pub fn pop(&mut self) -> Option<T> {
		let mut empty_wait = self.size.lock().unwrap();
		while *empty_wait == 0 {
			empty_wait = self.empty_var.wait(empty_wait).unwrap();
		}
		let out = (*self.heap.lock().unwrap()).pop();
		*empty_wait -= 1;
		self.full_var.notify_one();
		out
	}

	pub fn push(&mut self, item: T) {
		let mut full_wait = self.size.lock().unwrap();
		while *full_wait == QUEUE_CAP {
			full_wait = self.full_var.wait(full_wait).unwrap();
		}
		(*self.heap.lock().unwrap()).push(item);
		*full_wait += 1;
		self.empty_var.notify_one();
	}
}

impl<T: Ord> Clone for ThreadSafeBinaryHeap<T> {
	fn clone(&self) -> ThreadSafeBinaryHeap<T> {
		ThreadSafeBinaryHeap { heap: self.heap.clone(), size: self.size.clone(), empty_var: self.empty_var.clone(), full_var: self.full_var.clone() }
	}

	fn clone_from(&mut self, source: &ThreadSafeBinaryHeap<T>) {
		self.heap = source.heap.clone();
		self.size = source.size.clone();
		self.empty_var = source.empty_var.clone();
		self.full_var = source.full_var.clone();
	}
}