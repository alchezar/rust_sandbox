// IKinder

#![allow(
    dead_code,
    unused_variables,
    clippy::unnecessary_mut_passed,
    clippy::needless_borrow
)]

use std::ops::{Deref, DerefMut};

struct MySmartPointer<T> {
    value: T,
}

impl<T> MySmartPointer<T> {
    fn new(value: T) -> MySmartPointer<T> {
        MySmartPointer { value }
    }
}

impl<T> Deref for MySmartPointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.value
    }
}

impl<T> DerefMut for MySmartPointer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

pub fn run() {
    let s = MySmartPointer::new(Box::new("lets get rusty".to_string()));
    // &MySmartPointer -> &Box -> &String -> &str
    let mut r = &(s.deref().deref().deref());
    let t = &(**s);
    print(&mut r)
}

fn print(s: &str) {
    println!("{s}")
}
