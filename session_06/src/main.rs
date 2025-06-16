// IKinder

#![allow(dead_code, unused_variables, unused_imports, unused_mut)]
#![warn(clippy::all, clippy::must_use_candidate)]

mod apps;

mod prelude {
	pub use crate::apps::{pr_scr, temp_conv};
}

use prelude::*;

fn main() {
	if let Ok(current_dir) = std::env::current_dir() {
		println!("{}", current_dir.display());
	}

	let a = vec![1, 2];
	let test = a.get(3).unwrap_or_else(|| {
		println!("Out of bounds");
		&0
	});
}
