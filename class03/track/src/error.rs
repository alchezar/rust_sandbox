//! Top-level error types.

#[derive(Debug, thiserror::Error)]
#[error("An application error has occurred.")]
pub struct AppError;

/// A suggestion displayed to the user.
pub struct Suggestion(pub &'static str);
