// IKinder

#![allow(dead_code, unused_variables, unused_imports, unused_mut)]
#![warn(clippy::all, clippy::must_use_candidate)]

fn main() {
	println!("Hello, world!");

	let value: Option<u8> = None;
	let res: u8 = value.unwrap_or_else(|| 0);
}
