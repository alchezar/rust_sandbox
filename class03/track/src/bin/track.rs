use error_stack::{Result, ResultExt};
use track::error::AppError;
use track::feature::cli;
use track::init;

// track is the binary name
//
// track start
// track stop
// track report

fn main() -> Result<(), AppError> {
	init::error_reporting();
	init::tracing();

	cli::run()
		.change_context(AppError)
		.attach_printable("Failed to run CLI.")?;

	Ok(())
}
