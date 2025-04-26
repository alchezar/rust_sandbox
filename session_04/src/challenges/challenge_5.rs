// IKinder

/// # Easy
/// Write a function `singleChar` which gets a string `s`, every
/// character in `s` appears twice except for one, return from the
/// function the character which **appears only ones**!
/// # Example:
/// Input - `"aabdb"`\
/// Expected Output - `"d"`\
/// Explanation - `"a"` and `"b"` appear twice and `"d"` appears only ones.
///
/// ```
/// assert_eq!(single_char("aabdb".to_owned()), "d".to_owned());
/// ```
fn single_char(str: String) -> String {
	use std::collections::HashMap;

	let mut map = HashMap::<char, i32>::new();
	for char in str.chars() {
		*map.entry(char).or_insert(0) += 1;
	}
	for (&char, &count) in map.iter() {
		if count == 1 {
			return char.to_string();
		}
	}
	String::new()
}

/// # Medium
/// Write a function isConstructable which gets two strings, `s1`
/// and `s2`, and returns `true` if `s1` can be constructed using the
/// letters from `s2` or `false` otherwise.
/// # Example:
/// Input - `"aab"`, `"bcaa"`\
/// Expected Output - `true`\
/// Explanation - `"bcaa"` contains all the letters from `"aab"`
///
/// ```
/// assert_eq(is_constructable("aab".to_owned(), "bcaa".to_owned()), true);
/// ```
fn is_constructable(str1: String, str2: String) -> bool {
	use std::collections::HashMap;

	let map1 = str1
		.chars()
		.fold(HashMap::new(), |mut map, char| {
			*map.entry(char).or_insert(0) += 1;
			map
		});
	let map2 = str2
		.chars()
		.fold(HashMap::new(), |mut map, char| {
			*map.entry(char).or_insert(0) += 1;
			map
		});

	map1.iter().all(|(char, count)| {
		map2.get(char)
			.map_or(false, |count2| count2 >= count)
	})
}

/// # Hard
/// Write a function `countPrimes` that gets an integer `n`, and
/// returns the **number of prime numbers** that are less than `n`.
///
/// > *Note: Time complexity must be better than **O(nlogn)***
///
/// `1` <= `n` <= `1_000_000`
///
/// # Examples:
/// Input - `10`\
/// Expected Output - `4`\
/// Explanation - There are `4` prime numbers less than `10`: `2`, `3`,
/// `5`,`7`
///
/// Input - `13`\
/// Expected Output - `5`\
/// Explanation - There are `5` prime numbers less than `13`: `2`, `3`,
/// `5`, `7`, `11` - note that `13` does not count!
///
/// ```
/// assert_eq!(count_primes(100_000), 9592);
/// ```
fn count_primes(num: i32) -> i32 {
	let is_prime = |n| match n {
		0 | 1 => false,
		2 | 3 => true,
		n if n % 2 == 0 || n % 3 == 0 => false,
		_ => {
			let mut i = 5;
			while i * i <= n {
				if n % i == 0 || n % (i + 2) == 0 {
					return false;
				}
				i += 6
			}
			true
		}
	};
	let num = std::cmp::min(num, 1_000_000);
	(0..num).fold(0, |acc, n| acc + is_prime(n) as i32)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn single_char_1() {
		let data = ("aabdb", "d");
		assert_eq!(single_char(data.0.to_owned()), data.1.to_owned());
	}
	#[test]
	fn single_char_2() {
		let data = ("sixsix554112323", "4");
		assert_eq!(single_char(data.0.to_owned()), data.1.to_owned());
	}
	#[test]
	fn single_char_3() {
		let data = ("#$#", "$");
		assert_eq!(single_char(data.0.to_owned()), data.1.to_owned());
	}

	#[test]
	fn is_constructable_1() {
		let data = ("a", "b", false);
		assert_eq!(is_constructable(data.0.to_owned(), data.1.to_owned()), data.2)
	}
	#[test]
	fn is_constructable_2() {
		let data = ("a", "ab", true);
		assert_eq!(is_constructable(data.0.to_owned(), data.1.to_owned()), data.2)
	}
	#[test]
	fn is_constructable_3() {
		let data = ("caa", "abac", true);
		assert_eq!(is_constructable(data.0.to_owned(), data.1.to_owned()), data.2)
	}
	#[test]
	fn is_constructable_4() {
		let data = ("123", "321431", true);
		assert_eq!(is_constructable(data.0.to_owned(), data.1.to_owned()), data.2)
	}

	#[test]
	fn count_primes_1() {
		let data = (10, 4);
		assert_eq!(count_primes(data.0), data.1)
	}
	#[test]
	fn count_primes_2() {
		let data = (1, 0);
		assert_eq!(count_primes(data.0), data.1)
	}
	#[test]
	fn count_primes_3() {
		let data = (13, 5);
		assert_eq!(count_primes(data.0), data.1)
	}
	#[test]
	fn count_primes_4() {
		let data = (123, 30);
		assert_eq!(count_primes(data.0), data.1)
	}
	#[test]
	fn count_primes_5() {
		let data = (100_000, 9592);
		assert_eq!(count_primes(data.0), data.1)
	}
}
