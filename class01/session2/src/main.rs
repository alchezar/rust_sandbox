// IKinder

#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

mod theme;

pub mod prelude {
	pub use crate::theme::*;
}

fn main() {
	prelude::rayon_scopes::main();
}

fn show_name(file: &str) -> Option<()> {
	println!(
		"class1::session2::{}\n",
		file.split("\\")
			.last()?
			.split(".")
			.next()?
	);
	Some(())
}
