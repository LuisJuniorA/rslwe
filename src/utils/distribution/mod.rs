use crate::utils::random::RngState;

pub mod binary;
pub mod gaussian;
pub mod uniform;

/// A trait for sampling random vectors in Lattice-Based Cryptography (LWE).
///
/// This trait abstracts the distribution logic (Gaussian, Binomial, Uniform)
/// used to generate the error vector 'e' and the secret vector 's'.
pub trait Sampler {
    /// Fills and returns a vector of size 'n' with values sampled from
    /// the distribution, reduced modulo 'modulus'.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of coefficients to generate (the dimension of the vector).
    /// * `modulus` - The value 'q' used for the modular reduction.
    ///
    /// # Implementation Details
    ///
    /// The implementation must ensure that negative samples are correctly
    /// mapped to the range [0, modulus - 1]. For example, a sample of -1
    /// should be returned as (modulus - 1).
    fn fill(&self, n: usize, modulus: u64, rng: &mut impl RngState) -> Vec<u64>;
}
