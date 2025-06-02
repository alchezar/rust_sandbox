// IKinder

pub mod _1 {
	pub mod _1 {
		// FILL the blanks
		fn drink(beverage: &str) {
			if beverage == "lemonade" {
				println!("Success!");
				// IMPLEMENT the below code
				panic!()
			}

			println!("Exercise Failed if printing out this line!");
		}

		pub fn main() {
			drink("lemonade");

			println!("Exercise Failed if printing out this line!");
		}
	}
	pub mod _2 {
		// MAKE the code work by fixing all panics
		pub fn main() {
			assert_eq!("abc".as_bytes(), [97, 98, 99]);

			let v = vec![1, 2, 3];
			let ele = v[2];
			// unwrap may panic when get return a None
			let ele = v.get(2).unwrap();

			// Sometimes, the compiler is unable to find the overflow errors for you in compile time, so a panic will occur
			let v = production_rate_per_hour(2);

			// because of the same reason as above, we have to wrap it in a function to make the panic occur
			divide(15, 1);

			println!("Success!")
		}

		fn divide(x: u8, y: u8) {
			println!("{}", x / y)
		}

		fn production_rate_per_hour(speed: u8) -> f64 {
			let cph: u8 = 221;
			match speed {
				1..=4 => speed as f64 * cph as f64,
				5..=8 => (speed as f64 * cph as f64) * 0.9,
				9..=10 => (speed as f64 * cph as f64) * 0.77,
				_ => 0.0,
			}
		}

		pub fn working_items_per_minute(speed: u8) -> u32 {
			(production_rate_per_hour(speed) / 60.0) as u32
		}
	}
}
pub mod _2 {
	pub mod _1 {
		// FILL in the blanks and FIX the errors
		use std::num::ParseIntError;

		fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
			let n1 = n1_str.parse::<i32>().unwrap();
			let n2 = n2_str.parse::<i32>().unwrap();
			Ok(n1 * n2)
		}

		pub fn main() {
			let result = multiply("10", "2");
			assert_eq!(result, Ok(20));

			let result = multiply("4", "2");
			assert_eq!(result.unwrap(), 8);

			println!("Success!");
		}
	}
	pub mod _2 {
		use std::num::ParseIntError;

		// IMPLEMENT multiply with `?`
		// DON'T use unwrap here
		fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
			let n1 = str::parse::<i32>(n1_str)?;
			let n2 = str::parse::<i32>(n2_str)?;
			Ok(n1 * n2)
		}

		pub fn main() {
			assert_eq!(multiply("3", "4").unwrap(), 12);
			println!("Success!");
		}
	}
	pub mod _3 {
		use std::fs::File;
		use std::io::{self, Read};

		fn read_file1() -> Result<String, io::Error> {
			let f = File::open("hello.txt");
			let mut f = match f {
				Ok(file) => file,
				Err(e) => return Err(e),
			};

			let mut s = String::new();
			match f.read_to_string(&mut s) {
				Ok(_) => Ok(s),
				Err(e) => Err(e),
			}
		}

		// FILL in the blanks with one code line
		// DON'T change any code lines
		fn read_file2() -> Result<String, io::Error> {
			let mut s = String::new();
			File::open("hello.txt")?.read_to_string(&mut s)?;
			Ok(s)
		}

		pub fn main() {
			assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap_err().to_string());
			println!("Success!");
		}
	}
	pub mod _4 {
		// FILL in the blank in two ways: map, and then
		fn add_two(n_str: &str) -> Result<i32, std::num::ParseIntError> {
			n_str
				.parse::<i32>()
				.and_then(|n1| Ok(n1 + 2))
			// n_str.parse::<i32>().map(|n2| n2 + 2)
		}

		pub fn main() {
			assert_eq!(add_two("4").unwrap(), 6);
			println!("Success!");
		}
	}
	pub mod _5 {
		use std::num::ParseIntError;

		// With the return type rewritten, we use pattern matching without `unwrap()`.
		// But it's so Verbose...
		fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
			match n1_str.parse::<i32>() {
				Ok(n1) => match n2_str.parse::<i32>() {
					Ok(n2) => Ok(n1 * n2),
					Err(e) => Err(e),
				},
				Err(e) => Err(e),
			}
		}

		// Rewriting `multiply` to make it succinct
		// You should use BOTH of  `and_then` and `map` here.
		fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
			n1_str
				.parse::<i32>()
				.and_then(|n1| str::parse::<i32>(n2_str).map(|n2| n1 * n2))
		}

		fn print(result: Result<i32, ParseIntError>) {
			match result {
				Ok(n) => println!("n is {}", n),
				Err(e) => println!("Error: {}", e),
			}
		}

		pub fn main() {
			// This still presents a reasonable answer.
			let twenty = multiply1("10", "2");
			print(twenty);

			// The following now provides a much more helpful error message.
			let tt = multiply("t", "2");
			print(tt);

			println!("Success!");
		}
	}
	pub mod _6 {
		// FILL in the blank
		type Res<T> = Result<T, std::num::ParseIntError>;

		// Use the above alias to refer to our specific `Result` type.
		fn multiply(first_number_str: &str, second_number_str: &str) -> Res<i32> {
			first_number_str
				.parse::<i32>()
				.and_then(|first_number| {
					second_number_str
						.parse::<i32>()
						.map(|second_number| first_number * second_number)
				})
		}

		// Here, the alias again allows us to save some space.
		fn print(result: Res<i32>) {
			match result {
				Ok(n) => println!("n is {}", n),
				Err(e) => println!("Error: {}", e),
			}
		}

		fn main() {
			print(multiply("10", "2"));
			print(multiply("t", "2"));

			println!("Success!");
		}
	}
}
