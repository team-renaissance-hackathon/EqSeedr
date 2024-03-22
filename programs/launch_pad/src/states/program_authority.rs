use crate::utils::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct ProgramAuthority {
    pub bump: u8,

    // specify a authority or deployer?
    // pub authority: Pubkey,
    pub is_initialized: bool,
    pub is_signer: bool,
}

impl ProgramAuthority {
    pub const LEN: usize = DISCRIMINATOR + BUMP + BOOL + BOOL;

    pub fn initialize(&mut self, bump: u8) {
        self.is_initialized = true;
        self.is_signer = true;

        self.bump = bump;
    }
}
