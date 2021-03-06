use std::fmt::{Debug, Formatter, Result};
use std::io::Write;
use std::cmp::Ordering;
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashSet;
use std::fs::File;
use std::time::SystemTime;

/// Unsigned integer type to use for elements of partitions. For Schur(5), u16 is will do.
pub type Num = u16;

/// Represents a partition of `[1, ..., N]` for some `N` into `k` parts, each of which is sum-free (i.e. there are no `x`,`y`,`z` in the same part such that `x + y = z`).
/// Internally keeps track of the sums from each part (i.e. `x + y` for all `x`,`y` in the part) and the least number, if any (thus an `Option<Num>`), that all parts have elements that can sum to.
#[derive(PartialEq, Eq, Clone)]
pub struct SPartition {
	top: Num,
	partition: Vec<Vec<Num>>,
	sums: Vec<HashSet<Num>>,
	cap: Option<Num>,
}

impl SPartition {
	/// Creates a new partition of `[1, ..., 0] = []` into `k` parts. There is only one such partition, which is the one returned.
	pub fn new(k: usize) -> SPartition {
		SPartition { top: 0, partition: vec![vec![]; k], sums: vec![HashSet::new(); k], cap: None}
	}

	/// Returns the number `N` for which `self` is a partition of `[1, ..., N]`. Runs in `O(1)` time.
	pub fn top(&self) -> Num {
		self.top
	}

	/// If there is a number `p` such that in every part there are `x` and `y` such that `x + y = p`, returns `Some(p)` for the least such `p`. Otherwise, returns `None`. Runs in `O(1)` time.
	pub fn cap(&self) -> Option<Num> {
		self.cap
	}

	/// Adds the next number (`self.top()+1`) to part `i`. Due to updating internal data, runs in `O(N)` time, where `N = self.top()`.
	fn add_at(mut self, i: usize) -> SPartition {
		self.top += 1;
		self.partition.get_mut(i).unwrap().push(self.top);
		for x in self.partition.get(i).unwrap() {
			let sum = x + self.top;
			if let Some(val) = self.cap {
				if sum >= val {
					break;
				}
			}
			self.sums.get_mut(i).unwrap().insert(sum);
			if self.sums.iter().all(|ref set| set.contains(&sum)) {
				self.cap = Some(sum);
			}
		}
		self
	}

	/// 
	pub fn find_children(&self, best: Arc<RwLock<(Num,SPartition)>>, log_file: Arc<Mutex<File>>) -> Vec<SPartition> {
		let mut children: Vec<SPartition> = vec![];
		for i in 0..self.partition.len() {
			if i > 0 && self.partition.get(i-1).unwrap().is_empty() {
				break;
			}
			if !self.sums.get(i).unwrap().contains(&(self.top+1)) {
				let child = self.clone().add_at(i);
				let mut update = false;
				{
					let best_top = (*best.read().unwrap()).0;
					if child.top() > best_top {
						update = true;		
					}
					if let Some(val) = child.cap() {
						if val > best_top+1 {
							children.push(child.clone());
						}
					} else {
						children.push(child.clone());
					}
				}
				if update {
					*best.write().unwrap() = (child.top(), child.clone());
                    (*log_file.lock().unwrap()).write_fmt(format_args!("{:?} best is {} from {:?}\n", SystemTime::now(), child.top(), child));
				}
			}
		}
		children
	}
}

impl PartialOrd for SPartition {
	fn partial_cmp(&self, other: &SPartition) -> Option<Ordering> {
		Some(self.top.cmp(&other.top))
	}
}

impl Ord for SPartition {
	fn cmp(&self, other: &SPartition) -> Ordering {
		self.top.cmp(&other.top)
	}
}

impl Debug for SPartition {
	fn fmt(&self, f: &mut Formatter) -> Result {
		write!(f, "{:?}", self.partition)
	}
}
