//! XXH_64 : 64-bit implementation of xxHash.

mod acc;
mod acc1;
mod acc4;
mod digest;
mod primes;

pub use digest::digest;

#[cfg(test)]
mod tests;
