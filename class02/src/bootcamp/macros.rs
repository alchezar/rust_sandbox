// IKinder

#![allow(dead_code, unused_variables, unused_imports, unused_macros)]

use procedural_macros::*;
use std::collections::HashMap;

#[macro_export]
macro_rules! hello {
	() => {
		println!("Hello World!");
	};
}

#[macro_export]
macro_rules! map {
    ($key:ty, $val:ty) => {
	    {
			let map: HashMap<$key, $val> = HashMap::new();
			map
	    }
    };

    ($($key:expr => $val:expr),*) => {
	    {
			let mut map = HashMap::new();
			$(map.insert($key, $val);)*
			map
	    }
    };
}

trait Log {
	fn info(&self, msg: &str);
	fn warn(&self, msg: &str);
	fn error(&self, msg: &str);
}

#[derive(Debug, Log)]
struct Database {
	url: String,
	connections: u32,
}

impl Database {
	fn new(url: String) ->Database {
		Database{ url, connections: 0 }
	}
	fn connect(&mut self) {
		self.info(format!("new connection to {}", self.url).as_str());
		self.connections += 1;
		if self.connections >= 3 {
			self.warn("3 or more connections open!")
			// a lot of connections!
		}
	}
}

#[derive(Debug)]
struct Product {
	name: String,
	price: u32
}

pub fn run() {
	println!("--- Declarative macros ---");

	hello!();

	let scores: HashMap<String, i32> = HashMap::new();
	let mut scores2 = HashMap::new();
	scores2.insert("Red team".to_owned(), 3);
	scores2.insert("Blue team".to_owned(), 5);
	scores2.insert("Green team".to_owned(), 2);

	let scores3 = map!(
		"Red one" => 6,
		"Blue one" => 5,
		"Green one" => 4
	);

	println!("--- Procedural macros - function like ---");
	log_info!([TIME] starting program...);
	println!("--- Procedural macros - custom derive ---");
	let mut db = Database::new("localhost:5433".to_owned());
	for _ in 0..3 {
		db.connect();
	}
	println!("--- Procedural macros - attribute like ---");
	let laptop = Product {name: "MacBook Pro".to_owned(), price: 2000};
	buy_product(laptop, 20);
}

#[log_call]
fn buy_product(product: Product, discount: u32)
{
	
}