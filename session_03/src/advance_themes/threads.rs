// IKinder

// -----------------------------------------------------------------------------
// Advanced_closures: Intro
// -----------------------------------------------------------------------------

use std::ops::Add;

type AddFn<T> = dyn Fn(T, T) -> T;
fn math(a: i32, b: i32, op: Box<AddFn<i32>>) -> i32 {
	op(a, b)
}

pub fn advanced_closures() {
	let add = |a, b| a + b;
	let add = Box::new(add);
	let result = math(5, 4, add);
	println!("{}", result);
}

// -----------------------------------------------------------------------------
// Threads: Intro
// -----------------------------------------------------------------------------

pub fn intro() {
	use std::{thread, time::Duration};

	println!("Start");
	let handle = thread::spawn(move || -> i32 {
		thread::sleep(Duration::from_secs(2));
		666
	});
	if let Ok(result) = handle.join() {
		println!("{}", result);
	};
}

// -----------------------------------------------------------------------------
// Threads: Demo
// -----------------------------------------------------------------------------

pub fn demo1() {
	use std::thread;

	let iterations = 10;
	let a = thread::spawn(move || {
		for i in 1..=iterations {
			println!("A: {i}")
		}
	});
	let b = thread::spawn(move || {
		for i in 1..=iterations {
			println!("\tB: {i}")
		}
	});

	a.join().unwrap();
	b.join().unwrap();
}

pub fn demo2() {
	use std::thread;

	let data = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
	let caps = thread::spawn(move || {
		let data: Vec<char> = data
			.iter()
			.map(|x| x.to_ascii_uppercase())
			.collect();
		data
	});
	println!("Waiting for value...");
	if let Ok(result) = caps.join() {
		println!("value: {:?}", result)
	}
}

// -----------------------------------------------------------------------------
// Threads: Activity
// -----------------------------------------------------------------------------

// Topic: Multithreading
//
// Requirements:
// * 1. Run the provided functions in threads
// * 2. Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

const DELAY: u64 = 1;

fn msg_hello() -> &'static str {
	use std::{thread, time::Duration};

	thread::sleep(Duration::from_secs(DELAY));
	"Hello, "
}

fn msg_thread() -> &'static str {
	use std::{thread, time::Duration};

	thread::sleep(Duration::from_secs(DELAY));
	"thread"
}

fn msg_excited() -> &'static str {
	use std::{thread, time::Duration};

	thread::sleep(Duration::from_secs(DELAY));
	"!"
}

pub fn activity() {
	use chrono::Utc;
	use std::{thread, time::Duration};

	let timer = Utc::now();
	let mut result = vec![
		thread::spawn(move || msg_hello()),
		thread::spawn(move || msg_thread()),
		thread::spawn(move || msg_excited()),
	]
	.into_iter()
	.filter_map(|x| x.join().ok())
	.collect();

	if let Some(t) = (Utc::now() - timer).num_microseconds() {
		let t = t as u128 - Duration::from_secs(DELAY).as_micros();
		result = format!("{result} : {t} microseconds delay")
	}

	println!("{}", result);
}
