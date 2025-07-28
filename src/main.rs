// IKinder

#![allow(dead_code, unused)]
#![warn(clippy::all, clippy::must_use_candidate)]

fn main() {
	println!("Hello, world!+");

	let pair: (i32, bool) = (1, false);
	let f: fn(i32) -> i32 = |x| x + 1;

	let array: [i32; 3] = [1, 2, 3];
	let slice: Box<[i32]> = Box::new(array);
}
