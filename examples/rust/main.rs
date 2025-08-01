use quantumsafe_finance::crypto;

fn main() {
    // Example 1: Key Generation
    let key_pair = crypto::generate_key_pair();
    println!("Generated key pair:");
    println!("Public key length: {} bytes", key_pair.public_key.len());
    println!("Private key length: {} bytes", key_pair.private_key.len());

    // Example 2: Signing and Verification
    let message = "Hello, quantum world!";
    let signature = crypto::sign_message(message.as_bytes(), &key_pair.private_key);
    
    let is_valid = crypto::verify_signature(
        message.as_bytes(),
        &signature,
        &key_pair.public_key
    );

    println!("\nSignature verification: {}", is_valid);

    // Example 3: Key Pair Serialization
    let key_pair_json = crypto::key_pair_to_json(&key_pair);
    println!("\nSerialized key pair: {}", key_pair_json);

    let deserialized_key_pair = crypto::key_pair_from_json(&key_pair_json).unwrap();
    println!("Deserialized key pair: {:?}", deserialized_key_pair);

    // Example 4: Performance Benchmark
    let start = std::time::Instant::now();
    for _ in 0..1000 {
        crypto::sign_message(message.as_bytes(), &key_pair.private_key);
    }
    let duration = start.elapsed();
    
    println!("\nPerformance benchmark:");
    println!("Signatures per second: {}", 1000.0 / duration.as_secs_f64());
}
