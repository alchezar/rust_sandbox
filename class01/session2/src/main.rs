// IKinder

#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

fn main() {
	// themes::variables::main();
	println!("Hello from main thread!");

	let thread_handle = std::thread::spawn(hello_thread);
	thread_handle.join().unwrap();
}

fn hello_thread() {
	println!("Hello from thread!")
}
