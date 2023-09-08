use super::acc::Acc1;
use super::primes::{PRIME_1, PRIME_2, PRIME_3, PRIME_4, PRIME_5};
use crate::modular::ModU32;

impl Acc1 {
    // Initialize the accumulator. A _special case_ of _Step 1_ in
    // the xxHash spec.
    pub fn new(seed: u32) -> Self {
        Self(ModU32(seed) + PRIME_5)
    }

    /// Comprises _Steps 4, 5, and 6_ in the xxHash spec.
    pub fn finish(&mut self, total_length: usize, remainder: &[u8]) -> u32 {
        self.update_with_length(total_length);
        self.consume_remaining(remainder);
        self.avalanche()
    }

    /// Update the accumulator using the _total_ length of the entire input
    /// data. _Step 4_ in the xxHash spec.
    fn update_with_length(&mut self, length: usize) {
        self.0 += ModU32(length as u32);
    }

    /// Consume the remaining data, which must be less than 16 bytes. _Step 5_
    /// in the xxHash spec.
    fn consume_remaining(&mut self, remainder: &[u8]) {
        let mut chunks = remainder.chunks_exact(4);
        for chunk in chunks.by_ref() {
            debug_assert!(chunk.len() == 4);
            let lane = ModU32::from(chunk);
            self.0 += lane * PRIME_3;
            self.0 = self.0.rotate_left(17) * PRIME_4;
        }
        for byte in chunks.remainder() {
            let lane = ModU32(*byte as u32);
            self.0 += lane * PRIME_5;
            self.0 = self.0.rotate_left(11) * PRIME_1;
        }
    }

    /// Ensures that all bits have a chance to impact any bit in the output
    /// digest, resulting in an unbiased distribution. _Step 6_ in the xxHash
    /// spec.
    fn avalanche(self) -> u32 {
        let mut a = self.0;
        a ^= a >> 15;
        a *= PRIME_2;
        a ^= a >> 13;
        a *= PRIME_3;
        a ^= a >> 16;
        a.into()
    }
}
