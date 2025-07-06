// IKinder

pub fn main() {
	crate::show_name(file!());

	let core_ids = core_affinity::get_core_ids().unwrap();
	let handles = core_ids
		.into_iter()
		.map(|id| {
			std::thread::spawn(move || {
				let success = core_affinity::set_for_current(id);
				if success {
					println!("Hello from a thread on core {}", id.id);
				} else {
					println!("Unable to set affinity to core {}", id.id);
				}
			})
		})
		.collect::<Vec<_>>();

	handles
		.into_iter()
		.for_each(|handle| handle.join().unwrap())
}
