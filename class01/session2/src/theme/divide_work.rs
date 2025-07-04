// IKinder

pub fn main() {
	println!("::divide_work\n");

	const N_THREADS: usize = 8;
	let to_add = (0..5000).collect::<Vec<u32>>();
	let mut thread_handles = Vec::new();
	let chunks = to_add.chunks(N_THREADS);
	for chunk in chunks {
		let my_chunk = chunk.to_owned();
		let handle = std::thread::spawn(move || my_chunk.iter().sum::<u32>());
		thread_handles.push(handle);
	}

	let sum = thread_handles
		.into_iter()
		.map(|h| h.join().unwrap())
		.sum::<u32>();

	println!("Total sum: {}", sum)
}
