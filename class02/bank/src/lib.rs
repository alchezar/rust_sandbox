// IKinder

/// A saving account
pub struct SavingAccount {
	balance: i32,
}

impl SavingAccount {
	/// Creates a `SavingAccount` with a balance of 0
	///
	/// # Examples
	///
	/// ```
	/// let account = bank::SavingAccount::new();
	/// assert_eq!(account.get_balance(), 0);
	/// ```
	///
	pub fn new() -> SavingAccount {
		SavingAccount { balance: 0 }
	}

	pub fn get_balance(&self) -> i32 {
		self.balance
	}

	pub fn deposit(&mut self, amount: i32) {
		if amount < 0 {
			panic!("Can not deposit a negative amount!");
		}
		self.balance += amount;
	}

	pub fn transfer(&self, acc_number: u32, amount: i32) -> Result<String, String> {
		Ok(format!("Transferred ${amount} to {acc_number}"))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn should_have_a_starting_balance_of_0() {
		let account = SavingAccount::new();
		assert_eq!(account.get_balance(), 0);
	}

	#[test]
	fn should_be_able_to_deposit() {
		let mut account = SavingAccount::new();
		account.deposit(1);
		assert_eq!(account.get_balance(), 1);
	}

	#[test]
	#[should_panic]
	fn should_panic_if_deposit_is_negative() {
		let mut account = SavingAccount::new();
		account.deposit(-1);
	}

	#[test]
	fn should_transfer_money() -> Result<(), String> {
		let mut account = SavingAccount::new();
		account.deposit(100);
		account.transfer(666, 50)?;
		Ok(())
	}
}
