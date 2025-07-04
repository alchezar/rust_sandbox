// IKinder

use rand::Rng;
use rand::prelude::SliceRandom;

pub fn run() {
	let mut deck = Deck::new();
	println!("Here's your deck: {:?}\n\n", deck);
	deck.shuffle();
	println!("Here's your deck: {:?}\n", deck);
}

#[derive(Debug)]
struct Deck {
	cards: Vec<String>,
}

impl Deck {
	fn new() -> Self {
		let suits = ["♥️", "♣️", "♠️", "♦️"];
		let values = ['1', '2', '3', '4', '5', '6', '7', '8', '9', 'B', 'Q', 'K', 'A'];

		let mut cards = Vec::new();
		for suit in suits {
			for value in values {
				let card = format!("{} of {}\n", value, suit);
				cards.push(card);
			}
		}

		Self { cards }
	}
	fn shuffle(&mut self) {
		self.cards.shuffle(&mut rand::rng());
	}
}
