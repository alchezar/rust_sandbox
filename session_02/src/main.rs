// IKinder

#![allow(dead_code, unused_variables, unused_imports, unused_macros)]

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

mod bootcamp;

fn main() {
	bootcamp::macros::run();
}