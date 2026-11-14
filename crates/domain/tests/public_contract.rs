use ese_domain::{DocumentId, EthicalDecision, LanguageCode, PolicyVersion, SourceId};

#[test]
fn public_identifiers_preserve_unicode_and_reject_invalid_values() {
    let document = DocumentId::try_from("وثيقة-é-42").expect("valid Unicode identifier");
    let source = SourceId::try_from("source-42").expect("valid source identifier");

    assert_eq!(document.as_str(), "وثيقة-é-42");
    assert_eq!(source.to_string(), "source-42");
    assert!(DocumentId::try_from("   ").is_err());
    assert!(SourceId::try_from("source id").is_err());
}

#[test]
fn public_language_and_policy_values_have_canonical_forms() {
    assert_eq!("AR".parse::<LanguageCode>().unwrap().as_str(), "ar");
    assert_eq!("fr".parse::<LanguageCode>().unwrap().as_str(), "fr");
    assert_eq!("en".parse::<LanguageCode>().unwrap().as_str(), "en");
    assert_eq!("und".parse::<LanguageCode>().unwrap().as_str(), "und");
    assert_eq!(PolicyVersion::new("v0.1").unwrap().as_str(), "v0.1");
    assert!(PolicyVersion::new("v0/1").is_err());
}

#[test]
fn review_is_a_distinct_non_approval_outcome() {
    assert_ne!(EthicalDecision::Review, EthicalDecision::Allow);
    assert_ne!(EthicalDecision::Review, EthicalDecision::Exclude);
}
