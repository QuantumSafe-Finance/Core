//! Language bindings for QuantumSafe Finance

/// C bindings
pub mod c;

/// Python bindings
pub mod python;

/// TypeScript bindings
pub mod typescript;

pub use c::{KeyPairWrapper as CKeyPairWrapper, sign_message as c_sign_message, verify_signature as c_verify_signature};
pub use python::{KeyPairWrapper as PythonKeyPairWrapper, Signature as PythonSignature, sign_message as python_sign_message, verify_signature as python_verify_signature};
pub use typescript::{KeyPairWrapper as TypeScriptKeyPairWrapper, Signature as TypeScriptSignature, sign_message as typescript_sign_message, verify_signature as typescript_verify_signature};


