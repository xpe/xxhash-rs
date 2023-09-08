use super::acc::{round_mut, Acc1, Acc4};
use super::primes::{PRIME_1, PRIME_2};
use crate::modular::ModU32;
use std::slice::ChunksExact;

impl Acc4 {
    /// Initialize the accumulator. (_Step 1_ of the xxHash spec.)
    pub fn new(seed: u32) -> Self {
        let s = ModU32(seed);
        Self {
            word_1: s + PRIME_1 + PRIME_2,
            word_2: s + PRIME_2,
            word_3: s,
            word_4: s - PRIME_1,
        }
    }

    /// Consume data, provided as stripes. Each stripe is 16 bytes. This is
    /// _Step 2_ in the xxHash spec.
    #[rustfmt::skip]
    pub fn consume_stripes(&mut self, stripes: &mut ChunksExact<'_, u8>) {
        for stripe in stripes {
            debug_assert!(stripe.len() == 16);
            round_mut(&mut self.word_1, ModU32::from(&stripe[ 0.. 4]));
            round_mut(&mut self.word_2, ModU32::from(&stripe[ 4.. 8]));
            round_mut(&mut self.word_3, ModU32::from(&stripe[ 8..12]));
            round_mut(&mut self.word_4, ModU32::from(&stripe[12..16]));
        }
    }

    /// Converge accumulators. Combines words of `self` (a four lane
    /// accumulator) into one word. This is _Step 3_ in the xxHash spec.
    pub fn converge(self) -> Acc1 {
        let mut a = self.word_1.rotate_left(1);
        a += self.word_2.rotate_left(7);
        a += self.word_3.rotate_left(12);
        a += self.word_4.rotate_left(18);
        Acc1(a)
    }
}
