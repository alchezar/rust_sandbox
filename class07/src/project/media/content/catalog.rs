// IKinder

use super::media::Media;

#[derive(Debug, Default)]
pub struct Catalog {
	items: Vec<Media>,
}
impl Catalog {
	#[must_use]
	pub fn add_media(self, media: Media) -> Self {
		let mut catalog = Catalog { items: self.items };
		catalog.items.push(media);
		catalog
	}
	#[must_use]
	pub fn get_by_index(&self, index: usize) -> Option<&Media> {
		self.items.get(index)
	}
}
