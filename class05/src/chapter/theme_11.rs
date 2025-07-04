// IKinder

pub mod _2 {
	pub mod _1 {
		pub fn main() {
			let arr: [u8; 3] = [1, 2, 3];

			let v = Vec::from(arr);
			is_vec(&v);

			let v = vec![1, 2, 3];
			is_vec(&v);

			// vec!(..) and vec![..] are same macros, so
			let v = vec![1, 2, 3];
			is_vec(&v);

			// In code below, v is Vec<[u8; 3]>, not Vec<u8>
			// USE Vec::new and `for` to rewrite the below code
			let mut v1 = Vec::new();
			is_vec(&v1);
			arr.iter().for_each(|e| v1.push(*e));

			assert_eq!(v, v1);
			println!("Success!");
		}

		fn is_vec(_v: &Vec<u8>) {}
	}
	pub mod _2 {
		pub fn main() {
			let mut v1 = Vec::from([1, 2, 4]);
			v1.pop();
			v1.push(3);

			let mut v2 = Vec::new();
			v2.extend(&v1);

			assert_eq!(v1, v2);
			println!("Success!");
		}
	}
	pub mod _3 {
		// FILL in the blanks
		pub fn main() {
			// Array -> Vec
			// impl From<[T; N]> for Vec
			let arr = [1, 2, 3];
			let v1 = Vec::from(arr);
			let v2: Vec<_> = arr.into();

			assert_eq!(v1, v2);

			// String -> Vec
			// impl From<String> for Vec
			let s = "hello".to_string();
			let v1: Vec<u8> = s.into();

			let s = "hello".to_string();
			let v2 = s.into_bytes();
			assert_eq!(v1, v2);

			// impl<'_> From<&'_ str> for Vec
			let s = "hello";
			let v3 = Vec::from(s);
			assert_eq!(v2, v3);

			// Iterators can be collected into vectors
			let v4 = [0; 10].into_iter().collect::<Vec<_>>();
			assert_eq!(v4, vec![0; 10]);

			println!("Success!");
		}
	}
	pub mod _4 {
		// FIX the error and IMPLEMENT the code
		pub fn main() {
			let mut v = Vec::from([1, 2, 3]);
			for i in 0..5 {
				println!("{:?}", v.get(i))
			}

			for i in 0..5 {
				match v.get(i) {
					Some(e) => v[i] = e + 1,
					None => v.push(i + 2),
				}
			}

			assert_eq!(v, vec![2, 3, 4, 5, 6]);
			println!("Success!");
		}
	}
	pub mod _5 {
		// FIX the errors
		pub fn main() {
			let mut v = vec![1, 2, 3];

			let slice1 = &v[..];
			// Out of bounds will cause a panic
			// You must use `v.len` here
			let slice2 = &v[0..v.len()];
			assert_eq!(slice1, slice2);

			// A slice can also be mutable, in which
			// case mutating it will mutate its underlying Vec.
			// Note: slice and &Vec are different
			let vec_ref = &mut v;
			(*vec_ref).push(4);
			let slice3 = &mut v[0..=3];
			// slice3[3] = 42;
			match slice3.get_mut(3) {
				Some(val) => *val = 42,
				None => (),
			}
			assert_eq!(slice3, &[1, 2, 3, 42]);
			assert_eq!(v, &[1, 2, 3, 42]);

			println!("Success!");
		}
	}
	pub mod _6 {
		// FIX the errors
		pub fn main() {
			let mut vec = Vec::with_capacity(10);

			// The vector contains no items, even though it has capacity for more
			assert_eq!(vec.len(), 0);
			assert_eq!(vec.capacity(), 10);

			// These are all done without reallocating...
			for i in 0..10 {
				vec.push(i);
			}
			assert_eq!(vec.len(), 10);
			assert_eq!(vec.capacity(), 10);

			// ...but this may make the vector reallocate
			vec.push(11);
			assert_eq!(vec.len(), 11);
			assert!(vec.capacity() >= 11);

			// Fill in an appropriate value to make the `for` done without reallocating
			let mut vec = Vec::with_capacity(100);
			for i in 0..100 {
				vec.push(i);
			}

			assert_eq!(vec.len(), 100);
			assert_eq!(vec.capacity(), 100);

			println!("Success!");
		}
	}
	pub mod _7 {
		#[derive(Debug, Eq, PartialEq)]
		enum IpAddr {
			V4(String),
			V6(String),
		}
		pub fn main() {
			// FILL in the blank
			let v: Vec<IpAddr> = vec![IpAddr::V4("127.0.0.1".into()), IpAddr::V6("::1".into())];

			// Comparing two enum needs to derive the PartialEq trait
			assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
			assert_eq!(v[1], IpAddr::V6("::1".to_string()));

			println!("Success!");
		}
	}
	pub mod _8 {
		trait IpAddr {
			fn display(&self);
		}

		struct V4(String);
		impl IpAddr for V4 {
			fn display(&self) {
				println!("ipv4: {:?}", self.0)
			}
		}
		struct V6(String);
		impl IpAddr for V6 {
			fn display(&self) {
				println!("ipv6: {:?}", self.0)
			}
		}

		pub fn main() {
			// FILL in the blank
			let v: Vec<Box<dyn IpAddr>> = vec![Box::new(V4("127.0.0.1".to_string())), Box::new(V6("::1".to_string()))];

			for ip in v {
				ip.display();
			}
		}
	}
}
pub mod _3 {
	pub mod _1 {
		// FILL in the blanks and FIX the errors
		use std::collections::HashMap;
		pub fn main() {
			let mut scores = HashMap::new();
			scores.insert("Sunface", 98);
			scores.insert("Daniel", 95);
			scores.insert("Ashley", 69);
			scores.insert("Katie", 58);

			// Get returns an Option<&V>
			let score = scores.get("Sunface");
			assert_eq!(score, Some(&98));

			if scores.contains_key("Daniel") {
				// Indexing returns a value V
				let score = scores["Daniel"];
				assert_eq!(score, 95);
				scores.remove("Daniel");
			}

			assert_eq!(scores.len(), 3);

			for (name, score) in scores {
				println!("The score of {} is {}", name, score);
			}
		}
	}
	pub mod _2 {
		use std::collections::HashMap;
		use std::hash::Hash;

		pub fn main() {
			let teams = [("Chinese Team", 100), ("American Team", 10), ("France Team", 50)];

			let mut teams_map1 = HashMap::new();
			for team in &teams {
				teams_map1.insert(team.0, team.1);
			}

			// IMPLEMENT team_map2 in two ways
			// Tips: one of the approaches is to use `collect` method
			let teams_map2 = teams
				.iter()
				.map(|&x| x)
				.collect::<HashMap<_, _>>();
			let teams_map2 = teams.into();

			assert_eq!(teams_map1, teams_map2);
			println!("Success!");
		}
	}
	pub mod _3 {
		// FILL in the blanks
		use std::collections::HashMap;
		pub fn main() {
			// Type inference lets us omit an explicit type signature (which
			// would be `HashMap<&str, u8>` in this example).
			let mut player_stats = HashMap::new();

			// Insert a key only if it doesn't already exist
			player_stats
				.entry("health")
				.or_insert(100);

			assert_eq!(player_stats["health"], 100);

			// Insert a key using a function that provides a new value only if it
			// doesn't already exist
			player_stats
				.entry("health")
				.or_insert_with(random_stat_buff);
			assert_eq!(player_stats["health"], 100);

			// Ensures a value is in the entry by inserting the default if empty, and returns
			// a mutable reference to the value in the entry.
			let health = player_stats
				.entry("health")
				.or_insert(50);
			assert_eq!(health, &100);
			*health -= 50;
			assert_eq!(*health, 50);

			println!("Success!");
		}

		fn random_stat_buff() -> u8 {
			// Could actually return some random value here - let's just return
			// some fixed value for now
			42
		}
	}
	pub mod _4 {
		// FIX the error
		// Tips: `derive` is usually a good way to implement some common used traits
		use std::collections::HashMap;

		#[derive(Debug, PartialEq, Eq, Hash)]
		struct Viking {
			name: String,
			country: String,
		}

		impl Viking {
			/// Creates a new Viking.
			fn new(name: &str, country: &str) -> Viking {
				Viking {
					name: name.to_string(),
					country: country.to_string(),
				}
			}
		}

		pub fn main() {
			// Use a HashMap to store the vikings' health points.
			let vikings = HashMap::from([
				(Viking::new("Einar", "Norway"), 25),
				(Viking::new("Olaf", "Denmark"), 24),
				(Viking::new("Harald", "Iceland"), 12),
			]);

			// Use derived implementation to print the status of the vikings.
			for (viking, health) in &vikings {
				println!("{:?} has {} hp", viking, health);
			}
		}
	}
	pub mod _5 {
		// FIX the errors with the least changes
		// DON'T remove any code line
		use std::collections::HashMap;
		pub fn main() {
			let v1 = 10;
			let mut m1 = HashMap::new();
			m1.insert(v1, v1);
			println!("v1 is still usable after inserting to hashmap : {}", v1);

			let v2 = "hello".to_string();
			let mut m2 = HashMap::new();
			// Ownership moved here
			m2.insert(&v2, v1);

			assert_eq!(v2, "hello");
			println!("Success!");
		}
	}
}
