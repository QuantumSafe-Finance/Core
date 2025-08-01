# QuantumSafe Finance

[![GitHub](https://img.shields.io/github/license/QuantumSafe-Finance/Core)](LICENSE)
[![CI](https://github.com/QuantumSafe-Finance/Core/actions/workflows/ci.yml/badge.svg)](https://github.com/QuantumSafe-Finance/Core/actions/workflows/ci.yml)
[![Documentation](https://img.shields.io/badge/docs-latest-blue)](https://quantumsafe-finance.github.io/Core)

A quantum-resistant cryptography solution for financial applications, providing post-quantum secure key management and signature capabilities.

## Features

- 🛡️ Post-Quantum Cryptography (PQC) implementations:
  - Key Encapsulation Mechanisms (KEM): Kyber
  - Digital Signatures: Dilithium, Falcon, SPHINCS+
- 🌐 Multi-language bindings:
  - C (FFI)
  - Python
  - TypeScript/WASM
- 🔒 Memory-safe design with Rust
- 📊 Comprehensive test coverage
- 📚 Extensive documentation

## Project Structure

```
Core/
├── src/              # Source code
│   ├── crypto/      # PQC implementation
│   │   ├── kem/     # Key Encapsulation Mechanisms
│   │   └── sig/     # Digital Signatures
│   └── bindings/    # Language bindings
│       ├── c/       # C bindings
│       ├── python/  # Python bindings
│       └── typescript/ # TypeScript bindings
├── docs/            # Documentation
│   ├── architecture/ # System architecture
│   ├── api/         # API documentation
│   ├── examples/    # Examples and tutorials
│   ├── security/    # Security considerations
│   ├── contributing/ # Contributing guidelines
│   └── deployment/  # Deployment guides
├── tests/           # Test suite
└── examples/        # Example usage
```

## Getting Started

1. Prerequisites:
   - Rust (latest stable version)
   - CMake (for C bindings)
   - Git
   - Docker (for CI/CD)

2. Clone the repository:
   ```bash
   git clone https://github.com/QuantumSafe-Finance/Core.git
   cd Core
   ```

3. Install dependencies:
   ```bash
   rustup install stable
   rustup default stable
   ```

4. Build the project:
   ```bash
   cargo build
   ```

5. Run tests:
   ```bash
   cargo test
   ```

## Usage

### Rust
```rust
use quantumsafe_finance::crypto;

// Generate key pair
let key_pair = crypto::generate_key_pair();

// Sign message
let signature = crypto::sign_message("Hello, quantum world!", &key_pair.private_key);

// Verify signature
let is_valid = crypto::verify_signature(
    "Hello, quantum world!",
    &signature,
    &key_pair.public_key
);
```

### Python
```python
from quantumsafe_finance import KeyPairWrapper, sign_message, verify_signature

# Generate key pair
key_pair = KeyPairWrapper.generate()

# Sign message
signature = sign_message("Hello, quantum world!", key_pair.private_key)

# Verify signature
is_valid = verify_signature(
    "Hello, quantum world!",
    signature.signature,
    key_pair.public_key
)
```

## Documentation

- [Architecture](docs/architecture/README.md)
- [API Reference](docs/api/README.md)
- [Security Considerations](docs/security/README.md)
- [Contributing](docs/contributing/README.md)
- [Deployment Guides](docs/deployment/README.md)

## Security

For security-related issues, please refer to our [Security Policy](SECURITY.md).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Please read our [Contributing Guidelines](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## Support

For support, please:
- Open an issue on GitHub
- Join our community forum
- Contact us at support@quantumsafe-finance.com

## Development

Run tests:
```bash
cargo test
```

Run examples:
```bash
cargo run --example <example-name>
```
