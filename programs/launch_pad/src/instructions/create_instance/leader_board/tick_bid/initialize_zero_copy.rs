use crate::states::{LeaderBoard, Session};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeZeroCopy<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        zero,
        seeds = [
            session.key().as_ref(),
            b"tick-bid-leader-board",
        ],
        bump,
    )]
    pub new_leader_board: AccountLoader<'info, LeaderBoard>,

    #[account(
        mut,
        constraint = !session.has_tick_bid_leader_board,
    )]
    pub session: Account<'info, Session>,
}

pub fn handler(ctx: Context<InitializeZeroCopy>) -> Result<()> {
    ctx.accounts.new_leader_board.load_init()?;

    ctx.accounts.session.has_tick_bid_leader_board = true;

    Ok(())
}
