// IKinder

const MAX: u32 = 10;

pub fn main() {
	crate::show_name(file!());

	let mut threads = Vec::new();
	for i in 0..MAX {
		let thread = std::thread::spawn(move || {
			parkable_thread(i);
		});
		threads.push(thread);
	}

	'l: loop {
		println!("Thread to unpark: ");
		let input = read_line();
		if input == "q" {
			break 'l;
		}

		if let Ok(number) = input.parse::<usize>() {
			if number < MAX as usize {
				threads[number].thread().unpark();
			}
		}
	}
}

fn parkable_thread(n: u32) {
	std::thread::park();
	// The message below will be shown only after unparking.
	println!("Thread {n} is unparked, briefly");
}

fn read_line() -> String {
	let mut input = String::new();
	std::io::stdin()
		.read_line(&mut input)
		.unwrap();
	input.trim().to_string()
}
