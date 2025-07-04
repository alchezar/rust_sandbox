// IKinder

// -----------------------------------------------------------------------------
// Arrays & Slices: Intro
// -----------------------------------------------------------------------------

fn func1(arr: [u8; 3]) {}
fn func2(arr: &[u8]) {}
fn func3(arr: &mut [u8]) {}

pub fn intro() {
	let numbers = [1, 2, 3];
	let mut numbers: [u8; 3] = [1, 2, 3];

	func3(&mut numbers);
	func2(&numbers);
	func1(numbers);

	let numbers = vec![1, 2, 3];
	func2(&numbers);
	let numbers = numbers.as_slice();

	let chars = vec!['A', 'B', 'C', 'D'];
	let bcd = &chars[1..=3];
	let bc = &bcd[..2];
}

// -----------------------------------------------------------------------------
// Arrays & Slices: Patterns
// -----------------------------------------------------------------------------

pub fn patterns() {
	let chars = vec!['A', 'B', 'C', 'D'];
	match chars.as_slice() {
		[first, .., last] => (),
		[single] => (),
		[] => (),
	}
	match chars.as_slice() {
		[one, two, ..] => (),
		[.., last] => (),
		[] => (),
	}
	match chars.as_slice() {
		[first, ..] => (),
		// never reached
		// [.., last] => (),
		[] => (),
	}

	let nums = vec![7, 8, 9];
	match nums.as_slice() {
		[first @ 1..=3, rest @ ..] => (),
		[single] if single == &5 || single == &6 => (),
		[single @ 5..=6] => (),
		[a, b] => (),
		[..] => (),
	}
}

// -----------------------------------------------------------------------------
// Arrays & Slices: activity
// -----------------------------------------------------------------------------

fn data() -> &'static [u64] {
	&[5, 5, 4, 4, 3, 3, 1]
}

fn process_chunk(data: &[u64]) {
	match data {
		[lhs, rhs] => println!("{}+{}={}", lhs, rhs, lhs + rhs),
		[single] => println!("Unpaired value: {}", single),
		[] => println!("Data stream complete",),
		[..] => unreachable!("Chunk size should be at most 2"),
	}
}

pub fn activity() {
	let stream = data().chunks(2);
	for chunk in stream {
		process_chunk(chunk)
	}
}

// -----------------------------------------------------------------------------
// Type Aliases: intro
// -----------------------------------------------------------------------------

struct Contact {
	name: String,
	phone: String,
}

use std::collections::HashMap;
type ContactName = String;
type ContactIndex = HashMap<ContactName, Contact>;

fn add_contact(index: &mut ContactIndex, new_contact: Contact) {
	index.insert(new_contact.name.to_owned(), new_contact);
}
