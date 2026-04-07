use crate::config;
use crate::lwe::lwe_params::LweParams;
use crate::lwe::lwe_private_key::PrivateKey;
use crate::lwe::lwe_public_key::PublicKey;
use crate::utils::distribution::Sampler;
use crate::utils::matrix::Matrix;
use crate::utils::random::RngState;

#[derive(Clone)]
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
        let n = self.params.n;

        let r = config::DefaultLweDistribution::r_distribution().fill(m, q, &mut self.rng);

        let mut u = vec![0u64; n];
        let mut v = 0u64;

        for (i, &ri_raw) in r.iter().enumerate().take(m) {
            let ri = ri_raw as u128;
            for (j, uj) in u.iter_mut().enumerate().take(n) {
                let a_ij = pk.a.get(i, j) as u128;
                *uj = ((*uj as u128 + a_ij * ri) % (q as u128)) as u64;
            }

            let b_i = pk.b.get(i, 0) as u128;
            v = ((v as u128 + b_i * ri) % (q as u128)) as u64;
        }

        let offset = if bit { q / 2 } else { 0 };
        v = (v + offset) % q;

        Ciphertext { a: u, b: v }
    }

    pub fn decrypt(&self, sk: &PrivateKey, ciphertext: Ciphertext) -> bool {
        let q = self.params.q;
        let mut inner_product = 0u64;

        for j in 0..self.params.n {
            let u_j = ciphertext.a[j] as u128;
            let s_j = sk.s.get(j, 0) as u128;
            inner_product = ((inner_product as u128 + u_j * s_j) % (q as u128)) as u64;
        }

        let delta = (ciphertext.b + q - inner_product) % q;

        let q_over_4 = q / 4;
        let three_q_over_4 = q - q_over_4;

        delta >= q_over_4 && delta <= three_q_over_4
    }

    pub fn keygen(&mut self) -> (PublicKey, PrivateKey) {
        let a = Matrix {
            data: config::DefaultLweDistribution::a_distribution().fill(
                self.params.m * self.params.n,
                self.params.q,
                &mut self.rng,
            ),
            rows: self.params.m,
            cols: self.params.n,
        };

        let s = Matrix {
            data: config::DefaultLweDistribution::s_distribution().fill(
                self.params.n,
                self.params.q,
                &mut self.rng,
            ),
            rows: self.params.n,
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
