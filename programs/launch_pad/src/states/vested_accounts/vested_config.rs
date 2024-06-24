use crate::states::Session;
use crate::utils::*;
use anchor_lang::prelude::*;

#[account]
pub struct VestedConfig {
    // VALIDATION STATE
    pub bump: u8,
    pub session: Pubkey,
    pub token_mint: Pubkey,
    pub vested_token_escrow: Pubkey,

    // TRACKING STATE
    pub vested_index: u32,
    pub stats_by_session: Stats,
    pub rounds: [Round; MAX_ROUNDS as usize],
}

impl VestedConfig {
    pub const LEN: usize = DISCRIMINATOR
        + BYTE
        + PUBKEY_BYTES
        + PUBKEY_BYTES
        + PUBKEY_BYTES
        + UNSIGNED_32
        + Stats::LEN
        + (Round::LEN * MAX_ROUNDS as usize);

    // some data is just place holders. will need to implement correctly later.
    pub fn initialize(
        &mut self,
        bump: u8,
        session: &Account<Session>,
        token_mint: Pubkey,
        vested_token_escrow: Pubkey,
    ) {
        self.bump = bump;
        self.session = session.key();
        self.token_mint = token_mint;
        self.vested_token_escrow = vested_token_escrow;

        // pagination
        self.vested_index = 0;

        self.stats_by_session = Stats {
            // currently incorrect date being stored
            init_date: 0,
            // currently incorrect slot being stored
            init_slot: 0,

            // total of vested accounts that have placed a successful bid of the tick bid system
            total_vested_accounts: 0,
            // total of vested accounts that have claimed their locked tokens
            claimed_vested_accounts: 0,

            // total tokens allocated
            token_allocation: session.token_allocation,
            // total tokens that have been claimed
            tokens_claimed: 0,
        };

        self.rounds = [
            Round::new(1, session.allocate_tokens()),
            Round::new(2, session.allocate_tokens()),
            Round::new(3, session.allocate_tokens()),
            Round::new(4, session.allocate_tokens()),
            Round::new(5, session.allocate_tokens()),
            Round::new(6, session.allocate_tokens()),
            Round::new(7, session.allocate_tokens()),
            Round::new(8, session.allocate_tokens()),
            Round::new(9, session.allocate_tokens()),
            Round::new(10, session.allocate_tokens()),
        ];
    }

    pub fn next_index(&self) -> String {
        return (self.vested_index + 1).to_string();
    }

    pub fn update_index(&mut self) {
        self.vested_index += 1;
    }

    pub fn add_vested_member_by_session(&mut self) {
        // should change to total_vested_members
        self.stats_by_session.total_vested_accounts += 1;
    }

    pub fn add_vested_member_by_round(&mut self, index: u8) {
        // should change to total_vested_members
        self.rounds[index as usize].stats.total_vested_accounts += 1;
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Stats {
    pub init_date: i64,
    pub init_slot: u64,

    pub total_vested_accounts: u32,
    pub claimed_vested_accounts: u32,

    pub token_allocation: u64,
    pub tokens_claimed: u64,
}

impl Stats {
    pub const LEN: usize =
        SIGNED_64 + UNSIGNED_64 + UNSIGNED_32 + UNSIGNED_32 + UNSIGNED_64 + UNSIGNED_64;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct LockedStatus {
    pub maturity_date: i64,
    pub maturity_slot: u64,

    pub maturity_date_delta: i64,
    pub maturity_slot_delta: u64,
}

impl LockedStatus {
    pub const LEN: usize = SIGNED_64 + UNSIGNED_64 + SIGNED_64 + UNSIGNED_64;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct Round {
    pub round: u8,
    pub stats: Stats,
    pub status: LockedStatus,
}

impl Round {
    pub const LEN: usize = BYTE + Stats::LEN + LockedStatus::LEN;

    fn new(round_index: u8, token_allocation: u64) -> Self {
        Self {
            round: round_index,

            stats: Stats {
                // when does the init dat get set?
                // when a round closes?
                // currently incorrect date being stored
                init_date: 0,
                // currently incorrect slot being stored
                init_slot: 0,

                // total of vested accounts that have placed a successful bid of the tick bid round
                total_vested_accounts: 0,
                // total of vested accounts that have claimed their locked tokens of specified round
                claimed_vested_accounts: 0,

                // total tokens allocated of specified round
                token_allocation: token_allocation,
                // total tokens that have been claimed of specified round
                tokens_claimed: 0,
            },

            // conditions of claiming tokens
            status: LockedStatus {
                // when does the maturity date get set?
                maturity_date: 0,
                maturity_slot: 0,

                maturity_date_delta: 0,
                maturity_slot_delta: 0,
            },
        }
    }
}

// leader board is not part of the OpenBid | ExecutionBid | MatchBid instruction, but will be part of hte transaction
// TRANSACTION: ITX[]
//  ITX: OpenBid | ExecutionBid | MatchBid
//  ITX: UpdateLeaderBoard

// sum / total / accumulation::
// bid
// tickets

// average / cost basis
// bid
