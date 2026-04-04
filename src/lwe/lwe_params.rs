/// Parameters for a Learning With Errors (LWE) cryptosystem instance.
///
/// This struct defines the security level and the algebraic structure
/// used for key generation, encryption, and decryption.
pub struct LweParams {
    /// The ciphertext modulus (q).
    ///
    /// All arithmetic operations in the LWE system are performed
    /// modulo this value. It defines the range of the integers
    /// in the system: from 0 to q-1.
    ///
    /// Using u64 ensures consistent behavior across different
    /// CPU architectures and prevents overflow during intermediate sums.
    pub q: u64,

    /// The secret key dimension (n).
    ///
    /// This represents the number of elements in the secret vector 's'.
    /// It is the primary security parameter; higher values of 'n'
    /// make the LWE problem harder to solve.
    pub n: usize,

    /// The number of LWE samples (m).
    ///
    /// This represents the number of rows in the public matrix 'A'
    /// and the length of the public vector 'b'. Usually, m > n.
    pub m: usize,
}
