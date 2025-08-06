//! Python bindings for QuantumSafe Finance

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::PyType;
use serde::ser::SerializeMap;
use base64::{engine::general_purpose::STANDARD, Engine};
use crate::crypto;

/// Quantum-safe key pair
#[pyclass]
pub struct KeyPairWrapper {
    #[pyo3(get)]
    pub public_key: Vec<u8>,
    #[pyo3(get)]
    pub private_key: Vec<u8>,
}

#[pymethods]
impl KeyPairWrapper {
    #[new]
    fn new() -> PyResult<Self> {
        let key_pair = crypto::generate_key_pair();
        Ok(Self {
            public_key: key_pair.public_key,
            private_key: key_pair.private_key,
        })
    }

    fn __repr__(&self) -> String {
        format!("KeyPair(public_key_len={}, private_key_len={}", 
            self.public_key.len(),
            self.private_key.len()
        )
    }

    #[classmethod]
    pub fn from_json(_cls: &PyType, json: &str) -> PyResult<Self> {
        let data: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| PyValueError::new_err(format!("Invalid JSON: {}", e)))?;
        let public_key = data["public_key"].as_str()
            .ok_or_else(|| PyValueError::new_err("Missing public_key in JSON"))?
            .as_bytes()
            .to_vec();
        let private_key = data["private_key"].as_str()
            .ok_or_else(|| PyValueError::new_err("Missing private_key in JSON"))?
            .as_bytes()
            .to_vec();
        Ok(Self { public_key, private_key })
    }

    #[classmethod]
    pub fn to_json(_cls: &PyType, obj: &PyAny) -> PyResult<String> {
        let _py = obj.py();
        let public_key = obj.getattr("public_key")?.extract::<Vec<u8>>()?;
        let private_key = obj.getattr("private_key")?.extract::<Vec<u8>>()?;
        let json = serde_json::json!({
            "public_key": STANDARD.encode(&public_key),
            "private_key": STANDARD.encode(&private_key),
        });
        Ok(json.to_string())
    }
}

impl serde::Serialize for KeyPairWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let public_key_base64 = base64::engine::general_purpose::STANDARD.encode(&self.public_key);
        let private_key_base64 = base64::engine::general_purpose::STANDARD.encode(&self.private_key);
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("public_key", &public_key_base64)?;
        map.serialize_entry("private_key", &private_key_base64)?;
        map.end()
    }
}

/// Quantum-safe signature
#[pyclass]
#[derive(Debug)]
pub struct Signature {
    pub signature: Vec<u8>,
}

#[pymethods]
impl Signature {
    #[new]
    fn new(signature: Vec<u8>) -> Self {
        Self { signature }
    }

    fn __repr__(&self) -> String {
        format!("Signature(len={})", self.signature.len())
    }
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

/// Main module
#[pymodule]
fn quantumsafe_finance(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<KeyPairWrapper>()?;
    m.add_class::<Signature>()?;

    m.add_function(wrap_pyfunction!(sign_message, m)?)?;
    m.add_function(wrap_pyfunction!(verify_signature, m)?)?;
    Ok(())
}

/// Sign a message using quantum-safe signature
#[pyfunction]
pub fn sign_message(message: &str, private_key: &[u8]) -> PyResult<Signature> {
    let signature = crypto::sign_message(message.as_bytes(), private_key);
    Ok(Signature { signature })
}

/// Verify a signature
#[pyfunction]
pub fn verify_signature(message: &str, signature: &[u8], public_key: &[u8]) -> PyResult<bool> {
    Ok(crypto::verify_signature(message.as_bytes(), signature, public_key))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pyo3::Python;

    #[test]
    fn test_python_bindings() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let key_pair = KeyPairWrapper::new().unwrap();
            let message = "Test message";
            
            let signature = sign_message(message, &key_pair.private_key).unwrap();
            assert!(verify_signature(message, &signature.signature, &key_pair.private_key).unwrap());
        });
    }
}
