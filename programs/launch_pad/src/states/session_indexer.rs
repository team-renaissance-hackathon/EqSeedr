use super::super::utils::*;
use anchor_lang::prelude::*;

#[account]
pub struct SessionIndexer {
    pub bump: u8,
    pub authority: Pubkey,

    // not sure this will work in anchor
    pub list: Vec<(Indexer, Pubkey)>,
}

impl SessionIndexer {
    const LIMIT: usize = 100;
    pub const LEN: usize =
        DISCRIMINATOR + BUMP + PUBKEY_BYTES + (Indexer::LEN + PUBKEY_BYTES) * SessionIndexer::LIMIT;
}
