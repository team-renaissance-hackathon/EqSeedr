use crate::utils::*;
use anchor_lang::prelude::*;

#[account]
pub struct SessionIndexer {
    pub bump: u8,
    pub authority: Pubkey,

    pub list: Vec<Index>,
}

impl SessionIndexer {
    const LIMIT: usize = 100;
    pub const LEN: usize =
        DISCRIMINATOR + BUMP + PUBKEY_BYTES + (Indexer::LEN + PUBKEY_BYTES) * SessionIndexer::LIMIT;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Index {
    pub indexer: Indexer,
    pub session: Pubkey,
}
