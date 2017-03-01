use cell::UnsafeCell;
use std::sync::{Condvar, Mutex};

pub struct<T: ?Sized> WPrefLock {
	reads: Mutex<usize>,
	write: Mutex<bool>,
	block: Condvar,
	data: UnsafeCell<T>,
}

impl<T> WPrefLock<T> {

}

unsafe impl<T: ?Sized + Send + Sync> Send for WPrefLock<T> {}

unsafe impl<T: ?Sized + Send + Sync> Sync for WPrefLock<T> {}