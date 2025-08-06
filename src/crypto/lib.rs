//! Quantum-safe cryptography engine implementation

use base64::engine::general_purpose;
use base64::Engine;
use hmac::{Hmac, Mac};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use serde_json;
use sha3::{Digest, Sha3_256};
use std::string::String;

/// Quantum-safe key pair
#[derive(Serialize, Deserialize, Debug)]
pub struct KeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

/// Generate a new quantum-safe key pair
pub fn generate_key_pair() -> KeyPair {
    let mut rng = thread_rng();
    let private_key: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    let public_key = hash(&private_key);

    KeyPair {
        public_key,
        private_key,
    }
}

/// Hash function using SHA3-256
fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

/// Sign a message using quantum-safe signature
pub fn sign_message(message: &[u8], private_key: &[u8]) -> Vec<u8> {
    let mut mac =
        Hmac::<Sha3_256>::new_from_slice(private_key).expect("HMAC can take key of any size");
    mac.update(message);
    mac.finalize().into_bytes().to_vec()
}

/// Verify a signature
pub fn verify_signature(message: &[u8], signature: &[u8], private_key: &[u8]) -> bool {
    let mut mac =
        Hmac::<Sha3_256>::new_from_slice(private_key).expect("HMAC can take key of any size");
    mac.update(message);
    let computed_signature = mac.finalize().into_bytes().to_vec();
    signature == computed_signature
}

/// Convert key pair to JSON string
pub fn key_pair_to_json(key_pair: &KeyPair) -> String {
    serde_json::to_string(key_pair).expect("Failed to serialize key pair")
}

/// Convert JSON string to key pair
pub fn key_pair_from_json(json: &str) -> Result<KeyPair, serde_json::Error> {
    serde_json::from_str(json)
}

/// Convert signature to base64 string
pub fn signature_to_base64(signature: &[u8]) -> String {
    let mut buffer = Vec::new();
    general_purpose::STANDARD
        .encode_slice(signature, &mut buffer)
        .unwrap();
    String::from_utf8(buffer).unwrap()
}

/// Convert base64 string to signature
pub fn signature_from_base64(base64_str: &str) -> Result<Vec<u8>, base64::DecodeError> {
    let mut buffer = Vec::new();
    general_purpose::STANDARD.decode_vec(base64_str, &mut buffer)?;
    Ok(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let message = b"Test message";
        let key_pair = generate_key_pair();

        let signature = sign_message(message, &key_pair.private_key);
        assert!(verify_signature(message, &signature, &key_pair.private_key));
    }

    #[test]
    fn test_hash() {
        let data = b"Hello, world!";
        let hash1 = hash(data);
        let hash2 = hash(data);
        assert_eq!(hash1, hash2);
    }
}
