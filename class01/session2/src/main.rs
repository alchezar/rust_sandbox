// IKinder

#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

mod theme;

pub mod prelude {
	pub use crate::theme::*;
}

fn main() {
	print!("class1::session2");

	prelude::footgun::main();
}
