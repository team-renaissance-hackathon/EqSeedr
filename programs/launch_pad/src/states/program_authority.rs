use crate::utils::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct ProgramAuthority {
    pub bump: u8,

    // think about using multisig / threshold sig
    pub authority: Pubkey,

    pub is_initialized: bool,
    pub is_signer: bool,

    // bid token mints -> USDC | SOL TOKEN | STABLE COIN
    // attempting future proof. currently in mind only using USDC
    pub bid_token_mint_list: Vec<Pubkey>,

    // program token mint -> STAKING | UTILITY
    pub mint: Pubkey,
}

impl ProgramAuthority {
    const TOKEN_MINT_TOTAL: usize = 10;
    pub const LEN: usize = DISCRIMINATOR
        + BUMP
        + PUBKEY_BYTES
        + BOOL
        + BOOL
        + (UNSIGNED_64 + PUBKEY_BYTES * ProgramAuthority::TOKEN_MINT_TOTAL)
        + PUBKEY_BYTES;

    pub fn initialize(&mut self, bump: u8, authority: Pubkey) {
        self.bump = bump;
        self.authority = authority;

        self.is_initialized = true;
        self.is_signer = true;

        self.bid_token_mint_list = Vec::<Pubkey>::new();

        // right now I am not creating the token mint here
        // but in future I will set it that way.
        // self.mint = token_Mint
    }

    pub fn sign(&self) {}

    pub fn add_bid_token_mint(&mut self, token_mint: Pubkey) {
        self.bid_token_mint_list.push(token_mint);
    }

    pub fn is_valid_token(&self, token_mint: Pubkey) -> bool {
        // could be issue for stack... probably should use while and directly index into list
        for mint in self.bid_token_mint_list.clone() {
            if mint == token_mint {
                return true;
            }
        }
        return false;
    }
}
