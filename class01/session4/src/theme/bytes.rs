// IKinder

use std::io::Write;

const DATA_PATH: &str = "./bin/data.bin";
const BYTES_PATH: &str = "./bin/bytes.bin";

pub fn main() {
	crate::show_name(file!());

	byte_muck();
	standard_way();
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod)]
struct OurData {
	number: u16,
	tag: [u8; 8],
}

fn byte_muck() {
	let some_data = vec![
		OurData { number: 1, tag: *b"hello   " },
		OurData { number: 2, tag: *b"world   " },
	];

	let bytes = bytemuck::cast_slice::<OurData, u8>(&some_data);
	std::fs::write(DATA_PATH, bytes).unwrap();

	let bytes = std::fs::read(DATA_PATH).unwrap();
	let data = bytemuck::cast_slice::<u8, OurData>(&bytes);

	println!("{:?}", data);
}

#[derive(Debug)]
struct AnotherData {
	number: u16,
	tag: String,
}

fn standard_way() {
	let a = AnotherData { number: 12, tag: "Hello World".into() };

	// Write.
	let mut file = std::fs::File::create(BYTES_PATH).unwrap();
	assert_eq!(
		file.write(&a.number.to_le_bytes())
			.unwrap(),
		2
	);
	let len = a.tag.as_bytes().len();
	assert_eq!(
		file.write(&(len as u64).to_le_bytes())
			.unwrap(),
		8
	);
	assert_eq!(file.write(a.tag.as_bytes()).unwrap(), len);

	// Read back.
	let bytes = std::fs::read(BYTES_PATH).unwrap();
	let number = u16::from_le_bytes(bytes[0..2].try_into().unwrap());
	let length = u64::from_le_bytes(bytes[2..10].try_into().unwrap());
	let tag = std::str::from_utf8(&bytes[10..10 + length as usize]).unwrap();
	// let tag = std::str::from_utf8(&bytes[10..]).unwrap();

	let a = AnotherData { number, tag: tag.into() };
	println!("{a:?}");
}
