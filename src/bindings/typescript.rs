//! TypeScript bindings for QuantumSafe Finance

use crate::crypto;
use base64::engine::general_purpose;
use base64::Engine;
use js_sys::Uint8Array;
use serde::{ser::SerializeMap, Deserialize};
use std::string::String;
use wasm_bindgen::prelude::*;

/// Quantum-safe key pair
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct KeyPairWrapper {
    #[wasm_bindgen(skip)]
    pub public_key: Box<[u8]>,
    #[wasm_bindgen(skip)]
    pub private_key: Box<[u8]>,
}

impl serde::Serialize for KeyPairWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let public_key_base64 = base64::engine::general_purpose::STANDARD.encode(&self.public_key);
        let private_key_base64 =
            base64::engine::general_purpose::STANDARD.encode(&self.private_key);
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("public_key", &public_key_base64)?;
        map.serialize_entry("private_key", &private_key_base64)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for KeyPairWrapper {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct KeyPairData {
            public_key: String,
            private_key: String,
        }

        let data = KeyPairData::deserialize(deserializer)?;
        let public_key = base64::engine::general_purpose::STANDARD
            .decode(&data.public_key)
            .map_err(|e| {
                serde::de::Error::custom(format!("Invalid base64 in public_key: {}", e))
            })?;
        let private_key = base64::engine::general_purpose::STANDARD
            .decode(&data.private_key)
            .map_err(|e| {
                serde::de::Error::custom(format!("Invalid base64 in private_key: {}", e))
            })?;

        Ok(Self {
            public_key: public_key.into_boxed_slice(),
            private_key: private_key.into_boxed_slice(),
        })
    }
}

#[wasm_bindgen]
impl KeyPairWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let key_pair = crypto::generate_key_pair();
        Self {
            public_key: key_pair.public_key.into_boxed_slice(),
            private_key: key_pair.private_key.into_boxed_slice(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn public_key(&self) -> Uint8Array {
        Uint8Array::from(&self.public_key[..])
    }

    #[wasm_bindgen(getter)]
    pub fn private_key(&self) -> Uint8Array {
        Uint8Array::from(&self.private_key[..])
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

/// Quantum-safe signature
#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Signature {
    #[wasm_bindgen(skip)]
    pub signature: Box<[u8]>,
}

impl serde::Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let signature_base64 = base64::engine::general_purpose::STANDARD.encode(&self.signature);
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry("signature", &signature_base64)?;
        map.end()
    }
}

impl<'de> serde::Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct SignatureData {
            signature: String,
        }

        let data = SignatureData::deserialize(deserializer)?;
        let signature = base64::engine::general_purpose::STANDARD
            .decode(&data.signature)
            .map_err(|e| serde::de::Error::custom(format!("Invalid base64 in signature: {}", e)))?;

        Ok(Self {
            signature: signature.into_boxed_slice(),
        })
    }
}

#[wasm_bindgen]
impl Signature {
    #[wasm_bindgen(constructor)]
    pub fn new(signature: &[u8]) -> Self {
        Self {
            signature: signature.to_vec().into_boxed_slice(),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn signature(&self) -> Uint8Array {
        Uint8Array::from(&self.signature[..])
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

/// Sign a message using quantum-safe signature
#[wasm_bindgen]
pub fn sign_message(message: &[u8], private_key: &[u8]) -> Signature {
    let signature = crypto::sign_message(message, private_key);
    Signature::new(&signature)
}

/// Verify a signature
#[wasm_bindgen]
pub fn verify_signature(message: &[u8], signature: &[u8], public_key: &[u8]) -> bool {
    crypto::verify_signature(message, signature, public_key)
}

/// Convert Rust Vec<u8> to JavaScript Uint8Array
#[wasm_bindgen]
pub fn vec_to_uint8array(vec: Vec<u8>) -> js_sys::Uint8Array {
    let array = js_sys::Uint8Array::from(vec.as_slice());
    std::mem::forget(vec); // Prevent double-free
    array
}

/// Convert JavaScript Uint8Array to Rust Vec<u8>
#[wasm_bindgen]
pub fn uint8array_to_vec(array: js_sys::Uint8Array) -> Vec<u8> {
    let vec = array.to_vec();
    std::mem::forget(array); // Prevent double-free
    vec
}

/// Convert signature to base64 string
#[wasm_bindgen]
pub fn signature_to_base64(signature: &[u8]) -> String {
    let mut buffer = String::new();
    general_purpose::STANDARD.encode_string(signature, &mut buffer);
    buffer
}

/// Convert base64 string to signature
#[wasm_bindgen]
pub fn signature_from_base64(base64_str: &str) -> Result<Vec<u8>, JsValue> {
    let mut buffer = Vec::new();
    general_purpose::STANDARD
        .decode_vec(base64_str, &mut buffer)
        .map_err(|e| JsValue::from_str(&format!("Invalid base64: {}", e)))
        .map(|_| buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_arch = "wasm32")]
    #[test]
    fn test_key_pair() {
        let key_pair = KeyPairWrapper::new();
        assert_eq!(key_pair.public_key().length(), 32);
        assert_eq!(key_pair.private_key().length(), 32);
    }

    #[cfg(target_arch = "wasm32")]
    #[test]
    fn test_signature() {
        let key_pair = KeyPairWrapper::new();
        let message = b"Test message";
        let private_key = uint8array_to_vec(key_pair.private_key());
        let signature = sign_message(message, &private_key);
        let signature_array = signature.signature();
        let signature_slice = uint8array_to_vec(signature_array);
        let public_key = uint8array_to_vec(key_pair.public_key());
        assert!(verify_signature(message, &signature_slice, &public_key));
    }

    #[cfg(target_arch = "wasm32")]
    #[test]
    fn test_serialization() {
        let key_pair = KeyPairWrapper::new();
        let json = key_pair.to_json();
        let deserialized: KeyPairWrapper = serde_json::from_str(&json).unwrap();
        assert_eq!(
            key_pair.public_key().to_vec(),
            deserialized.public_key().to_vec()
        );
        assert_eq!(
            key_pair.private_key().to_vec(),
            deserialized.private_key().to_vec()
        );
    }
}
