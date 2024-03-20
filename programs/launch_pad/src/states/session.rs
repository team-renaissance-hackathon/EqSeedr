use crate::{instructions::SessionParams, utils::*};
use anchor_lang::prelude::*;

// configuration and status of a given session
#[account]
pub struct Session {
    // VALIDATION STATE
    pub bump: u8,
    // id / idendifier / index / indexer
    pub id: Indexer,

    // TODO: set the authority
    pub authority: Pubkey,

    // token
    pub token_mint: Pubkey,
    pub token_name: String,

    // staking
    pub staking_mint: Pubkey,
    pub staking_account: Pubkey,
    // an algorithm will set this. current just set it to a value of $100?
    pub staking_amount: u64,

    // session
    pub token_allocation: u64,
    pub total_rounds: u8, // incremental to 10, starts at 1
    launch_status: Status,

    // dates
    pub intialized_timestamp: i64,
    pub initialized_slot: u64,
    pub launch_date: i64,

    // have this in a separate state?
    // data that changes over time
    // we can track this in the leader board
    pub total_vested: u32,
    pub number_of_bids: u32,

    // total funds raised in USDC | SOL
    pub bid_sum: u64,

    // INITIALIZED STATE CONTRACTS:
    pub has_sealed_bid_round: bool,
    pub has_marketplace_positions: bool,
    pub has_vested_config: bool,
    pub has_tick_bid_leader_board: bool,
    pub has_commit_leader_board: bool,
    pub has_commit_queue: bool,
    pub has_max_rounds: bool,
}

impl Session {
    pub const LEN: usize = DISCRIMINATOR
        + Indexer::LEN
        + PUBKEY_BYTES
        + PUBKEY_BYTES
        + MAX_TEXT_BYTES
        + PUBKEY_BYTES
        + PUBKEY_BYTES

        + UNSIGNED_64
        + UNSIGNED_64
        + UNSIGNED_8
        + Status::LEN

        + SIGNED_64
        + SIGNED_64
        + SIGNED_64

        // + SIGNED_32
        // + SIGNED_32

        + UNSIGNED_32
        + UNSIGNED_32
        + UNSIGNED_64
        + (BOOL * 7);

    pub fn initialize(
        &mut self,
        authority: Pubkey,
        indexer: Indexer,
        token_mint: Pubkey,
        session: Pubkey,
        input: SessionParams,
    ) -> Result<()> {
        let clock = Clock::get()?;

        self.id = indexer.clone();
        self.token_mint = token_mint.key();
        self.authority = authority;

        self.intialized_timestamp = clock.unix_timestamp;
        self.initialized_slot = clock.slot;

        self.launch_date = input.launch_date;
        self.token_name = input.token_name;
        self.token_allocation = input.token_allocation;

        self.total_rounds = 0;
        self.total_vested = 0;
        self.number_of_bids = 0;
        self.bid_sum = 0;

        self.has_sealed_bid_round = false;
        self.has_marketplace_positions = false;
        self.has_vested_config = false;
        self.has_tick_bid_leader_board = false;
        self.has_commit_leader_board = false;
        self.has_commit_queue = false;
        self.has_max_rounds = false;

        msg!("TESTING");

        // this doesn't work.
        emit!(NewSession {
            message: String::from("New Session Created"),
            launch_date: self.launch_date,
            session_account: session.key().clone(),
            session_indexer: self.id.clone(),
        });

        Ok(())
    }

    pub fn add_sealed_bid_round(&mut self) {
        self.has_sealed_bid_round = true;
    }

    pub fn add_commit_leader_board(&mut self) {
        self.has_commit_leader_board = true;
    }

    pub fn add_commit_queue(&mut self) {
        self.has_commit_queue = true;
    }

    // CloseRoundStatus
    pub fn close_round(&self) -> Result<()> {
        // self.current_round += 1;
        // redistribute bag
        // update round bag
        // update session bag
        // log event
        Ok(())
    }

    pub fn next_round(&self) -> String {
        return (self.total_rounds + 1).to_string();
    }

    pub fn increment_round(&mut self) {
        if self.total_rounds == MAX_ROUNDS {
            self.has_max_rounds = true;
        }

        self.total_rounds += 1;
    }

    pub fn is_valid_staking_account(&self, account: Pubkey) -> bool {
        return self.staking_account == account;
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
enum Status {
    Enqueue,
    SealBid,
    TickBid,
    Closed,
    Canceled,
}

impl Status {
    pub const LEN: usize = 1;
}

#[event]
pub struct NewSession {
    message: String,
    launch_date: i64,
    session_account: Pubkey,
    session_indexer: Indexer,
}
