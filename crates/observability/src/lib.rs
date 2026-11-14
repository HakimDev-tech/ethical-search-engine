//! Structured logging and tracing initialization.

use std::fmt;
use tracing_subscriber::EnvFilter;

/// Initializes the process-wide tracing subscriber.
///
/// Call once during application startup, before emitting tracing events. The
/// function returns an error when the filter is invalid or another subscriber
/// has already been installed; it never panics on duplicate initialization.
pub fn init(log_filter: &str) -> Result<(), ObservabilityError> {
    let filter = EnvFilter::try_new(log_filter)
        .map_err(|error| ObservabilityError::InvalidFilter(error.to_string()))?;

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .try_init()
        .map_err(|error| ObservabilityError::SubscriberAlreadyInitialized(error.to_string()))
}

/// Errors returned while initializing observability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ObservabilityError {
    /// The supplied tracing filter directive is malformed.
    InvalidFilter(String),
    /// A global tracing subscriber has already been installed.
    SubscriberAlreadyInitialized(String),
}

impl fmt::Display for ObservabilityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFilter(message) => write!(f, "invalid tracing filter: {message}"),
            Self::SubscriberAlreadyInitialized(message) => {
                write!(f, "could not install tracing subscriber: {message}")
            }
        }
    }
}

impl std::error::Error for ObservabilityError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn malformed_filter_returns_an_error() {
        let result = init("this is not a valid filter directive !!!");
        assert!(matches!(
            result,
            Err(ObservabilityError::InvalidFilter(_))
        ));
    }
}
