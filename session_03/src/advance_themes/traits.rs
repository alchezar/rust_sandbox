// IKinder

trait Fall {
	fn hit_ground(&self);
}

struct Vase;
impl Fall for Vase {
	fn hit_ground(&self) {
		println!("the vase broke!")
	}
}

struct Cat;
impl Fall for Cat {
	fn hit_ground(&self) {
		println!("the cat casually walked away!")
	}
}

fn fall(thing: impl Fall) {
	thing.hit_ground();
}

pub fn run() {
	fall(Vase {});
	fall(Cat {})
}

// -----------------------------------------------------------------------------
// Trait Activity
// -----------------------------------------------------------------------------

trait Perimeter {
	fn get_perimeter(&self) -> f32;
}

#[derive(Debug)]
struct Triangle {
	a: f32,
	b: f32,
	c: f32,
}

impl Triangle {
	fn new(a: f32, b: f32, c: f32) -> Self {
		Self { a, b, c }
	}
}

impl Perimeter for Triangle {
	fn get_perimeter(&self) -> f32 {
		self.a + self.b + self.c
	}
}

#[derive(Debug)]
struct Square {
	a: f32,
}

impl Square {
	fn new(a: f32) -> Self {
		Self { a }
	}
}

impl Perimeter for Square {
	fn get_perimeter(&self) -> f32 {
		self.a * 4.0
	}
}

use core::fmt::Debug;
fn print_perimeter(shape: impl Perimeter + Debug) {
	println!("{:?} perimeter: {}", shape, shape.get_perimeter());
}

pub fn activity() {
	print_perimeter(Square::new(3.0));
	print_perimeter(Triangle::new(3.0, 4.0, 5.0))
}

// -----------------------------------------------------------------------------
// Default Trait
// -----------------------------------------------------------------------------

struct Package {
	weight: f32,
}

impl Package {
	fn new(weight: f32) -> Self {
		Self { weight }
	}
}

impl Default for Package {
	fn default() -> Self {
		Self { weight: 3.0 }
	}
}

pub fn run2() {
	let package = Package::default();
}
