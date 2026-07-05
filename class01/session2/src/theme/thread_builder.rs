// IKinder

#![allow(unused_qualifications)]

use std::thread;

pub fn main() {
    crate::show_name(file!());

    thread::Builder::new()
        .name("Named thread".to_owned())
        .stack_size(std::mem::size_of::<usize>() * 4)
        .spawn(my_thread)
        .unwrap()
        .join()
        .unwrap();
}

fn my_thread() {
    println!(
        "Hello from a thread named {}",
        thread::current().name().unwrap()
    );
}
