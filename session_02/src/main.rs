// IKinder

use rand::prelude::*;

mod bootcamp;

fn main() {
	bootcamp::option_result::run();

	let timeout: i32 = rand::rng().random_range(100..500);
	
	println!("Timeout is {}", timeout);
}
