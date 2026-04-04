pub mod lwe;
pub mod utils;

pub mod config {
    use crate::utils::distribution::binary::Binary;
    use crate::utils::distribution::gaussian::Gaussian;
    use crate::utils::distribution::uniform::Uniform;
    use crate::utils::exponential::libm_exp::LibmExp;
    use crate::utils::random::xor_shift::XorShift64;

    pub type DefaultExponential = LibmExp;
    pub type DefaultRandom = XorShift64;

    pub struct DefaultLweDistribution;

    impl DefaultLweDistribution {
        pub fn a_distribution() -> Uniform {
            Uniform
        }

        pub fn s_distribution() -> Uniform {
            Uniform
        }

        pub fn e_distribution() -> Gaussian<DefaultExponential> {
            Gaussian::new(0.0006, 3329, LibmExp)
        }

        pub fn r_distribution() -> Binary {
            Binary
        }
    }
}
