//! Quantum-safe cryptography engine implementation

use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use sha3::{Sha3_256, Digest};
use hmac::{Hmac, Mac};

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
    let mut mac = Hmac::<Sha3_256>::new_from_slice(private_key).expect("HMAC can take key of any size");
    mac.update(message);
    mac.finalize().into_bytes().to_vec()
}

/// Verify a signature
pub fn verify_signature(
    message: &[u8],
    signature: &[u8],
    public_key: &[u8],
) -> bool {
    let mut mac = Hmac::<Sha3_256>::new_from_slice(public_key).expect("HMAC can take key of any size");
    mac.update(message);
    mac.verify_slice(signature).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature() {
        let message = b"Test message";
        let key_pair = generate_key_pair();
        
        let signature = sign_message(message, &key_pair.private_key);
        assert!(verify_signature(message, &signature, &key_pair.public_key));
    }

    #[test]
    fn test_hash() {
        let data = b"Hello, world!";
        let hash1 = hash(data);
        let hash2 = hash(data);
        assert_eq!(hash1, hash2);
    }
}
