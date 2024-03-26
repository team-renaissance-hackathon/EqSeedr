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

    pub fn initialize(&mut self, bump: u8, authority: Pubkey) {
        self.bump = bump;
        self.authority = authority;
        self.status = Indexer::new();
    }
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

impl EnqueueSessionIndex {
    const ELEMENT_SIZE: usize = 100;
    pub const LEN: usize = DISCRIMINATOR
        + BYTE
        + PUBKEY_BYTES
        + (UNSIGNED_128 + Indexer::LEN * EnqueueSessionIndex::ELEMENT_SIZE)
        + (UNSIGNED_128 + UNSIGNED_16 * EnqueueSessionIndex::ELEMENT_SIZE);
    pub fn initialize(&mut self, bump: u8, authority: Pubkey) {
        self.bump = bump;
        self.authority = authority;
        self.list = Vec::<Indexer>::new();
        self.stack = Vec::<u16>::new();
    }
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

impl ActiveSessionIndex {
    const ELEMENT_SIZE: usize = 100;
    pub const LEN: usize = DISCRIMINATOR
        + BYTE
        + PUBKEY_BYTES
        + (UNSIGNED_128 + Indexer::LEN * ActiveSessionIndex::ELEMENT_SIZE)
        + (UNSIGNED_128 + UNSIGNED_16 * ActiveSessionIndex::ELEMENT_SIZE);

    pub fn initialize(&mut self, bump: u8, authority: Pubkey) {
        self.bump = bump;
        self.authority = authority;
        self.list = Vec::<Indexer>::new();
        self.stack = Vec::<u16>::new();
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct IndexerLinkedList {
    pub total: u32,
    head: u32,
    tail: u32,
    list: Vec<Option<Node>>,

    stack: Vec<[u8; 3]>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Node {
    pub next: Option<u32>,
    pub prev: Option<u32>,
    pub position: Indexer,
}

// currently EnqueueSessionIndex and ActiveSessionIndex are being created, but they are not fully
// implemented that handles the list data
// adding to list, removing from list, and tracking empty elements in the list
// that implementation detail is not important right now. will just use the IndexerStatus
// to track sessions
