use anchor_lang::{
    prelude::*,
    solana_program::{hash::Hasher, pubkey::PUBKEY_BYTES},
};

use crate::utils::{BOOL, BYTE, DISCRIMINATOR, UNSIGNED_32, UNSIGNED_64};

#[account]
pub struct SealedBidByIndex {
    // VALIDATION STATE
    pub bump: u8,
    pub index: u32,
    pub session: Pubkey,
    pub owner: Pubkey,

    // STATE
    pub commit_hash: Pubkey, // technially a hash [u8; 32]
    pub staked_amount: u64,
    pub is_unsealed: bool,
}

impl SealedBidByIndex {
    pub const LEN: usize =
        DISCRIMINATOR + BYTE + UNSIGNED_32 + PUBKEY_BYTES + PUBKEY_BYTES + UNSIGNED_64 + BOOL;

    pub fn initialize(
        &mut self,
        bump: u8,
        index: u32,
        session: Pubkey,
        owner: Pubkey,
        amount: u64,
        commit_hash: Pubkey,
    ) {
        self.bump = bump;
        self.index = index;
        self.session = session;
        self.owner = owner;

        self.commit_hash = commit_hash;
        self.staked_amount = amount;

        self.is_unsealed = false;

        // emit event
    }

    // VALIDATIONS:
    //  secret: = session + secret_pass_phrase
    pub fn is_valid_unsealed_bid(&self, amount: u64, secret: [u8; 32]) -> bool {
        let mut hasher = Hasher::default();
        hasher.hash(amount.to_string().as_ref());
        hasher.hash(self.owner.as_ref());
        hasher.hash(secret.as_ref());

        let hash = hasher.result();
        return Pubkey::new_from_array(hash.to_bytes()) == self.commit_hash;
    }

    pub fn unsealed_bid(&mut self) {
        self.is_unsealed = true;
    }
}
