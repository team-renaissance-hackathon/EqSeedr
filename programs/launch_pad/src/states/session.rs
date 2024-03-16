use crate::utils::{errors::ProgramError, *};
use anchor_lang::prelude::*;

#[account]
// configuration and status of a given session
pub struct Session {
    // idendifier
    pub id: Indexer,

    // TODO: set the authority
    pub authority: Pubkey,

    // token
    pub token_mint: Pubkey,
    pub token_name: String,

    // session
    pub token_allocation: u64,
    pub ticket_allocation: u64,
    pub target_rounds: u8,
    pub launch_status: Status,
    pub has_marketplace: bool,

    // dates
    pub intialized_timestamp: i64,
    pub launch_date: i64,

    // have this in a separate state?
    // data that changes over time
    pub total_bidders: u32,
    // don't think this makes sense or need this
    pub session_tick: u32,

    pub round: u8,
}

impl Session {
    pub const LEN: usize = DISCRIMINATOR
        + Indexer::LEN
        + PUBKEY_BYTES
        + MAX_TEXT_BYTES
        + UNSIGNED_64
        + UNSIGNED_8
        + Status::LEN
        + SIGNED_64
        + SIGNED_64
        + SIGNED_32
        + SIGNED_32;

    // CloseRoundStatus
    pub fn close_round(&self) -> Result<()> {
        // self.current_round += 1;
        // redistribute bag
        // update round bag
        // update session bag
        // log event
        Ok(())
    }

    pub fn set_round(&mut self) -> Result<u8> {
        if !(self.round < self.target_rounds) {
            return err!(ProgramError::MaxRoundSet);
        }

        let round = self.round;
        self.round += 1;

        Ok(round)
    }
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub enum Status {
    Enqueue,
    SealBid,
    TickBid,
    Closed,
}

impl Status {
    pub const LEN: usize = 1;
}
