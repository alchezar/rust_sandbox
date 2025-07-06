// IKinder

use std::sync::Mutex;

static NUMBERS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

pub fn main() {
	crate::show_name(file!());

	let start = std::time::Instant::now();

	let mut handles = Vec::new();
	for _ in 0..10 {
		let handle = std::thread::spawn(|| {
			let mut lock = NUMBERS.lock().unwrap();
			lock.push(1);
		});
		handles.push(handle);
	}
	handles
		.into_iter()
		.for_each(|h| h.join().unwrap());

	let lock = NUMBERS.lock().unwrap();
	println!("{:?}", lock);

	let elapsed = std::time::Instant::now()
		.duration_since(start)
		.as_micros();
	println!("elapsed: {} ms", elapsed);
}
