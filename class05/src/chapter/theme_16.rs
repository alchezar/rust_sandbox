// IKinder

pub mod _1 {
    pub mod _1 {
        pub fn main() {
            let s1 = "hello";
            /* Fill in the blank */
            let s = format!("{}, world!", s1);
            assert_eq!(s, "hello, world!");
        }
    }
    pub mod _2 {
        pub fn main() {
            /* Fill in the blanks to make it print:
            Hello world, I am
            Sunface!
            */
            print!("hello world, ");
            println!("I am");
            print!("Sunface!");
        }
    }
}
pub mod _2 {
    pub mod _1 {
        /* Fill in the blanks and Fix the errors */
        #[derive(Debug)]
        struct Structure(i32);

        pub fn main() {
            // Types in std and Rust have implemented the fmt::Debug trait
            println!("{} months in a year.", 12);

            println!("Now {:?} will print!", Structure(3));
        }
    }
    pub mod _2 {
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }

        pub fn main() {
            let person = Person {
                name: "Sunface".to_string(),
                age: 18,
            };

            /* Make it output:
            Person {
                name: "Sunface",
                age: 18,
            }
            */
            println!("{:#?}", person);
        }
    }
    pub mod _3 {
        use std::fmt::{Debug, Formatter, Result};

        #[derive(Debug)]
        struct Structure(i32);

        struct Deep(Structure);

        impl Debug for Deep {
            fn fmt(&self, f: &mut Formatter) -> Result {
                write!(f, "{}", self.0.0)
            }
        }

        pub fn main() {
            // The problem with `derive` is there is no control over how
            // the results look. What if I want this to just show a `7`?

            /* Make it print: Now 7 will print! */
            println!("Now {:?} will print!", Deep(Structure(7)));
        }
    }
    pub mod _4 {

        /* Make it work*/
        use std::fmt;

        struct Point2D {
            x: f64,
            y: f64,
        }

        impl fmt::Display for Point2D {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "Display: {} + {}i", self.x, self.y)
            }
        }

        impl fmt::Debug for Point2D {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
            }
        }

        pub fn main() {
            let point = Point2D { x: 3.3, y: 7.2 };
            assert_eq!(format!("{}", point), "Display: 3.3 + 7.2i");
            assert_eq!(
                format!("{:?}", point),
                "Debug: Complex { real: 3.3, imag: 7.2 }"
            );

            println!("Success!");
        }
    }
    pub mod _5 {

        /* Make it work */
        use std::fmt;

        struct List(Vec<i32>);

        impl fmt::Display for List {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                // Extract the value using tuple indexing,
                // and create a reference to `vec`.
                let vec = &self.0;

                write!(f, "[")?;

                // Iterate over `v` in `vec` while enumerating the iteration
                // count in `count`.
                for (count, v) in vec.iter().enumerate() {
                    // For every element except the first, add a comma.
                    // Use the ? operator to return on errors.
                    if count != 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: ", count)?;
                    write!(f, "{}", v)?;
                }

                // Close the opened bracket and return a fmt::Result value.
                write!(f, "]")
            }
        }

        pub fn main() {
            let v = List(vec![1, 2, 3]);
            assert_eq!(format!("{}", v), "[0: 1, 1: 2, 2: 3]");
            println!("Success!");
        }
    }
}
pub mod _3 {
    pub mod _1 {
        /* Fill in the blanks */
        pub fn main() {
            println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // => Alice, this is Bob. Bob, this is Alice
            assert_eq!(format!("{1}{0}", 1, 2), "21");
            assert_eq!(format!("{1}{0}{0}{1}", 1, 2), "2112");
            println!("Success!");
        }
    }
    pub mod _2 {
        pub fn main() {
            println!("{argument}", argument = "test"); // => "test"

            /* Fill in the blanks */
            assert_eq!(format!("{name}{}", 1, name = 2), "21");
            assert_eq!(format!("{a} {c} {b}", a = "a", b = 'b', c = 3), "a 3 b");

            /* Fix the error */
            // Named argument must be placed after other arguments
            println!("{abc} {d}", abc = "def", d = 2);

            println!("Success!");
        }
    }
    pub mod _3 {
        pub fn main() {
            // The following two are padding with 5 spaces
            println!("Hello {:5}!", "x"); // =>  "Hello x    !"  
            println!("Hello {:1$}!", "x", 5); // =>  "Hello x    !"

            /* Fill in the blanks */
            assert_eq!(format!("Hello {1:0$}!", 5, "x"), "Hello x    !");
            assert_eq!(format!("Hello {:width$}!", "x", width = 5), "Hello x    !");

            println!("Success!");
        }
    }
    pub mod _4 {
        pub fn main() {
            // Left align
            println!("Hello {:<5}!", "x"); // => `Hello x    !`
            // Right align
            assert_eq!(format!("Hello {:>5}!", "x"), "Hello     x!");
            // Center align
            assert_eq!(format!("Hello {:^5}!", "x"), "Hello   x  !");

            // Left align, pad with '&'
            assert_eq!(format!("Hello {:<5}!", "x"), "Hello x    !");

            println!("Success!");
        }
    }
    pub mod _5 {
        pub fn main() {
            println!("Hello {:5}!", 5); // => Hello     5!
            println!("Hello {:+}!", 5); // =>  Hello +5!
            println!("Hello {:05}!", 5); // => Hello 00005!
            println!("Hello {:05}!", -5); // => Hello -0005!

            /* Fill in the blank */
            assert_eq!(
                format!("{number:0>width$}", number = 1, width = 6),
                "000001"
            );

            println!("Success!");
        }
    }
    pub mod _6 {
        /* Fill in the blanks */
        pub fn main() {
            let v = std::f64::consts::PI;

            println!("{:.1$}", v, 4); // same as {:.4} => 3.1416 

            assert_eq!(format!("{:.2}", v), "3.14");
            assert_eq!(format!("{:+.2}", v), "+3.14");
            assert_eq!(format!("{:.0}", v), "3");

            println!("Success!");
        }
    }
    pub mod _7 {
        pub fn main() {
            let s = "Hello, world!";

            println!("{0:.5}", s); // => Hello

            assert_eq!(format!("Hello {1:.0$}!", 3, "abcdefg"), "Hello abc!");

            println!("Success!");
        }
    }
    pub mod _8 {
        pub fn main() {
            assert_eq!(format!("0b{:b}", 27), "0b11011");
            assert_eq!(format!("0o{:o}", 27), "0o33");
            assert_eq!(format!("0x{:x}", 27), "0x1b");
            assert_eq!(format!("0x{:X}", 27), "0x1B");

            println!("{:x}!", 27); // Hex with no prefix => 1b

            println!("{:#010b}", 27); // Pad binary with 0, width = 10,  => 0b00011011

            println!("Success!");
        }
    }
    pub mod _9 {
        fn get_person() -> String {
            String::from("sunface")
        }

        fn get_format() -> (usize, usize) {
            (4, 1)
        }

        pub fn main() {
            let person = get_person();
            println!("Hello, {person}!");

            let (width, precision) = get_format();
            let scores = [("sunface", 99.12), ("jack", 60.34)];
            /* Make it print:
            sunface: 99.1
            jack: 60.3
            */
            for (name, score) in scores {
                println!("{name}: {score}");
            }
        }
    }
}
