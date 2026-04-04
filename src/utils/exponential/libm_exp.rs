use crate::utils::exponential::Exponential;

pub struct LibmExp;

impl Exponential for LibmExp {
    fn exp(&self, x: f64) -> f64 {
        libm::exp(x)
    }
}
