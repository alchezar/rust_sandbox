// IKinder

use std::thread;

pub fn main() {
	println!("::thread_builder\n");

	let _builder = thread::Builder::new()
		.name("Named thread".to_owned())
		.stack_size(std::mem::size_of::<usize>() * 4)
		.spawn(my_thread)
		.unwrap()
		.join()
		.unwrap();
}

fn my_thread() {
	println!("Hello from a thread named {}", thread::current().name().unwrap());
}
