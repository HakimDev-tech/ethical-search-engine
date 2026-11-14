use ese_observability::{init, ObservabilityError};

#[test]
fn initialization_succeeds_once_and_duplicate_is_reported() {
    init("info").expect("first subscriber initialization should succeed");

    let duplicate = init("info");
    assert!(matches!(
        duplicate,
        Err(ObservabilityError::SubscriberAlreadyInitialized(_))
    ));
}
