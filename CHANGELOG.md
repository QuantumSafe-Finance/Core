# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2025-08-01

### Added
- Rust PQC engine implementation with Kyber, Dilithium, Falcon, and SPHINCS+
- Multi-language bindings (C, Python, TypeScript)
- Memory-safe FFI interface
- JSON serialization with base64 encoding
- Comprehensive test suite
- CI/CD pipeline

### Changed
- Improved memory management in C bindings
- Updated base64 usage to latest API
- Enhanced error handling

### Fixed
- Various memory safety issues
- Base64 encoding/decoding bugs
- Serialization/deserialization edge cases

## [0.1.0] - 2025-07-15

### Added
- Initial project structure
- Basic PQC implementations
- Initial test suite

[Unreleased]: https://github.com/QuantumSafe-Finance/Core/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/QuantumSafe-Finance/Core/compare/v0.1.0...v1.0.0
[0.1.0]: https://github.com/QuantumSafe-Finance/Core/releases/tag/v0.1.0
