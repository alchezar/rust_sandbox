// IKinder

pub mod _1 {
	pub mod _1 {
		use std::fmt;

		/* Define the Wrapper type */
		struct Wrapper(Vec<String>);

		// Display is an external trait
		impl fmt::Display for Wrapper {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "[{}]", self.0.join(", "))
			}
		}

		pub fn main() {
			// Vec is an external type, so you cannot implement Display trait on Vec type
			let w = Wrapper(vec![String::from("hello"), String::from("world")]);
			println!("w = {}", w);
		}
	}
	pub mod _2 {
		/* Make it work */
		struct Meters(u32);

		impl Meters {
			fn pow(&self, exp: u32) -> u32 {
				self.0.pow(exp)
			}
		}

		pub fn main() {
			let i: u32 = 2;
			assert_eq!(i.pow(2), 4);

			let n = Meters(i);
			// The `pow` method is defined on `u32` type, we can't directly call it
			assert_eq!(n.pow(2), 4);
		}
	}
	pub mod _3 {
		/* Make it work */
		struct Years(i64);

		struct Days(i64);

		impl Years {
			pub fn to_days(&self) -> Days {
				Days(self.0 * 365)
			}
		}

		impl Days {
			pub fn to_years(&self) -> Years {
				Years(self.0 / 365)
			}
		}

		// An age verification function that checks age in years, must be given a value of type Years.
		fn old_enough(age: &Years) -> bool {
			age.0 >= 18
		}

		pub fn main() {
			let age = Years(5);
			let age_days = age.to_days();
			println!("Old enough {}", old_enough(&age));
			println!("Old enough {}", old_enough(&age_days.to_years()));
		}
	}
	pub mod _4 {
		use std::fmt::{self, format};
		use std::ops::Add;

		struct Meters(u32);
		impl fmt::Display for Meters {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				write!(f, "There are still {} meters left", self.0)
			}
		}

		impl Add for Meters {
			type Output = Self;

			fn add(self, other: Meters) -> Self {
				Self(self.0 + other.0)
			}
		}
		pub fn main() {
			let d = calculate_distance(Meters(10), Meters(20));
			assert_eq!(format!("{}", d), "There are still 30 meters left");
		}

		/* Implement calculate_distance  */
		fn calculate_distance(a: Meters, b: Meters) -> Meters {
			a + b
		}
	}
	pub mod _5 {
		enum VeryVerboseEnumOfThingsToDoWithNumbers {
			Add,
			Subtract,
		}

		/* Fill in the blank */
		use VeryVerboseEnumOfThingsToDoWithNumbers as Operations;

		pub fn main() {
			// We can refer to each variant via its alias, not its long and inconvenient
			// name.
			let x = Operations::Add;
		}
	}
	pub mod _6 {
		enum VeryVerboseEnumOfThingsToDoWithNumbers {
			Add,
			Subtract,
		}

		impl VeryVerboseEnumOfThingsToDoWithNumbers {
			fn run(&self, x: i32, y: i32) -> i32 {
				match self {
					Self::Add => x + y,
					Self::Subtract => x - y,
				}
			}
		}
		pub fn main() {}
	}
	pub mod _7 {
		/* Make it work with const generics */
		fn my_function<const N: usize>() -> [u32; N] {
			[123; N]
		}

		pub fn main() {
			let arr = my_function::<6>();
			println!("{:?}", arr);
		}
	}
	pub mod _8 {
		/* Make it work with slice references */
		pub fn main() {
			let s: &str = "Hello there!";
			let arr: [u8; 3] = [1, 2, 3];
		}
	}
	pub mod _9 {
		/* Make it work in two ways */
		use std::fmt::Display;
		fn foobar(thing: impl Display) {}

		pub fn main() {}
	}
}
pub mod _2 {
	pub mod _1 {
		// Make it work
		pub fn main() {
			// Create a new box `b` that contains the integer 5
			let b = Box::new(5);
			assert_eq!(*b, 5);

			println!("Success!");
		}
	}
	pub mod _2 {

		// Make it work
		pub fn main() {
			let b = Box::new("Hello");
			print_boxed_string(b);
		}

		fn print_boxed_string(b: Box<&str>) {
			println!("{}", b);
		}
	}
	pub mod _3 {
		// Make it work
		pub fn main() {
			let b1 = Box::new(5);
			let b2 = b1;
			assert_eq!(*b2, 5);

			println!("Success!");
		}
	}

	pub mod _4 {
		// Make it work
		pub fn main() {
			// Create a box `b` with an array [1, 2, 3, 4, 5]
			let b = Box::new([1, 2, 3, 4, 5]);
			// Print each integer in `b`

			// (*b).iter()
			// 	.for_each(|x| println!("{}", *x));

			for item in *b {
				println!("{}", item)
			}
		}
	}
}
pub mod _3 {
	pub mod _1 {
		// FIX the error without removing any code line
		struct Test {
			list: Vec<i32>,
			a: i32,
		}

		impl Test {
			pub fn new() -> Self {
				Self { list: vec![1, 2, 3, 4, 5, 6, 7], a: 0 }
			}

			pub fn run(&mut self) {
				// for i in self.list.iter() {
				// 	self.do_something(*i)
				// }
				for i in 0..self.list.len() {
					let value = self.list[i];
					self.do_something(value)
				}
			}

			pub fn do_something(&mut self, n: i32) {
				self.a = n;
			}
		}

		pub fn main() {
			let mut test = Test::new();
			println!("Before: {}", test.a);
			test.run();
			println!("After: {}", test.a);
		}
	}
}
