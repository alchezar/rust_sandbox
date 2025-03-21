// IKinder

#![allow(dead_code, unused_imports)]

use chrono::prelude::*;
use humantime::format_duration;
use std::time::Duration;

// Crates.
fn crates() {
	let d = Duration::from_secs(9876);
	println!("{}", format_duration(d));

	let local = Local::now();
	println!("{:?}", local);
	let local = local
		.format("%Y-%m-%d %H:%M:%S")
		.to_string();
	println!("{}", local);
}

// Modules.
pub mod msg {
	pub fn trim(msg: &str) -> &str {
		msg.trim()
	}
	pub fn capitalize(msg: &str) -> std::borrow::Cow<'_, str> {
		if let Some(letter) = msg.get(0..1) {
			format!("{}{}", letter.to_uppercase(), &msg[1..]).into()
		} else {
			msg.into()
		}
	}
	pub fn exciting(msg: &str) -> String {
		format!("{}!", msg)
	}
}

pub mod math {
	pub fn add(lhs: isize, rhs: isize) -> isize {
		lhs + rhs
	}
	pub fn sub(lhs: isize, rhs: isize) -> isize {
		lhs - rhs
	}
	pub fn mul(lhs: isize, rhs: isize) -> isize {
		lhs * rhs
	}
}

pub fn run() {
	crates();

	// Modules.
	{
		use crate::crates_modules::msg as mes;

		let hello = {
			let msg = "hello ";
			let msg = mes::trim(msg);
			mes::capitalize(msg)
		};
		let world = {
			let msg = "world";
			mes::exciting(msg)
		};
		println!("{}, {}", hello, world)
	};
}
