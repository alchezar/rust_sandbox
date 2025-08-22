// IKinder

pub fn main() {
	crate::show_name(file!());
	let numbers = Counter::new(10).collect::<Vec<_>>();
	println!("{:?}", numbers);

	let mut bucket = HashMapBucket::new();
	bucket.insert("Hello", 1);
	bucket.insert("Hello", 2);
	bucket.insert("World", 3);

	for (key, value) in bucket.iter() {
		println!("{}: {}", key, value);
	}
}

struct Counter {
	count: u32,
	max: u32,
}
impl Counter {
	fn new(max: u32) -> Self {
		Self { count: 0, max }
	}
}
impl Iterator for Counter {
	type Item = u32;
	fn next(&mut self) -> Option<Self::Item> {
		if self.count < self.max {
			self.count += 1;
			Some(self.count)
		} else {
			None
		}
	}
}
impl ExactSizeIterator for Counter {
	fn len(&self) -> usize {
		self.max as usize
	}
}

use super::generics::HashMapBucket;

struct HashMapBucketIter<'a, K, V> {
	key_iter: std::collections::hash_map::Iter<'a, K, Vec<V>>,
	current_map_entry: Option<(&'a K, &'a Vec<V>)>,
	current_vec_index: usize,
}
impl<K, V> crate::theme::generics::HashMapBucket<K, V> {
	fn iter(&self) -> HashMapBucketIter<'_, K, V> {
		let mut key_iter = self.map.iter();
		let current_map_entry = key_iter.next();
		HashMapBucketIter {
			key_iter,
			current_map_entry,
			current_vec_index: 0,
		}
	}
}

impl<'a, K, V> Iterator for HashMapBucketIter<'a, K, V> {
	type Item = (&'a K, &'a V);
	fn next(&mut self) -> Option<Self::Item> {
		if let Some((key, values)) = self.current_map_entry {
			if self.current_vec_index < values.len() {
				let value = &values[self.current_vec_index];
				self.current_vec_index += 1;

				return Some((key, value));
			} else {
				self.current_map_entry = self.key_iter.next();
				self.current_vec_index = 0;

				if let Some((key, value)) = self.current_map_entry {
					if self.current_vec_index < values.len() {
						let value = &values[self.current_vec_index];
						self.current_vec_index += 1;

						return Some((key, value));
					}
				}
			}
		}
		None
	}
}
