use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::sync::{Arc, Mutex, RwLock};
use std::fs::File;

extern crate num;

use num::bigint::BigUint;

mod thread_safe_binary_heap;
mod spartition;

use thread_safe_binary_heap::ThreadSafeBinaryHeap;
use spartition::SPartition;

const K: usize = 5;
const NUM_THREADS: usize = 128;

fn main() {
    let mut heap: ThreadSafeBinaryHeap<SPartition> = ThreadSafeBinaryHeap::new();
    let active_count = Arc::new(Mutex::new(BigUint::new(vec![0u32])));
    let best = Arc::new(RwLock::new((0, SPartition::new(K))));
    let log_file = Arc::new(Mutex::new(File::create("schur5.log").unwrap()));
    for partition in SPartition::new(K).find_children(best.clone(), log_file.clone()) {
    	println!("main\t:\t{} - {:?}", partition.top(), partition);
    	heap.push(partition);
    	let mut ac = active_count.lock().unwrap();
    	*ac = (*ac).clone() + BigUint::new(vec![1u32]);
    }
    for i in 0..NUM_THREADS {
    	let mut my_heap = heap.clone();
    	let my_active_count = active_count.clone();
    	let my_best = best.clone();
        let my_log_file = log_file.clone();
    	thread::spawn(move || {
    		let mut partition = SPartition::new(K);
			let one = BigUint::new(vec![1u32]);
			loop {
				if *my_active_count.lock().unwrap() == BigUint::new(vec![0u32]) {
					let (num, ref part) = *my_best.read().unwrap();
					println!("Finished!\nSchur({}) = {}\nWinning partition = {:?}", K, num, part);
					break
				}
				if partition.top() == 0 {
					if let Some(party) = my_heap.pop() {
						if let Some(val) = party.cap() {
							if val > (*my_best.read().unwrap()).0 {
								partition = party;
							}
						} else {
							partition = party;
						}
					}
				}
				if partition.top() != 0 {
					println!("{}\t:\tbest {}\t{}/{} - {:?}", i, (*my_best.read().unwrap()).0, partition.top(), if let Some(val) = partition.cap() { val } else { 0 }, partition);
					let children = partition.find_children(my_best.clone(), my_log_file.clone());
					if children.is_empty() {
						partition = SPartition::new(K);
						let mut ac = my_active_count.lock().unwrap();
						*ac = (*ac).clone() - one.clone();
					} else {
						let mut first = true;
						for other in children {
							if first {
								partition = other;
								first = false;
							} else {
								let mut ac = my_active_count.lock().unwrap();
								*ac = (*ac).clone() + one.clone();
								my_heap.push(other);
							}
						}
					}
				}
			}
    	});
    }
    sleep(Duration::new(u64::max_value(), 0));
}
