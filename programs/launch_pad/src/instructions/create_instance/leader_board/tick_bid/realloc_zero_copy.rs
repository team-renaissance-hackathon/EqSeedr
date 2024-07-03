use crate::states::Session;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ReallocZeroCopy<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            session.key().as_ref(),
            b"tick-bid-leader-board",
        ],
        bump,
    )]
    /// CHECKED: reacllocating leader baord account
    pub new_leader_board: AccountInfo<'info>,

    #[account(
        mut,
        constraint = !session.has_tick_bid_leader_board,
    )]
    pub session: Account<'info, Session>,
}

pub fn handler(ctx: Context<ReallocZeroCopy>) -> Result<()> {
    let ReallocZeroCopy {
        new_leader_board,
        session,
        ..
    } = ctx.accounts;

    let space = session.tick_bid_leader_board_current_allocation + 10240;
    session.tick_bid_leader_board_current_allocation = space;

    new_leader_board.realloc(space as usize, false)?;

    Ok(())
}
