use super::acc::{Acc1, Acc4};

/// Returns the XXH_32 digest for the given data and seed.
pub fn digest(data: &[u8], seed: u32) -> u32 {
    if data.len() < 16 {
        digest_short(data, seed)
    } else {
        digest_long(data, seed)
    }
}

/// Returns the XXH_32 digest for less than 16 bytes of data.
pub fn digest_short(bytes: &[u8], seed: u32) -> u32 {
    debug_assert!(bytes.len() < 16);
    let mut a1 = Acc1::new(seed);
    a1.finish(bytes.len(), bytes)
}

/// Returns the XXH_32 digest for 16 bytes of data or more. (In XXH_32, a stripe
/// has exactly 16 bytes.)
pub fn digest_long(bytes: &[u8], seed: u32) -> u32 {
    debug_assert!(bytes.len() >= 16);
    let mut a4 = Acc4::new(seed);
    let mut chunks = bytes.chunks_exact(16);
    a4.consume_stripes(chunks.by_ref());
    let mut a1 = a4.converge();
    a1.finish(bytes.len(), chunks.remainder())
}
