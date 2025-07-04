// IKinder

mod basket;
mod container;
mod stack;

use basket::Basket;
use container::Container;
use stack::Stack;

pub fn run() {
	let mut b1 = Basket::new("hi there".to_owned());
	let b2 = Basket::new(10);
	let b3 = Basket::new(true);

	let mut s1 = Stack::new(vec!["hi".to_owned(), "there".to_owned()]);
	let s2 = Stack::new(vec![1, 2, 3]);

	add_string(&mut b1, "John".to_owned());
	add_string(&mut s1, "Ivan".to_owned());
}

fn add_string<T>(container: &mut T, string: String)
where
	T: Container<String>,
{
	container.put(string)
}
