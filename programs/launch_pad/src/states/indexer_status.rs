use crate::utils::*;
use anchor_lang::prelude::*;

#[account]
pub struct IndexerStatus {
    // VALIDATION STATE
    pub bump: u8,
    pub authority: Pubkey,

    // STATE
    pub status: Indexer,
}

impl IndexerStatus {
    pub const LEN: usize = DISCRIMINATOR + BUMP + PUBKEY_BYTES + Indexer::LEN;

    pub fn initialize(&self) {}
}

#[account]
pub struct EnqueueSessionIndex {
    // VALIDATION STATE
    pub bump: u8,
    pub authority: Pubkey,
    // STATE
    pub list: Vec<Indexer>,
    pub stack: Vec<u16>,
}

#[account]
pub struct ActiveSessionIndex {
    // VALIDATION STATE
    pub bump: u8,
    pub authority: Pubkey,
    // STATE
    pub list: Vec<Indexer>,
    pub stack: Vec<u16>,
}

// #[derive(AnchorDeserialize, AnchorSerialize, Clone)]
// pub struct IndexerLinkedList {
//     pub total: u32,
//     head: u32,
//     tail: u32,
//     list: Vec<Node>,

//     stack: Vec<[u8; 3]>,
// }

// #[derive(AnchorDeserialize, AnchorSerialize, Clone)]
// pub struct Node {
//     pub next: u8,
//     pub prev: u8,
//     pub position: Indexer,
// }

// currently EnqueueSessionIndex and ActiveSessionIndex are being created, but they are not fully
// implemented that handles the list data
// adding to list, removing from list, and tracking empty elements in the list
// that implementation detail is not important right now. will just use the IndexerStatus
// to track sessions
