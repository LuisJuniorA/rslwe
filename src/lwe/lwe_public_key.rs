use crate::utils::matrix::Matrix;

/// Public key for the LWE (Learning With Errors) cryptosystem.
///
/// A public key consists of a random matrix `A` and a vector `b` that
/// hides the secret key using an error term.
///
/// # Mathematical structure
/// The key is a pair (A, b) where:
/// * b = (A * s) + e
///
/// Where:
/// - `A`: Public random matrix (uniform distribution).
/// - `s`: The private secret vector.
/// - `e`: Small random noise (error vector).
///
/// All calculations are performed modulo `q`.
pub struct PublicKey {
    /// The public matrix `A` of size (m x n).
    pub a: Matrix,

    /// The public vector `b = As + e` of length `m`.
    pub b: Matrix,
}
