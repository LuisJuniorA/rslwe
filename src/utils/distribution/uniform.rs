use crate::utils::distribution::Sampler;
use crate::utils::random::RngState;

pub struct Uniform;

impl Sampler for Uniform {
    fn fill(&self, n: usize, modulus: u64, rng: &mut impl RngState) -> Vec<u64> {
        (0..n).map(|_| rng.next_u64() % modulus).collect()
    }
}
