// IKinder

#![allow(dead_code, unused_variables)]

use anyhow::Context;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::stdin;

#[derive(Debug)]
struct Expiration {
	year: u32,
	month: u32,
}

#[derive(Debug)]
struct Card {
	number: u32,
	exp: Expiration,
	cvv: u32,
}

#[derive(thiserror::Error, Debug)]
enum CreditCardError {
	#[error("{0}")]
	InvalidInput(String),
	#[error(transparent)]
	Other(#[from] anyhow::Error),
}

#[derive(thiserror::Error, Debug)]
#[error("{msg}")]
struct ParsePaymentInfoError {
	source: Option<anyhow::Error>,
	msg: String,
}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
	let numbers = card
		.split(" ")
		.into_iter()
		.map(|s| s.parse().with_context(|| format!("{s:?} could not be parsed as u32")))
		.collect::<Result<Vec<u32>, _>>()
		.map_err(|e| ParsePaymentInfoError {
			source: Some(e),
			msg: format!("Failed to parse input as numbers, Input: {card}."),
		})?;
	Ok(numbers)
}

fn parse_card(card: &&str) -> Result<Card, ParsePaymentInfoError> {
	let mut numbers = parse_card_numbers(card)?;

	let len = numbers.len();
	let expected_len = 4;

	if len != expected_len {
		return Err(ParsePaymentInfoError {
			source: None,
			msg: format!(
				"Incorrect number of elements parsed. Expected {expected_len}, but get {len}! Elements: {numbers:?}"
			),
		});
	}

	let cvv = numbers.pop().unwrap();
	let year = numbers.pop().unwrap();
	let month = numbers.pop().unwrap();
	let number = numbers.pop().unwrap();

	Ok(Card {
		number,
		exp: Expiration { year, month },
		cvv,
	})
}

fn get_credit_card_info(credit_cards: &HashMap<&str, &str>, name: &str) -> Result<Card, CreditCardError> {
	let card_string = credit_cards
		.get(name)
		.ok_or(CreditCardError::InvalidInput(format!("No credit card was found for {name}!")))?;
	let card = parse_card(card_string)
		.with_context(|| format!("{name}'s card could not be parsed!"))
		.map_err(|e| CreditCardError::Other(e))?;
	Ok(card)
}

pub fn run() {
	env_logger::init();

	let credit_cards = HashMap::from([
		("Amy", "1234567 04 23 123"),
		("Tim", "1234567 06 27"),
		("Bob", "1234567 Dec 27 123"),
	]);

	println!("Enter Name: ");
	let mut name = String::new();
	stdin().read_line(&mut name).expect("Error reading name");

	let result = get_credit_card_info(&credit_cards, name.trim());
	match result {
		Ok(card) => println!("\nCredit card Info {card:?}"),
		Err(err) => {
			match &err {
				CreditCardError::InvalidInput(msg) => println!("{msg}"),
				CreditCardError::Other(_) => println!("\nSomething went wrong, try again!"),
			}

			log::error!("\n{err:?}")
		}
	}
}
