// IKinder

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub fn main() {
    crate::show_name(file!());

    reference_counting();
    atomic_reference_counting();
    external_mutability();
    internal_mutability();
    cell_mutability();
}

#[derive(Debug)]
struct Droppable(i32);
impl Droppable {
    fn new(n: i32) -> Self {
        println!("Construction {n}");
        Self(n)
    }
}
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn move_me<T: std::fmt::Debug>(x: T) {
    println!("Moved {:?}", x);
}

fn reference_counting() {
    println!("-----------------------");
    let my_shared = Rc::new(Droppable::new(1));
    {
        let _x = my_shared.clone();
        let _y = my_shared.clone();
        let _z = my_shared.clone();
    }
    move_me(my_shared.clone());
    println!("{my_shared:?}");
    println!("Reference_counting exit");
}

fn atomic_reference_counting() {
    println!("-----------------------");
    let my_shared = Arc::new(Droppable::new(1));
    {
        let _x = my_shared.clone();
        let _y = my_shared.clone();
        let _z = my_shared.clone();
    }
    move_me(my_shared.clone());
    let mut threads = Vec::new();
    for _ in 0..10 {
        let my_clone = my_shared.clone();
        threads.push(std::thread::spawn(move || {
            move_me(my_clone);
        }));
    }
    threads.into_iter().for_each(|t| t.join().unwrap());
    println!("{my_shared:?}");
    println!("Atomic_reference_counting exit");
}

struct Sharable(String);

fn external_mutability() {
    println!("-----------------------");
    let my_shared = Arc::new(Mutex::new(Sharable("Hello".into())));
    let mut threads = Vec::new();
    for i in 0..10 {
        let my_clone = my_shared.clone();
        threads.push(std::thread::spawn(move || {
            let mut data = my_clone.lock().unwrap();
            data.0.push_str(&format!(" {i}"));
        }));
    }
    threads.into_iter().for_each(|t| t.join().unwrap());
    println!("{}", my_shared.lock().unwrap().0);
}

struct SharedData {
    data: Mutex<String>,
}
impl SharedData {
    fn new(s: &str) -> Self {
        Self {
            data: Mutex::new(s.into()),
        }
    }
}

fn internal_mutability() {
    println!("-----------------------");
    let my_shared = Arc::new(SharedData::new("Hello"));
    let mut threads = Vec::new();
    for i in 0..10 {
        let my_clone = my_shared.clone();
        threads.push(std::thread::spawn(move || {
            let mut data = my_clone.data.lock().unwrap();
            data.push_str(&format!(" {i}"));
        }));
    }
    threads.into_iter().for_each(|t| t.join().unwrap());
    println!("{}", my_shared.data.lock().unwrap());
}

struct MyData {
    data: RefCell<String>,
}
impl MyData {
    fn new() -> Self {
        Self {
            data: RefCell::new("Hello".into()),
        }
    }
}

fn move_data(d: Rc<MyData>) {
    let mut data = d.data.borrow_mut();
    data.push_str(" World");
}

fn cell_mutability() {
    println!("-----------------------");
    let shared_data = Rc::new(MyData::new());
    move_data(shared_data.clone());
    println!("{}", shared_data.data.borrow());
}
