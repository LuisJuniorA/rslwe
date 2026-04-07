use std::hint::black_box;
use std::env;
use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use rslwe::lwe::lwe_cypher::{Ciphertext, LweCypher};
use rslwe::lwe::lwe_params::LweParams;

// The environment approach is not working. 
fn get_iters(name: &str, default: usize) -> usize {
    env::var(name)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

#[library_benchmark]
fn bench_keygen() -> usize {
    let iters = get_iters("IAI_KEYGEN_ITERS", 100);
    let params = LweParams { n: 512, m: 1024, q: 3329 };
    let mut engine = LweCypher::new(params, 12345);
    let mut count = 0;

    for _ in 0..iters {
        let _ = black_box(engine.keygen());
        count += 1;
    }
    black_box(count)
}

#[library_benchmark]
fn bench_encrypt() -> Ciphertext {
    let iters = get_iters("IAI_ENCRYPT_ITERS", 1000);
    let params = LweParams { n: 512, m: 1024, q: 3329 };
    let mut engine = LweCypher::new(params, 12345);
    let (pk, _) = engine.keygen();

    let mut last_ct = engine.encrypt(&pk, true);
    for _ in 0..iters {
        last_ct = black_box(engine.encrypt(black_box(&pk), black_box(true)));
    }
    last_ct
}

#[library_benchmark]
fn bench_decrypt() -> usize {
    let iters = get_iters("IAI_DECRYPT_ITERS", 1000);
    let params = LweParams { n: 512, m: 1024, q: 3329 };
    let mut engine = LweCypher::new(params, 12345);
    let (pk, sk) = engine.keygen();
    let ct = engine.encrypt(&pk, true);

    let mut count = 0;
    for _ in 0..iters {
        if black_box(engine.decrypt(black_box(&sk), black_box(ct.clone()))) {
            count += 1;
        }
    }
    black_box(count)
}

library_benchmark_group!(
    name = lwe_group;
    benchmarks = bench_keygen, bench_encrypt, bench_decrypt
);

main!(library_benchmark_groups = lwe_group);