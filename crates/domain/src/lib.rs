//! Core domain types and invariants for Ethical Search Engine.
//!
//! This crate deliberately has no external dependencies and performs no I/O.

use std::fmt;
use std::str::FromStr;

const MAX_IDENTIFIER_CHARS: usize = 256;
const MAX_POLICY_VERSION_CHARS: usize = 64;

/// Validation failures for domain values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DomainValidationError {
    /// The supplied value is empty or contains only whitespace.
    EmptyValue,
    /// The value exceeds the documented maximum number of Unicode scalar values.
    ValueTooLong { max_chars: usize },
    /// Opaque identifiers cannot contain whitespace.
    WhitespaceNotAllowed,
    /// Control characters are not valid in domain identifiers.
    ControlCharacterNotAllowed,
    /// A policy version contains characters outside its allowed ASCII grammar.
    InvalidPolicyVersion,
}

impl fmt::Display for DomainValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyValue => f.write_str("value must not be empty"),
            Self::ValueTooLong { max_chars } => {
                write!(f, "value must not exceed {max_chars} characters")
            }
            Self::WhitespaceNotAllowed => f.write_str("value must not contain whitespace"),
            Self::ControlCharacterNotAllowed => {
                f.write_str("value must not contain control characters")
            }
            Self::InvalidPolicyVersion => {
                f.write_str("policy version must use ASCII letters, digits, '.', '-' or '_'")
            }
        }
    }
}

impl std::error::Error for DomainValidationError {}

fn validate_opaque_identifier(value: &str) -> Result<(), DomainValidationError> {
    if value.trim().is_empty() {
        return Err(DomainValidationError::EmptyValue);
    }

    if value.chars().count() > MAX_IDENTIFIER_CHARS {
        return Err(DomainValidationError::ValueTooLong {
            max_chars: MAX_IDENTIFIER_CHARS,
        });
    }

    if value.chars().any(char::is_control) {
        return Err(DomainValidationError::ControlCharacterNotAllowed);
    }

    if value.chars().any(char::is_whitespace) {
        return Err(DomainValidationError::WhitespaceNotAllowed);
    }

    Ok(())
}

macro_rules! opaque_identifier {
    ($name:ident) => {
        #[doc = concat!("Validated opaque ", stringify!($name), ".")]
        #[derive(Clone, Debug, Eq, Hash, PartialEq)]
        pub struct $name(String);

        impl $name {
            /// Creates an identifier after validating its invariants.
            pub fn new(value: impl Into<String>) -> Result<Self, DomainValidationError> {
                let value = value.into();
                validate_opaque_identifier(&value)?;
                Ok(Self(value))
            }

            /// Returns the exact identifier representation supplied by the caller.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<&str> for $name {
            type Error = DomainValidationError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl TryFrom<String> for $name {
            type Error = DomainValidationError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::new(value)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

opaque_identifier!(DocumentId);
opaque_identifier!(SourceId);

/// Supported language labels used by the initial multilingual domain model.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum LanguageCode {
    /// Arabic.
    Arabic,
    /// French.
    French,
    /// English.
    English,
    /// Language has not been determined.
    Undetermined,
}

impl LanguageCode {
    /// Returns the canonical lowercase language code.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Arabic => "ar",
            Self::French => "fr",
            Self::English => "en",
            Self::Undetermined => "und",
        }
    }
}

impl fmt::Display for LanguageCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Returned when a language code is not part of the V0.1 supported set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnsupportedLanguageCode(pub String);

impl fmt::Display for UnsupportedLanguageCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unsupported language code: {}", self.0)
    }
}

impl std::error::Error for UnsupportedLanguageCode {}

impl FromStr for LanguageCode {
    type Err = UnsupportedLanguageCode;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.eq_ignore_ascii_case("ar") {
            Ok(Self::Arabic)
        } else if value.eq_ignore_ascii_case("fr") {
            Ok(Self::French)
        } else if value.eq_ignore_ascii_case("en") {
            Ok(Self::English)
        } else if value.eq_ignore_ascii_case("und") {
            Ok(Self::Undetermined)
        } else {
            Err(UnsupportedLanguageCode(value.to_owned()))
        }
    }
}

/// Ethical eligibility outcome. This is intentionally separate from relevance ranking.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum EthicalDecision {
    /// The document is eligible under the active policy.
    Allow,
    /// The document requires additional review and is not implicitly allowed.
    Review,
    /// The document is excluded under the active policy.
    Exclude,
}

/// Validated identifier for a versioned ethical policy.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct PolicyVersion(String);

impl PolicyVersion {
    /// Creates a policy version using ASCII letters, digits, dots, hyphens or underscores.
    pub fn new(value: impl Into<String>) -> Result<Self, DomainValidationError> {
        let value = value.into();

        if value.is_empty() {
            return Err(DomainValidationError::EmptyValue);
        }

        if value.chars().count() > MAX_POLICY_VERSION_CHARS {
            return Err(DomainValidationError::ValueTooLong {
                max_chars: MAX_POLICY_VERSION_CHARS,
            });
        }

        if !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'_'))
        {
            return Err(DomainValidationError::InvalidPolicyVersion);
        }

        Ok(Self(value))
    }

    /// Returns the exact policy version representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PolicyVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_id_preserves_unicode() {
        let value = DocumentId::new("وثيقة-é-42").expect("valid Unicode identifier");
        assert_eq!(value.as_str(), "وثيقة-é-42");
    }

    #[test]
    fn source_id_rejects_empty_and_whitespace() {
        assert_eq!(
            SourceId::new("   ").unwrap_err(),
            DomainValidationError::EmptyValue
        );
        assert_eq!(
            SourceId::new("source id").unwrap_err(),
            DomainValidationError::WhitespaceNotAllowed
        );
    }

    #[test]
    fn identifiers_reject_control_characters_and_excessive_length() {
        assert_eq!(
            DocumentId::new("source\n42").unwrap_err(),
            DomainValidationError::ControlCharacterNotAllowed
        );
        assert_eq!(
            SourceId::new("x".repeat(MAX_IDENTIFIER_CHARS + 1)).unwrap_err(),
            DomainValidationError::ValueTooLong {
                max_chars: MAX_IDENTIFIER_CHARS
            }
        );
    }

    #[test]
    fn language_codes_parse_case_insensitively_and_render_canonically() {
        assert_eq!("AR".parse::<LanguageCode>(), Ok(LanguageCode::Arabic));
        assert_eq!("Fr".parse::<LanguageCode>(), Ok(LanguageCode::French));
        assert_eq!(LanguageCode::Undetermined.to_string(), "und");
        assert!("de".parse::<LanguageCode>().is_err());
    }

    #[test]
    fn policy_version_accepts_expected_ascii_grammar() {
        let version = PolicyVersion::new("v1.2-beta_3").expect("valid version");
        assert_eq!(version.as_str(), "v1.2-beta_3");
        assert_eq!(
            PolicyVersion::new("v1/2").unwrap_err(),
            DomainValidationError::InvalidPolicyVersion
        );
    }

    #[test]
    fn ethical_decision_has_explicit_review_state() {
        assert_ne!(EthicalDecision::Review, EthicalDecision::Allow);
        assert_ne!(EthicalDecision::Review, EthicalDecision::Exclude);
    }
}
