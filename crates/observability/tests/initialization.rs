use ese_observability::{ObservabilityError, init};

#[test]
fn initialization_succeeds_once_and_duplicate_is_reported() {
    init("info").expect("first subscriber initialization should succeed");

    let duplicate = init("info");
    assert!(matches!(
        duplicate,
        Err(ObservabilityError::SubscriberAlreadyInitialized(_))
    ));
}
