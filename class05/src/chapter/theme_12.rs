// IKinder

#![allow(
    clippy::cast_slice_different_sizes,
    clippy::char_lit_as_u8,
    clippy::eq_op,
    clippy::module_inception,
    clippy::needless_borrows_for_generic_args,
    clippy::size_of_ref,
    clippy::toplevel_ref_arg
)]

pub mod _1 {
    pub mod _1 {
        // FIX the errors and FILL in the blank
        // DON'T remove any code
        pub fn main() {
            let decimal = 97.123_f32;

            let integer: u8 = decimal as u8 + 1;

            let c1: char = decimal as u8 as char;
            let c2 = integer as char;

            assert_eq!(integer, 'b' as u8);

            println!("Success!");
        }
    }
    pub mod _2 {
        #[allow(overflowing_literals)]
        pub fn main() {
            assert_eq!(u8::MAX, 255);
            // The max of `u8` is 255 as shown above.
            // So the below code will cause an overflow error: literal out of range for `u8`.
            // PLEASE look for clues within compiler errors to FIX it.
            // DON'T modify any code in the main.
            let v = 1000u16 as u8;

            println!("Success!");
        }
    }
    pub mod _3 {
        pub fn main() {
            assert_eq!(1000u32 as u16, 1000);
            assert_eq!(1000u16 as u8, 232);

            // For positive numbers, this is the same as the modulus
            println!("1000 mod 256 is : {}", 1000 % 256);
            assert_eq!(-1_i8 as u8, 255);

            // Since Rust 1.45, the `as` keyword performs a *saturating cast*
            // when casting from float to int. If the floating point value exceeds
            // the upper bound or is less than the lower bound, the returned value
            // will be equal to the bound crossed.
            assert_eq!(300.1_f32 as u8, 255);
            assert_eq!(-100.1_f32 as u8, 0);

            // This behavior incurs a small runtime cost and can be avoided
            // with unsafe methods, however, the results might overflow and
            // return **unsound values**. Use these methods wisely:
            unsafe {
                // 300.0 is 44
                println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
                // -100.0 as u8 is 156
                println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
                // nan as u8 is 0
                println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
            }
        }
    }
    pub mod _4 {
        // FILL in the blanks
        pub fn main() {
            let mut values: [i32; 2] = [1, 2];
            let p1: *mut i32 = values.as_mut_ptr();
            let first_address: usize = p1 as usize;
            let second_address = first_address + 4; // 4 == std::mem::size_of::<i32>()
            let p2: *mut i32 = second_address as *mut i32; // p2 points to the 2nd element in values
            unsafe {
                // Add one to the second element
                *p2 += 1;
            }

            assert_eq!(values[1], 3);
            println!("Success!");
        }
    }
    pub mod _5 {
        pub fn main() {
            let arr: [u64; 13] = [0; 13];
            assert_eq!(size_of_val(&arr), 8 * 13);

            let a: *const [u64] = &arr;
            let b = a as *const [u8];
            unsafe {
                let ref test = *b;
                assert_eq!(size_of_val(&*b), 13);
                println!("{}", size_of_val(&test));
            }
            println!("Success!");
        }
    }
}

pub mod _2 {
    pub mod _1 {
        /// Custom wrapper class, to add the ability to convert
        /// `i32` from `char` variable.
        struct C32(i32);
        impl C32 {
            fn into_inner(self) -> i32 {
                self.0
            }
        }
        impl From<char> for C32 {
            fn from(value: char) -> Self {
                Self(value as u32 as i32)
            }
        }

        pub fn main() {
            // impl From<bool> for i32
            let i1: i32 = false.into();
            let i2: i32 = i32::from(false);
            assert_eq!(i1, i2);
            assert_eq!(i1, 0);

            // FIX the error in two ways
            /* 1. use a similar type which `impl From<char>`, maybe you
            should check the docs mentioned above to find the answer */
            // 2. a keyword from the last chapter
            // let i3: i32 = 'a'.into();
            let i3: u32 = 'a'.into();

            let wrapper: C32 = 'a'.into();
            let i3: i32 = wrapper.into_inner();
            let i3: i32 = C32::from('a').into_inner();

            // FIX the error in two ways
            // let s: String = 'a' as String;
            let s: String = 'a'.into();
            let s: String = String::from('a');

            println!("Success!");
        }
    }
    pub mod _2 {
        // From is now included in `std::prelude`, so there is no need to introduce it into the current scope
        // use std::convert::From;

        #[derive(Debug)]
        struct Number {
            value: i32,
        }

        impl From<i32> for Number {
            fn from(value: i32) -> Self {
                Self { value }
            }
        }

        // FILL in the blanks
        pub fn main() {
            let num = Number::from(30);
            assert_eq!(num.value, 30);

            let num: Number = 30.into();
            assert_eq!(num.value, 30);

            println!("Success!");
        }
    }
    pub mod _3 {
        use std::fs;
        use std::io;
        use std::num;

        enum CliError {
            IoError(io::Error),
            ParseError(num::ParseIntError),
        }

        impl From<io::Error> for CliError {
            fn from(value: io::Error) -> Self {
                CliError::IoError(value)
            }
        }

        impl From<num::ParseIntError> for CliError {
            fn from(value: num::ParseIntError) -> Self {
                CliError::ParseError(value)
            }
        }

        fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
            // ? automatically converts io::Error to CliError
            let contents = fs::read_to_string(&file_name)?;
            // num::ParseIntError -> CliError
            let num: i32 = contents.trim().parse()?;
            Ok(num)
        }

        pub fn main() {
            println!("Success!");
        }
    }
    pub mod _4 {
        // TryFrom and TryInto are included in `std::prelude`, so there is no need to introduce it into the current scope
        // use std::convert::TryInto;

        pub fn main() {
            let n: i16 = 256;

            // Into trait has a method `into`,
            // hence TryInto has a method ?

            // let n: u8 = match n.try_into() {
            // 	Ok(n) => n,
            // 	Err(e) => {
            // 		println!("there is an error when converting: {:?}, but we catch it", e.to_string());
            // 		0
            // 	}
            // };
            let n: u8 = n.try_into().unwrap_or_else(|e| {
                println!(
                    "there is an error when converting: {:?}, but we catch it",
                    e
                );
                0
            });

            assert_eq!(n, 0);
            println!("Success!");
        }
    }
    pub mod _5 {
        #[derive(Debug, PartialEq)]
        struct EvenNum(i32);

        impl TryFrom<i32> for EvenNum {
            type Error = ();

            // IMPLEMENT `try_from`
            fn try_from(value: i32) -> Result<Self, Self::Error> {
                if value % 2 == 0 {
                    Ok(EvenNum(value))
                } else {
                    Err(())
                }
            }
        }

        pub fn main() {
            assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
            assert_eq!(EvenNum::try_from(5), Err(()));

            // FILL in the blanks
            let result: Result<EvenNum, ()> = 8i32.try_into();
            assert_eq!(result, Ok(EvenNum(8)));
            let result: Result<EvenNum, ()> = 5i32.try_into();
            assert_eq!(result, Err(()));

            println!("Success!");
        }
    }
}
pub mod _3 {
    pub mod _1 {
        use std::fmt;

        struct Point {
            x: i32,
            y: i32,
        }

        impl fmt::Display for Point {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "The point is ({}, {})", self.x, self.y)
            }
        }

        pub fn main() {
            let origin = Point { x: 0, y: 0 };
            // FILL in the blanks
            assert_eq!(origin.to_string(), "The point is (0, 0)");
            assert_eq!(format!("{}", origin), "The point is (0, 0)");

            println!("Success!");
        }
    }
    pub mod _2 {
        // To use `from_str` method, you need to introduce this trait into the current scope.
        use std::str::FromStr;
        pub fn main() {
            let parsed: i32 = "5".parse().unwrap();
            let turbo_parsed = "10".parse::<i32>().unwrap();
            let from_str = i32::from_str("20").unwrap();
            let sum = parsed + turbo_parsed + from_str;
            assert_eq!(sum, 35);

            println!("Success!");
        }
    }
    pub mod _3 {
        use std::num::ParseIntError;
        use std::str::FromStr;

        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }

        impl FromStr for Point {
            type Err = ParseIntError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let coords: Vec<&str> = s
                    .trim_matches(|p| p == '(' || p == ')')
                    .split(',')
                    .map(|x| x.trim())
                    .collect();

                let x_from_str = coords[0].parse::<i32>()?;
                let y_from_str = coords[1].parse::<i32>()?;

                Ok(Point {
                    x: x_from_str,
                    y: y_from_str,
                })
            }
        }
        pub fn main() {
            // FILL in the blanks in two ways
            // DON'T change code anywhere else
            let p = "(3,4)".parse::<Point>();
            let p = Point::from_str("(3,4)");
            assert_eq!(p.unwrap(), Point { x: 3, y: 4 });

            println!("Success!");
        }
    }
    pub mod _4 {
        fn foo() -> i32 {
            0
        }
        fn main() {
            let pointer = foo as *const ();
            let function = unsafe { std::mem::transmute::<*const (), fn() -> i32>(pointer) };
        }
        struct R<'a>(&'a i32);
        unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
            unsafe { std::mem::transmute::<R<'b>, R<'static>>(r) }
        }

        unsafe fn shorten_invariant_lifetime<'b, 'c>(r: &'b mut R<'static>) -> &'b mut R<'c> {
            unsafe { std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r) }
        }

        fn main1() {
            /*Turning raw bytes(&[u8]) to u32, f64, etc.: */
            let raw_bytes = [0x78, 0x56, 0x34, 0x12];
            // let num = unsafe { std::mem::transmute::<[u8; 4], u32>(raw_bytes) };
            let num = u32::from_ne_bytes(raw_bytes);

            // Use `u32::from_ne_bytes` instead
            let num = u32::from_ne_bytes(raw_bytes);
            // Or use `u32::from_le_bytes` or `u32::from_be_bytes` to specify the endianness
            let num = u32::from_le_bytes(raw_bytes);
            assert_eq!(num, 0x12345678);
            let num = u32::from_be_bytes(raw_bytes);
            assert_eq!(num, 0x78563412);

            /*Turning a pointer into an usize: */
            let ptr = &0;
            let ptr_num_transmute = unsafe { std::mem::transmute::<&i32, usize>(ptr) };

            // Use an `as` cast instead
            let ptr_num_cast = ptr as *const i32 as usize;

            /*Turning an &mut T into an &mut U: */
            let ptr = &mut 0;
            let val_transmuted = unsafe { std::mem::transmute::<&mut i32, &mut u32>(ptr) };

            // Now, put together `as` and reborrowing - note the chaining of `as`
            // `as` is not transitive
            let val_casts = unsafe { &mut *(ptr as *mut i32 as *mut u32) };

            /*Turning a &str into a &[u8]: */
            // This is not a good way to do this.
            let slice = unsafe { std::mem::transmute::<&str, &[u8]>("Rust") };
            assert_eq!(slice, &[82, 117, 115, 116]);

            // You could use `str::as_bytes`
            let slice = "Rust".as_bytes();
            assert_eq!(slice, &[82, 117, 115, 116]);

            // Or, use a byte string if you have control over the string
            // literal
            assert_eq!(b"Rust", &[82, 117, 115, 116]);
        }
    }
}
