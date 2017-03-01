use std::fmt::{Debug, Formatter, Result};
use std::cmp::Ordering;
use std::sync::{Arc, RwLock};
use std::collections::HashSet;

pub type Num = u16;

#[derive(PartialEq, Eq, Clone)]
pub struct SPartition {
	top: Num,
	partition: Vec<Vec<Num>>,
	sums: Vec<HashSet<Num>>,
	cap: Option<Num>,
}

impl SPartition {
	pub fn new(k: usize) -> SPartition {
		SPartition { top: 0, partition: vec![vec![]; k], sums: vec![HashSet::new(); k], cap: None}
	}

	pub fn top(&self) -> Num {
		self.top
	}

	pub fn cap(&self) -> Option<Num> {
		self.cap
	}

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

	pub fn find_children(&self, best: Arc<RwLock<(Num,SPartition)>>) -> Vec<SPartition> {
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
						if val > best_top {
							children.push(child.clone());
						}
					} else {
						children.push(child.clone());
					}
				}
				if update {
					*best.write().unwrap() = (child.top(), child);
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