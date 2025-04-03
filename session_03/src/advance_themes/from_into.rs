// IKinder

use std::convert::TryFrom;
use thiserror::Error;

// -----------------------------------------------------------------------------
// From/Into: Intro
// -----------------------------------------------------------------------------

fn to_owned(slice: &str) -> String {
	slice.into()
}

pub fn intro1() {
	let owned = String::from("slice");
	let owned: String = "slice".into();
}

enum Status {
	Broken(u8),
	Working,
}

impl From<u8> for Status {
	fn from(value: u8) -> Self {
		match value {
			0 => Status::Working,
			v => Status::Broken(v),
		}
	}
}

fn legacy_interface() -> u8 {
	5
}

pub fn intro2() {
	let status = Status::from(legacy_interface());
	let status: Status = legacy_interface().into();
}

struct Job;
enum JobError {
	Expired,
	Missing,
	Other(u8),
}

impl From<u8> for JobError {
	fn from(value: u8) -> Self {
		match value {
			1 => Self::Expired,
			2 => Self::Missing,
			_ => Self::Other(value),
		}
	}
}

fn execute_job(job: Job) -> Result<(), JobError> {
	Err(2)?
}

// -----------------------------------------------------------------------------
// TryFrom/TryInto: Intro
// -----------------------------------------------------------------------------

enum NonZeroError {
	IsZero,
}
struct NonZero(i32);

impl TryFrom<i32> for NonZero {
	type Error = NonZeroError;
	fn try_from(value: i32) -> Result<Self, Self::Error> {
		if value == 0 {
			Err(NonZeroError::IsZero)
		} else {
			Ok(NonZero(value))
		}
	}
}

pub fn intro3() {
	match NonZero::try_from(9) {
		Ok(_) => println!("not zero"),
		Err(_) => println!("is zero"),
	}

	let whoops: Result<NonZero, _> = 0i32.try_into();
	match whoops {
		Ok(_) => println!("not zero"),
		Err(_) => println!("is zero"),
	}
}

// -----------------------------------------------------------------------------
// From/Into: Demo
// -----------------------------------------------------------------------------

struct Uppercase(String);

impl From<String> for Uppercase {
	fn from(value: String) -> Self {
		Uppercase(value.to_uppercase())
	}
}

impl From<&str> for Uppercase {
	fn from(value: &str) -> Self {
		Uppercase(value.to_uppercase())
	}
}

pub fn demo1() {
	let upper = Uppercase::from("lowercase");
	let upper: Uppercase = "lowercase".into();
}

enum KeyPress {
	Up,
	Down,
}
struct KeyEvent {
	keycode: u16,
	state: KeyPress,
}

enum InputEvent {
	Key(u16, KeyPress),
	Mouse,
}

impl From<KeyEvent> for InputEvent {
	fn from(event: KeyEvent) -> Self {
		Self::Key(event.keycode, event.state)
	}
}

pub fn demo2() {
	let key_event = KeyEvent { keycode: 5, state: KeyPress::Down };
	let input_event = InputEvent::from(key_event);
	let input_event: InputEvent = input_event.into();
}

#[derive(Debug, Error)]
enum NetworkError {
	#[error("connection timed out")]
	Timeout,
}

#[derive(Debug, Error)]
enum DatabaseError {
	#[error("error querying database")]
	QueryFailure,
}

#[derive(Debug, Error)]
enum ApiError {
	#[error("network error: {0}")]
	Network(#[from] NetworkError),
	#[error("database error: {0}")]
	Database(#[from] DatabaseError),
}

fn do_network_stuff() -> Result<(), ApiError> {
	Err(NetworkError::Timeout)?
}

fn do_database_stuff() -> Result<(), ApiError> {
	Err(DatabaseError::QueryFailure)?
}

pub fn demo3() {
	println!("{:?}", do_network_stuff());
	println!("{:?}", do_database_stuff());
}

// -----------------------------------------------------------------------------
// TryFrom/TryInto: Activity
// -----------------------------------------------------------------------------

use std::num::ParseIntError;

#[derive(Debug, Eq, PartialEq)]
struct Rgb(u8, u8, u8);

#[derive(Debug, Error)]
enum RgbError {
	#[error("Hex color must begin with a hash (#) symbol")]
	MissingHash,
	#[error("Invalid hex color length (must be 6)")]
	LengthError,
	#[error("Failed to parse hex digit: {0}")]
	ParseError(#[from] ParseIntError),
}

const HEX_LEN: usize = 7;

impl TryFrom<&str> for Rgb {
	type Error = RgbError;
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		match value {
			hex if !hex.starts_with('#') => Err(RgbError::MissingHash),
			hex if hex.len() != HEX_LEN => Err(RgbError::LengthError),
			hex => {
				let r = u8::from_str_radix(&hex[1..=2], 16)?;
				let g = u8::from_str_radix(&hex[3..=4], 16)?;
				let b = u8::from_str_radix(&hex[5..=6], 16)?;
				Ok(Self(r, g, b))
			}
		}
	}
}

pub fn activity() {
	let hex = "#0024ff";
	let rgb = Rgb::try_from(hex);
	println!("{} == {:?}", hex, rgb);
}

mod test {
	use super::Rgb;
	use std::convert::TryFrom;

	#[test]
	fn converts_valid_hex_color() {
		let expected = Rgb(0, 204, 102);
		let actual = Rgb::try_from("#00cc66");
		assert_eq!(actual.is_ok(), true, "valid hex code should be converted to Rgb");
		assert_eq!(actual.unwrap(), expected, "wrong Rgb value");
	}

	#[test]
	fn fails_on_invalid_hex_digits() {
		assert_eq!(
			Rgb::try_from("#0011yy").is_err(),
			true,
			"should be an error with invalid hex color"
		);
	}

	#[test]
	fn fails_when_missing_hash() {
		assert_eq!(
			Rgb::try_from("001100").is_err(),
			true,
			"should be an error when missing hash symbol"
		);
	}

	#[test]
	fn fails_when_missing_color_components() {
		assert_eq!(
			Rgb::try_from("#0011f").is_err(),
			true,
			"should be an error when missing one or more color components"
		);
	}

	#[test]
	fn fails_with_too_many_color_components() {
		assert_eq!(
			Rgb::try_from("#0011ffa").is_err(),
			true,
			"should be an error when too many color components are provided"
		)
	}
}
