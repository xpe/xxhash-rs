use super::primes::{PRIME_1, PRIME_2};
use crate::modular::ModU64;

/// A one word accumulator. (For XXH_64, a word is 64 bits.)
#[derive(Clone, Copy, Debug)]
pub struct Acc1(pub ModU64);

/// A four word accumulator. (For XXH_64, a word is 64 bits.)
#[derive(Clone, Debug)]
pub struct Acc4 {
    pub word_1: ModU64,
    pub word_2: ModU64,
    pub word_3: ModU64,
    pub word_4: ModU64,
}

/// Compute one _round_. This is part of _Step 2_ in the xxHash spec.
pub fn round(word: ModU64, lane: ModU64) -> ModU64 {
    let mut a = word + (lane * PRIME_2);
    a.rotate_left_mut(31);
    a * PRIME_1
}

/// Compute one _round_. This mutates a given accumulator word using _lane_.
/// This is part of _Step 2_ in the xxHash spec.
pub fn round_mut(word: &mut ModU64, lane: ModU64) {
    *word += lane * PRIME_2;
    word.rotate_left_mut(31);
    *word *= PRIME_1;
}
