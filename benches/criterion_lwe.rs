use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion, BatchSize};
use rslwe::lwe::lwe_cypher::LweCypher;
use rslwe::lwe::lwe_params::LweParams;

fn bench_lwe_operations(c: &mut Criterion) {
    let params = LweParams { n: 512, m: 1024, q: 3329 };
    let mut engine = LweCypher::new(params, 12345);
    let (pk, sk) = engine.keygen();
    let ct = engine.encrypt(&pk, true);

    // --- Keygen ---
    c.bench_function("LWE Keygen", |b| {
        b.iter(|| black_box(engine.keygen()))
    });

    // --- Encrypt ---
    c.bench_function("LWE Encrypt", |b| {
        b.iter(|| black_box(engine.encrypt(black_box(&pk), black_box(true))))
    });

    // --- Decrypt ---
    c.bench_function("LWE Decrypt", |b| {
        b.iter_batched(
            || ct.clone(),
            |ct_clone| engine.decrypt(black_box(&sk), black_box(ct_clone)),
            BatchSize::SmallInput
        )
    });
}

criterion_group!(benches, bench_lwe_operations);
criterion_main!(benches);