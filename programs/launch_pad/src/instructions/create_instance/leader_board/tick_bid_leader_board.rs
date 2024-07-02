use crate::{
    states::{Session, TickBidLeaderBoard},
    // utils::ProgramError,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateSessionTickBidLeaderBoard<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = TickBidLeaderBoard::LEN,
        seeds = [
            session.key().as_ref(),
            b"tick-bid-leader-board",
        ],
        bump
    )]
    pub new_tick_bid_leader_board: Account<'info, TickBidLeaderBoard>,

    #[account(
        mut,
        has_one = authority,
        constraint = !session.has_tick_bid_leader_board 
        // @ ProgramError::SessionTickBidLeaderBoardAlreadyExist
    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateSessionTickBidLeaderBoard>) -> Result<()> {
    let CreateSessionTickBidLeaderBoard {
        new_tick_bid_leader_board,
        session,
        ..
    } = ctx.accounts;

    new_tick_bid_leader_board.initialize(ctx.bumps.new_tick_bid_leader_board, session.key());
    session.add_tick_bid_leader_board();

    Ok(())
}

// transfer
// allocate
// reallocate to desired size
