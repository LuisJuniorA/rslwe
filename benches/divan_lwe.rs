use divan::{bench, black_box};
use rslwe::lwe::lwe_cypher::LweCypher;
use rslwe::lwe::lwe_params::LweParams;

fn main() {
    let args: Vec<String> = std::env::args()
        .filter(|arg| arg != "--bench")
        .collect();
    divan::Divan::from_args().main();
}

#[bench(args = [512, 1024])]
fn keygen(n: usize) {
    let params = LweParams { n, m: 1024, q: 3329 };
    let mut engine = LweCypher::new(params, 12345);
    black_box(engine.keygen());
}

#[bench(args = [512, 1024])]
fn encrypt(n: usize) {
    let params = LweParams { n, m: 1024, q: 3329 };
    let mut engine = LweCypher::new(params, 12345);
    let (pk, _) = engine.keygen();

    black_box(engine.encrypt(black_box(&pk), black_box(true)));
}

#[bench(args = [512, 1024])]
fn decrypt(n: usize) {
    let params = LweParams { n, m: 1024, q: 3329 };
    let mut engine = LweCypher::new(params, 12345);
    let (pk, sk) = engine.keygen();
    let ct = engine.encrypt(&pk, true);

    black_box(engine.decrypt(black_box(&sk), black_box(ct.clone())));
}