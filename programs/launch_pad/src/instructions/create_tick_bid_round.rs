use crate::{
    states::{Session, TickBidRound},
    // utils::ProgramError,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateSessionTickBidRound<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = TickBidRound::LEN,
        seeds = [
            session.next_round().to_string().as_ref(),
            session.key().as_ref(),
            b"tick-bid-round",
        ],
        bump
    )]
    pub new_tick_bid_round: Account<'info, TickBidRound>,

    #[account(
        mut,
        has_one = authority,
        constraint = !session.has_max_rounds
        //  @ ProgramError::SessionTickBidRoundMaxRoundSet,

    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateSessionTickBidRound>) -> Result<()> {
    let CreateSessionTickBidRound {
        new_tick_bid_round,
        session,
        ..
    } = ctx.accounts;

    new_tick_bid_round.initialize(ctx.bumps.new_tick_bid_round, session);
    session.increment_round();

    Ok(())
}
