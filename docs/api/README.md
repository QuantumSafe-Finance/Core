# QuantumSafe Finance API Reference

## Core API

### Key Management

```rust
pub struct KeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
}

impl KeyPair {
    /// Generate a new key pair
    pub fn generate() -> Self;

    /// Export key pair to JSON
    pub fn to_json(&self) -> String;

    /// Import key pair from JSON
    pub fn from_json(json: &str) -> Result<Self, Error>;
}
```

### Signature Operations

```rust
pub fn sign_message(
    message: &[u8],
    private_key: &[u8]
) -> Result<Vec<u8>, Error>;

pub fn verify_signature(
    message: &[u8],
    signature: &[u8],
    public_key: &[u8]
) -> Result<bool, Error>;
```

## C Bindings

### Key Management

```c
// Generate key pair
int generate_key_pair(
    uint8_t** public_key,
    size_t* public_key_len,
    uint8_t** private_key,
    size_t* private_key_len
);

// Free allocated memory
void free_key_pair(uint8_t* key);
```

### Signature Operations

```c
// Sign message
int sign_message(
    const uint8_t* message,
    size_t message_len,
    const uint8_t* private_key,
    size_t private_key_len,
    uint8_t** signature,
    size_t* signature_len
);

// Verify signature
int verify_signature(
    const uint8_t* message,
    size_t message_len,
    const uint8_t* signature,
    size_t signature_len,
    const uint8_t* public_key,
    size_t public_key_len
);
```

## Python Bindings

### Key Management

```python
class KeyPairWrapper:
    @classmethod
    def generate(cls) -> 'KeyPairWrapper':
        """Generate a new key pair"""
        
    @property
    def public_key(self) -> bytes:
        """Get public key"""
        
    @property
    def private_key(self) -> bytes:
        """Get private key"""
        
    def to_json(self) -> str:
        """Export to JSON"""
        
    @classmethod
    def from_json(cls, json_str: str) -> 'KeyPairWrapper':
        """Import from JSON"""
```

### Signature Operations

```python
def sign_message(message: str, private_key: bytes) -> bytes:
    """Sign a message using quantum-safe signature"""

def verify_signature(
    message: str, 
    signature: bytes, 
    public_key: bytes
) -> bool:
    """Verify a quantum-safe signature"""
```

## TypeScript Bindings

### Key Management

```typescript
interface KeyPair {
    publicKey: Uint8Array;
    privateKey: Uint8Array;
}

class KeyPairWrapper {
    static generate(): Promise<KeyPair>;
    
    static fromJson(json: string): Promise<KeyPair>;
    
    static toJson(keyPair: KeyPair): Promise<string>;
}
```

### Signature Operations

```typescript
async function signMessage(
    message: string,
    privateKey: Uint8Array
): Promise<Uint8Array>;

async function verifySignature(
    message: string,
    signature: Uint8Array,
    publicKey: Uint8Array
): Promise<boolean>;
```

## Error Handling

### Rust

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid key length")]
    InvalidKeyLength,
    
    #[error("Invalid signature")]
    InvalidSignature,
    
    #[error("Memory allocation failed")]
    MemoryError,
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
}
```

### Python

```python
class QuantumSafeError(Exception):
    """Base exception for all QuantumSafe errors"""

class InvalidKeyError(QuantumSafeError):
    """Invalid key provided"""

class InvalidSignatureError(QuantumSafeError):
    """Invalid signature"""

class MemoryError(QuantumSafeError):
    """Memory allocation failed"""
```

### TypeScript

```typescript
export class QuantumSafeError extends Error {
    constructor(message: string) {
        super(message);
        this.name = 'QuantumSafeError';
    }
}

export class InvalidKeyError extends QuantumSafeError {
    constructor() {
        super('Invalid key provided');
    }
}

export class InvalidSignatureError extends QuantumSafeError {
    constructor() {
        super('Invalid signature');
    }
}
```

## Performance Considerations

### Key Generation
- Average time: 10-20ms
- Memory usage: ~1KB

### Signature Operations
- Signing: 20-30ms
- Verification: 15-25ms
- Memory usage: ~2KB

## Security Considerations

### Key Management
- Secure memory allocation
- Zeroing on drop
- Constant-time operations
- Side-channel protection

### Cryptographic Operations
- Post-quantum secure
- Resistance to timing attacks
- Secure random number generation
- Memory isolation
