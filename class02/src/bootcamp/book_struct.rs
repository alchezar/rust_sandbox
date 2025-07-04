// IKinder

#![allow(dead_code, unused_variables)]

struct Product {
	name: String,
	category: ProductCategory,
	price: f32,
	in_stock: bool,
}

enum ProductCategory{
	Books,
	Cloth,
	Electronic
}

impl Product {
	fn new(name: String, category: ProductCategory, price: f32) -> Product {
		Self { name, category, price, in_stock: true }
	}

	fn get_default_sales_tax() -> f32 {
		0.1
	}

	fn calculate_sales_tax(&self) -> f32 {
		self.price * Product::get_default_sales_tax()
	}

	fn set_price(&mut self, price: f32) {
		self.price = price;
	}

	fn buy(self) -> i32 {
		println!("{} was bought", self.name);
		123
	}
}

pub fn run() {
	let mut book = Product::new(String::from("Potter"), ProductCategory::Books, 28.85);
	let _price = book.price;
	book.in_stock = false;

	println!("Sales tax for {} is {}", book.name, book.calculate_sales_tax());
	book.set_price(50.0);
	println!("Sales tax for {} is {}", book.name, book.calculate_sales_tax());
	book.buy();
}
