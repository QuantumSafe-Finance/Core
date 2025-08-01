use quantumsafe_finance::crypto;
use criterion::{criterion_group, criterion_main, Criterion};

fn key_generation_benchmark(c: &mut Criterion) {
    c.bench_function("key generation", |b| b.iter(|| crypto::generate_key_pair()));
}

fn signature_benchmark(c: &mut Criterion) {
    let message = "Hello, quantum world!";
    let key_pair = crypto::generate_key_pair();
    
    c.bench_function("sign message", |b| {
        b.iter(|| crypto::sign_message(message.as_bytes(), &key_pair.private_key))
    });
}

fn verification_benchmark(c: &mut Criterion) {
    let message = "Hello, quantum world!";
    let key_pair = crypto::generate_key_pair();
    let signature = crypto::sign_message(message.as_bytes(), &key_pair.private_key);
    
    c.bench_function("verify signature", |b| {
        b.iter(|| crypto::verify_signature(
            message.as_bytes(),
            &signature,
            &key_pair.public_key
        ))
    });
}

fn serialization_benchmark(c: &mut Criterion) {
    let key_pair = crypto::generate_key_pair();
    
    c.bench_function("key pair serialization", |b| {
        b.iter(|| crypto::key_pair_to_json(&key_pair))
    });
}

criterion_group!(benches, 
    key_generation_benchmark,
    signature_benchmark,
    verification_benchmark,
    serialization_benchmark
);
criterion_main!(benches);
