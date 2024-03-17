pub const DISCRIMINATOR: usize = 8;
pub const SIGNED_128: usize = 16;
pub const SIGNED_64: usize = 8;
pub const SIGNED_32: usize = 4;
pub const SIGNED_16: usize = 2;
pub const SIGNED_8: usize = 1;

pub const UNSIGNED_128: usize = 16;
pub const UNSIGNED_64: usize = 8;
pub const UNSIGNED_32: usize = 4;
pub const UNSIGNED_16: usize = 2;
pub const UNSIGNED_8: usize = 1;

pub const BYTE: usize = 1;
pub const BOOL: usize = 1;
pub const BUMP: usize = 1;

pub const MAX_TEXT_BYTES: usize = 32;
pub const MAX_PARTICPANTS: usize = u16::MAX as usize;
pub use anchor_lang::solana_program::pubkey::PUBKEY_BYTES;
