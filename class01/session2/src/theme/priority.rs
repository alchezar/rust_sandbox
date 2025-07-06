// IKinder

use std::sync::atomic::{AtomicI32, Ordering};
use thread_priority::*;

static LOW_COUNT: AtomicI32 = AtomicI32::new(0);
static REG_COUNT: AtomicI32 = AtomicI32::new(0);

pub fn main() {
	crate::show_name(file!());

	std::thread::spawn(low_priority);
	std::thread::spawn(regular_priority);
	std::thread::sleep(std::time::Duration::from_secs(5));
	println!("Low: {}", LOW_COUNT.load(Ordering::Acquire));
	println!("Mid: {}", REG_COUNT.load(Ordering::Acquire));
}

fn low_priority() {
	set_current_thread_priority(ThreadPriority::Min).unwrap();
	loop {
		LOW_COUNT.fetch_add(1, Ordering::AcqRel);
		std::thread::yield_now();
	}
}

fn regular_priority() {
	// set_current_thread_priority(ThreadPriority::Max).unwrap();
	loop {
		REG_COUNT.fetch_add(1, Ordering::AcqRel);
		std::thread::yield_now();
	}
}

fn high_priority() {}
