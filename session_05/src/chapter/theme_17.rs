// IKinder

pub mod _1 {
	pub mod _1 {
		/* Annotate the lifetime of `i` and `borrow2` */

		// Lifetimes are annotated below with lines denoting the creation
		// and destruction of each variable.
		// `i` has the longest lifetime because its scope entirely encloses
		// both `borrow1` and `borrow2`. The duration of `borrow1` compared
		// to `borrow2` is irrelevant since they are disjoint.
		pub fn main() {
			let i = 3;
			{
				let borrow1 = &i; // `borrow1` lifetime starts. ──┐
				//                                                │
				println!("borrow1: {}", borrow1); //              │
			} // `borrow1 ends. ──────────────────────────────────┘
			{
				let borrow2 = &i;

				println!("borrow2: {}", borrow2);
			}
		}
	}
	pub mod _2 {
		/* Annotate `r` and `x` as above, and explain why this code fails to compile, in the lifetime aspect. */
		pub fn main() {
			{
				let x = 5;
				let r;
				{
					r = &x;
				}
				println!("r: {}", r);
			}
		}
	}
	pub mod _3 {
		fn example() {
			// One input reference with lifetime `'a` which must live
			// at least as long as the function.
			fn print_one<'a>(x: &'a i32) {
				println!("`print_one`: x is {}", x);
			}

			// Mutable references are possible with lifetimes as well.
			fn add_one<'a>(x: &'a mut i32) {
				*x += 1;
			}

			// Multiple elements with different lifetimes. In this case, it
			// would be fine for both to have the same lifetime `'a`, but
			// in more complex cases, different lifetimes may be required.
			fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
				println!("`print_multi`: x is {}, y is {}", x, y);
			}

			// Returning references that have been passed in is acceptable.
			// However, the correct lifetime must be returned.
			fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
				x
			}

			fn main() {
				let x = 7;
				let y = 9;

				print_one(&x);
				print_multi(&x, &y);

				let z = pass_x(&x, &y);
				print_one(z);

				let mut t = 3;
				add_one(&mut t);
				print_one(&t);
			}
		}

		/* Make it work by adding proper lifetime annotation */
		fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
			if x.len() > y.len() { x } else { y }
		}

		pub fn main() {
			let x = "long";
			let y = "longer";
			println!("{}", longest(x, y));
		}
	}
	pub mod _4 {
		// `'a` must live longer than the function.
		// Here, `&String::from("foo")` would create a `String`, followed by a
		// reference. Then the data is dropped upon exiting the scope, leaving
		// a reference to invalid data to be returned.

		/* Fix the error in three ways  */
		// fn invalid_output<'a>() -> &'a String {
		// 	&String::from("foo")
		// }
		fn invalid_output_1() -> &'static str {
			"foo"
		}
		fn invalid_output_2() -> String {
			String::from("foo")
		}
		fn invalid_output_3(s: &String) -> &String {
			s
		}

		pub fn main() {
			let x = invalid_output_1();
			let y = invalid_output_2();
			let z = invalid_output_3(&y);
		}
	}
	pub mod _5 {
		// `print_refs` takes two references to `i32` which have different
		// lifetimes `'a` and `'b`. These two lifetimes must both be at
		// least as long as the function `print_refs`.
		fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
			println!("x is {} and y is {}", x, y);
		}

		/* Make it work */
		// A function which takes no arguments, but has a lifetime parameter `'a`.
		fn failed_borrow<'a>() {
			let _x = 12;

			// ERROR: `_x` does not live long enough
			// let y: &'a i32 = & _x;
			let y: &i32 = &_x;
			// Attempting to use the lifetime `'a` as an explicit type annotation
			// inside the function will fail because the lifetime of `&_x` is shorter
			// than `'a`. A short lifetime cannot be coerced into a longer one.
		}

		pub fn main() {
			let (four, nine) = (4, 9);

			// Borrows (`&`) of both variables are passed into the function.
			print_refs(&four, &nine);
			// Any input that is borrowed must outlive the borrower.
			// In other words, the lifetime of `four` and `nine` must
			// be longer than that of `print_refs`.

			failed_borrow();
			// `failed_borrow` contains no references to force `'a` to be
			// longer than the lifetime of the function, but `'a` is longer.
			// Because the lifetime is never constrained, it defaults to `'static`.
		}
	}
	pub mod _6 {
		/* Make it work by adding proper lifetime annotation */

		// A type `Borrowed` which houses a reference to an
		// `i32`. The reference to `i32` must outlive `Borrowed`.
		#[derive(Debug)]
		struct Borrowed<'a>(&'a i32);

		// Similarly, both references here must outlive this structure.
		#[derive(Debug)]
		struct NamedBorrowed<'a> {
			x: &'a i32,
			y: &'a i32,
		}

		// An enum which is either an `i32` or a reference to one.
		#[derive(Debug)]
		enum Either<'a> {
			Num(i32),
			Ref(&'a i32),
		}

		pub fn main() {
			let x = 18;
			let y = 15;

			let single = Borrowed(&x);
			let double = NamedBorrowed { x: &x, y: &y };
			let reference = Either::Ref(&x);
			let number = Either::Num(y);

			println!("x is borrowed in {:?}", single);
			println!("x and y are borrowed in {:?}", double);
			println!("x is borrowed in {:?}", reference);
			println!("y is *not* borrowed in {:?}", number);
		}
	}
	pub mod _7 {
		/* Make it work */

		#[derive(Debug)]
		struct NoCopyType {}

		#[derive(Debug)]
		struct Example<'a, 'b> {
			a: &'a u32,
			b: &'b NoCopyType,
		}

		pub fn main() {
			/* 'a tied to fn-main stack-frame */
			let var_a = 35;
			let example: Example;

			{
				/* Lifetime 'b tied to new stack-frame/scope */
				let var_b = NoCopyType {};

				/* fixme */
				example = Example { a: &var_a, b: &var_b };
				println!("(Success!) {:?}", example);
			}
		}
	}
	pub mod _8 {
		#[derive(Debug)]
		struct NoCopyType {}

		#[derive(Debug)]
		#[allow(dead_code)]
		struct Example<'a, 'b> {
			a: &'a u32,
			b: &'b NoCopyType,
		}

		/* Fix function signature */
		fn fix_me<'b>(foo: &'b Example) -> &'b NoCopyType {
			foo.b
		}

		pub fn main() {
			let no_copy = NoCopyType {};
			let example = Example { a: &1, b: &no_copy };
			fix_me(&example);
			println!("Success!")
		}
	}
	pub mod _9 {
		fn example() {
			struct Owner(i32);

			impl Owner {
				// Annotate lifetimes as in a standalone function.
				fn add_one<'a>(&'a mut self) {
					self.0 += 1;
				}
				fn print<'a>(&'a self) {
					println!("`print`: {}", self.0);
				}
			}

			fn main() {
				let mut owner = Owner(18);

				owner.add_one();
				owner.print();
			}
		}

		/* Make it work by adding proper lifetime annotations */
		struct ImportantExcerpt<'a> {
			part: &'a str,
		}

		impl<'a> ImportantExcerpt<'_> {
			fn level(&'a self) -> i32 {
				3
			}
		}

		pub fn main() {}
	}
	pub mod _10 {
		/* Remove all the lifetimes that can be elided */
		fn input(x: &i32) {
			println!("`annotated_input`: {}", x);
		}

		fn pass(x: &i32) -> &i32 {
			x
		}

		fn longest<'a>(x: &'a str, y: &'_ str) -> &'a str {
			x
		}

		struct Owner(i32);

		impl Owner {
			// Annotate lifetimes as in a standalone function.
			fn add_one(&mut self) {
				self.0 += 1;
			}
			fn print(&self) {
				println!("`print`: {}", self.0);
			}
		}

		struct Person<'a> {
			age: u8,
			name: &'a str,
		}

		enum Either<'a> {
			Num(i32),
			Ref(&'a i32),
		}

		pub fn main() {}
	}
}
pub mod _2 {
	pub mod _1 {
		/* Fill in the blank in two ways */
		pub fn main() {
			let v = "hello";
			need_static(v);

			println!("Success!")
		}

		fn need_static(r: &'static str) {
			assert_eq!(r, "hello");
		}
	}
	pub mod _2 {
		#[derive(Debug, Clone)]
		struct Config {
			a: String,
			b: String,
		}
		// static mut CONFIG: Option<&mut Config> = None;
		static mut CONFIG: Option<*mut Config> = None;

		/* Make it work without changing the function signatures of `init`*/
		fn init() -> Option<&'static mut Config> {
			// Some(&mut Config { a: "A".to_string(), b: "B".to_string() })
			let boxed = Box::new(Config { a: "A".to_string(), b: "B".to_string() });
			Some(Box::leak(boxed))
		}

		pub fn main() {
			unsafe {
				CONFIG = init().map(|cfg| cfg as *mut Config);
				if let Some(ptr) = CONFIG {
					println!("{:?}", &*ptr)
				}
			}
		}
	}
	pub mod _3 {
		pub fn main() {
			let mut static_string;
			{
				// Make a `string` literal and print it:
				static_string = "I'm in read-only memory";
				println!("static_string: {}", static_string);

				// When `static_string` goes out of scope, the reference
				// can no longer be used, but the data remains in the binary.
			}

			println!("static_string reference remains alive: {}", static_string);
		}
	}
	pub mod _4 {
		// Make a constant with `'static` lifetime.
		static NUM: i32 = 18;

		// Returns a reference to `NUM` where its `'static`
		// lifetime is coerced to that of the input argument.
		fn coerce_static(_: &i32) -> &i32 {
			&NUM
		}

		pub fn main() {
			{
				// Make an integer to use for `coerce_static`:
				let lifetime_num = 9;

				// Coerce `NUM` to lifetime of `lifetime_num`:
				let coerced_static = coerce_static(&lifetime_num);

				println!("coerced_static: {}", coerced_static);
			}

			println!("NUM: {} stays accessible!", NUM);
		}
	}
	pub mod _5 {
		/* Make it work */
		use std::fmt::Debug;

		fn print_it<T: 'static + Debug>(input: T) {
			println!("'static value passed in is: {:?}", input);
		}

		fn print_it1(input: impl 'static + Debug) {
			println!("'static value passed in is: {:?}", input);
		}

		fn print_it2<T: 'static + Debug>(input: &T) {
			println!("'static value passed in is: {:?}", input);
		}

		pub fn main() {
			// `i` is owned and contains no references, thus it's 'static:
			static I: i32 = 5;
			print_it(I);

			// oops, &i only has the lifetime defined by the scope of
			// main(), so it's not 'static:
			print_it(&I);
			print_it1(&I);

			// but this one WORKS!
			print_it2(&I);
		}
	}
	pub mod _6 {
		use std::fmt::Display;

		pub fn main() {
			let mut string = "First".to_owned();

			string.push_str(string.to_uppercase().as_str());
			print_a(&string);
			print_b(&string);
			print_c(Box::leak(Box::new(string.clone()))); // Compilation error
			print_d(Box::leak(Box::new(string.clone()))); // Compilation error
			print_e(&string);
			print_f(&string);
			print_g(Box::leak(Box::new(string))); // Compilation error
		}

		fn print_a<T: Display + 'static>(t: &T) {
			println!("{}", t);
		}

		fn print_b<T>(t: &T)
		where
			T: Display + 'static,
		{
			println!("{}", t);
		}

		fn print_c(t: &'static dyn Display) {
			println!("{}", t)
		}

		fn print_d(t: &'static impl Display) {
			println!("{}", t)
		}

		fn print_e(t: &(dyn Display + 'static)) {
			println!("{}", t)
		}

		fn print_f(t: &(impl Display + 'static)) {
			println!("{}", t)
		}

		fn print_g(t: &'static String) {
			println!("{}", t);
		}
	}
}
pub mod _3 {
	pub mod _1 {
		/* Annotate struct with lifetime:
		1. `r` and `s` must have different lifetimes
		2. Lifetime of `s` is bigger than that of 'r'
		*/
		struct DoubleRef<'r, 's, T>
		where
			's: 'r,
		{
			r: &'r T,
			s: &'s T,
		}
		pub fn main() {
			println!("Success!")
		}
	}
	pub mod _2 {
		/* Adding trait bounds to make it work */
		struct ImportantExcerpt<'a> {
			part: &'a str,
		}

		impl<'a, 'b> ImportantExcerpt<'a>
		where
			'a: 'b,
		{
			fn announce_and_return_part(&'a self, announcement: &'b str) -> &'b str {
				println!("Attention please: {}", announcement);
				self.part
			}
		}

		pub fn main() {
			println!("Success!")
		}
	}
	#[allow(unused_assignments)]
	pub mod _3 {
		/* Adding trait bounds to make it work */
		fn f<'a, 'b>(x: &'a i32, mut y: &'b i32)
		where
			'a: 'b,
		{
			y = x;
			let r: &'b &'a i32 = &&0;
		}

		pub fn main() {
			println!("Success!")
		}
	}
	pub mod _4 {
		/* Adding HRTB to make it work!*/
		fn call_on_ref_zero<F>(f: F)
		where
			F: for<'a> Fn(&'a i32),
		{
			let zero = 0;
			f(&zero);
		}

		pub fn main() {
			println!("Success!");
		}
	}
	pub mod _5 {
		/* Make it work by reordering some code */
		pub fn main() {
			let mut data = 10;
			let ref1 = &mut data;
			*ref1 += 1;

			let ref2 = &mut *ref1;
			*ref2 += 2;

			println!("{}", data);
		}
	}
	pub mod _6 {
		/* Make it work */
		struct Interface<'a, 'b> {
			manager: &'b mut Manager<'a>,
		}

		impl<'a, 'b> Interface<'a, 'b> {
			pub fn noop(self) {
				println!("interface consumed");
			}
		}

		struct Manager<'a> {
			text: &'a str,
		}

		struct List<'a> {
			manager: Manager<'a>,
		}

		impl<'a> List<'a> {
			pub fn get_interface<'b>(&'b mut self) -> Interface<'a, 'b>
			where
				'a: 'b,
			{
				Interface { manager: &mut self.manager }
			}
		}

		pub fn main() {
			let mut list = List { manager: Manager { text: "hello" } };

			list.get_interface().noop();

			println!("Interface should be dropped here and the borrow released");

			use_list(&list);
		}

		fn use_list(list: &List) {
			println!("{}", list.manager.text);
		}
	}
}
