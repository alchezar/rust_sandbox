// IKinder

#![allow(dead_code, unused_variables, unused_imports, unused_mut)]
#![warn(clippy::all, clippy::must_use_candidate)]

pub mod project;

pub mod prelude {
	pub use crate::project::{bank, cards, logs, media, iter, lifetimes};
}

fn main() {
	println!("Session 07");
	project::lifetimes::run();
}
