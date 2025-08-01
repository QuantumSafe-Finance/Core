import { KeyPairWrapper, signMessage, verifySignature } from 'quantumsafe-finance';
import { performance } from 'perf_hooks';

async function main() {
    // Example 1: Key Generation
    console.log('Generating key pair...');
    const keyPair = await KeyPairWrapper.generate();
    console.log(`Public key length: ${keyPair.publicKey.length} bytes`);
    console.log(`Private key length: ${keyPair.privateKey.length} bytes`);

    // Example 2: Signing and Verification
    const message = "Hello, quantum world!";
    console.log('\nSigning message...');
    const signature = await signMessage(message, keyPair.privateKey);
    
    console.log('\nVerifying signature...');
    const is_valid = await verifySignature(
        message,
        signature,
        keyPair.publicKey
    );
    console.log(`Signature verification: ${is_valid}`);

    // Example 3: Key Pair Serialization
    console.log('\nSerializing key pair...');
    const keyPairJson = await KeyPairWrapper.toJson(keyPair);
    console.log(`Serialized key pair: ${keyPairJson}`);

    console.log('\nDeserializing key pair...');
    const deserializedKeyPair = await KeyPairWrapper.fromJson(keyPairJson);
    console.log(`Deserialized key pair: ${deserializedKeyPair}`);

    // Example 4: Performance Benchmark
    console.log('\nRunning performance benchmark...');
    const start = performance.now();
    for (let i = 0; i < 1000; i++) {
        await signMessage(message, keyPair.privateKey);
    }
    const duration = performance.now() - start;
    
    console.log('\nPerformance results:');
    console.log(`Signatures per second: ${(1000 / (duration / 1000)).toFixed(2)}`);
}

main().catch(console.error);
