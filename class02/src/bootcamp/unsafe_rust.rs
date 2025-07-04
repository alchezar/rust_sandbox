// IKinder

#![allow(dead_code, unused_variables, static_mut_refs)]

pub fn run() {
	dereference_raw_pointer();
	unsafe {
		unsafe_function();
	}
	increment_counter();
}

// Dereference raw pointer.
fn dereference_raw_pointer() {
	let mut s = "Let's  get rusty".to_owned();

	let raw1 = &s as *const String;
	let raw2 = &mut s as *mut String;

	let address = 0x012345usize;
	let raw3 = address as *const String;

	unsafe {
		(*raw2).push_str("!!!");
		println!("raw2 is {}", *raw2)
	}
}

// Calling an unsafe function.
unsafe fn unsafe_function() {
	println!("calling unsafe function");
}

// Implementing an unsafe trait.
unsafe trait UnsafeTrait {
	fn some_function(&self);
}

unsafe impl UnsafeTrait for String {
	fn some_function(&self) {
		//
	}
}

// Mutable static variable
static mut COUNTER: u32 = 0;
fn increment_counter() {
	unsafe {
		for _ in 0..10 {
			COUNTER += 1;
		}
		println!("counter is {}", COUNTER);
	}
}

// Inline assembly
fn add(x: u64, y: u64) -> u64 {
	let result: u64;

	unsafe {
		// x86/x86-64 assembly
		std::arch::asm!("add {0}, {1}", inout(reg) x => result, in(reg) y);
	}

	result
}
