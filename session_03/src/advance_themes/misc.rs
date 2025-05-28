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

// -----------------------------------------------------------------------------
// 169. Serde crate.
// -----------------------------------------------------------------------------

mod topic_169 {
	use serde::{Deserialize, Serialize};

	#[derive(Serialize, Deserialize, Debug)]
	struct Form {
		email: String,
		name: String,
		age: usize,
	}

	pub fn main() {
		let form = Form {
			email: "kinder.wit@gmail.com".to_owned(),
			name: "Ivan".to_owned(),
			age: 32,
		};
		let serialized = serde_json::to_string(&form).expect("failed to serialize");
		println!("{}", serialized);

		let deserialized = serde_json::from_str::<Form>(&serialized).expect("failed to deserialize");
		println!("{:?}", deserialized)
	}
}

// -----------------------------------------------------------------------------
// 170. Rand crate.
// -----------------------------------------------------------------------------

mod topic_170 {
	use rand::Rng;
	use rand::seq::{IteratorRandom, SliceRandom};

	pub fn main() {
		let number = rand::random::<u8>();
		let yes_no = rand::random::<bool>();

		let mut rng = rand::thread_rng();
		let number = rng.gen_range(0..20);

		let letters = ['a', 'b', 'c'];
		let letter = letters.iter().choose(&mut rng);

		let mut letters = letters;
		letters.shuffle(&mut rng)
	}
}

// -----------------------------------------------------------------------------
// 173. Chrono crate.
// -----------------------------------------------------------------------------

pub(crate) mod topic_173 {
	use chrono::Duration;
	use chrono::prelude::*;

	pub fn main() {
		// EST is UTC+2
		let est = FixedOffset::east_opt(2 * 3600)
			.expect("invalid UTC offset")
			.with_ymd_and_hms(2025, 04, 17, 16, 48, 34)
			.single()
			.expect("not single unique result");
		println!("{}", est);
		let utc = est.with_timezone(&Utc);
		let utc: DateTime<Utc> = est.into();
		let utc = DateTime::<Utc>::from(est);

		let now = Utc::now();
		let an_hour = Duration::hours(1);
		let when = now + an_hour;
		let duration = when - now;
		println!("{}", duration.num_minutes());

		let date_str = "2025-04-17 16:48:34 +02:00";
		let fmt_str = "%Y-%m-%d %H:%M:%S %z";
		let date = DateTime::parse_from_str(date_str, fmt_str).expect("invalid date");

		let date = Local::now();
		let date_str = date
			.format("%Y-%m-%d %H:%M:%S")
			.to_string();
		println!("{}", date_str);
	}
}
