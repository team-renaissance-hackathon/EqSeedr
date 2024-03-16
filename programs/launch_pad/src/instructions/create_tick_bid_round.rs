use crate::states::{RoundStatus, Session};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateRound<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = RoundStatus::LEN,
        seeds = [
            session.set_round().unwrap().as_bytes().as_ref(),
            session.key().as_ref(),
            b"round-status",
        ],
        bump
    )]
    pub new_tick_bid_round: Account<'info, RoundStatus>,

    #[account(
        mut,
        has_one = authority,
    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateRound>) -> Result<()> {
    let CreateRound { round, session, .. } = ctx.accounts;

    round.init();
    session.create_round();

    Ok(())
}
