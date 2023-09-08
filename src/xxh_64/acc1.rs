use super::acc::{round, Acc1};
use super::primes::{PRIME_1, PRIME_2, PRIME_3, PRIME_4, PRIME_5};
use crate::modular::{ModU32, ModU64};

impl Acc1 {
    // Initialize the accumulator. A _special case_ of _Step 1_ in
    // the xxHash spec.
    pub fn new(seed: u64) -> Self {
        Self(ModU64(seed) + PRIME_5)
    }

    /// Comprises _Steps 4, 5, and 6_ in the xxHash spec.
    pub fn finish(&mut self, total_length: usize, remainder: &[u8]) -> u64 {
        self.update_with_length(total_length);
        self.consume_remaining(remainder);
        self.avalanche()
    }

    /// Update the accumulator using the _total_ length of the entire input
    /// data. _Step 4_ in the xxHash spec.
    fn update_with_length(&mut self, length: usize) {
        self.0 += ModU64(length as u64);
    }

    /// Consume the remaining data, which must be less than 32 bytes. _Step 5_
    /// in the xxHash spec.
    fn consume_remaining(&mut self, remainder: &[u8]) {
        let mut chunks_of_8 = remainder.chunks_exact(8);
        for chunk in chunks_of_8.by_ref() {
            debug_assert!(chunk.len() == 8);
            let lane = ModU64::from(chunk);
            self.0 ^= round(ModU64(0), lane);
            self.0 = self.0.rotate_left(27) * PRIME_1;
            self.0 += PRIME_4;
        }
        let mut chunks_of_4 = chunks_of_8.remainder().chunks_exact(4);
        for chunk in chunks_of_4.by_ref() {
            debug_assert!(chunk.len() == 4);
            let lane = ModU64::from(ModU32::from(chunk));
            self.0 ^= lane * PRIME_1;
            self.0 = self.0.rotate_left(23) * PRIME_2;
            self.0 += PRIME_3;
        }
        for byte in chunks_of_4.remainder() {
            let lane = ModU64(*byte as u64);
            self.0 ^= lane * PRIME_5;
            self.0 = self.0.rotate_left(11) * PRIME_1;
        }
    }

    /// Ensures that all bits have a chance to impact any bit in the output
    /// digest, resulting in an unbiased distribution. _Step 6_ in the xxHash
    /// spec.
    fn avalanche(self) -> u64 {
        let mut a = self.0;
        a ^= a >> 33;
        a *= PRIME_2;
        a ^= a >> 29;
        a *= PRIME_3;
        a ^= a >> 32;
        a.into()
    }
}
