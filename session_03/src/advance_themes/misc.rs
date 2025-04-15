// IKinder

// -----------------------------------------------------------------------------
// 160. Turbofish.
// -----------------------------------------------------------------------------

fn turbofish() {
	let numbers = vec![1, 2, 3];
	let odds = numbers
		.iter()
		.filter(|n| **n % 2 == 1)
		.collect::<Vec<_>>();
}

// -----------------------------------------------------------------------------
// 161. Loop labels.
// -----------------------------------------------------------------------------

fn loop_labels() {
	// Syntax
	// 'name: loop {}

	let matrix = [[2, 4, 6], [8, 9, 10], [12, 14, 16]];

	'rows: for row in matrix.iter() {
		'cols: for col in row.iter() {
			if col % 2 == 0 {
				continue 'cols;
			}
			println!("Odd: {}", col);
			break 'rows;
		}
	}

	type UserInput<'a> = Result<&'a str, String>;
	'menu: loop {
		println!("Menu");
		'input: loop {
			let user_input: UserInput = Ok("next");
			match user_input {
				Ok(input) => break 'menu,
				Err(_) => {
					println!("try again");
					continue 'input;
				}
			}
		}
	}
}

// -----------------------------------------------------------------------------
// 162. Loop expressions.
// -----------------------------------------------------------------------------

fn loop_expressions() {
	let value: usize = 5;
	let result = 'ident: loop {
		match value {
			v if v % 2 == 0 => break 0,
			_ => break 'ident 1,
		}
	};

	let nums = vec![1, 2, 3, 4, 5, 6, 7, 8];
	let div_by_three = 'outer: loop {
		for n in &nums {
			if n % &3 == 0 {
				break 'outer Some(*n);
			}
		}
		break None;
	};
}

// -----------------------------------------------------------------------------
// 163. Struct update syntax.
// -----------------------------------------------------------------------------

mod topic_163 {
	struct Particle {
		color: (u8, u8, u8),
		alpha: u8,
		size: (u32, u32),
		position: (i32, i32),
		velocity: i32,
		direction: f32,
	}

	impl Default for Particle {
		fn default() -> Self {
			Self {
				color: (255, 0, 255),
				alpha: 255,
				size: (100, 100),
				position: (0, 0),
				velocity: 0,
				direction: 0.0,
			}
		}
	}

	pub fn main() {
		// Base approach.
		let mut particle = Particle::default();
		particle.alpha = 127;
		let particle = particle;

		// Struct update approach.
		let particle = Particle { alpha: 127, ..Particle::default() };
		let red_particle = Particle {
			color: (255, 0, 0),
			..Particle::default()
		};
		let fast_particle = Particle {
			velocity: 10,
			direction: 0.5,
			..red_particle
		};
	}
}

// -----------------------------------------------------------------------------
// 164. Escape sequences & raw strings.
// -----------------------------------------------------------------------------

mod topic_164 {
	pub fn main() {
		// Escape sequences.
		let msg = "Hello\nworld!";
		let msg = "Hello\tworld";
		let msg = "left\\right";
		let msg = "over\"there\"";
		let smiley = "\u{1f642}";

		// Row string.
		let msg = r"Hello
		world!";
		let msg = r"Hello	world";
		let msg = r"left\right";
		let msg = r#"Over"there""#;
		let msg = r##"Over #"#there#"#"##;
		let smiley = r"😅";
	}
}
