// IKinder

use std::collections::HashMap;

enum MainMenu {
	Add,
	View,
	Remove,
	Update,
	Exit,
}

impl MainMenu {
	fn from_str(input: &str) -> Option<Self> {
		match input {
			"1" => Some(MainMenu::Add),
			"2" => Some(MainMenu::View),
			"3" => Some(MainMenu::Remove),
			"4" => Some(MainMenu::Update),
			"0" => Some(MainMenu::Exit),
			_ => None,
		}
	}
	fn show() {
		println!("=== Bill Manager ===");
		println!("1.Add Bill");
		println!("2.View Bill");
		println!("3.Remove Bill");
		println!("4.Update Bill");
		println!("0.Exit");
		println!("Enter selection:");
	}
}

#[derive(Debug, Clone)]
struct Bill {
	name: String,
	amount: f64,
}

struct Bills {
	inner: HashMap<String, Bill>,
}

impl Bills {
	fn new() -> Self {
		Self { inner: HashMap::new() }
	}
	fn add(&mut self, bill: Bill) {
		self.inner
			.insert(bill.name.to_string(), bill);
	}
	fn get_all(&self) -> Vec<&Bill> {
		self.inner
			.values()
			.collect()
	}
	fn remove(&mut self, name: &str) -> bool {
		self.inner
			.remove(name)
			.is_some()
	}
	fn update(&mut self, name: &str, amount: f64) -> bool {
		match self.inner.get_mut(name) {
			Some(bill) => {
				bill.amount = amount;
				true
			}
			None => false,
		}
	}
}

mod input {
	///
	/// Handles input from the user.
	///
	pub fn get_input() -> Option<String> {
		// Read the line from the terminal.
		let mut buffer = String::new();
		while std::io::stdin()
			.read_line(&mut buffer)
			.is_err()
		{
			println!("Please enter correct input!");
		}

		// Remove spaces and make input case-insensitive.
		let input = buffer
			.trim()
			.to_lowercase()
			.to_owned();

		// Return the final input.
		if input.is_empty() { None } else { Some(input) }
	}

	///
	/// Parse string to the f64 number.
	///
	pub fn get_bill_amount() -> Option<f64> {
		loop {
			let input = get_input()?;
			if input.is_empty() {
				return None;
			};

			// Parse the string into f64 number.
			let parsed_input: Result<f64, _> = input.parse();
			match parsed_input {
				Ok(amount) => return Some(amount),
				Err(_) => println!("Please enter a number!"),
			}
		}
	}
}

mod menu {
	use super::input::{get_bill_amount, get_input};
	use super::{Bill, Bills};

	pub fn add_bill(bills: &mut Bills) {
		println!("Bill name: ");
		let name = match get_input() {
			Some(input) => input,
			None => return,
		};
		println!("Amount: ");
		let amount = match get_bill_amount() {
			Some(amount) => amount,
			None => return,
		};
		let bill = Bill { name, amount };
		bills.add(bill);
		println!("Bill added!");
	}
	pub fn view_bills(bills: &Bills) {
		for bill in bills.get_all() {
			println!("{:?}", bill)
		}
	}
	pub fn remove_bill(bills: &mut Bills) {
		view_bills(bills);
		println!("Enter bill name to remove: ");
		let name = match get_input() {
			Some(name) => name,
			None => return,
		};
		if Bills::remove(bills, name.as_str()) {
			println!("Bill {name} removed!");
		} else {
			println!("Bill {name} was not found!");
		}
	}
	pub fn update_bill(bills: &mut Bills) {
		view_bills(bills);
		println!("Enter bill name to update: ");
		let name = match get_input() {
			Some(name) => name,
			None => return,
		};
		println!("Enter new bill amount: ");
		let amount = match get_bill_amount() {
			Some(amount) => amount,
			None => return,
		};
		if Bills::update(bills, name.as_str(), amount) {
			println!("Bill {name} updated!");
		} else {
			println!("Bill {name} was not found!");
		}
	}
}

pub fn run() -> Option<()> {
	let mut bills = Bills::new();

	loop {
		MainMenu::show();
		let input = input::get_input()?;
		match MainMenu::from_str(&input) {
			Some(MainMenu::Add) => menu::add_bill(&mut bills),
			Some(MainMenu::View) => menu::view_bills(&bills),
			Some(MainMenu::Remove) => menu::remove_bill(&mut bills),
			Some(MainMenu::Update) => menu::update_bill(&mut bills),
			Some(MainMenu::Exit) => return None,
			None => (),
		}
	}
}
