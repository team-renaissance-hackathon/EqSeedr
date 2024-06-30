use crate::utils::{BYTE, DISCRIMINATOR, UNSIGNED_32};
use anchor_lang::{prelude::*, solana_program::pubkey::PUBKEY_BYTES};

#[account]
pub struct VestedAccountByIndex {
    // VALIDATION STATE
    pub bump: u8,
    pub session: Pubkey,
    pub vested_config: Pubkey,
    pub owner: Pubkey,

    // I'm not sure what this is supposed to be...
    pub bid_index: u32,
}

impl VestedAccountByIndex {
    pub fn initialize(
        &mut self,
        bump: u8,
        bid_index: u32,
        owner: Pubkey,
        session: Pubkey,
        vested_config: Pubkey,
    ) {
        self.bump = bump;
        self.bid_index = bid_index;
        self.owner = owner;
        self.session = session;
        self.vested_config = vested_config;
    }
}

impl VestedAccountByIndex {
    pub const LEN: usize =
        DISCRIMINATOR + BYTE + PUBKEY_BYTES + PUBKEY_BYTES + PUBKEY_BYTES + UNSIGNED_32;
}
