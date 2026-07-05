// IKinder

#![allow(
    unused_qualifications,
    clippy::module_inception,
    clippy::useless_conversion,
    clippy::useless_vec
)]

pub mod _1 {
    pub mod _1 {
        /* Make it work with the least number of changes. */
        pub fn main() {
            let color = String::from("green");

            let print = || println!("`color`: {}", color);

            print();
            print();

            // `color` can be borrowed immutably again, because the closure only holds
            // an immutable reference to `color`.
            let _reborrow = &color;

            println!("{}", color);
        }
    }
    pub mod _2 {
        /* Make it work
        - Dont use `_reborrow` and `_count_reborrowed`
        - Dont modify `assert_eq`
        */
        pub fn main() {
            let mut count = 0;

            let mut inc = move || {
                count += 1;
                println!("`count`: {}", count);
            };

            inc();

            let _reborrow = &count;

            inc();

            // The closure no longer needs to borrow `&mut count`. Therefore, it is
            // possible to reborrow without an error
            let _count_reborrowed = &mut count;

            assert_eq!(count, 0);
        }
    }
    pub mod _3 {
        /* Make it work in two ways; none of them is to remove `take(movable)` away from the code */
        pub fn main() {
            let movable = Box::new(3);

            let consume = || {
                println!("`movable`: {:?}", movable);
                take(&movable);
            };

            consume();
            consume();
        }

        fn take<T>(_v: T) {}
    }
    pub mod _4 {
        pub fn main() {
            let example_closure = |x| x;

            let s = example_closure(String::from("hello"));

            /* Make it work, only change the following line */
            let n = example_closure(5.to_string());
        }
    }
    pub mod _5 {
        /* Make it work by changing the trait bound, in two ways*/
        fn fn_once<F>(func: F)
        where
            F: Fn(usize) -> bool,
        {
            println!("{}", func(3));
            println!("{}", func(4));
        }

        pub fn main() {
            let x = vec![1, 2, 3];
            fn_once(|z| z == x.len())
        }
    }
    pub mod _6 {
        pub fn main() {
            let mut s = String::new();
            let update_string = |str| s.push_str(str);
            exec(update_string);
            println!("{:?}", s);
        }

        /* Fill in the blank */
        fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
            f("hello")
        }
    }
    pub mod _7 {
        /* Fill in the blank */

        // A function which takes a closure as an argument and calls it.
        // <F> denotes that F is a "Generic type parameter"
        fn apply<F>(f: F)
        // The closure takes no input and returns nothing.
        where
            F: FnOnce(),
        {
            f();
        }

        // A function which takes a closure and returns an `i32`.
        fn apply_to_3<F>(f: F) -> i32
        // The closure takes an `i32` and returns an `i32`.
        where
            F: Fn(i32) -> i32,
        {
            f(3)
        }

        pub fn main() {
            use std::mem;

            let greeting = "hello";
            // A non-copy type.
            // `to_owned` creates owned data from borrowed one
            let mut farewell = "goodbye".to_owned();

            // Capture 2 variables: `greeting` by reference and
            // `farewell` by value.
            let diary = || {
                // `greeting` is by reference: requires `Fn`.
                println!("I said {}.", greeting);

                // Mutation forces `farewell` to be captured by
                // mutable reference. Now requires `FnMut`.
                farewell.push_str("!!!");
                println!("Then I screamed {}.", farewell);
                println!("Now I can sleep. zzzzz");

                // Manually calling drop forces `farewell` to
                // be captured by value. Now requires `FnOnce`.
                mem::drop(farewell);
            };

            // Call the function which applies the closure.
            apply(diary);

            // `double` satisfies `apply_to_3`'s trait bound
            let double = |x| 2 * x;

            println!("3 doubled: {}", apply_to_3(double));
        }
    }
    pub mod _8 {
        /* Fill in the blank */
        pub fn main() {
            let mut s = String::new();
            let update_string = |str| -> String {
                s.push_str(str);
                s
            };
            exec(update_string);
        }

        fn exec<'a, F: FnOnce(&'a str) -> String>(mut f: F) {
            let _ = f("hello");
        }
    }
    pub mod _9 {
        /* Implement `call_me` to make it work */
        fn call_me<F: Fn()>(f: F) {
            f();
        }

        fn function() {
            println!("I'm a function!");
        }

        pub fn main() {
            let closure = || println!("I'm a closure!");

            call_me(closure);
            call_me(function);
        }
    }
    pub mod _10 {
        /* Fill in the blank using two approaches, and fix the error */
        fn create_fn() -> impl Fn(i32) -> i32 {
            let num = 5;

            // How does the following closure capture the environment variable `num`
            // &T, &mut T, T?
            move |x| x + num
        }
        fn create_fn_1() -> Box<dyn Fn(i32) -> i32> {
            let num = 5;

            // How does the following closure capture the environment variable `num`
            // &T, &mut T, T?
            Box::new(move |x| x + num)
        }

        pub fn main() {
            let fn_plain = create_fn();
            fn_plain(1);
        }
    }
    pub mod _11 {
        /* Fill in the blank and fix the error*/
        fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
            let num = 5;

            if x > 1 {
                Box::new(move |x| x + num)
            } else {
                Box::new(move |x| x + num)
            }
        }
        pub fn main() {}
    }
}
pub mod _2 {
    pub mod _1 {
        /* Refactoring the following code using iterators */
        pub fn main() {
            let arr = [0; 10];

            // for i in 0..arr.len() {
            // 	println!("{}", arr[i]);
            // }

            arr.iter().for_each(|x| println!("{}", x));
        }
    }
    pub mod _2 {
        /* Fill in the blank */
        pub fn main() {
            let mut v = Vec::new();
            for n in 0..100 {
                v.push(n);
            }
            assert_eq!(v.len(), 100);

            v.clear();
            (0..100).into_iter().for_each(|x| v.push(x));

            assert_eq!(v.len(), 100);
        }
    }
    pub mod _3 {
        /* Fill the blanks and fix the errors. Using two ways if possible */
        pub fn main() {
            let mut v1 = vec![1, 2].into_iter();

            assert_eq!(v1.next(), Some(1));
            assert_eq!(v1.next(), Some(2));
            assert_eq!(v1.next(), None);
        }
    }
    pub mod _4 {
        /* Make it work */
        pub fn main() {
            let arr = vec![0; 10];
            for i in arr.iter() {
                println!("{}", i);
            }

            println!("{:?}", arr);
        }
    }
    pub mod _5 {
        /* Fill in the blank */
        pub fn main() {
            let mut names = vec!["Bob", "Frank", "Ferris"];

            for name in names.iter_mut() {
                *name = match name {
                    &mut "Ferris" => "There is a rustacean among us!",
                    _ => "Hello",
                }
            }

            names.iter().for_each(|name| println!("name: {:?}", name));

            println!("names: {:?}", names);
        }
    }
    pub mod _6 {
        /* Fill in the blank */
        pub fn main() {
            let mut values = vec![1, 2, 3];
            let mut values_iter = values.iter_mut();

            if let Some(v) = values_iter.next() {
                *v = 0;
            }

            assert_eq!(values, vec![0, 2, 3]);
        }
    }
    pub mod _7 {
        struct Fibonacci {
            curr: u32,
            next: u32,
        }

        // Implement `Iterator` for `Fibonacci`.
        // The `Iterator` trait only requires a method to be defined for the `next` element.
        impl Iterator for Fibonacci {
            // We can refer to this type using Self::Item
            type Item = u32;

            /* Implement next method */
            fn next(&mut self) -> Option<Self::Item> {
                let temp = self.curr;
                self.curr = self.next;
                self.next += temp;
                Some(self.curr)
            }
        }

        // Returns a Fibonacci sequence generator
        fn fibonacci() -> Fibonacci {
            Fibonacci { curr: 0, next: 1 }
        }

        pub fn main() {
            let mut fib = fibonacci();
            assert_eq!(fib.next(), Some(1));
            assert_eq!(fib.next(), Some(1));
            assert_eq!(fib.next(), Some(2));
            assert_eq!(fib.next(), Some(3));
            assert_eq!(fib.next(), Some(5));
        }
    }
    pub mod _8 {

        /* Fill in the blank and fix the errors */
        pub fn main() {
            let v1 = vec![1, 2, 3];
            let v1_iter = v1.iter();

            // The sum method will take the ownership of the iterator and iterates through the items by repeatedly calling next method
            let total = v1_iter.clone().sum::<i32>();

            assert_eq!(total, 6);

            println!("{:?}, {:?}", v1, v1_iter);
        }
    }
    pub mod _9 {
        /* Make it work */
        use std::collections::HashMap;
        pub fn main() {
            let names = [("sunface", 18), ("sunfei", 18)];
            let folks: HashMap<_, _> = names.into_iter().collect();
            println!("{:?}", folks);

            let v1: Vec<i32> = vec![1, 2, 3];
            let v2 = v1.into_iter().collect::<Vec<_>>();
            assert_eq!(v2, vec![1, 2, 3]);
        }
    }
    pub mod _10 {
        /* Fill in the blanks */
        pub fn main() {
            let v1: Vec<i32> = vec![1, 2, 3];
            let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
            assert_eq!(v2, vec![2, 3, 4]);
        }
    }
    pub mod _11 {
        /* Fill in the blanks */
        use std::collections::HashMap;
        pub fn main() {
            let names = ["sunface", "sunfei"];
            let ages = [18, 18];
            let folks = names
                .into_iter()
                .zip(ages.into_iter())
                .collect::<HashMap<_, _>>();

            println!("{:?}", folks);
        }
    }
    pub mod _12 {
        /* Fill in the blanks */
        #[derive(PartialEq, Debug)]
        struct Shoe {
            size: u32,
            style: String,
        }

        fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
            shoes.into_iter().filter(|s| s.size == shoe_size).collect()
        }

        pub fn main() {
            let shoes = vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 13,
                    style: String::from("sandal"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ];

            let in_my_size = shoes_in_size(shoes, 10);

            assert_eq!(
                in_my_size,
                vec![
                    Shoe {
                        size: 10,
                        style: String::from("sneaker")
                    },
                    Shoe {
                        size: 10,
                        style: String::from("boot")
                    },
                ]
            );
        }
    }
}
