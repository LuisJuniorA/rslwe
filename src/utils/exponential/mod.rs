pub mod libm_exp;
pub trait Exponential {
    fn exp(&self, x: f64) -> f64;
}
