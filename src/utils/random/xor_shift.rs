use crate::utils::random::RngState;

pub struct XorShift64 {
    seed: u64,
}

impl RngState for XorShift64 {
    fn new(seed: u64) -> Self {
        assert_ne!(seed, 0, "seed can't be 0");
        Self { seed }
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.seed;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.seed = x;
        x
    }
}
