use super::super::utils::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct ProgramAuthority {
    pub bump: u8,

    // specify a authority or deployer?
    // pub authority: Pubkey,
    pub is_initialzied: bool,
    pub is_signer: bool,
}

impl ProgramAuthority {
    pub const LEN: usize = DISCRIMINATOR + BUMP + BOOL + BOOL;
}
