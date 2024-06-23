use crate::{
    states::{CommitLeaderBoard, SealedBidRound, Session},
    utils::errors::ProgramError,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ReallocateCommitLeaderBoard<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = sealed_bid_round.is_valid_commit_leader_board_allocation()
        @ ProgramError::SessionCommitLeaderBoardMaxAllocation,
        seeds = [
            session.key().as_ref(),
            b"commit-leader-board",
        ],
        bump,
        realloc = sealed_bid_round.commit_leader_board_realloc(),
        realloc::zero = false,
        realloc::payer = authority,
    )]
    pub commit_leader_board: Box<Account<'info, CommitLeaderBoard>>,

    #[account(
        mut,
        constraint = sealed_bid_round.session == session.key()
          @ ProgramError::InvalidSealedBidRound,
        has_one = authority,

    )]
    pub sealed_bid_round: Account<'info, SealedBidRound>,

    #[account(
        mut,
        has_one = authority,
    )]
    pub session: Account<'info, Session>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ReallocateCommitLeaderBoard>) -> Result<()> {
    let ReallocateCommitLeaderBoard {
        sealed_bid_round, ..
    } = ctx.accounts;

    sealed_bid_round.update_commit_leader_board_allocation();

    Ok(())
}
