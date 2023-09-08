use super::ModU32;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use std::ops::{BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign};

impl From<&[u8]> for ModU32 {
    fn from(bytes: &[u8]) -> Self {
        debug_assert!(bytes.len() == 4);
        ModU32(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

impl From<ModU32> for u32 {
    fn from(item: ModU32) -> u32 {
        item.0
    }
}

// +
impl Add for ModU32 {
    type Output = ModU32;

    #[must_use]
    fn add(self, other: ModU32) -> ModU32 {
        ModU32(self.0.wrapping_add(other.0))
    }
}

// +=
impl AddAssign for ModU32 {
    fn add_assign(&mut self, other: ModU32) {
        self.0 = self.0.wrapping_add(other.0);
    }
}

// *
impl Mul for ModU32 {
    type Output = ModU32;

    #[must_use]
    fn mul(self, other: ModU32) -> ModU32 {
        ModU32(self.0.wrapping_mul(other.0))
    }
}

// *=
impl MulAssign for ModU32 {
    fn mul_assign(&mut self, other: ModU32) {
        self.0 = self.0.wrapping_mul(other.0);
    }
}

// -
impl Sub for ModU32 {
    type Output = ModU32;

    #[must_use]
    fn sub(self, other: ModU32) -> ModU32 {
        ModU32(self.0.wrapping_sub(other.0))
    }
}

// -=
impl SubAssign for ModU32 {
    fn sub_assign(&mut self, other: ModU32) {
        self.0 = self.0.wrapping_sub(other.0);
    }
}

impl BitXor for ModU32 {
    type Output = ModU32;

    #[must_use]
    fn bitxor(self, other: ModU32) -> ModU32 {
        ModU32(self.0 ^ other.0)
    }
}

impl BitXorAssign for ModU32 {
    fn bitxor_assign(&mut self, other: ModU32) {
        self.0 ^= other.0;
    }
}

// <<
impl Shl<u32> for ModU32 {
    type Output = ModU32;

    #[must_use]
    fn shl(self, n: u32) -> ModU32 {
        ModU32(self.0.wrapping_shl(n))
    }
}

// <<=
impl ShlAssign<u32> for ModU32 {
    fn shl_assign(&mut self, n: u32) {
        self.0 = self.0.wrapping_shl(n);
    }
}

// >>
impl Shr<u32> for ModU32 {
    type Output = ModU32;

    #[must_use]
    fn shr(self, n: u32) -> ModU32 {
        ModU32(self.0.wrapping_shr(n))
    }
}

// >>=
impl ShrAssign<u32> for ModU32 {
    fn shr_assign(&mut self, n: u32) {
        self.0 = self.0.wrapping_shr(n);
    }
}

impl ModU32 {
    #[must_use]
    pub fn rotate_left(self, n: u32) -> Self {
        ModU32(self.0.rotate_left(n))
    }

    pub fn rotate_left_mut(&mut self, n: u32) {
        self.0 = self.0.rotate_left(n);
    }
}
