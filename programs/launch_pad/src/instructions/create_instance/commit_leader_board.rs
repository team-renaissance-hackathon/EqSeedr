use crate::{
    states::{CommitLeaderBoard, SealedBidRound, Session},
    utils::errors::ProgramError,
    utils::*,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateCommitLeaderBoard<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = MAX_STATE_ALLOCATION,
        seeds = [
            session.key().as_ref(),
            b"commit-leader-board",
        ],
        bump
    )]
    pub new_commit_leader_board: Box<Account<'info, CommitLeaderBoard>>,

    #[account(
        mut,
        has_one = authority,
    )]
    pub sealed_bid_round: Account<'info, SealedBidRound>,

    #[account(
        mut,
        has_one = authority,
        constraint = !session.has_commit_leader_board 
            @ ProgramError::SessionCommitLeaderBoardAlreadyExist,
    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateCommitLeaderBoard>) -> Result<()> {
    let CreateCommitLeaderBoard {
        new_commit_leader_board,
        session,
        sealed_bid_round,
        ..
    } = ctx.accounts;

    new_commit_leader_board.initialize(ctx.bumps.new_commit_leader_board, sealed_bid_round.key());
    session.add_commit_leader_board();
    sealed_bid_round.update_commit_leader_board_allocation();

    Ok(())
}

// transfer
// assign
// reallocate
// size -> 10240 * 5 = 102400 / 2

// (1 + 32 + 8 + n)
// n = (4 + 4 + 4 + (4 + 1 + c) + (4 + ( b * 3)))
// c = (4 + (1 + 4) + (1 + 4) + (8 + 4)) = 26

// (1 + 32 + 8 + n)
// n = (2 + 2 + 2 + (4 + 1 + c) + (4 + ( b * 2)))
// c = (2 + (1 + 2) + (1 + 2) + (2 + 4)) = 14
