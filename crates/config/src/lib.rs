//! Typed configuration loading and validation.
//!
//! Configuration is read from `ESE_*` environment variables. The lookup-based
//! constructor keeps parsing deterministic and makes tests independent of process state.

use std::env;
use std::fmt;

/// Supported application log levels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LogLevel {
    /// Error messages only.
    Error,
    /// Warnings and errors.
    Warn,
    /// Informational messages, warnings and errors.
    Info,
    /// Debug-level diagnostics.
    Debug,
    /// Trace-level diagnostics.
    Trace,
}

impl LogLevel {
    /// Returns the canonical lowercase representation used by configuration.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warn => "warn",
            Self::Info => "info",
            Self::Debug => "debug",
            Self::Trace => "trace",
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for LogLevel {
    type Err = ConfigError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.eq_ignore_ascii_case("error") {
            Ok(Self::Error)
        } else if value.eq_ignore_ascii_case("warn") {
            Ok(Self::Warn)
        } else if value.eq_ignore_ascii_case("info") {
            Ok(Self::Info)
        } else if value.eq_ignore_ascii_case("debug") {
            Ok(Self::Debug)
        } else if value.eq_ignore_ascii_case("trace") {
            Ok(Self::Trace)
        } else {
            Err(ConfigError::InvalidLogLevel(value.to_owned()))
        }
    }
}

/// Validated application configuration for V0.1.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AppConfig {
    log_level: LogLevel,
}

impl AppConfig {
    /// Loads configuration from the process environment.
    ///
    /// Missing `ESE_LOG_LEVEL` uses the documented `info` default. Non-Unicode
    /// environment values are rejected rather than silently ignored.
    pub fn from_env() -> Result<Self, ConfigError> {
        Self::from_lookup(|key| match env::var(key) {
            Ok(value) => Ok(Some(value)),
            Err(env::VarError::NotPresent) => Ok(None),
            Err(env::VarError::NotUnicode(_)) => Err(ConfigError::NonUnicodeValue(key.to_owned())),
        })
    }

    /// Loads configuration through a caller-provided lookup function.
    ///
    /// This is public to allow deterministic integration and tests without mutating
    /// process-global environment variables.
    pub fn from_lookup<F>(mut lookup: F) -> Result<Self, ConfigError>
    where
        F: FnMut(&str) -> Result<Option<String>, ConfigError>,
    {
        let log_level = match lookup("ESE_LOG_LEVEL")? {
            Some(value) => value.parse()?,
            None => LogLevel::Info,
        };

        Ok(Self { log_level })
    }

    /// Returns the configured log level.
    pub const fn log_level(self) -> LogLevel {
        self.log_level
    }
}

/// Configuration loading and validation failures.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConfigError {
    /// The configured log level is not supported.
    InvalidLogLevel(String),
    /// An environment variable could not be represented as Unicode.
    NonUnicodeValue(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLogLevel(value) => {
                write!(f, "invalid ESE_LOG_LEVEL value: {value}")
            }
            Self::NonUnicodeValue(key) => {
                write!(f, "environment variable {key} is not valid Unicode")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_log_level_uses_info_default() {
        let config = AppConfig::from_lookup(|_| Ok(None)).expect("default configuration");
        assert_eq!(config.log_level(), LogLevel::Info);
    }

    #[test]
    fn explicit_log_level_is_parsed_case_insensitively() {
        let config =
            AppConfig::from_lookup(|key| Ok((key == "ESE_LOG_LEVEL").then(|| "DeBuG".to_owned())))
                .expect("valid configuration");

        assert_eq!(config.log_level(), LogLevel::Debug);
    }

    #[test]
    fn malformed_explicit_value_is_not_silently_replaced() {
        let result = AppConfig::from_lookup(|_| Ok(Some("verbose".to_owned())));
        assert_eq!(
            result.unwrap_err(),
            ConfigError::InvalidLogLevel("verbose".to_owned())
        );
    }

    #[test]
    fn lookup_errors_are_propagated() {
        let result =
            AppConfig::from_lookup(|key| Err(ConfigError::NonUnicodeValue(key.to_owned())));

        assert_eq!(
            result.unwrap_err(),
            ConfigError::NonUnicodeValue("ESE_LOG_LEVEL".to_owned())
        );
    }
}
