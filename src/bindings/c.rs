#![allow(clippy::not_unsafe_ptr_arg_deref)]

//! C bindings for QuantumSafe Finance

use crate::crypto;
use std::os::raw::c_int;
use std::slice;

/// Quantum-safe key pair
#[repr(C)]
pub struct KeyPairWrapper {
    pub public_key_len: usize,
    pub private_key_len: usize,
    pub public_key: *mut u8,
    pub private_key: *mut u8,
}

/// Create a new quantum-safe key pair
#[no_mangle]
pub extern "C" fn create_key_pair() -> *mut KeyPairWrapper {
    let rust_key_pair = crypto::generate_key_pair();
    let key_pair = Box::new(KeyPairWrapper {
        public_key_len: rust_key_pair.public_key.len(),
        private_key_len: rust_key_pair.private_key.len(),
        public_key: Box::into_raw(rust_key_pair.public_key.into_boxed_slice()) as *mut u8,
        private_key: Box::into_raw(rust_key_pair.private_key.into_boxed_slice()) as *mut u8,
    });

    Box::into_raw(key_pair)
}

/// Free a quantum-safe key pair
#[no_mangle]
pub extern "C" fn free_key_pair(key_pair: *mut KeyPairWrapper) {
    if !key_pair.is_null() {
        let key_pair = unsafe { Box::from_raw(key_pair) };
        // Free public and private keys
        unsafe {
            drop(Box::from_raw(key_pair.public_key));
            drop(Box::from_raw(key_pair.private_key));
        }
        drop(key_pair);
    }
}

/// Sign a message using quantum-safe signature
#[no_mangle]
pub extern "C" fn sign_message(
    message: *const u8,
    message_len: usize,
    private_key: *const u8,
    private_key_len: usize,
    signature: *mut *mut u8,
    signature_len: *mut usize,
) -> c_int {
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let private_key = unsafe { std::slice::from_raw_parts(private_key, private_key_len) };
    let signature_result = crypto::sign_message(message, private_key);
    let result_len = signature_result.len();
    let signature_ptr = Box::into_raw(signature_result.into_boxed_slice()) as *mut u8;
    unsafe {
        *signature = signature_ptr;
        *signature_len = result_len;
    }
    0
}

/// Free a signature
#[no_mangle]
pub extern "C" fn free_signature(signature: *mut u8) {
    if signature.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(signature));
    }
}

/// Verify a quantum-safe signature
#[no_mangle]
pub extern "C" fn verify_signature(
    message: *const u8,
    message_len: usize,
    signature: *const u8,
    signature_len: usize,
    private_key: *const u8,
    private_key_len: usize,
) -> c_int {
    let message_slice = unsafe { slice::from_raw_parts(message, message_len) };
    let signature_slice = unsafe { slice::from_raw_parts(signature, signature_len) };
    let private_key_slice = unsafe { slice::from_raw_parts(private_key, private_key_len) };

    if crypto::verify_signature(message_slice, signature_slice, private_key_slice) {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_bindings() {
        // Test key pair generation
        let key_pair = create_key_pair();
        assert!(!key_pair.is_null());

        // Test signing
        let message = b"Test message";
        let mut signature: *mut u8 = std::ptr::null_mut();
        let mut signature_len: usize = 0;
        let sign_result = unsafe {
            sign_message(
                message.as_ptr(),
                message.len(),
                (*key_pair).private_key,
                (*key_pair).private_key_len,
                &mut signature,
                &mut signature_len,
            )
        };
        assert_eq!(sign_result, 0);
        assert!(!signature.is_null());

        // Test verification
        let result = unsafe {
            verify_signature(
                message.as_ptr(),
                message.len(),
                signature,
                signature_len,
                (*key_pair).private_key,
                (*key_pair).private_key_len,
            )
        };
        assert_eq!(result, 1);

        // Free allocated memory
        free_signature(signature);
        free_key_pair(key_pair);
    }
}
