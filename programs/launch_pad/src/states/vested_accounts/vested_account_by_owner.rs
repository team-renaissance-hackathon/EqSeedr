use crate::utils::{BYTE, DISCRIMINATOR, UNSIGNED_32, UNSIGNED_64};
use anchor_lang::{prelude::*, solana_program::pubkey::PUBKEY_BYTES};

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
            is_vested: false,
            total_tokens: 0,
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

    pub fn update(&mut self, bid: u64, amount: u64, index: u8) {
        self.session_status.bid_sum += bid;
        self.session_status.total_tokens += amount;

        self.round_status[index as usize].bid_sum += bid;
        self.round_status[index as usize].total_tokens += amount;
    }

    pub fn update_vested_by_session(&mut self) {
        self.session_status.is_vested = true;
    }

    pub fn update_vested_by_round(&mut self, index: u8) {
        self.round_status[index as usize].is_vested = true;
    }

    pub fn claimed_updated(&mut self, index: u8) {
        // Subtract the Round Vested Tokens to the SessionVested Tokens
        self.session_status.total_tokens -= self.round_status[index as usize].total_tokens;

        // Set Round Vested Tokens to 0
        self.round_status[index as usize].total_tokens = 0;

        // Set the is_claimed flag to true
        self.round_status[index as usize].is_claimed = true;
    }

    pub fn get_avg_bid_by_round(&self, index: usize) -> (u32, u64) {
        return (self.bid_index, self.round_status[index].cost_basis());
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct VestedSession {
    pub is_vested: bool,
    pub total_tokens: u64,
    pub bid_sum: u64,
    // computeable state
    //  - cost_basis / bid_average
}

impl VestedSession {
    pub const LEN: usize = BYTE + UNSIGNED_64 + UNSIGNED_64;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct VestedRound {
    pub round: u8,
    pub is_vested: bool,
    pub is_claimed: bool,
    pub is_on_leader_board: bool,
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
    const LEN: usize = BYTE + BYTE + BYTE + BYTE + UNSIGNED_64 + UNSIGNED_64;
    pub fn new(round_index: u8) -> Self {
        Self {
            round: round_index,
            is_claimed: false,
            is_vested: false,
            is_on_leader_board: false,
            total_tokens: 0,
            bid_sum: 0,
        }
    }

    pub fn cost_basis(&self) -> u64 {
        return self.bid_sum / self.total_tokens;
    }
}
