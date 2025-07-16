// IKinder

pub fn main() {
	crate::show_name(file!());

	let my_vec = vec![1, 2, 3, 4, 5];
	let first = my_vec[1];
	let second = if let Some(value) = my_vec.get(2) { value } else { &0 };
	let third = unsafe { my_vec.get_unchecked(3) };

	allocate_memory_with_libc();
	allocate_memory_with_rust();
}

/// # Safety
///
/// This function is unsafe, because ...
unsafe fn my_fn() {}

fn allocate_memory_with_libc() {
	unsafe {
		// Allocate memory with libc (one 32-bit integer)
		let my_num = libc::malloc(size_of::<i32>() as libc::size_t) as *mut i32;
		if my_num.is_null() {
			panic!("malloc failed");
		}

		// Set the allocated variable = dereference the pointer and set to 42.
		*my_num = 42;
		assert_eq!(*my_num, 42);

		// Free the memory with libc - this is NOT automatic!
		libc::free(my_num as *mut libc::c_void)
	}
}

fn allocate_memory_with_rust() {
	unsafe {
		// Allocate memory with Rust. It's safer to force alignment.
		let layout = std::alloc::Layout::new::<u16>();
		let ptr = std::alloc::alloc(layout);

		// Set the allocated variable = dereference the pointer and set to 42.
		*ptr = 42;
		assert_eq!(42, *ptr);

		// Free the memory - this is not automatic.
		std::alloc::dealloc(ptr, layout);
	}
}
