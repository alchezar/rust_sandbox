use error_stack::Result;
use track::error::AppError;
use track::init;

// track is the binary name
//
// track start
// track stop
// track report

fn main() -> Result<(), AppError> {
	init::error_reporting();
	init::tracing();

	Ok(())
}
