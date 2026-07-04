// IKinder

pub mod _1 {
    pub mod _1 {
        // Fill in the blanks to make it work
        struct A; // Concrete type `A`.
        struct S(A); // Concrete type `S`.
        struct SGen<T>(T); // Generic type `SGen`.

        fn reg_fn(_s: S) {}

        fn gen_spec_t(_s: SGen<A>) {}

        fn gen_spec_i32(_s: SGen<i32>) {}

        fn generic<T>(_s: SGen<T>) {}

        pub fn main() {
            // Using the non-generic functions
            reg_fn(S(A)); // Concrete type.
            gen_spec_t(SGen(A)); // Implicitly specified type parameter `A`.
            gen_spec_i32(SGen(666)); // Implicitly specified type parameter `i32`.

            // Explicitly specified type parameter `char` to `generic()`.
            generic::<char>(SGen('a'));

            // Implicitly specified type parameter `char` to `generic()`.
            generic(SGen('a'));

            println!("Success!");
        }
    }
    pub mod _2 {
        // Implement the generic function below.
        fn sum<T>(left: T, right: T) -> T
        where
            T: std::ops::Add<Output = T>,
        {
            left + right
        }

        pub fn main() {
            assert_eq!(5, sum(2i8, 3i8));
            assert_eq!(50, sum(20, 30));
            assert_eq!(2.46, sum(1.23, 1.23));

            println!("Success!");
        }
    }
    pub mod _3 {
        // Implement struct Point to make it work.
        struct Point<T> {
            x: T,
            y: T,
        }

        pub fn main() {
            let integer = Point::<u8> { x: 5, y: 10 };
            let float = Point::<f32> { x: 1.0, y: 4.0 };

            println!("Success!");
        }
    }
    pub mod _4 {
        // Modify this struct to make the code work
        struct Point<T, U> {
            x: T,
            y: U,
        }

        pub fn main() {
            // DON'T modify this code.
            let p = Point {
                x: 5,
                y: "hello".to_string(),
            };

            println!("Success!");
        }
    }
    pub mod _5 {
        // Add generic for Val to make the code work, DON'T modify the code in `main`.
        struct Val<T> {
            val: T,
        }

        impl<T> Val<T> {
            fn value(&self) -> &T {
                &self.val
            }
        }

        pub fn main() {
            let x = Val { val: 3.0 };
            let y = Val {
                val: "hello".to_string(),
            };
            println!("{}, {}", x.value(), y.value());
        }
    }
    pub mod _6 {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> {
            // Implement mix_up to make it work, DON'T modify other code.
            fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                let x = self.x;
                let y = other.y;

                Point { x, y }
            }
        }

        pub fn main() {
            let p1 = Point { x: 5, y: 10 };
            let p2 = Point {
                x: "Hello",
                y: '中',
            };

            let p3 = p1.mix_up(p2);

            assert_eq!(p3.x, 5);
            assert_eq!(p3.y, '中');

            println!("Success!");
        }
    }
    pub mod _7 {
        // Fix the errors to make the code work.
        struct Point<T> {
            x: T,
            y: T,
        }

        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        pub fn main() {
            let p = Point { x: 5.0, y: 10.0 };
            println!("{}", p.distance_from_origin());
        }
    }
}
pub mod _2 {
    pub mod _1 {
        struct Array<T, const N: usize> {
            data: [T; N],
        }

        pub fn main() {
            let arrays = [
                Array { data: [1, 2, 3] },
                Array { data: [1, 2, 3] },
                Array { data: [1, 2, 3] },
            ];

            println!("Success!");
        }
    }
    pub mod _2 {
        // Fill in the blanks to make it work.
        fn print_array<T, const N: usize>(arr: [T; N])
        where
            T: std::fmt::Debug,
        {
            println!("{:?}", arr);
        }
        pub fn main() {
            let arr = [1, 2, 3];
            print_array(arr);

            let arr = ["hello", "world"];
            print_array(arr);
        }
    }
    pub mod _3 {
        pub fn main() {
            check_size([0u8; 767]);
            check_size([0i32; 191]);
            check_size(["hello你好"; 9]); // Size of &str?
            check_size([(); 0].map(|_| "hello你好".to_string())); // Size of String?
            check_size(['中'; 1]); // Size of char?

            println!("Success!");
        }

        pub enum Assert<const CHECK: bool> {}

        pub trait IsTrue {}

        impl IsTrue for Assert<true> {}

        // fn check_size<T>(val: T)
        // where
        // 	Assert<{ size_of::<T>() < 768 }>: IsTrue,
        // {
        // 	//...
        // }

        fn check_size<T>(val: T) {
            assert!(size_of::<T>() < 768);
        }
    }
}
pub mod _3 {
    pub mod _1 {
        // Fill in the two impl blocks to make the code work.
        // DON'T modify the code in `main`.
        trait Hello {
            fn say_hi(&self) -> String {
                String::from("hi")
            }

            fn say_something(&self) -> String;
        }

        struct Student {}
        impl Hello for Student {
            fn say_something(&self) -> String {
                "I'm a good student".into()
            }
        }
        struct Teacher {}
        impl Hello for Teacher {
            fn say_hi(&self) -> String {
                "Hi, I'm your new teacher".into()
            }
            fn say_something(&self) -> String {
                "I'm not a bad teacher".into()
            }
        }

        pub fn main() {
            let s = Student {};
            assert_eq!(s.say_hi(), "hi");
            assert_eq!(s.say_something(), "I'm a good student");

            let t = Teacher {};
            assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
            assert_eq!(t.say_something(), "I'm not a bad teacher");

            println!("Success!");
        }
    }
    pub mod _2 {
        // `Centimeters`, a tuple struct that can be compared
        #[derive(PartialEq, PartialOrd)]
        struct Centimeters(f64);

        // `Inches`, a tuple struct that can be printed
        #[derive(Debug)]
        struct Inches(i32);

        impl Inches {
            fn to_centimeters(&self) -> Centimeters {
                let inches = self.0 as f64;
                Centimeters(inches * 2.54)
            }
        }

        // ADD some attributes to make the code work!
        // DON'T modify other code!
        #[derive(Debug, PartialEq, PartialOrd)]
        struct Seconds(i32);

        pub fn main() {
            let one_second = Seconds(1);
            let _this_is_true = one_second == one_second;
            let _this_is_false = one_second > one_second;
            println!("One second looks like: {:?}", one_second);

            let foot = Inches(12);
            println!("One foot equals {:?}", foot);

            let meter = Centimeters(100.0);
            let cmp = if foot.to_centimeters() < meter {
                "smaller"
            } else {
                "bigger"
            };
            println!("One foot is {} than one meter.", cmp);
        }
    }
    pub mod _3 {
        // Implement fn multiply to make the code work.
        // As mentioned above, `+` needs `T` to implement `std::ops::Add` Trait.
        // So, what about `*`?  You can find the answer here: https://doc.rust-lang.org/core/ops/
        fn multiply<T>(left: T, right: T) -> T
        where
            T: std::ops::Mul<Output = T>,
        {
            left * right
        }

        pub fn main() {
            assert_eq!(6, multiply(2u8, 3u8));
            assert_eq!(5.0, multiply(1.0, 5.0));

            println!("Success!");
        }
    }
    pub mod _4 {
        use std::ops;

        struct Foo;
        struct Bar;

        #[derive(Debug, PartialEq)]
        struct FooBar;

        #[derive(Debug, PartialEq)]
        struct BarFoo;

        // The `std::ops::Add` trait is used to specify the functionality of `+`.
        // Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
        // The following block implements the operation: Foo + Bar = FooBar
        impl ops::Add<Bar> for Foo {
            type Output = FooBar;

            fn add(self, _rhs: Bar) -> FooBar {
                FooBar
            }
        }

        impl ops::Sub<Bar> for Foo {
            type Output = BarFoo;

            fn sub(self, _rhs: Bar) -> BarFoo {
                BarFoo
            }
        }

        pub fn main() {
            // DON'T modify the code below.
            // You need to derive some trait for FooBar to make it comparable.
            assert_eq!(Foo + Bar, FooBar);
            assert_eq!(Foo - Bar, BarFoo);

            println!("Success!");
        }
    }
    pub mod _5 {
        use rand::{Rng, random};

        struct Sheep {}
        struct Cow {}

        trait Animal {
            fn noise(&self) -> String;
        }

        impl Animal for Sheep {
            fn noise(&self) -> String {
                "baaaaah!".to_string()
            }
        }

        impl Animal for Cow {
            fn noise(&self) -> String {
                "moooooo!".to_string()
            }
        }

        // Returns some struct that implements Animal, but we don't know which one at compile time.
        // FIX the errors here, you can make a fake random, or you can use a trait object.
        fn random_animal<'a>(random_number: f64) -> &'a dyn Animal {
            if random_number < 0.5 {
                &Sheep {}
            } else {
                &Cow {}
            }
        }

        pub fn main() {
            let random_number = rand::rng().random_range(0.0..=1.0);
            let animal = random_animal(random_number);
            println!(
                "You've randomly chosen an animal, and it says {}",
                animal.noise()
            );
        }
    }
}
pub mod _4 {
    pub mod _2 {
        trait Bird {
            fn quack(&self);
        }

        struct Duck;
        impl Duck {
            fn fly(&self) {
                println!("Look, the duck is flying")
            }
        }
        struct Swan;
        impl Swan {
            fn fly(&self) {
                println!("Look, the duck.. oh sorry, the swan is flying")
            }
        }

        impl Bird for Duck {
            fn quack(&self) {
                println!("{}", "duck duck");
            }
        }

        impl Bird for Swan {
            fn quack(&self) {
                println!("{}", "swan swan");
            }
        }

        enum Birds {
            Duck(Duck),
            Swan(Swan),
        }

        pub fn main() {
            // FILL in the blank to make the code work.
            // v1
            let birds: [&dyn Bird; 2] = [&Duck, &Swan];
            // v2
            // let birds: Vec<Box<dyn Bird>> = vec![Box::new(Duck {}), Box::new(Swan {})];
            // v3
            // let birds = [Birds::Duck(Duck {}), Birds::Swan(Swan {})];

            for bird in birds {
                // if let Birds::Duck(bird) = bird {
                // 	bird.quack();
                // }
                // else if let Birds::Swan(bird) = bird {
                // 	bird.quack();
                // }
                bird.quack();
                // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
                // So, the code below will cause an error.
                // bird.fly();
            }
        }
    }
}
