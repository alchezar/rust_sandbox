// IKinder

use std::thread;

pub fn main() {
	println!("::scoped_threads\n");
	const NUMBER_OF_THREADS: usize = 8;

	let to_add = (0..5000).collect::<Vec<u32>>();
	let chunks = to_add
		.chunks(NUMBER_OF_THREADS)
		.collect::<Vec<_>>();

	let sum = thread::scope(|scope| {
		let mut thread_handles = Vec::new();

		chunks.iter().for_each(|chunk| {
			let thread_handle = scope.spawn(|| chunk.iter().sum::<u32>());
			thread_handles.push(thread_handle);
		});

		thread_handles
			.into_iter()
			.map(|handle| handle.join().unwrap())
			.sum::<u32>()
	});

	println!("Sum: {:?}\nItems per chunk: {}.", sum, chunks.len());
}
