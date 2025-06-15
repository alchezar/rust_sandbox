// IKinder

#![allow(dead_code, unused_variables, unused_imports, unused_mut)]
#![warn(clippy::all, clippy::must_use_candidate)]

pub mod apps;

fn main() {
	apps::pr_scr::start().unwrap();
}
