use crate::config;
use crate::lwe::lwe_params::LweParams;
use crate::lwe::lwe_private_key::PrivateKey;
use crate::lwe::lwe_public_key::PublicKey;
use crate::utils::distribution::Sampler;
use crate::utils::matrix::Matrix;
use crate::utils::random::RngState;
pub struct Ciphertext {
    pub a: Vec<u64>,
    pub b: u64,
}

pub struct LweCypher {
    rng: config::DefaultRandom,
    params: LweParams,
}

impl LweCypher {
    pub fn new(params: LweParams, seed: u64) -> Self {
        Self {
            rng: config::DefaultRandom::new(seed),
            params,
        }
    }
    pub fn encrypt(&mut self, pk: &PublicKey, bit: bool) -> Ciphertext {
        let q = self.params.q;
        let m = self.params.m;
        let n = pk.a.cols;

        let r = config::DefaultLweDistribution::r_distribution().fill(
            m,
            q,
            &mut self.rng,
        );

        let mut system = vec![0u64; n];
        let mut solutions = 0u64;

        for i in 0..m {
            let ri = r[i];

            let start = i * n;
            let row = &pk.a.data[start..start + n];

            for (y, &val) in row.iter().enumerate() {
                let prod = val.wrapping_mul(ri);
                system[y] = (system[y].wrapping_add(prod)) % q;
            }

            let sol_prod = pk.b.data[i].wrapping_mul(ri);
            solutions = solutions.wrapping_add(sol_prod);
        }

        let offset = if bit { q / 2 } else { 0 };
        solutions = (solutions.wrapping_add(offset)) % q;

        Ciphertext {
            a: system,
            b: solutions,
        }
    }

    pub fn decrypt(&self, pk: &PrivateKey, ciphertext: Ciphertext) -> bool {
        let result = ciphertext.a.iter().enumerate().map(|(i, value)| value * pk.s.data[i]).sum::<u64>();
        let delta = if result < ciphertext.b {self.params.q - result - ciphertext.b} else {result - ciphertext.b};
        delta % self.params.q <= self.params.q/2
    }

    pub fn keygen(&mut self) -> (PublicKey, PrivateKey) {
        let a = Matrix {
            data: config::DefaultLweDistribution::a_distribution().fill(
                self.params.n,
                self.params.q,
                &mut self.rng,
            ),
            rows: self.params.m,
            cols: self.params.n,
        };

        let s = Matrix {
            data: config::DefaultLweDistribution::s_distribution().fill(
                self.params.m,
                self.params.q,
                &mut self.rng,
            ),
            rows: self.params.m,
            cols: 1,
        };

        let e = Matrix {
            data: config::DefaultLweDistribution::e_distribution().fill(
                self.params.m,
                self.params.q,
                &mut self.rng,
            ),
            rows: self.params.m,
            cols: 1,
        };

        let b = a.mul_mod(&s, self.params.q).add_mod(&e, self.params.q);

        let sk = PrivateKey { s };
        let pk = PublicKey { a, b };
        (pk, sk)
    }
}
