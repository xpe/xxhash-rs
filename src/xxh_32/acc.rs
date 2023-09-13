use super::primes::{PRIME_1, PRIME_2};
use crate::modular::ModU32;

/// A one word accumulator. (For XXH_32, a word is 32 bits.)
#[derive(Clone, Copy, Debug)]
pub struct Acc1(pub ModU32);

/// A four word accumulator. (For XXH_32, a word is 32 bits.)
#[derive(Clone, Debug)]
pub struct Acc4 {
    pub word_1: ModU32,
    pub word_2: ModU32,
    pub word_3: ModU32,
    pub word_4: ModU32,
}

/// Compute one _round_. This is part of _Step 2_ in the xxHash spec.
#[allow(dead_code)]
pub fn round(word: ModU32, lane: ModU32) -> ModU32 {
    let mut a = word + (lane * PRIME_2);
    a.rotate_left_mut(13);
    a * PRIME_1
}

/// Compute one _round_. This mutates a given accumulator word using _lane_.
/// This is part of _Step 2_ in the xxHash spec.
pub fn round_mut(word: &mut ModU32, lane: ModU32) {
    *word += lane * PRIME_2;
    word.rotate_left_mut(13);
    *word *= PRIME_1;
}
