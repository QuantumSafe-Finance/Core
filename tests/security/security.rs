use quantumsafe_finance::crypto;
use std::str::FromStr;

#[test]
fn test_constant_time_operations() {
    let message1 = "Hello, quantum world!";
    let message2 = "Different message";
    let key_pair = crypto::generate_key_pair();
    
    // Sign both messages
    let signature1 = crypto::sign_message(message1.as_bytes(), &key_pair.private_key);
    let signature2 = crypto::sign_message(message2.as_bytes(), &key_pair.private_key);
    
    // Verify timing is constant
    let start1 = std::time::Instant::now();
    let _ = crypto::verify_signature(
        message1.as_bytes(),
        &signature1,
        &key_pair.public_key
    );
    let duration1 = start1.elapsed();
    
    let start2 = std::time::Instant::now();
    let _ = crypto::verify_signature(
        message2.as_bytes(),
        &signature2,
        &key_pair.public_key
    );
    let duration2 = start2.elapsed();
    
    // Timing should be within 10% difference
    assert!(duration1.as_nanos() as f64 / duration2.as_nanos() as f64 >= 0.9);
    assert!(duration1.as_nanos() as f64 / duration2.as_nanos() as f64 <= 1.1);
}

#[test]
fn test_memory_safety() {
    let key_pair = crypto::generate_key_pair();
    
    // Test that private key is properly zeroed after use
    let private_key = key_pair.private_key;
    drop(key_pair);
    
    // Private key should be zeroed
    assert!(private_key.iter().all(|&b| b == 0));
}

#[test]
fn test_error_handling() {
    let message = "Hello, quantum world!";
    let key_pair = crypto::generate_key_pair();
    
    // Test invalid signature verification
    let invalid_signature = vec![0; 100];
    assert!(!crypto::verify_signature(
        message.as_bytes(),
        &invalid_signature,
        &key_pair.public_key
    ));
    
    // Test invalid key pair
    let invalid_key_pair = "invalid_json";
    assert!(crypto::key_pair_from_json(invalid_key_pair).is_err());
}

#[test]
fn test_input_validation() {
    let key_pair = crypto::generate_key_pair();
    
    // Test empty message
    assert!(crypto::sign_message(&[], &key_pair.private_key).is_empty());
    
    // Test empty signature
    assert!(!crypto::verify_signature(
        "Hello, quantum world!".as_bytes(),
        &[],
        &key_pair.public_key
    ));
}
