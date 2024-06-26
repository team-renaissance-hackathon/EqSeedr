use anchor_lang::{
    prelude::*,
    solana_program::{hash::Hasher, pubkey::PUBKEY_BYTES},
};

use crate::utils::{BOOL, BYTE, DISCRIMINATOR, UNSIGNED_32, UNSIGNED_64};

use super::{SealedBidRound, Session};

// this account can be closed after sealed bid phase is completed
// and unsealed bid has been submitted
#[account]
pub struct SealedBidByIndex {
    // VALIDATION STATE
    pub bump: u8,
    pub bid_index: u32,
    pub session: Pubkey,
    pub owner: Pubkey,

    // STATE
    pub commit_hash: Pubkey, // technially a hash [u8; 32]
    pub bid_amount: u64,
    pub staked_amount: u64,
    pub is_unsealed: bool,
    pub is_commit: bool,
    pub is_bid_refunded: bool,
    pub is_stake_unlocked: bool,

    // when investor submits their unsealed bid
    // the index should get recorded, and will
    // be used for the submit commit bid
    pub commit_leader_board_index: u32,
}

impl SealedBidByIndex {
    pub const LEN: usize = DISCRIMINATOR
        + BYTE
        + UNSIGNED_32
        + PUBKEY_BYTES
        + PUBKEY_BYTES
        + PUBKEY_BYTES
        + UNSIGNED_64
        + UNSIGNED_64
        + BOOL
        + BOOL
        + BOOL
        + BOOL
        + UNSIGNED_32;

    // sealed bid step
    pub fn initialize(
        &mut self,
        bump: u8,
        sealed_bid_round: &Account<SealedBidRound>,
        session: &Account<Session>,
        owner: Pubkey,
        commit_hash: Pubkey,
    ) {
        self.bump = bump;
        self.bid_index = sealed_bid_round.get_index();
        self.session = session.key();
        self.owner = owner;

        self.commit_hash = commit_hash;
        self.staked_amount = session.staking_amount;

        self.is_unsealed = false;

        // emit event
    }

    // unsealed bid step
    pub fn unsealed_bid(&mut self, index: u32, amount: u64) {
        // question... why was I subtracting - 1 from index?
        // what was the intetion behind that?
        // something to do with the leader board.
        // assuming the number of nodes that exist
        //  on the commit leader board
        // self.commit_leader_board_index = index - 1;
        self.commit_leader_board_index = index;
        self.is_unsealed = true;
        self.bid_amount = amount;
    }

    // commit step
    pub fn add_commit(&mut self) {
        self.is_commit = true;
    }

    // bid_refund flag
    pub fn bid_refunded(&mut self) {
        self.is_bid_refunded = true;
    }

    // stake_unlocked flag
    pub fn stake_unlocked(&mut self) {
        self.is_stake_unlocked = true;
    }

    // VALIDATIONS:
    //  secret: = session + secret_pass_phrase
    pub fn is_valid_unsealed_bid(&self, amount: u64, secret: [u8; 32]) -> bool {
        let mut hasher = Hasher::default();
        hasher.hash(amount.to_string().as_ref());
        hasher.hash(self.owner.as_ref());
        hasher.hash(secret.as_ref());

        let hash = hasher.result();
        let commit = Pubkey::new_from_array(hash.to_bytes());

        return !(commit == self.commit_hash);
    }
}
