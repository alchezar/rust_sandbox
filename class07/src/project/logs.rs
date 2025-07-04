// IKinder

use std::fs;
use std::io::Error;
use std::path::Path;

pub fn run() -> Result<(), Error> {
	let folder = std::env::current_dir()?.join("src\\project\\logs");
	let input_filepath = folder.clone().join("logs.txt");

	let logs = fs::read_to_string(&input_filepath)?;
	let errors = extract_errors(&logs);

	let output_filepath = folder.clone().join("errors.txt");
	fs::write(&output_filepath, errors.join("\n"))?;
	Ok(())
}

fn extract_errors(log: &str) -> Vec<String> {
	log.split("\n")
		.filter(|&line| line.starts_with("ERROR"))
		.map(|line| line.to_owned())
		.collect()
}

fn divide(a: f64, b: f64) -> Result<f64, Error> {
	if b == 0.0 {
		Err(Error::other("Divide by zero"))
	} else {
		Ok(a / b)
	}
}

fn validate_email(email: &str) -> Result<(), Error> {
	if email.contains('@') {
		Ok(())
	} else {
		Err(Error::other("Email contains invalid characters"))
	}
}
