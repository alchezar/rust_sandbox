// IKinder

pub fn main() {
	cipher();
	decipher();
}

fn read_str() -> String {
	let mut input = String::new();
	std::io::stdin()
		.read_line(&mut input)
		.unwrap();
	input.trim().into()
}

fn cipher() {
	println!("Enter your string: ");
	let input = read_str();
	println!("Enter your password: ");
	let password = pad_or_trim(input.len(), read_str().as_bytes());
	let ciphered = zip_and_xor(&password, bytes::Bytes::from(input));
	println!("Ciphered string: \n{:?}", ciphered);
}

fn pad_or_trim(limit: usize, pass: &[u8]) -> Vec<u8> {
	let pass_len = pass.len();

	match limit.cmp(&pass_len) {
		std::cmp::Ordering::Less => pass[..limit].to_vec(),
		std::cmp::Ordering::Equal => pass.to_vec(),
		std::cmp::Ordering::Greater => {
			let mut as_bytes = pass.to_vec();
			let times = limit / pass_len;
			for i in pass_len..limit {
				as_bytes.push(pass[i % pass_len]);
			}
			as_bytes
		}
	}
}

fn zip_and_xor(pass: &[u8], bytes: bytes::Bytes) -> Vec<u8> {
	pass.iter()
		.zip(bytes.iter())
		.map(|(&a, &b)| a ^ b)
		.collect()
}

fn decipher() {
	println!("Enter your ciphered data: ");
	let input = read_str()
		.trim()
		.trim_matches(|m| m == '[' || m == ']')
		.split(',')
		.map(|el| {
			el.trim()
				.parse::<u8>()
				.expect("invalid input!")
		})
		.collect::<Vec<_>>();

	println!("Enter your password: ");
	let password = pad_or_trim(input.len(), read_str().as_bytes());
	let deciphered = zip_and_xor(&password, bytes::Bytes::from(input));

	println!("Deciphered: \n{}", String::from_utf8(deciphered).unwrap());
}
