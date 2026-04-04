use crate::utils::distribution::Sampler;
use crate::utils::exponential::Exponential;
use crate::utils::random::RngState;

/// A Discrete Gaussian Sampler over integers.
///
/// This sampler uses a Cumulative Distribution Table (CDT) to draw samples
/// from a discrete Gaussian distribution centered at 'c' with parameter 'sigma'.
/// It is typically used for generating small noise in lattice-based cryptography.
pub struct Gaussian<E>
where
    E: Exponential,
{
    /// Standard deviation parameter: sigma = alpha * q.
    sigma: f64,
    /// Center of the distribution (default at 0.0).
    c: f64,
    /// Exponential function provider.
    exp: E,
    /// Precomputed Cumulative Distribution Table for non-negative integers.
    cdt: Vec<f64>,
}

impl<E: Exponential> Gaussian<E> {
    /// Creates a new Gaussian sampler and precomputes the CDT.
    ///
    /// The standard deviation is defined as sigma = alpha * q.
    /// The table is populated until the Gaussian weight rho(x) falls below 1e-15.
    ///
    /// # Arguments
    /// * `alpha` - Scaling factor for the noise.
    /// * `q` - The modulus of the cryptosystem.
    /// * `exp` - An implementation of the Exponential trait.
    pub fn new(alpha: f64, q: u64, exp: E) -> Self {
        let sigma = alpha * (q as f64);
        let precision_threshold = 1e-15;
        let c = 0.0;

        let mut sampler = Self {
            sigma,
            c,
            exp,
            cdt: vec![],
        };

        // Step 1: Compute total weight (normalization factor)
        // This calculates the sum of rho(k) for all k in integers.
        let mut sum_weight = sampler.rho(c);
        let mut k = 1.0;
        loop {
            let w = sampler.rho(k);
            if w < precision_threshold {
                break;
            }
            sum_weight += w * 2.0; // Symmetry: rho(k) == rho(-k)
            k += 1.0;
        }

        // Step 2: Build the Cumulative Distribution Table
        let mut acc = sampler.rho(c) / sum_weight;
        sampler.cdt.push(acc);

        let mut i = 1.0;
        loop {
            let w = sampler.rho(i);
            if w < precision_threshold {
                break;
            }
            acc += (w * 2.0) / sum_weight;
            sampler.cdt.push(acc);
            i += 1.0;
        }

        sampler
    }

    /// Calculates the Gaussian weight for a given value x.
    ///
    /// Formula: rho(x) = exp(-pi * (x - c)^2 / sigma^2)
    pub fn rho(&self, x: f64) -> f64 {
        self.exp
            .exp(-core::f64::consts::PI * (x - self.c) * (x - self.c) / (self.sigma * self.sigma))
    }
}

impl<E: Exponential> Sampler for Gaussian<E> {
    /// Fills a vector with 'n' samples drawn from the discrete Gaussian distribution.
    ///
    /// Each sample is mapped into the range [0, modulus[.
    ///
    /// # Sampling Process:
    /// 1. Draw a uniform random value to find the magnitude (x_abs) via the CDT.
    /// 2. Use the least significant bit (LSB) of the random source to determine the sign.
    /// 3. If the LSB is 1 and x_abs > 0, the result is treated as negative: (modulus - x_abs).
    fn fill(&self, n: usize, modulus: u64, rng: &mut impl RngState) -> Vec<u64> {
        (0..n)
            .map(|_| {
                let val = rng.next_u64();
                let normalize = (val as f64) / (u64::MAX as f64);

                // Linear search in the CDT to find the interval
                let mut x_abs = self.cdt.len() - 1;
                for (i, &threshold) in self.cdt.iter().enumerate() {
                    if normalize < threshold {
                        x_abs = i;
                        break;
                    }
                }

                // Apply sign and modulo arithmetic
                if x_abs > 0 && (val & 1) == 1 {
                    modulus - (x_abs as u64)
                } else {
                    x_abs as u64
                }
            })
            .collect()
    }
}
