pub struct XorshiftGenerator(u64);

impl XorshiftGenerator {
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self(seed)
    }

    #[must_use]
    pub fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}
