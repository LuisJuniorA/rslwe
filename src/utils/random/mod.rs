pub mod xor_shift;

pub trait RngState {
    fn new(seed: u64) -> Self;
    fn next_u64(&mut self) -> u64;
}
