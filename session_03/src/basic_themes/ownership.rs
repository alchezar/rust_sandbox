// IKinder

enum Color {
	Brown,
	Red,
}

impl Color {
	fn print(&self) {
		match self {
			Color::Brown => println!("color: Brown"),
			Color::Red => println!("color: Red"),
		}
	}
}

struct Dimensions {
	width: u32,
	height: u32,
	depth: u32,
}

impl Dimensions {
	fn new(width: u32, height: u32, depth: u32) -> Self {
		Self { width, height, depth }
	}
	fn print(&self) {
		println!("width: {}, height: {}, depth: {}", self.width, self.height, self.depth);
	}
}

struct ShippingBox {
	dimension: Dimensions,
	color: Color,
	weight: u32,
}

impl ShippingBox {
	pub fn new(dimension: Dimensions, color: Color, weight: u32) -> Self {
		Self { dimension, color, weight }
	}
	fn print(&self) {
		self.dimension.print();
		self.color.print();
		println!("weight: {:?}", self.weight);
	}
}

pub fn run() {
	let small_dimensions = Dimensions::new(1, 2, 3);
	let small_box = ShippingBox::new(small_dimensions, Color::Red, 10);
	small_box.print();
}
