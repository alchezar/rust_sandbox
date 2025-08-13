// IKinder

#![allow(dead_code)]

use std::time::{SystemTime, UNIX_EPOCH};

pub const DATA_COLLECTOR_ADDRESS: &str = "localhost:9004";
const MAGIC_NUMBER: u16 = 1234;
const VERSION_NUMBER: u16 = 1;

fn unix_now() -> u32 {
	let start = SystemTime::now();
	let since_the_epoch = start
		.duration_since(UNIX_EPOCH)
		.expect("Time went backwards");
	since_the_epoch.as_secs() as u32
}

#[derive(Debug, Clone, PartialEq, bincode::Encode, bincode::Decode)]
pub enum CollectorCommandV1 {
	SubmitData {
		collector_id: u128,
		total_memory: u64,
		used_memory: u64,
		average_cpu_usage: f32,
	},
}

pub fn encode_v1(command: &CollectorCommandV1) -> Vec<u8> {
	let payload_bytes = bincode::encode_to_vec(&command, bincode::config::standard()).unwrap();

	let crc = crc32fast::hash(&payload_bytes);
	let payload_size = payload_bytes.len() as u32;
	let timestamp = unix_now();

	// Encode into bytes.
	let mut result = Vec::with_capacity(140);
	result.extend_from_slice(&MAGIC_NUMBER.to_be_bytes());
	result.extend_from_slice(&VERSION_NUMBER.to_be_bytes());
	result.extend_from_slice(&timestamp.to_be_bytes());
	result.extend_from_slice(&payload_size.to_be_bytes());
	result.extend_from_slice(&payload_bytes);
	result.extend_from_slice(&crc.to_be_bytes());
	result
}

pub fn decode_v1(bytes: &[u8]) -> (u32, CollectorCommandV1) {
	let magic_number = u16::from_be_bytes(bytes[0..2].try_into().unwrap());
	let version_number = u16::from_be_bytes(bytes[2..4].try_into().unwrap());
	let timestamp = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
	let payload_size = u32::from_be_bytes(bytes[8..12].try_into().unwrap());

	let p_size = 12 + payload_size as usize;
	let payload = &bytes[12..p_size];
	let crc = u32::from_be_bytes(
		bytes[p_size..4 + p_size]
			.try_into()
			.unwrap(),
	);

	assert_eq!(magic_number, MAGIC_NUMBER);
	assert_eq!(version_number, VERSION_NUMBER);
	assert_eq!(crc, crc32fast::hash(payload));

	let command = bincode::decode_from_slice(&payload, bincode::config::standard())
		.unwrap()
		.0;

	(timestamp, command)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_encode_decode() {
		let command = CollectorCommandV1::SubmitData {
			collector_id: 1234,
			total_memory: 100,
			used_memory: 50,
			average_cpu_usage: 0.5,
		};

		let encoded = encode_v1(&command);
		let (timestamp, decoded) = decode_v1(&encoded);

		assert_eq!(decoded, command);
		assert!(timestamp > 0);
	}
}
