use error_stack::Result;
use track::error::AppError;
use track::init;

fn main() -> Result<(), AppError> {
	init::error_reporting();
	init::tracing();

	Ok(())
}
