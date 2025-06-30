// IKinder

#[derive(Debug)]
pub enum Media {
	Book { title: String, author: String },
	Movie { title: String, director: String },
	Audio { title: String },
	Podcast(u32),
	Placeholder,
}

impl Media {
	#[must_use]
	pub fn description(&self) -> String {
		match self {
			Media::Book { title, author } => format!("Book: {} by {}", title, author),
			Media::Movie { title, director } => format!("Movie: {} by {}", title, director),
			Media::Audio { title } => format!("Audio: {}", title),
			Media::Podcast(id) => format!("Podcast: {}", id),
			Media::Placeholder => "Placeholder".to_owned(),
		}
	}
	pub fn print(&self) {
		println!("{}", self.description());
	}
}
