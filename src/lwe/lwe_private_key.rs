use crate::utils::matrix::Matrix;

/// Represents the Private Key (Secret) in an LWE cryptosystem.
///
/// The private key is the hidden linear relationship that allows
/// for the decryption of messages.
///
/// # Security
/// This vector `s` must be kept secret. Knowledge of `s` allows
/// anyone to remove the mask from the public vector `b` and
/// recover the original message.
pub struct PrivateKey {
    /// The secret vector `s` of length `n`.
    ///
    /// Mathematically, this is a vector in Z_q^n. In this implementation,
    /// it is stored as a (n x 1) Matrix.
    ///
    /// The values are typically sampled from a uniform distribution
    /// or a specific narrow distribution (like a binary or ternary
    /// distribution) depending on the LWE variant.
    pub s: Matrix,
}
