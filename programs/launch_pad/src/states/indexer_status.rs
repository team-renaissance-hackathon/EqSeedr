use super::super::utils::*;
use anchor_lang::prelude::*;

#[account]
pub struct IndexerStatus {
    pub bump: u8,
    pub authority: Pubkey,
    pub status: Indexer,
}

impl IndexerStatus {
    pub const LEN: usize = DISCRIMINATOR + BUMP + PUBKEY_BYTES + Indexer::LEN;
}
