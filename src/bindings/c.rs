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

/// Generate a quantum-safe key pair
#[no_mangle]
pub extern "C" fn generate_key_pair() -> *mut KeyPairWrapper {
    let key_pair = crypto::generate_key_pair();
    let public_key = Box::into_raw(key_pair.public_key.clone().into_boxed_slice());
    let private_key = Box::into_raw(key_pair.private_key.clone().into_boxed_slice());

    let result = Box::new(KeyPairWrapper {
        public_key_len: key_pair.public_key.len(),
        private_key_len: key_pair.private_key.len(),
        public_key: public_key as *mut u8,
        private_key: private_key as *mut u8,
    });
    Box::into_raw(result)
}

/// Free a quantum-safe key pair
#[no_mangle]
pub extern "C" fn free_key_pair(key_pair: *mut KeyPairWrapper) {
    if key_pair.is_null() {
        return;
    }
    let key_pair = unsafe { Box::from_raw(key_pair) };
    unsafe {
        drop(Box::from_raw(key_pair.public_key));
        drop(Box::from_raw(key_pair.private_key));
    }
}

/// Sign a message using quantum-safe signature
#[no_mangle]
pub extern "C" fn sign_message(
    message: *const u8,
    message_len: usize,
    private_key: *const u8,
    private_key_len: usize,
) -> *mut u8 {
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let private_key = unsafe { std::slice::from_raw_parts(private_key, private_key_len) };
    let signature = crypto::sign_message(message, private_key);
    Box::into_raw(signature.into_boxed_slice()) as *mut u8
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

#[no_mangle]
pub extern "C" fn verify_signature(
    message: *const u8,
    message_len: usize,
    signature: *const u8,
    signature_len: usize,
    public_key: *const u8,
    public_key_len: usize,
) -> c_int {
    let message_slice = unsafe { slice::from_raw_parts(message, message_len) };
    let signature_slice = unsafe { slice::from_raw_parts(signature, signature_len) };
    let public_key_slice = unsafe { slice::from_raw_parts(public_key, public_key_len) };

    crypto::verify_signature(message_slice, signature_slice, public_key_slice) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_bindings() {
        // Test key pair generation
        let key_pair = generate_key_pair();
        assert!(!key_pair.is_null());

        // Test signing
        let message = b"Test message";
        let signature = unsafe {
            sign_message(
                message.as_ptr(),
                message.len(),
                (*key_pair).private_key,
                (*key_pair).private_key_len,
            )
        };
        assert!(!signature.is_null());

        // Test verification
        let result = unsafe {
            verify_signature(
                message.as_ptr(),
                message.len(),
                signature,
                (*key_pair).private_key_len,
                (*key_pair).private_key,
                (*key_pair).private_key_len,
            )
        };
        assert_eq!(result, 1);

        // Clean up
        unsafe {
            free_signature(signature);
            free_key_pair(key_pair);
        }
    }
}
