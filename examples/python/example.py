from quantumsafe_finance import KeyPairWrapper, sign_message, verify_signature
import time

def main():
    # Example 1: Key Generation
    print("Generating key pair...")
    key_pair = KeyPairWrapper.generate()
    print(f"Public key length: {len(key_pair.public_key)} bytes")
    print(f"Private key length: {len(key_pair.private_key)} bytes")

    # Example 2: Signing and Verification
    message = "Hello, quantum world!"
    print("\nSigning message...")
    signature = sign_message(message, key_pair.private_key)
    
    print("\nVerifying signature...")
    is_valid = verify_signature(
        message,
        signature.signature,
        key_pair.public_key
    )
    print(f"Signature verification: {is_valid}")

    # Example 3: Key Pair Serialization
    print("\nSerializing key pair...")
    key_pair_json = key_pair.to_json()
    print(f"Serialized key pair: {key_pair_json}")

    print("\nDeserializing key pair...")
    deserialized_key_pair = KeyPairWrapper.from_json(key_pair_json)
    print(f"Deserialized key pair: {deserialized_key_pair}")

    # Example 4: Performance Benchmark
    print("\nRunning performance benchmark...")
    start = time.time()
    for _ in range(1000):
        sign_message(message, key_pair.private_key)
    duration = time.time() - start
    
    print(f"\nPerformance results:")
    print(f"Signatures per second: {1000 / duration:.2f}")

if __name__ == "__main__":
    main()
