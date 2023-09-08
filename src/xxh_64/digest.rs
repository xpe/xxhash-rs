use super::acc::{Acc1, Acc4};

/// Returns the XXH_64 digest for the given data and seed.
pub fn digest(data: &[u8], seed: u64) -> u64 {
    if data.len() < 32 {
        digest_short(data, seed)
    } else {
        digest_long(data, seed)
    }
}

/// Returns the XXH_64 digest for less than 32 bytes of data.
pub fn digest_short(bytes: &[u8], seed: u64) -> u64 {
    let length = bytes.len();
    debug_assert!(length < 32);
    let mut a1 = Acc1::new(seed);
    a1.finish(length, bytes)
}

/// Returns the XXH_64 digest for 32 bytes of data or more. This corresponds to
/// the size of a stripe. In XXH_64, a stripe has exactly 32 bytes.
pub fn digest_long(bytes: &[u8], seed: u64) -> u64 {
    let length = bytes.len();
    debug_assert!(length >= 32);
    let mut a4 = Acc4::new(seed);
    let mut chunks_of_32 = bytes.chunks_exact(32);
    a4.consume_stripes(chunks_of_32.by_ref());
    let mut a1 = a4.converge();
    a1.finish(length, chunks_of_32.remainder())
}
