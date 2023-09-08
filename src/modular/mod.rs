pub mod modular_u32;
pub mod modular_u64;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ModU32(pub u32);

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ModU64(pub u64);
