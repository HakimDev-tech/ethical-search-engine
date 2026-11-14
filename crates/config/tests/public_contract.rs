use ese_config::{AppConfig, ConfigError, LogLevel};

#[test]
fn lookup_constructor_applies_default_and_explicit_value() {
    let default = AppConfig::from_lookup(|_| Ok(None)).unwrap();
    assert_eq!(default.log_level(), LogLevel::Info);

    let explicit =
        AppConfig::from_lookup(|key| Ok((key == "ESE_LOG_LEVEL").then(|| "TRACE".to_owned())))
            .unwrap();
    assert_eq!(explicit.log_level(), LogLevel::Trace);
}

#[test]
fn malformed_values_and_lookup_failures_are_returned() {
    let malformed = AppConfig::from_lookup(|_| Ok(Some("verbose".to_owned())));
    assert_eq!(
        malformed.unwrap_err(),
        ConfigError::InvalidLogLevel("verbose".to_owned())
    );

    let lookup_failure =
        AppConfig::from_lookup(|key| Err(ConfigError::NonUnicodeValue(key.to_owned())));
    assert_eq!(
        lookup_failure.unwrap_err(),
        ConfigError::NonUnicodeValue("ESE_LOG_LEVEL".to_owned())
    );
}
