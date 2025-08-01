use quantumsafe_finance::crypto;
use std::str::FromStr;

#[test]
fn test_signature_flow() {
    let message = "Hello, quantum world!";
    let key_pair = crypto::generate_key_pair();
    
    // Sign message
    let signature = crypto::sign_message(message.as_bytes(), &key_pair.private_key);
    assert!(!signature.is_empty());
    
    // Verify signature
    let is_valid = crypto::verify_signature(
        message.as_bytes(),
        &signature,
        &key_pair.public_key
    );
    
    assert!(is_valid);
}

#[test]
fn test_invalid_signature() {
    let message = "Hello, quantum world!";
    let key_pair = crypto::generate_key_pair();
    
    // Sign message
    let signature = crypto::sign_message(message.as_bytes(), &key_pair.private_key);
    
    // Try to verify with different message
    let is_valid = crypto::verify_signature(
        "Different message".as_bytes(),
        &signature,
        &key_pair.public_key
    );
    
    assert!(!is_valid);
}

#[test]
fn test_serialized_signature() {
    let message = "Hello, quantum world!";
    let key_pair = crypto::generate_key_pair();
    
    // Sign message
    let signature = crypto::sign_message(message.as_bytes(), &key_pair.private_key);
    
    // Serialize signature
    let signature_base64 = crypto::signature_to_base64(&signature);
    assert!(!signature_base64.is_empty());
    
    // Deserialize signature
    let deserialized_signature = crypto::signature_from_base64(&signature_base64).unwrap();
    assert_eq!(signature, deserialized_signature);

    // Verify deserialized signature
    let is_valid = crypto::verify_signature(
        message.as_bytes(),
        &deserialized_signature,
        &key_pair.public_key
    );
    
    assert!(is_valid);
}
