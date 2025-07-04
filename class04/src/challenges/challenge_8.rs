// IKinder

use std::collections::HashSet;

/// # Reverse a String
/// ## Easy
/// You are given a string, and your task is to write a function
/// named `reverseString` that reverses the given string.
fn reverse_string(s: String) -> String {
	s.chars().rev().collect()
}

/// # Find the Missing Number
/// ## Medium
/// You are given an array containing n distinct numbers taken
/// from `0`, `1`, `2`, `...`, `n`, except for one number that is missing.
/// Write a function to find and return the missing number.
fn find_missing_number(nums: Vec<i32>) -> i32 {
	let max = nums.len() as i32;
	let set = nums
		.into_iter()
		.collect::<HashSet<i32>>();
	(0..max)
		.find(|i| !set.contains(i))
		.unwrap_or(0)
}

/// # Longest Substring
/// ## Hard
/// Write a function that takes in a string `s` and returns
/// an integer representing the length of the longest substring
/// without repeating characters.
fn longest_substring(s: String) -> i32 {
	let chars = s.chars().collect::<Vec<_>>();
	let mut set = HashSet::new();

	let mut longest = 0;
	let mut left = 0;

	for right in 0..s.len() {
		while set.contains(&chars[right]) {
			set.remove(&chars[left]);
			left += 1;
		}
		set.insert(chars[right]);
		longest = std::cmp::max(longest, right - left + 1);
	}
	longest as i32
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn reverse_string_1() {
		let data = ("Hello, World!", "!dlroW ,olleH");
		assert_eq!(reverse_string(data.0.to_owned()), data.1.to_owned())
	}
	#[test]
	fn reverse_string_2() {
		let data = ("Python is fun", "nuf si nohtyP");
		assert_eq!(reverse_string(data.0.to_owned()), data.1.to_owned())
	}
	#[test]
	fn reverse_string_3() {
		let data = ("12345", "54321");
		assert_eq!(reverse_string(data.0.to_owned()), data.1.to_owned())
	}
	#[test]
	fn reverse_string_4() {
		let data = ("", "");
		assert_eq!(reverse_string(data.0.to_owned()), data.1.to_owned())
	}

	#[test]
	fn find_missing_number_1() {
		let data = (vec![3, 0, 1], 2);
		assert_eq!(find_missing_number(data.0), data.1);
	}
	#[test]
	fn find_missing_number_2() {
		let data = (vec![9, 6, 4, 2, 3, 5, 7, 0, 1], 8);
		assert_eq!(find_missing_number(data.0), data.1);
	}
	#[test]
	fn find_missing_number_3() {
		let data = (vec![1, 4, 3, 5, 6, 8, 7, 0], 2);
		assert_eq!(find_missing_number(data.0), data.1);
	}
	#[test]
	fn find_missing_number_4() {
		let data = (vec![1], 0);
		assert_eq!(find_missing_number(data.0), data.1);
	}

	#[test]
	fn longest_substring_1() {
		let data = ("abcabcbb", 3);
		assert_eq!(longest_substring(data.0.to_owned()), data.1)
	}
	#[test]
	fn longest_substring_2() {
		let data = ("bbbbb", 1);
		assert_eq!(longest_substring(data.0.to_owned()), data.1)
	}
	#[test]
	fn longest_substring_3() {
		let data = ("pwwkew", 3);
		assert_eq!(longest_substring(data.0.to_owned()), data.1)
	}
	#[test]
	fn longest_substring_4() {
		let data = ("abcde", 5);
		assert_eq!(longest_substring(data.0.to_owned()), data.1)
	}
}
