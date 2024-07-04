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
    pub current_round: u8,
    pub launch_status: SessionStatus,

    // tracking state
    pub is_claimed: bool,

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
    pub total_tokens: u64,
    pub market_value: u64,

    // pub bonus_pool: u64,

    // INITIALIZED STATE CONTRACTS:
    pub has_sealed_bid_round: bool,
    pub has_marketplace_positions: bool,
    pub has_vested_config: bool,
    pub has_tick_bid_leader_board: bool,
    pub has_commit_leader_board: bool,
    pub has_commit_queue: bool,
    pub has_max_rounds: bool,
    pub has_valid_commit_bid_vault: bool,

    pub tick_bid_leader_board_current_allocation: u64,
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

        + BOOL

        + SIGNED_64
        + SIGNED_64
        + SIGNED_64

        // + SIGNED_32
        // + SIGNED_32

        + UNSIGNED_32
        + UNSIGNED_32
        + UNSIGNED_64
        + UNSIGNED_64
        + (BOOL * 7);

    pub fn initialize(
        &mut self,
        bump: u8,
        authority: Pubkey,
        indexer: Indexer,
        token_mint: Pubkey,
        session: Pubkey,
        input: SessionParams,
    ) -> Result<()> {
        let clock = Clock::get()?;

        self.bump = bump;
        self.id = indexer.clone();
        self.authority = authority;

        self.token_mint = token_mint.key();
        self.token_name = input.token_name;

        // not being set atm.
        // self.staking_mint;
        // self.staking_account;
        // self.staking_amount = 0;

        self.intialized_timestamp = clock.unix_timestamp;
        self.initialized_slot = clock.slot;

        self.launch_date = input.launch_date;
        self.token_allocation = input.token_allocation;

        self.total_rounds = 0;
        // self.current_round = 0;
        // for testing. will have a way to set it to 1 later
        self.current_round = 1;

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
        self.has_valid_commit_bid_vault = false;

        self.launch_status = SessionStatus::Enqueue;

        self.is_claimed = false;

        // need implement
        // self.staking_account = staking_account

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

    pub fn update_current_round(&mut self) {
        self.current_round += 1;
    }
    pub fn execute_bid(&mut self, bid: u64, amount: u64) {
        self.number_of_bids += 1;

        self.bid_sum += bid;
        self.total_tokens += amount;

        self.market_value = self.bid_sum / self.total_tokens
    }

    pub fn add_vested_member(&mut self) {
        // total_vested_members -> rename?
        self.total_vested += 1;
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

    pub fn add_tick_bid_leader_board(&mut self) {
        self.has_tick_bid_leader_board = true;
    }

    pub fn add_marketplace_positions(&mut self) {
        self.has_marketplace_positions = true;
    }

    pub fn add_vested_config_by_session(&mut self) {
        self.has_vested_config = true;
    }

    pub fn add_valid_commit_bid_vault(&mut self) {
        self.has_valid_commit_bid_vault = true;
    }

    pub fn allocate_tokens(&self) -> u64 {
        return (self.token_allocation - self.token_allocation / PERCENT_10) / MAX_ROUNDS as u64;
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

    pub fn next_round(&self) -> u8 {
        return self.total_rounds + 1;
    }

    pub fn increment_round(&mut self) {
        self.total_rounds += 1;

        if self.total_rounds == MAX_ROUNDS {
            self.has_max_rounds = true;
        }
    }

    pub fn is_valid_staking_account(&self, account: Pubkey) -> bool {
        return self.staking_account == account;
    }

    pub fn is_valid_token_mint(&self, token_mint: Pubkey) -> bool {
        return !(self.token_mint == token_mint);
    }

    pub fn claimed_update(&mut self) {
        self.is_claimed = true;
    }

    pub fn is_valid_tick_bid_status(&self) -> bool {
        return self.launch_status == SessionStatus::TickBid;
    }
}

// session should be called instance?
// launchInstance
// launchStatus
// launchInstanceStatus
#[derive(AnchorDeserialize, AnchorSerialize, Clone, PartialEq)]
pub enum SessionStatus {
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
