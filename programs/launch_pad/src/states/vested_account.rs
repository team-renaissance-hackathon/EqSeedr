use crate::{
    states::Session,
    utils::{BYTE, DISCRIMINATOR, MAX_ROUNDS, SIGNED_64, UNSIGNED_32, UNSIGNED_64},
};
use anchor_lang::{prelude::*, solana_program::pubkey::PUBKEY_BYTES};

#[account]
pub struct VestedConfigBySession {
    // VALIDATION STATE
    pub bump: u8,
    pub session_id: Pubkey,
    pub token_mint: Pubkey,

    // TRACKING STATE
    pub stats_by_session: Stats,
    pub rounds: [Round; MAX_ROUNDS as usize],
}

impl VestedConfigBySession {
    pub const LEN: usize = DISCRIMINATOR
        + BYTE
        + PUBKEY_BYTES
        + PUBKEY_BYTES
        + Stats::LEN
        + (Round::LEN * MAX_ROUNDS as usize);

    // some data is just place holders. will need to implement correctly later.
    pub fn initialize(&mut self, bump: u8, session: Account<Session>, token_mint: Pubkey) {
        self.bump = bump;
        self.session_id = session.key();
        self.token_mint = token_mint;

        // what is the init date of the vesting period?
        self.stats_by_session.init_date;
        self.stats_by_session.init_slot;

        self.stats_by_session.total_vested_accounts = 0;
        self.stats_by_session.claimed_vested_accounts = 0;

        self.stats_by_session.token_allocation = session.token_allocation;
        self.stats_by_session.tokens_claimed = 0;

        let mut index = 0;
        while index < MAX_ROUNDS as usize {
            self.rounds[index] = Round {
                round: index as u8 + 1,

                stats: Stats {
                    init_date: 0,
                    init_slot: 0,

                    total_vested_accounts: 0,
                    claimed_vested_accounts: 0,

                    token_allocation: session.allocate_tokens(),
                    tokens_claimed: 0,
                },

                status: LockedStatus {
                    maturity_date: 0,
                    maturity_slot: 0,

                    maturity_date_delta: 0,
                    maturity_slot_delta: 0,
                },
            };

            index += 1;
        }
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
}

// // VestedAccountByIndex makes investors of a sesssion queirable
// #[account]
// pub struct VestedAccountByIndex {
//     // VALIDATION STATE
//     pub bump: u8,
//     pub index: u32,
//     // is authority the right name?
//     pub authority: Pubkey,
//     pub vested_config: Pubkey,
//     // or indexer?
//     // the indexer can be use to search for the session key
//     // if the indexer is known can be used to query the mint and session key and match them
//     // if the indexer and mint is known, can be used to derived the session key
//     pub session: Pubkey,
// }

// impl VestedAccountByIndex {
//     fn initialize(&mut self, bump: u8, auhtority: Pubkey, session: Session) {
//         self.bump = bump;
//         self.index;
//         self.authority = authority;
//         self.session_address = session;
//     }
// }

// #[account]
// pub struct VestedAccountbyOwner {
//     // VALIDATION STATE
//     pub bump: u8,
//     pub index: u32,
//     pub authorty: Pubkey,
//     pub vested_config: Pubkey,
//     pub session: Pubkey, // or indexer?
//     pub session_status: VestedSession,
//     pub round_status: [VestedRounds; 10],
// }

// pub struct VestedSession {
//     pub total_tickets: u64,
//     pub bid_sum: u64,
//     // computeable state
//     //  - cost_basis / bid_average
// }

// pub struct VestedRounds {
//     pub round: u8,
//     pub is_vested: bool,
//     pub is_claimed: bool,
//     pub total_tickets: u64,
//     pub bid_sum: u64,
//     // computeable state
//     //  - cost_basis / bid_average

//     // this has to connect / sync with the tick bid leader baord
//     // describe the user flow
//     // investor executes bid at current market tick / value
//     // accounts and inputs are validated, input is sanitized
//     // token is transfered from user token account to session token account -> USDC | SOL?
//     // compute the changed values
//     //      - input values:
//     //          tick depth
//     //          market value
//     //          ticket amount +1
//     //      - computed values
//     //          fill amount of bonus bag -> most intensive algorithm of the sytem is applied here. also with the MatchBid | OpenBid | ExecuteBid
//     //          average price of current token distribution
//     //          average price of user token accumulation
//     //
//     // state is updated
//     //  session contract
//     //  tick bid round contract
//     //  tick bid leader board contract
//     //  vested account by owner
//     //  vested config contract?
//     //
// }

// impl VestedRounds {
//     fn cost_basis(&self) -> u64 {
//         return self.bid_sum / self.total_tickets;
//     }
// }

// leader board is not part of the OpenBid | ExecutionBid | MatchBid instruction, but will be part of hte transaction
// TRANSACTION: ITX[]
//  ITX: OpenBid | ExecutionBid | MatchBid
//  ITX: UpdateLeaderBoard

// sum / total / accumulation::
// bid
// tickets

// average / cost basis
// bid
