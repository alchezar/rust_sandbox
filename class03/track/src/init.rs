//! Application setup.

use crate::error::Suggestion;
use error_stack::Report;
use error_stack::fmt::ColorMode;
use owo_colors::OwoColorize;

pub fn error_reporting() {
	Report::set_color_mode(ColorMode::Color);
	Report::install_debug_hook::<Suggestion>(|value, context| {
		let msg = value.0;
		let body = format!("Suggestion: {}", msg);
		match context.color_mode() {
			ColorMode::Color => context.push_body(body.bright_yellow().to_string()),
			ColorMode::Emphasis => context.push_body(body.italic().to_string()),
			ColorMode::None => context.push_body(body),
		}
	})
}
