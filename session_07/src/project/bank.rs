// IKinder

use std::fmt::Debug;

pub fn run() {
	let mut bank = Bank::new();
	let account = Account::new(1, "John".to_owned());
	bank.add_account(account);
	bank.new_account("Ivan".to_owned(), 666);
	print_debug(&bank);

	let balance = bank
		.find_account("John".to_owned())
		.unwrap()
		.deposit(100)
		.unwrap();
	print_debug(&balance);

	let summary = bank
		.find_account("Ivan".to_owned())
		.unwrap()
		.summary();
	println!("{}", summary);

	let total_balance = bank.total_balance();
	let bank_summary = bank.summary();
	println!("Total balance: {:?} of accounts: {:?}", total_balance, bank_summary);
}

fn print_debug<T: Debug>(target: &T) {
	println!("{:#?}", target);
}

#[derive(Debug)]
struct Account {
	id: u32,
	balance: i32,
	holder: String,
}

impl Account {
	fn new(id: u32, holder: String) -> Self {
		Self { id, balance: 0, holder }
	}
	fn deposit(&mut self, amount: i32) -> Option<i32> {
		if amount < 0 {
			return None;
		}
		self.balance += amount;
		Some(self.balance)
	}
	fn withdraw(&mut self, amount: i32) -> Option<i32> {
		if self.balance < amount {
			return None;
		}
		self.balance -= amount;
		Some(self.balance)
	}
	fn summary(&self) -> String {
		format!("{} has a balance {}", self.holder, self.balance)
	}
}

#[derive(Debug)]
struct Bank {
	accounts: Vec<Account>,
	next_id: u32,
}

impl Bank {
	fn new() -> Self {
		Self { accounts: Vec::new(), next_id: 1 }
	}
	fn add_account(&mut self, account: Account) {
		self.accounts.push(account);
	}
	fn new_account(&mut self, holder: String, balance: i32) {
		let mut new_account = Account::new(self.next_id, holder);
		new_account.balance = balance;
		self.accounts.push(new_account);
		self.next_id += 1;
	}
	fn find_account(&mut self, holder: String) -> Option<&mut Account> {
		self.accounts
			.iter_mut()
			.find(|a| a.holder == holder)
	}
	fn total_balance(&self) -> i32 {
		self.accounts
			.iter()
			.map(|a| a.balance)
			.sum()
	}
	fn summary(&self) -> Vec<String> {
		self.accounts
			.iter()
			.map(|a| a.summary())
			.collect()
	}
}
