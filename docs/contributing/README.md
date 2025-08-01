# Contributing to QuantumSafe Finance

Thank you for considering contributing to QuantumSafe Finance! We welcome contributions from the community to help make this project better.

## Code of Conduct

Please note that this project is governed by our [Code of Conduct](../../CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How to Contribute

### 1. Getting Started

1. Clone the repository:
   ```bash
   git clone https://github.com/QuantumSafe-Finance/Core.git
   cd Core
   ```

2. Install dependencies:
   ```bash
   rustup install stable
   rustup default stable
   ```

### 2. Development Setup

1. Create a development environment:
   ```bash
   cargo build
   cargo test
   ```

2. Run the development server:
   ```bash
   cargo run
   ```

### 3. Development Guidelines

#### Code Style

- Follow Rust's official style guidelines
- Use meaningful commit messages
- Document public APIs
- Write tests for new features
- Keep pull requests small and focused

#### Testing

- Write tests for new features
- Run existing tests before committing
- Ensure test coverage is maintained
- Document test cases

#### Documentation

- Document all public APIs
- Update documentation with new features
- Keep examples up to date
- Document security considerations

### 4. Submitting Changes

1. Fork the repository and create your branch from `main`:
   ```bash
   git checkout -b feature/your-feature
   ```

2. Make your changes

3. Commit your changes:
   ```bash
   git commit -m "feat: your feature description"
   ```

4. Push to the branch:
   ```bash
   git push origin feature/your-feature
   ```

5. Create a Pull Request

### 5. Code Review Process

1. Your PR will be reviewed by maintainers
2. You may be asked to make changes
3. Once approved, your changes will be merged

### 6. Security Issues

If you discover a security vulnerability, please send an email to security@quantumsafe-finance.com instead of using the issue tracker.

## Development Environment

### Required Tools

- Rust (latest stable version)
- CMake (for C bindings)
- Git
- Docker (for CI/CD)

### Optional Tools

- IDE with Rust support
- Docker Desktop
- GitHub CLI

## Testing

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with coverage
cargo tarpaulin
```

### Performance Testing

```bash
# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench benchmark_name
```

## Documentation

### Building Documentation

```bash
# Build documentation
cargo doc

# Open documentation
xdg-open target/doc/quantumsafe_finance/index.html
```

## Security

### Security Testing

```bash
# Run security checks
cargo clippy

# Run fuzz testing
cargo fuzz run fuzz_target
```

### Security Best Practices

- Follow security guidelines
- Implement proper error handling
- Use secure memory allocation
- Implement constant-time operations
- Validate all inputs

## Community

### Communication

- GitHub Issues for bug reports and feature requests
- GitHub Discussions for general questions
- Email for security issues

### Support

- Open an issue on GitHub
- Join our community forum
- Contact us at support@quantumsafe-finance.com

## License

By contributing to QuantumSafe Finance, you agree that your contributions will be licensed under the [MIT License](../../LICENSE).
