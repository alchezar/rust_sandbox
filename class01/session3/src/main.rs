// IKinder

#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

mod theme;

pub mod prelude {
	pub use crate::theme::*;
}

fn main() {
	let _ = prelude::channels::main();
}

fn show_name(file: &str) -> Option<()> {
	println!(
		"class1::session3::{}\n",
		file.split("\\")
			.last()?
			.split(".")
			.next()?
	);
	Some(())
}
