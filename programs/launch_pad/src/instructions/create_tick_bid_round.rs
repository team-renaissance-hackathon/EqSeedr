// use crate::states::{TickBidRound, Session};
// use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateSessionTickBidRound<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = TickBidRound::LEN,
        seeds = [
            session.set_round().unwrap().as_bytes().as_ref(),
            session.key().as_ref(),
            b"round-status",
        ],
        bump
    )]
    pub new_tick_bid_round: Account<'info, TickBidRound>,

    #[account(
        mut,
        has_one = authority,
        constraint = !session.all_tick_bid_rounds_set @ ProgramError::SessionTickBidRoundsAlreadyExist

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

    new_tick_bid_round.initialize(session);
    session.set_next_round();

    Ok(())
}
