// IKinder

#![allow(dead_code, unused_variables, unused_imports, unused_mut)]
#![warn(clippy::all, clippy::must_use_candidate)]

mod apps;

mod prelude {
	pub use crate::apps::{pr_scr, temp_conv, xor_cipher};
	pub use std::collections::HashMap;
}

use prelude::*;

fn main() {
	println!("Hello, world!");
	xor_cipher::main();
}

struct Deck {
	cards: [String; 52],
}
