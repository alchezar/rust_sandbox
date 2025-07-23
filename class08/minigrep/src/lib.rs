// IKinder

#![allow(dead_code, unused)]
#![warn(clippy::all, clippy::must_use_candidate)]

use std::error::Error;
use std::fs;

pub struct Config {
	pub query: String,
	pub path: String,
	pub ignore_case: bool,
}
impl Config {
	pub fn build(args: &[String]) -> Result<Self, &'static str> {
		if args.len() < 3 {
			return Err("Not enough arguments");
		}

		dotenv::dotenv().expect("Could not read .env file");

		let query = args[1].clone();
		let path = args[2].clone();
		// let ignore_case = std::env::var("IGNORE_CASE").is_ok();
		// let ignore_case = args
		// 	.get(3)
		// 	.unwrap_or(&"false".to_owned())
		// 	.clone()
		// 	.eq("true");
		let ignore_case = std::env::var("IGNORE_CASE")
			.unwrap()
			.parse::<bool>()
			.unwrap();

		Ok(Self { query, path, ignore_case })
	}
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	let content = fs::read_to_string(config.path).expect("Should have been able to read the file");

	let results = if config.ignore_case {
		search_case_insensitive(&config.query, &content)
	} else {
		search(&config.query, &content)
	};

	for line in results {
		println!("{}", line)
	}

	Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let mut result = Vec::new();

	for line in content.lines() {
		if line.contains(query) {
			result.push(line.trim());
		}
	}

	result
}

fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
	let query = query.to_lowercase();
	let mut result = Vec::new();

	for line in content.lines() {
		if line.to_lowercase().contains(&query) {
			result.push(line.trim())
		}
	}

	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn one_result() {
		let query = "duct";
		let content = "\
Rust:
safe, fast, productive.
Pick three.";

		assert_eq!(vec!["safe, fast, productive."], search(query, content));
	}

	#[test]
	fn case_insensitive() {
		let query = "rUsT";
		let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

		assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, content))
	}
}
