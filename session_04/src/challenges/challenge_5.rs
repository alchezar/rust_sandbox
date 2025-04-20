fn single_char(str: String) -> String {
	let mut map = std::collections::HashMap::<char, i32>::new();
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn single_char_1() {
		let data = vec!["aabdb", "d"];
		assert_eq!(single_char(data[0].to_string()), data[1].to_string());
	}
	#[test]
	fn single_char_2() {
		let data = vec!["sixsix554112323", "4"];
		assert_eq!(single_char(data[0].to_string()), data[1].to_string());
	}
	#[test]
	fn single_char_3() {
		let data = vec!["#$#", "$"];
		assert_eq!(single_char(data[0].to_string()), data[1].to_string());
	}
}
