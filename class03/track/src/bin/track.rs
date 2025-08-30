use error_stack::{Report, Result, ResultExt};
use track::error::{AppError, Suggestion};
use track::init;

fn main() -> Result<(), AppError> {
	init::error_reporting();

	return Err(Report::from(AppError))
		.attach(Suggestion("Do something else!"))
		.attach_printable("Printable attachment.");

	Ok(())
}
