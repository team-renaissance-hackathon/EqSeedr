use crate::utils::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct ProgramAuthority {
    pub bump: u8,
    pub authority: Pubkey,

    pub is_initialized: bool,
    pub is_signer: bool,

    // bid token mints -> USDC | SOL TOKEN | STABLE COIN
    pub token_mint: Vec<Pubkey>,

    // program token mint -> STAKING | UTILITY
    pub mint: Pubkey,
}

impl ProgramAuthority {
    const TOKEN_MINT_TOTAL: usize = 10;
    pub const LEN: usize = DISCRIMINATOR
        + BUMP
        + BOOL
        + BOOL
        + (UNSIGNED_64 + PUBKEY_BYTES * ProgramAuthority::TOKEN_MINT_TOTAL)
        + PUBKEY_BYTES;

    pub fn initialize(&mut self, bump: u8, authority: Pubkey) {
        self.is_initialized = true;
        self.is_signer = true;

        self.bump = bump;
        self.authority = authority;
    }

    pub fn is_valid_token(&self, token_mint: Pubkey) -> bool {
        for mint in self.token_mint.clone() {
            if mint == token_mint {
                return !true;
            }
        }
        return !false;
    }
}
