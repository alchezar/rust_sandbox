// IKinder

mod content;

pub mod prelude {
	pub use crate::project::media::content::{Catalog, Media};
}

use prelude::*;

pub fn run() {
	let audiobook = Media::Audio { title: "Synth".to_owned() };
	let good_movie = Media::Movie {
		title: "Good Movie".to_owned(),
		director: "Good Director".to_owned(),
	};
	let bad_book = Media::Book {
		title: "Bad Book".to_owned(),
		author: "Bad Author".to_owned(),
	};
	let podcast = Media::Podcast(66);
	let placeholder = Media::Placeholder;

	let catalog = Catalog::default()
		.add_media(audiobook)
		.add_media(good_movie)
		.add_media(bad_book)
		.add_media(podcast)
		.add_media(placeholder);

	println!("{:#?}", catalog);
	println!("{:#?}", catalog.get_by_index(0));
}
