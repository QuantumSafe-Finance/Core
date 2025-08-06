//! Language bindings for QuantumSafe Finance

/// C bindings
pub mod c;

/// Python bindings
pub mod python;

/// TypeScript bindings
pub mod typescript;

pub use c::{
    sign_message as c_sign_message, verify_signature as c_verify_signature,
    KeyPairWrapper as CKeyPairWrapper,
};
pub use python::{
    sign_message as python_sign_message, verify_signature as python_verify_signature,
    KeyPairWrapper as PythonKeyPairWrapper, Signature as PythonSignature,
};
pub use typescript::{
    sign_message as typescript_sign_message, verify_signature as typescript_verify_signature,
    KeyPairWrapper as TypeScriptKeyPairWrapper, Signature as TypeScriptSignature,
};
