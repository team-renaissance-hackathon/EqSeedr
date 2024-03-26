use crate::{
    states::Session,
    utils::{BYTE, DISCRIMINATOR, MAX_ROUNDS, SIGNED_64, UNSIGNED_32, UNSIGNED_64, UNSIGNED_8},
};
use anchor_lang::{prelude::*, solana_program::pubkey::PUBKEY_BYTES};

#[account]
pub struct VestedConfigBySession {
    // VALIDATION STATE
    pub bump: u8,
    pub session: Pubkey,
    pub token_mint: Pubkey,

    // TRACKING STATE
    pub vested_index: u32,
    pub stats_by_session: Stats,
    pub rounds: [Round; MAX_ROUNDS as usize],
}

impl VestedConfigBySession {
    pub const LEN: usize = DISCRIMINATOR
        + BYTE
        + PUBKEY_BYTES
        + PUBKEY_BYTES
        + UNSIGNED_32
        + Stats::LEN
        + (Round::LEN * MAX_ROUNDS as usize);

    // some data is just place holders. will need to implement correctly later.
    pub fn initialize(&mut self, bump: u8, session: &Account<Session>, token_mint: Pubkey) {
        self.bump = bump;
        self.session = session.key();
        self.token_mint = token_mint;

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
                maturity_date: 0,
                maturity_slot: 0,

                maturity_date_delta: 0,
                maturity_slot_delta: 0,
            },
        }
    }
}

// VestedAccountByIndex makes investors of a sesssion queirable
#[account]
pub struct VestedAccountByIndex {
    // VALIDATION STATE
    pub bump: u8,
    pub session: Pubkey,
    pub vested_config: Pubkey,
    pub owner: Pubkey,
    pub bid_index: u32,
}

impl VestedAccountByIndex {
    pub fn initialize(
        &mut self,
        bump: u8,
        bid_index: u32,
        owner: Pubkey,
        session: Pubkey,
        vested_config: Pubkey,
    ) {
        self.bump = bump;
        self.bid_index = bid_index;
        self.owner = owner;
        self.session = session;
        self.vested_config = vested_config;
    }
}

impl VestedAccountByIndex {
    pub const LEN: usize =
        DISCRIMINATOR + UNSIGNED_8 + PUBKEY_BYTES + PUBKEY_BYTES + PUBKEY_BYTES + UNSIGNED_32;
}

#[account]
pub struct VestedAccountByOwner {
    // VALIDATION STATE
    pub bump: u8,
    pub session: Pubkey,
    pub vested_config: Pubkey,
    pub owner: Pubkey,
    pub bid_index: u32,

    // TRACKING STATE
    pub session_status: VestedSession,
    pub round_status: [VestedRound; 10],
}

impl VestedAccountByOwner {
    pub const LEN: usize = DISCRIMINATOR
        + BYTE
        + PUBKEY_BYTES
        + PUBKEY_BYTES
        + PUBKEY_BYTES
        + UNSIGNED_32
        + VestedSession::LEN
        + (VestedRound::LEN * 10);

    pub fn initialize(
        &mut self,
        bump: u8,
        bid_index: u32,
        owner: Pubkey,
        session: Pubkey,
        vested_config: Pubkey,
    ) {
        self.bump = bump;
        self.bid_index = bid_index;
        self.owner = owner;
        self.vested_config = vested_config;
        self.session = session;

        self.session_status = VestedSession {
            total_tickets: 0,
            bid_sum: 0,
        };

        self.round_status = [
            VestedRound::new(1),
            VestedRound::new(2),
            VestedRound::new(3),
            VestedRound::new(4),
            VestedRound::new(5),
            VestedRound::new(6),
            VestedRound::new(7),
            VestedRound::new(8),
            VestedRound::new(9),
            VestedRound::new(10),
        ]
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct VestedSession {
    pub total_tickets: u64,
    pub bid_sum: u64,
    // computeable state
    //  - cost_basis / bid_average
}

impl VestedSession {
    pub const LEN: usize = UNSIGNED_64 + UNSIGNED_64;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct VestedRound {
    pub round: u8,
    pub is_vested: bool,
    pub is_claimed: bool,
    pub total_tokens: u64,
    pub bid_sum: u64,
    // computeable state
    //  - cost_basis / bid_average

    // this has to connect / sync with the tick bid leader baord
    // describe the user flow
    // investor executes bid at current market tick / value
    // accounts and inputs are validated, input is sanitized
    // token is transfered from user token account to session token account -> USDC | SOL?
    // compute the changed values
    //      - input values:
    //          tick depth
    //          market value
    //          ticket amount +1
    //      - computed values
    //          fill amount of bonus bag -> most intensive algorithm of the sytem is applied here. also with the MatchBid | OpenBid | ExecuteBid
    //          average price of current token distribution
    //          average price of user token accumulation
    //
    // state is updated
    //  session contract
    //  tick bid round contract
    //  tick bid leader board contract
    //  vested account by owner
    //  vested config contract?
    //
}

impl VestedRound {
    const LEN: usize = BYTE + BYTE + BYTE + UNSIGNED_64 + UNSIGNED_64;
    pub fn new(round_index: u8) -> Self {
        Self {
            round: round_index,
            is_claimed: false,
            is_vested: false,
            total_tokens: 0,
            bid_sum: 0,
        }
    }

    pub fn cost_basis(&self) -> u64 {
        return self.bid_sum / self.total_tokens;
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
