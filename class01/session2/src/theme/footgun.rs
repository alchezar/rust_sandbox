// IKinder

use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

static mut COUNTER_1: u32 = 0;
static COUNTER_2: AtomicU32 = AtomicU32::new(0);

pub fn main() {
	crate::show_name(file!());

	//footgun();
	not_footgun();
}

fn footgun() {
	let mut handles = Vec::new();
	for _ in 0..1000 {
		let handle = std::thread::spawn(|| {
			for _ in 0..1100 {
				unsafe {
					COUNTER_1 += 1;
				}
			}
		});
		handles.push(handle);
	}

	handles
		.into_iter()
		.for_each(|handle| handle.join().unwrap());

	println!("Counter: {}", unsafe { COUNTER_1 });
}

fn not_footgun() {
	let mut handles = Vec::new();
	for _ in 0..1000 {
		let handle = std::thread::spawn(|| {
			for _ in 0..1100 {
				COUNTER_2.fetch_add(1, Ordering::AcqRel);
			}
		});
		handles.push(handle);
	}

	handles
		.into_iter()
		.for_each(|handle| handle.join().unwrap());

	println!("Counter: {}", COUNTER_2.fetch_or(0, Ordering::Acquire));
}
