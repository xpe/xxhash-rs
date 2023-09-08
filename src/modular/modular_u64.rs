use super::ModU64;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use std::ops::{BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign};

impl From<super::ModU32> for ModU64 {
    fn from(item: super::ModU32) -> Self {
        ModU64(item.0 as u64)
    }
}

impl From<&[u8]> for ModU64 {
    fn from(bytes: &[u8]) -> Self {
        debug_assert!(bytes.len() == 8);
        ModU64(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }
}

impl From<[u8; 8]> for ModU64 {
    fn from(bytes: [u8; 8]) -> Self {
        debug_assert!(bytes.len() == 8);
        ModU64(u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]))
    }
}

impl From<[u8; 4]> for ModU64 {
    fn from(bytes: [u8; 4]) -> Self {
        debug_assert!(bytes.len() == 8);
        let x: u32 = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        ModU64(x as u64)
    }
}

impl From<ModU64> for u64 {
    fn from(item: ModU64) -> u64 {
        item.0
    }
}

// +
impl Add for ModU64 {
    type Output = ModU64;

    #[must_use]
    fn add(self, other: ModU64) -> ModU64 {
        ModU64(self.0.wrapping_add(other.0))
    }
}

// +=
impl AddAssign for ModU64 {
    fn add_assign(&mut self, other: ModU64) {
        self.0 = self.0.wrapping_add(other.0);
    }
}

// *
impl Mul for ModU64 {
    type Output = ModU64;

    #[must_use]
    fn mul(self, other: ModU64) -> ModU64 {
        ModU64(self.0.wrapping_mul(other.0))
    }
}

// *=
impl MulAssign for ModU64 {
    fn mul_assign(&mut self, other: ModU64) {
        self.0 = self.0.wrapping_mul(other.0);
    }
}

// -
impl Sub for ModU64 {
    type Output = ModU64;

    #[must_use]
    fn sub(self, other: ModU64) -> ModU64 {
        ModU64(self.0.wrapping_sub(other.0))
    }
}

// -=
impl SubAssign for ModU64 {
    fn sub_assign(&mut self, other: ModU64) {
        self.0 = self.0.wrapping_sub(other.0);
    }
}

impl BitXor for ModU64 {
    type Output = ModU64;

    #[must_use]
    fn bitxor(self, other: ModU64) -> ModU64 {
        ModU64(self.0 ^ other.0)
    }
}

impl BitXorAssign for ModU64 {
    fn bitxor_assign(&mut self, other: ModU64) {
        self.0 ^= other.0;
    }
}

// <<
impl Shl<u32> for ModU64 {
    type Output = ModU64;

    #[must_use]
    fn shl(self, n: u32) -> ModU64 {
        ModU64(self.0.wrapping_shl(n))
    }
}

// <<=
impl ShlAssign<u32> for ModU64 {
    fn shl_assign(&mut self, n: u32) {
        self.0 = self.0.wrapping_shl(n);
    }
}

// >>
impl Shr<u32> for ModU64 {
    type Output = ModU64;

    #[must_use]
    fn shr(self, n: u32) -> ModU64 {
        ModU64(self.0.wrapping_shr(n))
    }
}

// >>=
impl ShrAssign<u32> for ModU64 {
    fn shr_assign(&mut self, n: u32) {
        self.0 = self.0.wrapping_shr(n);
    }
}

impl ModU64 {
    #[must_use]
    pub fn rotate_left(self, n: u32) -> Self {
        ModU64(self.0.rotate_left(n))
    }

    pub fn rotate_left_mut(&mut self, n: u32) {
        self.0 = self.0.rotate_left(n);
    }
}
