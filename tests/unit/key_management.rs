use quantumsafe_finance::crypto;
use std::str::FromStr;

#[test]
fn test_key_pair_generation() {
    let key_pair = crypto::generate_key_pair();
    assert!(!key_pair.public_key.is_empty());
    assert!(!key_pair.private_key.is_empty());
}

#[test]
fn test_key_pair_serialization() {
    let key_pair = crypto::generate_key_pair();
    let json = crypto::key_pair_to_json(&key_pair);
    let deserialized = crypto::key_pair_from_json(&json).unwrap();
    
    assert_eq!(key_pair.public_key, deserialized.public_key);
    assert_eq!(key_pair.private_key, deserialized.private_key);
}

#[test]
fn test_invalid_key_pair_serialization() {
    let invalid_json = "{\"public_key\": \"invalid\"}";
    assert!(crypto::key_pair_from_json(invalid_json).is_err());
}

#[test]
fn test_key_pair_equality() {
    let key_pair1 = crypto::generate_key_pair();
    let key_pair2 = crypto::generate_key_pair();
    
    assert_ne!(key_pair1.public_key, key_pair2.public_key);
    assert_ne!(key_pair1.private_key, key_pair2.private_key);
}
